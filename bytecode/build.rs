use serde_json::Value;
use std::io::Write;
extern crate alloc;

#[path = "src/instructions.rs"]
mod instructions;

fn main() {
    //Read bytecode.json
    let bytecode_json = std::fs::read_to_string(
        env!("CARGO_MANIFEST_DIR").to_owned() + &"/bytecode.json".to_owned(),
    )
    .unwrap();
    //Parse json
    let bytecode_json: Value = serde_json::from_str(&bytecode_json).unwrap();

    let instructions: Vec<crate::instructions::InstructionStruct> = bytecode_json["instructions"]
        .as_object()
        .unwrap()
        .iter()
        .map(|x| crate::instructions::InstructionStruct {
            op_code: x.0.to_string(),
            rtype: x.1["type"].as_str().unwrap().to_owned(),
            mode: match x.1["mode"].as_str().unwrap() {
                "implicit" => crate::instructions::AddressingModesStruct::Implicit,
                "immediate" => crate::instructions::AddressingModesStruct::Immediate,
                "absolute" => crate::instructions::AddressingModesStruct::Absolute,
                "indirecta" => crate::instructions::AddressingModesStruct::IndirectA,
                "indirectb" => crate::instructions::AddressingModesStruct::IndirectB,
                "indirectc" => crate::instructions::AddressingModesStruct::IndirectC,
                "indirectx" => crate::instructions::AddressingModesStruct::IndirectX,
                "indirecty" => crate::instructions::AddressingModesStruct::IndirectY,
                _ => panic!("Unknown addressing mode"),
            },
        })
        .collect();
    let mut instruction_markdown_table = String::new();
    instruction_markdown_table.push_str("OpCode | Instruction | Mode Y\n");
    instruction_markdown_table.push_str("--- | --- | --- |\n");
    for instruction in instructions {
        instruction_markdown_table.push_str(&format!(
            "{} | {} | {}\n",
            instruction.op_code,
            instruction.rtype,
            match instruction.mode {
                crate::instructions::AddressingModesStruct::Implicit => "Implicit",
                crate::instructions::AddressingModesStruct::Immediate => "Immediate",
                crate::instructions::AddressingModesStruct::Absolute => "Absolute",
                crate::instructions::AddressingModesStruct::IndirectA => "IndirectA",
                crate::instructions::AddressingModesStruct::IndirectB => "IndirectB",
                crate::instructions::AddressingModesStruct::IndirectC => "IndirectC",
                crate::instructions::AddressingModesStruct::IndirectX => "IndirectX",
                crate::instructions::AddressingModesStruct::IndirectY => "IndirectY",
            }
        ));
    }

    //Create file
    let mut file = std::fs::File::create(
        env!("CARGO_MANIFEST_DIR").to_owned() + &"/instructions.md".to_owned(),
    )
    .unwrap();
    file.write_all(instruction_markdown_table.as_bytes())
        .unwrap();
}
