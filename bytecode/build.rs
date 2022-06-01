use std::io::Write;
extern crate alloc;

#[path = "src/instructions.rs"]
mod instructions;

fn main() {
    let revision = 1;
    let instructions_csv = std::fs::read_to_string(
        env!("CARGO_MANIFEST_DIR").to_owned() + &"/instructions.csv".to_owned(),
    )
    .unwrap_or_else(|_| {
        panic!("{}/instructions.csv", env!("CARGO_MANIFEST_DIR"));
    });

    //Parse csv
    let instruction_entries: Vec<(String, String, u8)> = instructions_csv
        .split("\n")
        .map(|line| {
            let mut parts = line.split(",");
            let rtype = parts.next().unwrap().to_owned();
            let mode = parts.next().unwrap().to_owned();
            let op_code = parts.next().unwrap().parse::<u8>().unwrap();
            (rtype, mode, op_code)
        })
        .collect();

    //Build instruction table
    let mut instructions: Vec<instructions::InstructionStruct> = Vec::new();

    for entry in &instruction_entries {
        if instructions.iter().find(|x| x.rtype == entry.0).is_none() {
            instructions.push(instructions::InstructionStruct {
                op_code: entry.2,
                rtype: &entry.0,
                modes: vec![instructions::AddressingModesStruct::from_str(
                    &entry.1, entry.2,
                )],
            });
        } else {
            instructions
                .iter_mut()
                .find(|x| x.rtype == entry.0)
                .unwrap()
                .modes
                .push(instructions::AddressingModesStruct::from_str(
                    &entry.1, entry.2,
                ));
        }
    }

    let mut instruction_markdown_table =
        String::from("Auto builded from `instructions.csv` by `build.rs` dont modify while language server is running\n");

    instruction_markdown_table.push_str("| Instruction | Implicit | Immediate | Absolute | Absolute Index | Absolute Property | IndirectA | IndirectB | IndirectC | IndirectX | IndirectY |\n");
    instruction_markdown_table.push_str("| ----------- | -------- | --------- | -------- | -------------- | ----------------- | --------- | --------- | --------- | --------- | --------- |\n");

    for instruction in instructions.clone() {
        let mut modes_string = String::new();

        modes_string.push_str(&format!(
            "| {}{:width$}       |",
            instruction.rtype,
            "",
            width = 5 - instruction.rtype.len()
        ));

        for m in 0..10 {
            let op_code = match m {
                0 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::Implicit(0)),
                1 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::Immediate(0)),
                2 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::Absolute(0)),
                3 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::AbsoluteIndex(0)),
                4 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::AbsoluteProperty(0)),
                5 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::IndirectA(0)),
                6 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::IndirectB(0)),
                7 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::IndirectC(0)),
                8 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::IndirectX(0)),
                9 => instruction
                    .modes
                    .iter()
                    .find(|x| *x == &instructions::AddressingModesStruct::IndirectY(0)),
                _ => unreachable!(),
            };

            let blank = match m {
                0 => 8,
                1 => 9,
                2 => 8,
                3 => 14,
                4 => 17,
                5 => 9,
                6 => 9,
                7 => 9,
                8 => 9,
                9 => 9,
                _ => unreachable!(),
            };

            match op_code {
                Some(op_code) => {
                    modes_string.push_str(&format!(
                        " {}{:^blank$} |",
                        "",
                        format!("0x{:02x}", op_code.val())
                    ));
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
        env!("CARGO_MANIFEST_DIR").to_owned() + &"/src/instruction_table.rs".to_owned(),
    )
    .unwrap();
    let mut map_content = format!("//Auto builded from `instructions.csv` by `build.rs` dont modify while language server is running\nuse alloc::vec;\nuse lazy_static;\nuse crate::instructions::{{InstructionStruct, AddressingModesStruct}};\npub static Revision : i8 = {};\n\n pub struct Instruction {{\n\trtype: &'static str,\n\tcode: i8,\n\tmode: &'static str,\n}}\n\nlazy_static! {{\n\tlet instructions: [Instruction, {}] = [\n", revision, instruction_entries.len());

    for entry in instruction_entries {
        map_content.push_str(&format!(
            "\t\tInstruction {{\n\t\t\trtype: \"{}\",\n\t\t\tcode: {},\n\t\t\tmode: \"{}\",\n\t\t}},\n",
            entry.0, entry.2, entry.1
        ));
    }
    map_content.push_str("\t];\n}\n");
    map_file.write_all(map_content.as_bytes()).unwrap();
}
