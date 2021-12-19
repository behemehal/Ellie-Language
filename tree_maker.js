let gpath = process.argv.slice(2);
var tree = require(gpath[0]);
var path = require('path');
var fs = require('fs');
let output = "digraph dependencies {";
output += "\n    ratio=fill;";
output += "\n    node [style=filled];\n";

//http://webgraphviz.com/
for (var i = 0; i < tree.length; i++) {
    var item = tree[i];
    for (var j = 0; j < item.dependencies.length; j++) {
        var dep = item.dependencies[j];
        if (item.inner == null) {
            if (dep.deep_link == null) {
                var cr = path.basename(item.path).split(".")[0]; //Ellie
                var tg = path.basename(tree.find(x => x.hash == dep.hash).path).split(".")[0];
                if (cr == tg) {
                    console.error("ERROR:", cr, tg, i, item);
                    process.exit()
                }
                output += `    ${cr} -> ${tg} [ label= "depends to"]\n`
            } else if (process.argv.length == 4) {
                var cr = path.basename(item.path).split(".")[0];
                var tg = path.basename(tree.find(x => x.hash == dep.hash).path).split(".")[0];
                var parent = path.basename(tree.find(x => x.hash == dep.deep_link).path).split(".")[0];
                output += `    ${cr} -> ${tg} [ label= "deep depends to from '${parent}'" color="gray"]\n`
            }
        }


    }
}
output += "}";
console.log("Graph wrote to: './outputTree.txt'");
fs.writeFileSync("./outputTree.txt", output);