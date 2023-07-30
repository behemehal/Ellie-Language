const fs = require("fs");

//Get working directory
const working_dir = process.cwd();

console.log("Working directory: " + working_dir + "");

//If we are in the tools folder, throw an error
if (working_dir.endsWith("tools")) {
  throw new Error("Please run this script from the root directory");
}

//!Targets

let instruction_table_path = "./ellie_engine/bytecode/src/instruction_table.rs";
let instruction_utils_path = "./ellie_engine/vm/src/instruction_utils.rs";
let instruction_md_path = "./tools/instructions.md";

const byteCodeFile = require("./instructions.json");
const bytecodeRev = byteCodeFile.rev;

console.log("Building instructions...");
console.log("Bytecode rev: " + (bytecodeRev + 1));

let op_codes = [];

for (const instruction of byteCodeFile.instructions) {
  for (const addressing_mode of instruction.addressingModes) {
    op_codes.push({
      rtype: instruction.instruction,
      code: op_codes.length + 1,
      mode: addressing_mode,
    });
  }
}

//! Build instructions_table.rs
console.log("Building instructions_table.rs...");
let instruction_table = "";
instruction_table +=
  "//Auto builded from `instructions.json` by `reAssembler.js` rev: " +
  (bytecodeRev + 1) +
  "\n";
instruction_table += "use alloc::{vec::Vec, vec};\n";
instruction_table += "use alloc::string::String;\n";
instruction_table += "use crate::instructions::Instruction;\n";
instruction_table += "use ellie_core::defs::PlatformArchitecture;\n\n";
instruction_table += `#[derive(Clone, Debug, PartialEq)]
pub enum Instructions {\n`;

for (const instruction of byteCodeFile.instructions) {
  instruction_table += `    ${instruction.instruction}(Instruction),\n`;
}
instruction_table += "}\n\n";

instruction_table += "impl Instructions {\n";
instruction_table +=
  "    pub fn op_code(&self, platform_size: PlatformArchitecture) -> Vec<u8> {\n";
instruction_table += "        match &self {\n";

for (const instruction of byteCodeFile.instructions) {
  let existing_instruction_names = [
    "Implicit",
    "Immediate",
    "Absolute",
    "AbsoluteIndex",
    "AbsoluteProperty",
    "AbsoluteStatic",
    "IndirectA",
    "IndirectB",
    "IndirectC",
    "IndirectX",
    "IndirectY",
  ];
  instruction_table += `            Instructions::${instruction.instruction}(e) => {\n`;
  let op_code = op_codes.find((x) => x.rtype == instruction.instruction);
  let op_code_list = existing_instruction_names.map((x) => {
    return (
      op_codes.find((y) => y.rtype == instruction.instruction && y.mode == x)
        ?.code || -1
    );
  });
  instruction_table += `                let op_code_list: [isize; 11] = [${op_code_list.join(
    ", "
  )}];\n`;
  instruction_table += `                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];\n`;
  instruction_table += `                if real_op_code == -1 {\n`;
  instruction_table += `                  panic!("Wrong addresing_mode accessed");\n`;
  instruction_table += `                }\n`;
  instruction_table += `                let mut op_code = vec![real_op_code as u8];\n`;
  instruction_table += `                op_code.extend(e.addressing_mode.arg(platform_size));\n`;
  instruction_table += `                op_code\n`;
  instruction_table += `            },\n`;
}
instruction_table += `        }\n`;
instruction_table += `    }\n`;

//!

instruction_table += `\n    pub fn get_addressing_mode(&self) -> String {\n`;
instruction_table += "        match &self {\n";

for (const instruction of byteCodeFile.instructions) {
  instruction_table += `            Instructions::${instruction.instruction}(e) => e.addressing_mode.clone(),\n`;
}
instruction_table += `        }\n`;
instruction_table += `        .to_string()\n`;
instruction_table += `    }\n`;

//!

instruction_table += `\n    pub fn get_addressing_mode_mut(&mut self) -> String {\n`;
instruction_table += "        match self {\n";

for (const instruction of byteCodeFile.instructions) {
  instruction_table += `            Instructions::${instruction.instruction}(e) => e,\n`;
}
instruction_table += `        }\n`;
instruction_table += `    }\n`;

//!

instruction_table += `\n    pub fn get_arg(&self, platform_size: PlatformArchitecture) -> Vec<u8> {\n`;
instruction_table += "        match &self {\n";

for (const instruction of byteCodeFile.instructions) {
  instruction_table += `            Instructions::${instruction.instruction}(e) => e.addressing_mode.arg(platform_size),\n`;
}
instruction_table += `        }\n`;
instruction_table += `    }\n`;

//!
instruction_table += `}\n\n`;

instruction_table += `impl core::fmt::Display for Instructions {\n`;
instruction_table += `    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {\n`;
instruction_table += `        match &self {\n`;

for (const instruction of byteCodeFile.instructions) {
  instruction_table += `            Instructions::${instruction.instruction}(e) => write!(f, "${instruction.instruction} {}", e.addressing_mode),\n`;
}
instruction_table += `        }\n`;
instruction_table += `    }\n`;

