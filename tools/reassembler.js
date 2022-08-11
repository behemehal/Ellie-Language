import fs from "fs";

//check repo is dirty
function isDirty() {
  return fs.existsSync(".git/index.lock");
}

console.log("isDirty", isDirty());