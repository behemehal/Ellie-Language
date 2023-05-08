use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use crate::{
    addressing_modes::AddressingModes, instruction_table::Instructions, instructions::Instruction,
    types::Types,
};

fn parse_type_text(text: String) -> Option<(Types, Vec<u8>)> {
    //#(typeText)value
    //#(int)10
    //#(float)10.0
    //#(double)true
    //#(byte)1
    //#(bool)true
    //#(string)"Hello World"
    //#(char)'a'
    //#(void)
    //#(null)

    let cut = text.split_at(2).1;
    let mut type_text = cut.split(")");
    let _type = type_text.next().unwrap();
    let _value = type_text.next().unwrap();
    let value = match _type {
        "int" => _value.parse::<i64>().unwrap().to_le_bytes().to_vec(),
        "float" => _value.parse::<f32>().unwrap().to_le_bytes().to_vec(),
        "double" => _value.parse::<f64>().unwrap().to_le_bytes().to_vec(),
        "byte" => _value.parse::<u8>().unwrap().to_le_bytes().to_vec(),
        "bool" => vec![_value.parse::<bool>().unwrap().into()],
        "char" => _value
            .chars()
            .next()
            .unwrap()
            .to_string()
            .into_bytes()
            .to_vec(),
        "void" => vec![0],
        "null" => vec![0],
        _ => panic!("Unknown type"),
    }
    .try_into()
    .unwrap();
    match _type {
        "int" => Some((Types::Integer, value)),
        "float" => Some((Types::Float, value)),
        "double" => Some((Types::Double, value)),
        "byte" => Some((Types::Byte, value)),
        "bool" => Some((Types::Bool, value)),
        "char" => Some((Types::Char, value)),
        "void" => Some((Types::Void, value)),
        "null" => Some((Types::Null, value)),
        _ => None,
    }
}

pub fn parse_instruction_text(text: String) -> Option<Instructions> {
    let (instruction, operand) = match text.split_whitespace().collect::<Vec<_>>().as_slice() {
        &[instruction, operand] => (instruction, operand),
        &[_] => (text.as_str(), ""),
        _ => panic!("Invalid line format"),
    };

    let addressing_mode = if operand == "" {
        AddressingModes::Implicit
    } else {
        let prefix = operand.chars().next().unwrap();
        let rest = &operand[1..];
        match prefix {
            '#' => match parse_type_text(operand.to_string()) {
                Some(parts) => AddressingModes::Immediate(parts.0, parts.1),
                None => return None,
            },
            '$' => {
                if rest.contains('[') {
                    let data = &operand[1..];
                    let pointer = data.split("[").next().unwrap();
                    let idx = data.split("[").last().unwrap().trim_end_matches("]");
                    AddressingModes::AbsoluteIndex(
                        pointer.parse::<usize>().unwrap(),
                        idx.parse::<usize>().unwrap(),
                    )
                } else {
                    AddressingModes::Absolute(rest.parse::<usize>().unwrap())
                }
            }
            '@' => match rest {
                "A" => AddressingModes::IndirectA,
                "B" => AddressingModes::IndirectB,
                "C" => AddressingModes::IndirectC,
                "X" => AddressingModes::IndirectX,
                "Y" => AddressingModes::IndirectY,
                _ => return None,
            },
            _ => AddressingModes::Implicit,
        }
    };

    match instruction {
        "LDA" => Some(Instructions::LDA(Instruction { addressing_mode })),
        "LDB" => Some(Instructions::LDB(Instruction { addressing_mode })),
        "LDC" => Some(Instructions::LDC(Instruction { addressing_mode })),
        "LDX" => Some(Instructions::LDX(Instruction { addressing_mode })),
        "LDY" => Some(Instructions::LDY(Instruction { addressing_mode })),
        "STA" => Some(Instructions::STA(Instruction { addressing_mode })),
        "STB" => Some(Instructions::STB(Instruction { addressing_mode })),
        "STC" => Some(Instructions::STC(Instruction { addressing_mode })),
        "STX" => Some(Instructions::STX(Instruction { addressing_mode })),
        "STY" => Some(Instructions::STY(Instruction { addressing_mode })),
        "EQ" => Some(Instructions::EQ(Instruction { addressing_mode })),
        "NE" => Some(Instructions::NE(Instruction { addressing_mode })),
        "GT" => Some(Instructions::GT(Instruction { addressing_mode })),
        "LT" => Some(Instructions::LT(Instruction { addressing_mode })),
        "GQ" => Some(Instructions::GQ(Instruction { addressing_mode })),
        "LQ" => Some(Instructions::LQ(Instruction { addressing_mode })),
        "AND" => Some(Instructions::AND(Instruction { addressing_mode })),
        "OR" => Some(Instructions::OR(Instruction { addressing_mode })),
        "ADD" => Some(Instructions::ADD(Instruction { addressing_mode })),
        "SUB" => Some(Instructions::SUB(Instruction { addressing_mode })),
        "MUL" => Some(Instructions::MUL(Instruction { addressing_mode })),
        "EXP" => Some(Instructions::EXP(Instruction { addressing_mode })),
        "DIV" => Some(Instructions::DIV(Instruction { addressing_mode })),
        "MOD" => Some(Instructions::MOD(Instruction { addressing_mode })),
        "JMP" => Some(Instructions::JMP(Instruction { addressing_mode })),
        "CALL" => Some(Instructions::CALL(Instruction { addressing_mode })),
        "CALLN" => Some(Instructions::CALLN(Instruction { addressing_mode })),
        "RET" => Some(Instructions::RET(Instruction { addressing_mode })),
        "PUSH" => Some(Instructions::PUSH(Instruction { addressing_mode })),
        "LEN" => Some(Instructions::LEN(Instruction { addressing_mode })),
        "A2I" => Some(Instructions::A2I(Instruction { addressing_mode })),
        "A2F" => Some(Instructions::A2F(Instruction { addressing_mode })),
        "A2D" => Some(Instructions::A2D(Instruction { addressing_mode })),
        "A2B" => Some(Instructions::A2B(Instruction { addressing_mode })),
        "A2S" => Some(Instructions::A2S(Instruction { addressing_mode })),
        "A2C" => Some(Instructions::A2C(Instruction { addressing_mode })),
        "A2O" => Some(Instructions::A2O(Instruction { addressing_mode })),
        "JMPA" => Some(Instructions::JMPA(Instruction { addressing_mode })),
        "POPS" => Some(Instructions::POPS(Instruction { addressing_mode })),
        "BRK" => Some(Instructions::BRK(Instruction { addressing_mode })),
        "CO" => Some(Instructions::CO(Instruction { addressing_mode })),
        "FN" => Some(Instructions::FN(Instruction { addressing_mode })),
        _ => None,
    }
}

#[macro_export]
macro_rules! lines_to_instructions {
    ($($line:tt),*) => {
        {
            let mut instructions = Vec::new();
            $(
                instructions.push(stringify!($line).to_string().replace("\"", ""));
            )*

            let mut insts = Vec::new();
            for instruction in instructions {
                let instruction = match parse_instruction_text(instruction) {
                    Some(instruction) => instruction,
                    None => panic!("Invalid instruction"),
                };
                insts.push(instruction);
            }
            insts
        }
    }
}
