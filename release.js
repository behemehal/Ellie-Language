import moment from "moment";
import ora from "ora";
import chalk from "chalk";
import { exec } from "child_process";
import fs from "fs";
import crypto from "crypto";

const targeted_archs = {
  "aarch64-apple-darwin": ["elliec_aarch64_apple_m1", ""],
  //"aarch64-pc-windows-msvc": ["elliec_aarch64_windows", "exe"],
  //"aarch64-unknown-linux-gnu": ["elliec_aarch64_linux", ""],
  //"aarch64-linux-android": ["elliec_aarch64_android", ""],
  //"armv7-linux-androideabi": ["elliec_armv7_android", ""],
  //"armv7-unknown-linux-gnueabi": ["elliec_armv7_gnuabi_linux", ""],
  //"armv7-unknown-linux-gnueabihf": ["elliec_armv7_gnuabihf_linux", ""],
  //"i686-pc-windows-gnu": ["elliec_i686_windows", "exe"],
  //"i686-unknown-linux-gnu": ["elliec_i686_linux", ""],
  //"mips-unknown-linux-gnu": ["elliec_mips_linux", ""],
  //"mips64-unknown-linux-gnuabi64": ["elliec_mips64_linux", ""],
  "x86_64-apple-darwin": ["elliec_x86_64_apple_darwin", ""],
  "x86_64-pc-windows-gnu": ["elliec_x86_64_windows", "exe"],
  "x86_64-unknown-linux-gnu": ["elliec_x86_64_linux", ""],
  //"x86_64-unknown-redox": ["elliec_x86_64_redox", ""],
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
    exec(`mv ${src} ${dest}`,
      (err, stdout, stderr) => {
        if (err) {
          reject(err);
        }
        resolve(stdout);
      }
    );
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
      await moveItem(
        `./target/${arch}/release/elliec${
          target_list[arch][1] != "" ? "." + target_list[arch][1] : ""
        }`,
        `./ellieRelease/${target_list[arch][0]}${target_list[arch][1] != "" ? "." + target_list[arch][1] : ""}`
      );
      var hash = await resolveHash(`./ellieRelease/${target_list[arch][0]}${target_list[arch][1] != "" ? "." + target_list[arch][1] : ""}`);
      SHASUMS.push(hash);
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
        buildTargets(targeted_archs).then((binary_shasums) => {
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

          var shasums = `EllieVersion = v${ellie_version} - ${ellie_ver_code}`;
          shasums += "\n\t" + binary_shasums.join("\n\t");
          fs.writeFileSync("./ellieRelease/SHASUMS256.txt", shasums);
          console.log(
            `${chalk.green("✔")} SHASUM of all ${chalk.yellow(
              Object.keys(targeted_archs).length
            )} builded archs saved in '${chalk.yellow(
              "./release/SHASUMS256.txt"
            )}'`
          );
          spinner.succeed(`All builds finished in '${chalk.yellow(start_time.toNow(true))}'`);
        });
      });
    }
  );
});
