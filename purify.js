var fs = require("fs");

var file = JSON.parse(fs.readFileSync("./data.json", "utf-8"));

var code = file.value;


var getValue = (element) => element.value;

function purifyCode(type, element) {
    var purified = "";
    if (type == "Number") {
        purified += element.value;
    } else if (type == "Array") {
        var pr = element.collective.map(x => x.value).map(x => purifyCode(Object.keys(x), x[Object.keys(x)]))[0];
        purified += "[ "
        purified += pr
        purified += " ]"

        //purified += ` ${element.operator_collect} `
        //purified += purifyCode(Object.keys(element.second)[0], element.second[Object.keys(element.second)[0]]) + ")";
    } else if (type == "String") {
        purified += element.value;
    } else if (type == "Cloak") {
        var pr = element.collective.map(x => x.value).map(x => purifyCode(Object.keys(x), x[Object.keys(x)]))[0];
        purified += "( "
        purified += pr
        purified += " )"

        //purified += ` ${element.operator_collect} `
        //purified += purifyCode(Object.keys(element.second)[0], element.second[Object.keys(element.second)[0]]) + ")";
    } else if (type == "Operator") {
        purified += "" + purifyCode(Object.keys(element.first)[0], element.first[Object.keys(element.first)[0]]);
        purified += ` ${element.operator_collect} `
        purified += purifyCode(Object.keys(element.second)[0], element.second[Object.keys(element.second)[0]]) + "";
    } else {
        console.error("Unknown element " + type + " cannot be parsed");
    }
    return purified;
}



console.log(
    purifyCode(
        Object.keys(code)[0],
        code[Object.keys(code)[0]]
    )
)
