import fs from "fs";

var csv = fs.readFileSync("./bytecode/instructions.csv", "utf8");
var line_ending = csv.includes("\r\n") ? "\r\n" : "\n";
var _instructions = csv.split(line_ending);

let instructions = _instructions.map((instruction_line) => {
  let entries = instruction_line.split(",");
  let instruction = {
    instructionString: entries[0],
    addressing_mode: entries[1],
    op_code: entries[2],
  };
  return instruction;
});

function makeFirstLetterBig(string) {
  return string[0].toUpperCase() + string.slice(1, string.length);
}

function snakeCaseToCamelCase(input) {
  let builded = "";
  let is_upper = true;
  for (var i = 0; i < input.length; i++) {
    let char = input[i];
    if (char === "_") {
      is_upper = true;
    } else {
      if (is_upper) {
        builded += char.toUpperCase();
        is_upper = false;
      } else {
        builded += char.toLowerCase();
      }
    }
  }
  return builded;
}

function makeSpace(len) {
  return Array(len).fill(" ").join("");
}

console.log("instructions", instructions);

//vm/src/utils.rs
let utils = instructions.map((x) => {
  return `${
    x.op_code
  } => Some(Instructions::${x.instructionString.toUpperCase()}(Instruction {
        addressing_mode: AddressingModes::${snakeCaseToCamelCase(
          x.addressing_mode
        )},
    })),\n`;
});

//bytecode/src/instruction_table.rs
let instruction_table = instructions.map((x) => {
  return `i.insert("${x.instructionString}_${x.addressing_mode}",Instruction {
        rtype: "${x.instructionString}",
        code: ${x.op_code},
        mode: "${x.addressing_mode}",
    });`;
});

fs.writeFileSync("./generated_utils.rs", utils.join(line_ending));
fs.writeFileSync(
  "./generated_instruction_table.rs",
  instruction_table.join(line_ending)
);
