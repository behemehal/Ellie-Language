var fs = require("fs");
var os = require("os");
var crypto = require("crypto");

var log = (message, color) => {
    console.log(`Ellie: ${message}`);
}

var createLine = (len) => Array(len).fill("-").join("");

function createDebugLabel() {
    const id = crypto.randomBytes(16).toString("hex");
    return id
}

function whichLineEnding(source) {
    var temp = source.indexOf('\n');
    if (source[temp - 1] === '\r')
        return '\r\n'
    return "\n"
}

var cargoconf = fs.readFileSync("./Cargo.toml", "utf8");
var ellidbg = fs.readFileSync("./DEBUG_HEADERS.eidbg", "utf8");
var version = cargoconf.split(whichLineEnding(cargoconf)).find(x => x.split("=")[0].trim() == "version").split("=")[1].trim().replace(new RegExp("\"", "g"), "");
var dbgversion = ellidbg.split(whichLineEnding(ellidbg))[0].split(":")[1];
var changeHeaders = dbgversion.split(".")[0] != version.split(".")[0] || dbgversion.split(".")[1] != version.split(".")[1]


let debugLabels = ":" + version + os.EOL;
debugLabels += "Ellie Debug Headers [DONT MODIFY DIRECTLY]" + os.EOL;
debugLabels += "|------------------------------------|" + os.EOL;
function refactorFile(file, fileDir) {
    var eol = whichLineEnding(file);
    var lines = file.split(eol);
    var factoredFile = "";
    var factored = false;
    for (var i = 0; i < lines.length; i++) {
        var line = lines[i];
        if (line.includes("debug_message: \"") && fileDir != "./core/src/error/mod.rs") {
            var dbgId = createDebugLabel();
            debugLabels += "|  " + dbgId + "  :  " + fileDir + ":" + (i + 1) + eol;
            var first = line.split("debug_message: \"")[0];
            factoredFile += first + "debug_message: \"" + dbgId + "\"" + line.split("debug_message: \"")[1].split("\"")[1] + ((lines.length - 1) == i ? "" : eol)
            factored = true;
        } else {
            factoredFile += line + ((lines.length - 1) == i ? "" : eol);
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
                q = await scanDirectory(fs.readdirSync(path + dir[i].name, { withFileTypes: true }), path + dir[i].name + "/");
                files = [...q, ...files];
            }
        }
        resolve(files);
    });
}

log("Searching Errors");
if (!changeHeaders) {
    log(`Factoring not required`);
} else {
    scanDirectory(fs.readdirSync("./", { withFileTypes: true }), "./").then((files) => {
        log(`Factoring ${files.length} files`);
        log(`--------------------------------`);
        for (let i = 0; i < files.length; i++) {
            refactorFile(fs.readFileSync(files[i], "utf-8"), files[i]);
        }
        log(`--------------------------------`);
        log(`Writing debug headers`);
        debugLabels += "|------------------------------------|" + os.EOL;
        debugLabels += "END";
        fs.writeFileSync("./DEBUG_HEADERS.eidbg", debugLabels, 'utf8');
    })
}

