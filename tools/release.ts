import moment from 'npm:moment';
import ora from 'npm:ora';
import chalk from 'npm:chalk';
import * as path from "https://deno.land/std/path/mod.ts";
import { Hash } from "https://deno.land/std@0.110.0/node/crypto.ts";

enum TargetType {
    Library,
    Executable,
}

type Target = {
    type: TargetType;
    name: string;
    path: string;
}

const targetedArchs = [

    //ARM
    "aarch64-apple-darwin",
    "aarch64-unknown-linux-gnu",
    "aarch64-pc-windows-msvc",

    //i686
    "i686-pc-windows-msvc",
    "i686-unknown-linux-gnu",

    //x86
    "x86_64-apple-darwin",
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc",
    "x86_64-unknown-redox"
    //See all targets by running `rustup target list`
]

const targets: Target[] = [
    {
        type: TargetType.Executable,
        name: "elliec",
        path: "./elliec",
    },
    {
        type: TargetType.Executable,
        name: "elliefmt",
        path: "./elliefmt",
    },
    {
        type: TargetType.Executable,
        name: "ellievm",
        path: "./ellievm",
    },
    /* 
    {
        type: TargetType.Library,
        name: "ellie_engine",
        path: "./",
    }
    */
]

let startTime = moment();

console.clear();
const spinner = ora({
    text: 'Checking available architectures',
    spinner: 'dots8Bit',
}).start();

async function resolveHash(file: string) {
    const fileBuffer = await Deno.readFile(file);
    const hashSum = new Hash('sha256');
    hashSum.update(fileBuffer);
    const hex = hashSum.digest('hex');
    return `${hex}: ${file}`;
}

async function buildTargetedArch(target: Target, arch: string) {
    const currentDir = Deno.cwd();
    const targetDir = path.join(currentDir, target.name);

    await Deno.chdir(targetDir);
    const command = new Deno.Command('cargo', {
        args: [
            "build",
            "--release",
            "--quiet",
            "--target",
            arch,
        ]
    });

    const output = await command.output();

    if (!output.success) {
        const stderr = new TextDecoder().decode(output.stderr);
        spinner.fail("Failed to build targeted arch");
        spinner.info(stderr);
        Deno.exit(1);
    }
    await Deno.chdir(currentDir);
}


async function moveBuild(target: string, arch: string, extension: string): Promise<string> {
    const outputPath = path.join(Deno.cwd(), `/ellieRelease/${target}_${arch.replaceAll('-', '_')}${extension}`)
    await Deno.rename(
        path.join(Deno.cwd(), `/${target}/target/${arch}/release/${target}${extension}`),
        outputPath
    );
    return outputPath;
}

async function installArch(arch: string) {
    const command = new Deno.Command("rustup", { args: ["target", "add", arch] });
    const output = await command.output();
    if (output.success) {
        const stdout = new TextDecoder().decode(output.stdout);
        return stdout;
    } else {
        spinner.fail(`Failed to install target: ${arch}`);
        Deno.exit(1);
    }
}

async function installMissingArchs(missingList: string[]) {
    let newlyInstalledArchs = 0;
    for (const arch of missingList) {
        spinner.text = `Installing '${chalk.yellow(arch)}'\n`;
        spinner.start();
        try {
            const response = await installArch(arch);
            spinner.stop();
            if (response.toString().includes('is up to date')) {
                console.log(chalk.cyan('ℹ') + " '" + chalk.yellow(arch) + "' is up to date");
            } else {
                console.log(chalk.green('✔') + " '" + chalk.yellow(arch) + "' installed");
                newlyInstalledArchs++;
            }
        } catch (err) {
            spinner.stop();
            spinner.fail(chalk.red(`Failed to install ${arch}\n${chalk.red(err)}`));
            Deno.exit(1);
        }
    }
    return newlyInstalledArchs;
}

async function checkInstalledTargets(): Promise<string[]> {
    const command = new Deno.Command("rustup", { args: ["target", "list", "--installed"] });
    const output = await command.output();

    if (output.success) {
        const stdout = new TextDecoder().decode(output.stdout);
        const availableArchs = stdout.split('\n');
        const missingArchs = targetedArchs.filter(
            (arch) => !availableArchs.includes(arch)
        );
        return missingArchs;
    } else {
        spinner.fail("Failed to list installed rust targets, is rust installed?");
        Deno.exit(1);
    }
}

async function removePreviousBuilds() {
    await Deno.remove(path.join(Deno.cwd(), './ellie_engine/target'), { recursive: true }).catch(() => { })
    await Deno.remove(path.join(Deno.cwd(), './ellie_engine/target'), { recursive: true }).catch(() => { });
    await Deno.remove(path.join(Deno.cwd(), './elliec/target'), { recursive: true }).catch(() => { });
    await Deno.remove(path.join(Deno.cwd(), './elliefmt/target'), { recursive: true }).catch(() => { });
    await Deno.remove(path.join(Deno.cwd(), './ellievm/target'), { recursive: true }).catch(() => { });
    await Deno.remove(path.join(Deno.cwd(), './ellieRelease')).catch(() => { });
}

const missingArchs = await checkInstalledTargets()

spinner.succeed();

if (missingArchs.length > 0) {
    spinner.fail(`Missing archs: ${missingArchs.join(', ')}`);
    spinner.info('Installing missing archs...');
    const newlyInstalledArchs = await installMissingArchs(missingArchs);
    spinner.info(`Missing '${newlyInstalledArchs}' archs installed`);
} else {
    spinner.succeed('All archs are available');
}

spinner.info('Cleaning previous builds');

await removePreviousBuilds();

spinner.info('Previous builds cleaned');

const buildHashes: { [key: string]: string; } = {};

for (const target of targets) {
    //await Deno.rename(src, dest)
    for (const arch of targetedArchs) {
        spinner.text = `Building '${chalk.yellow(target.name)}' in '${chalk.yellow(arch)}'.\n`;
        spinner.start();
        try {
            await buildTargetedArch(target, arch);
            const file = await moveBuild(target.name, arch, Deno.build.os == "windows" ? ".exe" : "")
            const fileHash = await resolveHash(file);
            buildHashes[path.parse(file).name] = fileHash;
            spinner.info("Build complete.")
        } catch (_) {
            await removePreviousBuilds();
            spinner.fail("Build failed.");
            Deno.exit(1)
        }
    }
}
spinner.info("Writing hashes, release complete");

let output = "";

for (const hash of Object.entries(buildHashes)) {
    output += `${hash[0]}: ${hash[1]}\n`;
}

await Deno.writeFile(
    path.join(Deno.cwd(), `/ellieRelease/SHASUMS256.txt`),
    new TextEncoder().encode(output)
)


