import moment from "moment";
import ora from "ora";
import chalk from "chalk";
import { exec } from "child_process";
import fs from "fs";
import crypto from "crypto";

const targeted_archs = {
  "aarch64-apple-darwin": [
    ["elliec_aarch64_apple_darwin", ""],
    ["ellievm_aarch64_apple_darwin", ""],
    ["elliefmt_aarch64_apple_darwin", ""],
  ],
  "x86_64-apple-darwin": [
    ["elliec_x86_64_apple_darwin", ""],
    ["ellievm_x86_64_apple_darwin", ""],
    ["elliefmt_x86_64_apple_darwin", ""],
  ],
  "x86_64-pc-windows-gnu": [
    ["elliec_x86_64_windows", "exe"],
    ["ellievm_x86_64_windows", "exe"],
    ["elliefmt_x86_64_windows", "exe"],
  ],
  "x86_64-unknown-linux-gnu": [
    ["elliec_x86_64_linux", ""],
    ["ellievm_x86_64_linux", ""],
    ["elliefmt_x86_64_linux", ""],
  ],
};

var start_time = moment();
console.clear();
const spinner = ora("Checking available architectures", {
  spinner: "dots8Bit",
}).start();

function resolveHash(file) {
  return new Promise((resolve, reject) => {
    const fileBuffer = fs.readFileSync(file);
    const hashSum = crypto.createHash("sha256");
    hashSum.update(fileBuffer);
    const hex = hashSum.digest("hex");
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
        const executable_name = target[0].split("_")[0];
        const current = `./target/${arch}/release/${executable_name}${
          target[1] == "exe" ? ".exe" : ""
        }`;

        const targeted = `./ellieRelease/${target[0]}`;

        await moveItem(current, targeted);
        var hash = await resolveHash(targeted);
        SHASUMS.push({
          arch: target[0].split(executable_name + "_"),
          hash: hash.split(":")[0].trim(),
          file: target[0],
        });
      }

      console.log(
        chalk.green("✔") + " Arch '" + chalk.yellow(arch) + "' builded"
      );
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
      if (response.toString().includes("is up to date")) {
        console.log(
          chalk.cyan("ℹ") + " '" + chalk.yellow(arch) + "' is up to date"
        );
      } else {
        console.log(
          chalk.green("✔") + " '" + chalk.yellow(arch) + "' installed"
        );
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

exec(`rustup target list --installed`, (err, stdout, stderr) => {
  if (err) {
    spinner.fail(chalk.red(err));
    return;
  }
  spinner.succeed();
  const available_archs = stdout.split("\n");
  const missing_archs = Object.keys(targeted_archs).filter(
    (arch) => !available_archs.includes(arch)
  );
  if (missing_archs.length > 0) {
    spinner.fail(chalk.red(`Missing archs: ${missing_archs.join(", ")}`));
  } else {
    spinner.succeed("All archs are available");
  }
  spinner.info(
    missing_archs.length == 0
      ? "No missing archs available but updating existing ones"
      : "Installing missing archs and updating existing ones"
  );
  installMissingArchs(Object.keys(targeted_archs)).then(
    (newly_installed_archs) => {
      if (newly_installed_archs > 0) {
        console.log(
          `${chalk.green("✔")} All ${chalk.yellow(
            newly_installed_archs
          )} missing archs installed`
        );
      } else {
        console.log(chalk.cyan("ℹ") + " All archs are up to date");
      }
      spinner.start();
      exec(`cargo clean`, (err, stdout, stderr) => {
        if (err) {
          spinner.fail(chalk.red(err));
          process.exit(1);
        }
        fs.mkdir("./ellieRelease", (err) => {
          if (err) {
            console.log(
              `${chalk.cyan("ℹ")} Release dir already exists, deleting`
            );
            fs.rm("./ellieRelease", { recursive: true, force: true }, (err) => {
              if (err) {
                spinner.fail(chalk.red(err));
                process.exit(1);
              }
              fs.mkdir("./ellieRelease", (err) => {
                if (err) {
                  spinner.fail(chalk.red(err));
                  process.exit(1);
                }
              });
            });
          }
          console.log(`${chalk.cyan("ℹ")} Release dir created`);
        });
        spinner.succeed("Cleaned previous builds");
        spinner.start();
        console.log(
          `${chalk.cyan("ℹ")} Starting build for '${chalk.yellow(
            Object.keys(targeted_archs).length
          )}' archs`
        );
        buildTargets(targeted_archs).then(async (binary_shasums) => {
          console.log(
            `${chalk.green("✔")} All ${chalk.yellow(
              Object.keys(targeted_archs).length
            )} archs builded`
          );

          var cargoToml = fs.readFileSync("./Cargo.toml", "utf8");
          var endl = cargoToml.includes("\r\n") ? "\r\n" : "\n";
          var ellie_version = cargoToml
            .split(endl)[3]
            .split("=")[1]
            .replaceAll('"', "")
            .trim();
          var ellie_ver_code = cargoToml
            .split(endl)[2]
            .split("=")[1]
            .replaceAll('"', "")
            .trim();

          //Move file to release folder
          await moveItem(
            "./target/elliec_completion_bash",
            "./ellieRelease/elliec_completion_bash"
          );
          await moveItem(
            "./target/elliec_completion_zsh",
            "./ellieRelease/elliec_completion_zsh"
          );
          await moveItem(
            "./target/elliec_completion_fish",
            "./ellieRelease/elliec_completion_fish"
          );
          await moveItem(
            "./target/elliec_completion_powershell",
            "./ellieRelease/elliec_completion_powershell"
          );

          let elliec_completion_powershell_shasum = await resolveHash(
            "./ellieRelease/elliec_completion_powershell"
          );
          let elliec_completion_bash_shasum = await resolveHash(
            "./ellieRelease/elliec_completion_bash"
          );
          let elliec_completion_zsh_shasum = await resolveHash(
            "./ellieRelease/elliec_completion_zsh"
          );
          let elliec_completion_fish_shasum = await resolveHash(
            "./ellieRelease/elliec_completion_fish"
          );

          await moveItem(
            "./target/ellievm_completion_bash",
            "./ellieRelease/ellievm_completion_bash"
          );
          await moveItem(
            "./target/ellievm_completion_zsh",
            "./ellieRelease/ellievm_completion_zsh"
          );
          await moveItem(
            "./target/ellievm_completion_fish",
            "./ellieRelease/ellievm_completion_fish"
          );
          await moveItem(
            "./target/ellievm_completion_powershell",
            "./ellieRelease/ellievm_completion_powershell"
          );

          let ellievm_completion_powershell_shasum = await resolveHash(
            "./ellieRelease/ellievm_completion_powershell"
          );
          let ellievm_completion_bash_shasum = await resolveHash(
            "./ellieRelease/ellievm_completion_bash"
          );
          let ellievm_completion_zsh_shasum = await resolveHash(
            "./ellieRelease/ellievm_completion_zsh"
          );
          let ellievm_completion_fish_shasum = await resolveHash(
            "./ellieRelease/ellievm_completion_fish"
          );

          binary_shasums.push({
            arch: "powershell",
            hash: elliec_completion_powershell_shasum.split(":")[0].trim(),
            file: "elliec_completion_powershell",
          });
          binary_shasums.push({
            arch: "bash",
            hash: elliec_completion_bash_shasum.split(":")[0].trim(),
            file: "elliec_completion_bash",
          });
          binary_shasums.push({
            arch: "zsh",
            hash: elliec_completion_zsh_shasum.split(":")[0].trim(),
            file: "elliec_completion_zsh",
          });
          binary_shasums.push({
            arch: "fish",
            hash: elliec_completion_fish_shasum.split(":")[0].trim(),
            file: "elliec_completion_fish",
          });

          /// ----

          binary_shasums.push({
            arch: "powershell",
            hash: ellievm_completion_powershell_shasum.split(":")[0].trim(),
            file: "ellievm_completion_powershell",
          });
          binary_shasums.push({
            arch: "bash",
            hash: ellievm_completion_bash_shasum.split(":")[0].trim(),
            file: "ellievm_completion_bash",
          });
          binary_shasums.push({
            arch: "zsh",
            hash: ellievm_completion_zsh_shasum.split(":")[0].trim(),
            file: "ellievm_completion_zsh",
          });
          binary_shasums.push({
            arch: "fish",
            hash: ellievm_completion_fish_shasum.split(":")[0].trim(),
            file: "ellievm_completion_fish",
          });

          var shasums = `EllieVersion = v${ellie_version} - ${ellie_ver_code}`;
          shasums +=
            "\n\t" +
            binary_shasums.map((x) => x.hash + " : " + x.file).join("\n\t");
          fs.writeFileSync(
            "./ellieRelease/output.json",
            JSON.stringify(
              binary_shasums.map((x) => {
                return {
                  arch: x.arch,
                  hash: x.hash,
                  file: x.file.split("/").pop(),
                };
              })
            )
          );
          fs.writeFileSync("./ellieRelease/SHASUMS256.txt", shasums);
          console.log(
            `${chalk.green("✔")} SHASUM of all ${chalk.yellow(
              Object.keys(targeted_archs).length
            )} builded archs saved in '${chalk.yellow(
              "./ellieRelease/SHASUMS256.txt"
            )}'`
          );
          spinner.succeed(
            `All builds finished in '${chalk.yellow(start_time.toNow(true))}'`
          );
        });
      });
    }
  );
});
