import { exec } from 'node:child_process';
import fs from 'node:fs';
import crypto from 'node:crypto';

const targeted_archs = [
  
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
  "x86_64-pc-windows-msvc"
  //See all targets by running `rustup target list`
]

const targets = [
  {
    type: "binary",
    name: "elliec",
    path: "./elliec",
    features: []
  },
  {
    type: "binary",
    name: "elliefmt",
    path: "./elliefmt",
    features: []
  },
  {
    type: "binary",
    name: "ellievm",
    path: "./ellievm",
    features: []
  },
  {
    type: "library",
    name: "ellie_engine",
    path: "./",
    features: []
  }
]

var start_time = moment();
console.clear();
const spinner = ora('Checking available architectures', {
   spinner: 'dots8Bit',
}).start();

function resolveHash(file) {
   return new Promise((resolve, reject) => {
      const fileBuffer = fs.readFileSync(file);
      const hashSum = crypto.createHash('sha256');
      hashSum.update(fileBuffer);
      const hex = hashSum.digest('hex');
      resolve(`${hex}: ${file}`);
   });
}

function moveItem(src, dest) {
   return new Promise((resolve, reject) => {
      exec(`mv ${src} ${dest}`, (err, stdout, stderr) => {
         if (err) {
            reject(err);
         }
         resolve(stdout);
      });
   });
}

function buildTargetedArch(arch) {
   return new Promise((resolve, reject) => {
      exec(`cargo build --release -q --target ${arch}`, (err, _, stderr) => {
         if (err) {
            reject(stderr);
         }
         resolve(stderr);
      });
   });
}

async function buildTargets(target_list) {
   var SHASUMS = [];
   for (let arch of Object.keys(target_list)) {
      spinner.text = `Building '${chalk.yellow(arch)}'\n`;
      spinner.start();
      try {
         await buildTargetedArch(arch);
         spinner.stop();
         let list = [];
         for (let target of target_list[arch]) {
            const executable_name = target[0].split('_')[0];
            const current = `./target/${arch}/release/${executable_name}${
               target[1] == 'exe' ? '.exe' : ''
            }`;

            const targeted = `./ellieRelease/${target[0]}`;

            await moveItem(current, targeted);
            var hash = await resolveHash(targeted);
            SHASUMS.push({
               arch: target[0].split(executable_name + '_'),
               hash: hash.split(':')[0].trim(),
               file: target[0],
            });
         }

         console.log(chalk.green('✔') + " Arch '" + chalk.yellow(arch) + "' builded");
      } catch (err) {
         spinner.stop();
         spinner.fail(chalk.red(`Failed to build ${arch}\n${chalk.red(err)}`));
         process.exit(1);
      }
   }
   return SHASUMS;
}

function installArch(arch) {
   return new Promise((resolve, reject) => {
      exec(`rustup target add ${arch}`, (err, _, stderr) => {
         if (err) {
            reject(stderr);
         }
         resolve(stderr);
      });
   });
}

async function installMissingArchs(missing_list) {
   var newly_installed_archs = 0;
   for (let arch of missing_list) {
      spinner.text = `Installing '${chalk.yellow(arch)}'\n`;
      spinner.start();
      try {
         let response = await installArch(arch);
         spinner.stop();
         if (response.toString().includes('is up to date')) {
            console.log(chalk.cyan('ℹ') + " '" + chalk.yellow(arch) + "' is up to date");
         } else {
            console.log(chalk.green('✔') + " '" + chalk.yellow(arch) + "' installed");
            newly_installed_archs++;
         }
      } catch (err) {
         spinner.stop();
         spinner.fail(chalk.red(`Failed to install ${arch}\n${chalk.red(err)}`));
         process.exit(1);
      }
   }
   return newly_installed_archs;
}

async function checkInstalledTargets() {
   return new Promise((resolve, reject) => {
      exec('rustup target list --installed', (err, stdout, stderr) => {
         if (err) {
            reject(stderr);
         }
         const available_archs = stdout.split('\n');
         const missing_archs = targeted_archs.filter(
            (arch) => !available_archs.includes(arch)
         );
         resolve(missing_archs);
      });
   });
}

async function cleanDir(directory) {
   return new Promise((resolve, reject) => {
      exec(`cargo clean --target-dir ${directory}`, (err, stdout, stderr) => {
         if (err) {
            reject(err);
         }
         resolve(stdout);
      });
   });
}

function main() {
   checkInstalledTargets()
      .then(async (missing_archs) => {
         spinner.succeed();
         if (missing_archs.length > 0) {
            spinner.fail(`Missing archs: ${missing_archs.join(', ')}`);
            spinner.info('Installing missing archs...');
            let installed_targets = await installMissingArchs(missing_archs);
            spinner.info('Missing archs installed');
         } else {
            spinner.succeed('All archs are available');
         }
         spinner.info('Cleaning previous builds');
         await cleanDir('./');
         await cleanDir('./elliec');
         await cleanDir('./elliefmt');
         await cleanDir('./ellievm');
      })
      .catch((err) => {
         spinner.fail(chalk.red(err));
      });
}
main();
