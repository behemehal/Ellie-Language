import { trace } from "console";
import fs from "fs";
import chalk from "chalk";

const bytecode_rev = 3;

var csv = fs.readFileSync("./bytecode/instructions.csv", "utf8");
var line_ending = csv.includes("\r\n") ? "\r\n" : "\n";
var _instructions = csv.split(line_ending);

console.log(chalk`{green [Info]:} {yellow Building instructions...}`);

var _instructions_in_order = [];

var out_of_order = false;

for (var i = 0; i < _instructions.length; i++) {
  let entries = _instructions[i].split(",");
  let instruction = {
    instructionString: entries[0],
    addressing_mode: entries[1],
    op_code: Number(entries[2]),
  };
  if ((i + 1) != instruction.op_code) {
    out_of_order = true;
  }
  _instructions_in_order.push({
    instructionString: entries[0],
    addressing_mode: entries[1],
    op_code: Number(i + 1),
  });
}

if (out_of_order) {
  console.log(
    chalk`{green [Info]:} {yellow Ordering {cyan './bytecode/instructions.csv'}}`
  );
  fs.writeFileSync(
    "./bytecode/instructions.csv",
    _instructions_in_order
      .map((x) => {
        return x.instructionString + "," + x.addressing_mode + "," + x.op_code;
      })
      .join(line_ending)
  );
}

let full_instructions = [];

function index_of_addresing_mode(mode) {
  return mode === "implicit"
    ? 0
    : mode == "immediate"
    ? 1
    : mode == "absolute"
    ? 2
    : mode == "absolute_index"
    ? 3
    : mode == "absolute_property"
    ? 4
    : mode == "indirect_a"
    ? 5
    : mode == "indirect_b"
    ? 6
    : mode == "indirect_c"
    ? 7
    : mode == "indirect_x"
    ? 8
    : 9;
}

function addresing_mode_index(idx) {
  if (idx === 0) {
    return "implicit";
  } else if (idx === 1) {
    return "immediate";
  } else if (idx === 2) {
    return "absolute";
  } else if (idx === 3) {
    return "absolute_index";
  } else if (idx === 4) {
    return "absolute_property";
  } else if (idx === 5) {
    return "indirect_a";
  } else if (idx === 6) {
    return "indirect_b";
  } else if (idx === 7) {
    return "indirect_c";
  } else if (idx === 8) {
    return "indirect_x";
  } else {
    return "indirect_y";
  }
}

let instructions = _instructions_in_order.map((instruction) => {
  if (
    full_instructions.find(
      (i) => i.instructionString === instruction.instructionString
    )
  ) {
    full_instructions
      .find((i) => i.instructionString === instruction.instructionString)
      .addressing_modes.push([
        instruction.addressing_mode,
        instruction.op_code,
      ]);
  } else {
    full_instructions.push({
      instructionString: instruction.instructionString,
      addressing_modes: [[instruction.addressing_mode, instruction.op_code]],
    });
  }
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

let md =
  `Rev: ${bytecode_rev}\n` +
  "Auto builded from `instructions.csv` by `reassembler.js`\n" +
  "| Instruction | Implicit | Immediate | Absolute | Absolute Index | Absolute Property | IndirectA | IndirectB | IndirectC | IndirectX | IndirectY |\n" +
  "| ----------- | -------- | --------- | -------- | -------------- | ----------------- | --------- | --------- | --------- | --------- | --------- |";

//bytecode/instructions.md
let instructions_md = full_instructions.map((x) => {
  let instruction = x.instructionString;
  let addressing_modes = "";

  for (var idx = 0; idx <= 9; idx++) {
    let mode = x.addressing_modes.find(
      (x) => addresing_mode_index(idx) == x[0]
    );
    let len =
      idx == 0
        ? 10
        : idx == 1
        ? 11
        : idx == 2
        ? 10
        : idx == 3
        ? 16
        : idx == 4
        ? 19
        : 11;

    if (mode) {
      let op_code = "0x" + Number(mode[1]).toString(16).padStart(2, "0");

      addressing_modes += `${makeSpace(
        Math.round((len - op_code.length) / 2)
      )}${op_code}${makeSpace(Math.floor((len - op_code.length) / 2))}|`;
    } else {
      addressing_modes += `${makeSpace(Math.round((len - 2) / 2))}--${makeSpace(
        Math.floor((len - 2) / 2)
      )}|`;
    }
  }
  return `| ${instruction}${makeSpace(
    11 - instruction.length
  )} |${addressing_modes}`;
});

console.log(
  chalk`{green [Info]:} {yellow Instruction markdown writed {cyan './bytecode/instructions.md'}}`
);
fs.writeFileSync(
  "./bytecode/instructions.md",
  md + "\n" + instructions_md.join("\n")
);
console.log(
  chalk`{green [Info]:} {yellow Utils writed {cyan './tools/generated_utils.csv'}}`
);
fs.writeFileSync("./tools/generated_utils.rs", utils.join(line_ending));
console.log(
  chalk`{green [Info]:} {yellow Instruction table writed {cyan './tools/generated_instruction_table.csv'}}`
);
fs.writeFileSync(
  "./tools/generated_instruction_table.rs",
  instruction_table.join(line_ending)
);
console.log(chalk`{green [Info]:} {yellow ReAssembly completed}`);
