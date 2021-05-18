var chalk = require('chalk');
var fs = require("fs");

var log = (message, color) => {
    console.log(`${chalk.cyan("Ellie")}: ${chalk[color || "red"](message)}`);
}

var createLine = (len) => Array(len).fill("-").join("");

function refactorFile(file, fileDir) {
    var lines = file.split("\r\n");
    var factoredFile = "";
    var factored = false;
    for (var i = 0; i < lines.length; i++) {
        var line = lines[i];
        if (line.includes("debug_message: \"") && fileDir != "./core/src/error/mod.rs") {
            var first = line.split("debug_message: \"")[0];
            factoredFile += first + "debug_message: \"" + fileDir + ":" + i + "\".to_string(),\n"
            factored = true;
        } else {
            factoredFile += line + "\n";
        }
    }
    if (factored) {
        log(`Factoring ${fileDir}:${i + 1}`);
        fs.writeFileSync(fileDir, factoredFile, 'utf8');
    }
}

function scanDirectory(dir, path) {
    var files = [];
    return new Promise(async (resolve) => {
        for (var i = 0; i < dir.length; i++) {
            if (dir[i].isFile()) {
                if (dir[i].name.split(".")[1] == "rs") {
                    files.push(path + dir[i].name);
                }
            } else if (dir[i].isDirectory()) {
                q = await scanDirectory(fs.readdirSync(path + dir[i].name, {withFileTypes: true}), path + dir[i].name + "/");
                files = [...q, ...files];
            }
        }
        resolve(files);
    });
}

log("Searching Errors");
scanDirectory(fs.readdirSync("./", {withFileTypes: true}), "./").then((files) => {
    log(`Factoring ${files.length} files`);
    
    for (let i = 0; i < files.length; i++) {
        refactorFile(fs.readFileSync(files[i], "utf-8"), files[i]);
    }

})