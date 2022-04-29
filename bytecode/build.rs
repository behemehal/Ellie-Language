use serde_json::Value;
use std::io::Write;
extern crate alloc;

#[path = "src/instructions.rs"]
mod instructions;

fn main() {
    let revision = 1;
    //Read bytecode.json
    let bytecode_json = std::fs::read_to_string(
        env!("CARGO_MANIFEST_DIR").to_owned() + &"/bytecode.json".to_owned(),
    )
    .unwrap();
    //Parse json
    let bytecode_json: Value = serde_json::from_str(&bytecode_json).unwrap();

    let instructions = bytecode_json["instructions"].as_object().unwrap();
    let instruction_values = bytecode_json["instructions"]
        .as_object()
        .unwrap()
        .iter()
        .map(|x| {
            (
                x.0,
                x.1["mode"].as_str().unwrap().to_owned(),
                x.1["type"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<Vec<_>>();

    let mut instructions_vec: Vec<instructions::InstructionStruct> = Vec::new();
    for (key, value) in instructions {
        //If given instruction already exists, add mode to it
        if instructions_vec
            .iter()
            .any(|x| x.rtype == *value["type"].as_str().unwrap())
        {
            let instruction = instructions_vec
                .iter_mut()
                .find(|x| x.rtype == *value["type"].as_str().unwrap())
                .unwrap();
            instruction
                .modes
                .push(match value["mode"].as_str().unwrap() {
                    "implicit" => crate::instructions::AddressingModesStruct::Implicit,
                    "immediate" => crate::instructions::AddressingModesStruct::Immediate,
                    "absolute" => crate::instructions::AddressingModesStruct::Absolute,
                    "indirecta" => crate::instructions::AddressingModesStruct::IndirectA,
                    "indirectb" => crate::instructions::AddressingModesStruct::IndirectB,
                    "indirectc" => crate::instructions::AddressingModesStruct::IndirectC,
                    "indirectx" => crate::instructions::AddressingModesStruct::IndirectX,
                    "indirecty" => crate::instructions::AddressingModesStruct::IndirectY,
                    _ => panic!("Unknown addressing mode"),
                });
        } else {
            //Else create new instruction
            instructions_vec.push(instructions::InstructionStruct {
                op_code: key.to_string(),
                rtype: value["type"].as_str().unwrap().to_string(),
                modes: vec![match value["mode"].as_str().unwrap() {
                    "implicit" => crate::instructions::AddressingModesStruct::Implicit,
                    "immediate" => crate::instructions::AddressingModesStruct::Immediate,
                    "absolute" => crate::instructions::AddressingModesStruct::Absolute,
                    "indirecta" => crate::instructions::AddressingModesStruct::IndirectA,
                    "indirectb" => crate::instructions::AddressingModesStruct::IndirectB,
                    "indirectc" => crate::instructions::AddressingModesStruct::IndirectC,
                    "indirectx" => crate::instructions::AddressingModesStruct::IndirectX,
                    "indirecty" => crate::instructions::AddressingModesStruct::IndirectY,
                    _ => panic!("Unknown addressing mode"),
                }],
            });
        }
    }

    let mut instruction_markdown_table =
        String::from("Auto builded from `bytecode.json` by `build.rs`\n");

    instruction_markdown_table.push_str("| Instruction | Implicit | Immediate | Absolute | IndirectA | IndirectB | IndirectC | IndirectX | IndirectY |\n");
    instruction_markdown_table.push_str("| ----------- | -------- | --------- | -------- | --------- | --------- | --------- | --------- | --------- |\n");

    for instruction in instructions_vec.clone() {
        let mut modes_string = String::new();

        modes_string.push_str(&format!(
            "| {}{:width$}       |",
            instruction.rtype,
            "",
            width = 5 - instruction.rtype.len()
        ));

        for m in 0..8 {
            let op_code = match m {
                0 => instruction_values
                    .iter()
                    .find(|x| x.2 == instruction.rtype && x.1 == "implicit"),
                1 => instruction_values
                    .iter()
                    .find(|x| x.2 == instruction.rtype && x.1 == "immediate"),
                2 => instruction_values
                    .iter()
                    .find(|x| x.2 == instruction.rtype && x.1 == "absolute"),
                3 => instruction_values
                    .iter()
                    .find(|x| x.2 == instruction.rtype && x.1 == "indirecta"),
                4 => instruction_values
                    .iter()
                    .find(|x| x.2 == instruction.rtype && x.1 == "indirectb"),
                5 => instruction_values
                    .iter()
                    .find(|x| x.2 == instruction.rtype && x.1 == "indirectc"),
                6 => instruction_values
                    .iter()
                    .find(|x| x.2 == instruction.rtype && x.1 == "indirectx"),
                7 => instruction_values
                    .iter()
                    .find(|x| x.2 == instruction.rtype && x.1 == "indirecty"),
                _ => unreachable!(),
            };

            let blank = match m {
                0 => 7,
                1 => 10,
                2 => 8,
                3 => 9,
                4 => 9,
                5 => 9,
                6 => 9,
                7 => 9,
                _ => unreachable!(),
            };

            match op_code {
                Some((op_code, _, _)) => {
                    modes_string.push_str(&format!(" {}{:^blank$} |", "", op_code));
                }
                None => {
                    modes_string.push_str(&format!(" {:^blank$} |", "-"));
                }
            }
        }
        instruction_markdown_table.push_str(&format!("{}\n", modes_string));
    }

    //Create file for markdown
    let mut md_file = std::fs::File::create(
        env!("CARGO_MANIFEST_DIR").to_owned() + &"/instructions.md".to_owned(),
    )
    .unwrap();
    md_file
        .write_all(instruction_markdown_table.as_bytes())
        .unwrap();

    let mut map_file = std::fs::File::create(
        env!("CARGO_MANIFEST_DIR").to_owned() + &"/instruction_table.rs".to_owned(),
    )
    .unwrap();
    let mut map_content = format!("use crate::instructions::{{InstructionStruct, AddressingModesStruct}};\npub static Revision : i8 = {};\n\nstatic Instructions: phf::Map<&'static str, Keyword> = phf_map! {{\n", revision);

    for instruction in instructions_vec {
        map_content.push_str(&format!(
            "\t\"{}\" => Keyword {{\n\t\top_code: \"{}\",\n\t\trtype: \"{}\",\n\t\tmodes: vec![\n",
            instruction.op_code, instruction.op_code, instruction.rtype
        ));

        for mode in instruction.modes {
            map_content.push_str(&format!("\t\t\t{:?},\n", mode));
        }

        map_content.push_str("\t\t]\n\t},\n");
    }
    map_content.push_str("};\n");

    map_file.write_all(map_content.as_bytes()).unwrap();
}