instruction_table += `}\n\n`;

fs.writeFileSync(instruction_table_path, instruction_table);
//! End of instructions_table.rs

//? Build instructions_utils.rs
console.log("Building instructions_utils.rs...");

let instruction_utils =
  "//Auto generated from `instructions.json` by `reAssembler.js rev: " +
  (bytecodeRev + 1) +
  "\n";
instruction_utils += `
use ellie_core::defs::PlatformArchitecture;
use crate::{
    heap_memory::HeapMemory,
    instructions::{ExecuterPanic, ExecuterResult, InstructionExecuter, StaticProgram},
    stack_memory::StackMemory,
    stack::Stack,
    utils::{AddressingModes, AddressingValues},
};\n\n`
for (const instruction of byteCodeFile.instructions) {
  instruction_utils += `#[derive(Clone, Copy, Debug)]\n`;
  instruction_utils += `pub struct ${instruction.instruction} {\n`;
  instruction_utils += `    pub addressing_mode: AddressingModes,\n`;
  instruction_utils += `}\n\n`;
}

instruction_utils += `#[derive(Clone, Copy, Debug)]\n`;
instruction_utils += `pub enum Instructions {\n`;

for (const instruction of byteCodeFile.instructions) {
  instruction_utils += `    ${instruction.instruction}(${instruction.instruction}),\n`;
}
instruction_utils += "}\n\n";

instruction_utils += `impl Instructions {\n`;
instruction_utils += `    pub fn from(op_code: &u8) -> Option<Self> {\n`;
instruction_utils += `        match op_code {\n`;

for (const op_code of op_codes) {
  instruction_utils += `            ${op_code.code} => Some(Instructions::${op_code.rtype}(${op_code.rtype} {\n`;
  instruction_utils += `                addressing_mode: AddressingModes::${op_code.mode},\n`;
  instruction_utils += `            })),\n`;
}
instruction_utils += `            _ => None,\n`;
instruction_utils += `        }\n`;
instruction_utils += `    }\n\n`;

//!

//!

instruction_utils += `    pub fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {\n`;
instruction_utils += "        match &self {\n";

for (const instruction of byteCodeFile.instructions) {
  instruction_utils += `            Instructions::${instruction.instruction}(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),\n`;
}
instruction_utils += `        }\n`;
instruction_utils += `    }\n`;

//!

instruction_utils += `    pub fn addressing_mode(&self) -> AddressingModes {\n`;
instruction_utils += `        match &self {\n`;

for (const instruction of byteCodeFile.instructions) {
  instruction_utils += `            Instructions::${instruction.instruction}(e) => e.addressing_mode,\n`;
}
instruction_utils += `        }\n`;
instruction_utils += `    }\n`;

instruction_utils += `}\n\n`;

fs.writeFileSync(instruction_utils_path, instruction_utils);

//? Build instructions.json

//# Build instructions.md

console.log("Building instructions.md...");

let instruction_md = "# Instructions\n\n";
instruction_md += "## Rev: " + (bytecodeRev + 1) + "\n\n";

// headers

let header_names = [
  "Instruction",
  "Implicit",
  "Immediate",
  "Absolute",
  "AbsoluteIndex",
  "AbsoluteProperty",
  "AbsoluteStatic",
  "IndirectA",
  "IndirectB",
  "IndirectC",
  "IndirectX",
  "IndirectY",
];

let op_code_idx = 1;

function centerText(text, size) {
  let spaces = size - text.length;
  let left = Math.floor(spaces / 2);
  let right = spaces - left;
  return (
    " ".repeat(left < 0 ? 0 : left) + text + " ".repeat(right < 0 ? 0 : right)
  );
}

for (const header of header_names) {
  instruction_md += "|";
  instruction_md += centerText(header, header.length + 2);
}

instruction_md += "|\n";

for (const header of header_names) {
  instruction_md += "|";
  instruction_md += "-".repeat(header.length + 2);
}

instruction_md += "|\n";

for (const instruction of byteCodeFile.instructions) {
  for (const header of header_names) {
    instruction_md += "|";
    if (header == "Instruction") {
      instruction_md += centerText(instruction.instruction, header.length + 2);
    } else if (instruction.addressingModes.includes(header)) {
      instruction_md += centerText(
        "0x" + op_code_idx.toString(16),
        header.length + 2
      );
      op_code_idx++;
    } else {
      instruction_md += centerText("-", header.length + 2);
    }
  }
  instruction_md += "|\n";
}

instruction_md += "\n";
instruction_md +=
  "*__Note:__ Revision is incremented when the instruction set changes.*\n";
instruction_md += "\n";
instruction_md +=
  "*_This file is auto generated from `instructions.json` by `reAssembler.js_*\n";

fs.writeFileSync(instruction_md_path, instruction_md);

console.log("ReAssembler.js finished.");
console.log("Running cargo fmt --all in terminal");
const { spawn } = require("child_process");
process.chdir("..");
const ls = spawn("cargo", ["fmt", "--all"]);
