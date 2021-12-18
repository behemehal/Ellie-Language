var tree = require('./test.ei_parsed.json');
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
        var cr = path.basename(item.path).split(".")[0];
        var tg = path.basename(tree.find(x => x.hash == dep).path).split(".")[0];
        output += `    ${cr} -> ${tg} [ label= "depends to"]\n`
    }
}
output += "}";
fs.writeFileSync("./outputTree.txt", output);