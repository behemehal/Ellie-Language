use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};

use crate::types::Types;

/// Macro to create a new instruction with operands and op_code

//export macro

#[macro_export]
macro_rules! create_instruction {
    // When there are no operands
    ($op_code:expr) => {
        Instruction {
            op_code: $op_code,
            operand_0: None,
            operand_1: None,
            operand_2: None,
        }
    };

    // When there is one operand
    ($op_code:expr, $operand_0:expr) => {
        Instruction {
            op_code: $op_code,
            operand_0: Some($operand_0),
            operand_1: None,
            operand_2: None,
        }
    };

    // When there are two operands
    ($op_code:expr, $operand_0:expr, $operand_1:expr) => {
        Instruction {
            op_code: $op_code,
            operand_0: Some($operand_0),
            operand_1: Some($operand_1),
            operand_2: None,
        }
    };

    // When there are three operands
    ($op_code:expr, $operand_0:expr, $operand_1:expr, $operand_2:expr) => {
        Instruction {
            op_code: $op_code,
            operand_0: Some($operand_0),
            operand_1: Some($operand_1),
            operand_2: Some($operand_2),
        }
    };
}

/* fn parse_type_text(text: String) -> Option<(Types, Vec<u8>)> {
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
    let mut type_text = cut.split(')');
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

    let addressing_mode = if operand.is_empty() {
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
                    let pointer = data.split('[').next().unwrap();
                    let idx = data.split('[').last().unwrap().trim_end_matches(']');
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
 */
