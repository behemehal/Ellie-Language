//Auto builded from `instructions.json` by `reAssembler.js` rev: 3
use alloc::{vec::Vec, vec};
use alloc::string::String;
use crate::instructions::Instruction;
use ellie_core::defs::PlatformArchitecture;

#[derive(Clone, Debug, PartialEq)]
pub enum Instructions {
    LDA(Instruction),
    LDB(Instruction),
    LDC(Instruction),
    LDX(Instruction),
    LDY(Instruction),
    STA(Instruction),
    STB(Instruction),
    STC(Instruction),
    STX(Instruction),
    STY(Instruction),
    EQ(Instruction),
    NE(Instruction),
    GT(Instruction),
    LT(Instruction),
    GQ(Instruction),
    LQ(Instruction),
    AND(Instruction),
    OR(Instruction),
    ADD(Instruction),
    SUB(Instruction),
    MUL(Instruction),
    EXP(Instruction),
    DIV(Instruction),
    MOD(Instruction),
    JMP(Instruction),
    JMPA(Instruction),
    CALL(Instruction),
    RET(Instruction),
    PUSH(Instruction),
    SPUS(Instruction),
    LEN(Instruction),
    A2I(Instruction),
    A2F(Instruction),
    A2D(Instruction),
    A2B(Instruction),
    A2S(Instruction),
    A2C(Instruction),
    A2O(Instruction),
    ARR(Instruction),
    STR(Instruction),
    SAR(Instruction),
    POPS(Instruction),
    BRK(Instruction),
    CALLN(Instruction),
    CO(Instruction),
    FN(Instruction),
    DEA(Instruction),
}

impl Instructions {
    pub fn op_code(&self, platform_size: &PlatformArchitecture) -> Vec<u8> {
        match &self {
            Instructions::LDA(e) => {
                let op_code_list: [isize; 11] = [-1, 1, 2, 3, 4, 5, -1, 6, 7, 8, 9];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::LDB(e) => {
                let op_code_list: [isize; 11] = [-1, 10, 11, 12, 13, 14, 15, -1, 16, 17, 18];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::LDC(e) => {
                let op_code_list: [isize; 11] = [-1, 19, 20, 21, 22, 23, 24, 25, -1, 26, 27];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::LDX(e) => {
                let op_code_list: [isize; 11] = [-1, 28, 29, 30, 31, 32, 33, 34, 35, -1, 36];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::LDY(e) => {
                let op_code_list: [isize; 11] = [-1, 37, 38, 39, 40, 41, 42, 43, 44, 45, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::STA(e) => {
                let op_code_list: [isize; 11] = [46, 47, 48, 49, 50, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::STB(e) => {
                let op_code_list: [isize; 11] = [51, 52, 53, 54, 55, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::STC(e) => {
                let op_code_list: [isize; 11] = [56, 57, 58, 59, 60, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::STX(e) => {
                let op_code_list: [isize; 11] = [61, 62, 63, 64, 65, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::STY(e) => {
                let op_code_list: [isize; 11] = [66, 67, 68, 69, 70, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::EQ(e) => {
                let op_code_list: [isize; 11] = [71, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::NE(e) => {
                let op_code_list: [isize; 11] = [72, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::GT(e) => {
                let op_code_list: [isize; 11] = [73, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::LT(e) => {
                let op_code_list: [isize; 11] = [74, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::GQ(e) => {
                let op_code_list: [isize; 11] = [75, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::LQ(e) => {
                let op_code_list: [isize; 11] = [76, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::AND(e) => {
                let op_code_list: [isize; 11] = [77, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::OR(e) => {
                let op_code_list: [isize; 11] = [78, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::ADD(e) => {
                let op_code_list: [isize; 11] = [79, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::SUB(e) => {
                let op_code_list: [isize; 11] = [80, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::MUL(e) => {
                let op_code_list: [isize; 11] = [81, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::EXP(e) => {
                let op_code_list: [isize; 11] = [82, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::DIV(e) => {
                let op_code_list: [isize; 11] = [83, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::MOD(e) => {
                let op_code_list: [isize; 11] = [84, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::JMP(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 85, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::JMPA(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 86, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::CALL(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 87, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::RET(e) => {
                let op_code_list: [isize; 11] = [88, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::PUSH(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 89, 90, -1, -1, 91, 92, 93, 94, 95];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::SPUS(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 96, 97, -1, -1, 98, 99, 100, 101, 102];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::LEN(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 103, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::A2I(e) => {
                let op_code_list: [isize; 11] = [104, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::A2F(e) => {
                let op_code_list: [isize; 11] = [105, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::A2D(e) => {
                let op_code_list: [isize; 11] = [106, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::A2B(e) => {
                let op_code_list: [isize; 11] = [107, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::A2S(e) => {
                let op_code_list: [isize; 11] = [108, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::A2C(e) => {
                let op_code_list: [isize; 11] = [109, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::A2O(e) => {
                let op_code_list: [isize; 11] = [110, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::ARR(e) => {
                let op_code_list: [isize; 11] = [111, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::STR(e) => {
                let op_code_list: [isize; 11] = [112, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::SAR(e) => {
                let op_code_list: [isize; 11] = [-1, 113, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::POPS(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 114, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::BRK(e) => {
                let op_code_list: [isize; 11] = [115, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::CALLN(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 116, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::CO(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 117, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::FN(e) => {
                let op_code_list: [isize; 11] = [-1, 118, -1, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
            Instructions::DEA(e) => {
                let op_code_list: [isize; 11] = [-1, -1, 119, -1, -1, -1, -1, -1, -1, -1, -1];
                let real_op_code: isize = op_code_list[e.addressing_mode.idx()];
                if real_op_code == -1 {
                  panic!("Wrong addresing_mode accessed");
                }
                let mut op_code = vec![real_op_code as u8];
                op_code.extend(e.addressing_mode.arg(platform_size));
                op_code
            },
        }
    }

    pub fn get_addressing_mode(&self) -> String {
        match &self {
            Instructions::LDA(e) => e.addressing_mode.clone(),
            Instructions::LDB(e) => e.addressing_mode.clone(),
            Instructions::LDC(e) => e.addressing_mode.clone(),
            Instructions::LDX(e) => e.addressing_mode.clone(),
            Instructions::LDY(e) => e.addressing_mode.clone(),
            Instructions::STA(e) => e.addressing_mode.clone(),
            Instructions::STB(e) => e.addressing_mode.clone(),
            Instructions::STC(e) => e.addressing_mode.clone(),
            Instructions::STX(e) => e.addressing_mode.clone(),
            Instructions::STY(e) => e.addressing_mode.clone(),
            Instructions::EQ(e) => e.addressing_mode.clone(),
            Instructions::NE(e) => e.addressing_mode.clone(),
            Instructions::GT(e) => e.addressing_mode.clone(),
            Instructions::LT(e) => e.addressing_mode.clone(),
            Instructions::GQ(e) => e.addressing_mode.clone(),
            Instructions::LQ(e) => e.addressing_mode.clone(),
            Instructions::AND(e) => e.addressing_mode.clone(),
            Instructions::OR(e) => e.addressing_mode.clone(),
            Instructions::ADD(e) => e.addressing_mode.clone(),
            Instructions::SUB(e) => e.addressing_mode.clone(),
            Instructions::MUL(e) => e.addressing_mode.clone(),
            Instructions::EXP(e) => e.addressing_mode.clone(),
            Instructions::DIV(e) => e.addressing_mode.clone(),
            Instructions::MOD(e) => e.addressing_mode.clone(),
            Instructions::JMP(e) => e.addressing_mode.clone(),
            Instructions::JMPA(e) => e.addressing_mode.clone(),
            Instructions::CALL(e) => e.addressing_mode.clone(),
            Instructions::RET(e) => e.addressing_mode.clone(),
            Instructions::PUSH(e) => e.addressing_mode.clone(),
            Instructions::SPUS(e) => e.addressing_mode.clone(),
            Instructions::LEN(e) => e.addressing_mode.clone(),
            Instructions::A2I(e) => e.addressing_mode.clone(),
            Instructions::A2F(e) => e.addressing_mode.clone(),
            Instructions::A2D(e) => e.addressing_mode.clone(),
            Instructions::A2B(e) => e.addressing_mode.clone(),
            Instructions::A2S(e) => e.addressing_mode.clone(),
            Instructions::A2C(e) => e.addressing_mode.clone(),
            Instructions::A2O(e) => e.addressing_mode.clone(),
            Instructions::ARR(e) => e.addressing_mode.clone(),
            Instructions::STR(e) => e.addressing_mode.clone(),
            Instructions::SAR(e) => e.addressing_mode.clone(),
            Instructions::POPS(e) => e.addressing_mode.clone(),
            Instructions::BRK(e) => e.addressing_mode.clone(),
            Instructions::CALLN(e) => e.addressing_mode.clone(),
            Instructions::CO(e) => e.addressing_mode.clone(),
            Instructions::FN(e) => e.addressing_mode.clone(),
            Instructions::DEA(e) => e.addressing_mode.clone(),
        }
        .to_string()
    }

    pub fn get_arg(&self, platform_size: &PlatformArchitecture) -> Vec<u8> {
        match &self {
            Instructions::LDA(e) => e.addressing_mode.arg(platform_size),
            Instructions::LDB(e) => e.addressing_mode.arg(platform_size),
            Instructions::LDC(e) => e.addressing_mode.arg(platform_size),
            Instructions::LDX(e) => e.addressing_mode.arg(platform_size),
            Instructions::LDY(e) => e.addressing_mode.arg(platform_size),
            Instructions::STA(e) => e.addressing_mode.arg(platform_size),
            Instructions::STB(e) => e.addressing_mode.arg(platform_size),
            Instructions::STC(e) => e.addressing_mode.arg(platform_size),
            Instructions::STX(e) => e.addressing_mode.arg(platform_size),
            Instructions::STY(e) => e.addressing_mode.arg(platform_size),
            Instructions::EQ(e) => e.addressing_mode.arg(platform_size),
            Instructions::NE(e) => e.addressing_mode.arg(platform_size),
            Instructions::GT(e) => e.addressing_mode.arg(platform_size),
            Instructions::LT(e) => e.addressing_mode.arg(platform_size),
            Instructions::GQ(e) => e.addressing_mode.arg(platform_size),
            Instructions::LQ(e) => e.addressing_mode.arg(platform_size),
            Instructions::AND(e) => e.addressing_mode.arg(platform_size),
            Instructions::OR(e) => e.addressing_mode.arg(platform_size),
            Instructions::ADD(e) => e.addressing_mode.arg(platform_size),
            Instructions::SUB(e) => e.addressing_mode.arg(platform_size),
            Instructions::MUL(e) => e.addressing_mode.arg(platform_size),
            Instructions::EXP(e) => e.addressing_mode.arg(platform_size),
            Instructions::DIV(e) => e.addressing_mode.arg(platform_size),
            Instructions::MOD(e) => e.addressing_mode.arg(platform_size),
            Instructions::JMP(e) => e.addressing_mode.arg(platform_size),
            Instructions::JMPA(e) => e.addressing_mode.arg(platform_size),
            Instructions::CALL(e) => e.addressing_mode.arg(platform_size),
            Instructions::RET(e) => e.addressing_mode.arg(platform_size),
            Instructions::PUSH(e) => e.addressing_mode.arg(platform_size),
            Instructions::SPUS(e) => e.addressing_mode.arg(platform_size),
            Instructions::LEN(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2I(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2F(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2D(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2B(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2S(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2C(e) => e.addressing_mode.arg(platform_size),
            Instructions::A2O(e) => e.addressing_mode.arg(platform_size),
            Instructions::ARR(e) => e.addressing_mode.arg(platform_size),
            Instructions::STR(e) => e.addressing_mode.arg(platform_size),
            Instructions::SAR(e) => e.addressing_mode.arg(platform_size),
            Instructions::POPS(e) => e.addressing_mode.arg(platform_size),
            Instructions::BRK(e) => e.addressing_mode.arg(platform_size),
            Instructions::CALLN(e) => e.addressing_mode.arg(platform_size),
            Instructions::CO(e) => e.addressing_mode.arg(platform_size),
            Instructions::FN(e) => e.addressing_mode.arg(platform_size),
            Instructions::DEA(e) => e.addressing_mode.arg(platform_size),
        }
    }
}

impl core::fmt::Display for Instructions {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match &self {
            Instructions::LDA(e) => write!(f, "LDA {}", e.addressing_mode),
            Instructions::LDB(e) => write!(f, "LDB {}", e.addressing_mode),
            Instructions::LDC(e) => write!(f, "LDC {}", e.addressing_mode),
            Instructions::LDX(e) => write!(f, "LDX {}", e.addressing_mode),
            Instructions::LDY(e) => write!(f, "LDY {}", e.addressing_mode),
            Instructions::STA(e) => write!(f, "STA {}", e.addressing_mode),
            Instructions::STB(e) => write!(f, "STB {}", e.addressing_mode),
            Instructions::STC(e) => write!(f, "STC {}", e.addressing_mode),
            Instructions::STX(e) => write!(f, "STX {}", e.addressing_mode),
            Instructions::STY(e) => write!(f, "STY {}", e.addressing_mode),
            Instructions::EQ(e) => write!(f, "EQ {}", e.addressing_mode),
            Instructions::NE(e) => write!(f, "NE {}", e.addressing_mode),
            Instructions::GT(e) => write!(f, "GT {}", e.addressing_mode),
            Instructions::LT(e) => write!(f, "LT {}", e.addressing_mode),
            Instructions::GQ(e) => write!(f, "GQ {}", e.addressing_mode),
            Instructions::LQ(e) => write!(f, "LQ {}", e.addressing_mode),
            Instructions::AND(e) => write!(f, "AND {}", e.addressing_mode),
            Instructions::OR(e) => write!(f, "OR {}", e.addressing_mode),
            Instructions::ADD(e) => write!(f, "ADD {}", e.addressing_mode),
            Instructions::SUB(e) => write!(f, "SUB {}", e.addressing_mode),
            Instructions::MUL(e) => write!(f, "MUL {}", e.addressing_mode),
            Instructions::EXP(e) => write!(f, "EXP {}", e.addressing_mode),
            Instructions::DIV(e) => write!(f, "DIV {}", e.addressing_mode),
            Instructions::MOD(e) => write!(f, "MOD {}", e.addressing_mode),
            Instructions::JMP(e) => write!(f, "JMP {}", e.addressing_mode),
            Instructions::JMPA(e) => write!(f, "JMPA {}", e.addressing_mode),
            Instructions::CALL(e) => write!(f, "CALL {}", e.addressing_mode),
            Instructions::RET(e) => write!(f, "RET {}", e.addressing_mode),
            Instructions::PUSH(e) => write!(f, "PUSH {}", e.addressing_mode),
            Instructions::SPUS(e) => write!(f, "SPUS {}", e.addressing_mode),
            Instructions::LEN(e) => write!(f, "LEN {}", e.addressing_mode),
            Instructions::A2I(e) => write!(f, "A2I {}", e.addressing_mode),
            Instructions::A2F(e) => write!(f, "A2F {}", e.addressing_mode),
            Instructions::A2D(e) => write!(f, "A2D {}", e.addressing_mode),
            Instructions::A2B(e) => write!(f, "A2B {}", e.addressing_mode),
            Instructions::A2S(e) => write!(f, "A2S {}", e.addressing_mode),
            Instructions::A2C(e) => write!(f, "A2C {}", e.addressing_mode),
            Instructions::A2O(e) => write!(f, "A2O {}", e.addressing_mode),
            Instructions::ARR(e) => write!(f, "ARR {}", e.addressing_mode),
            Instructions::STR(e) => write!(f, "STR {}", e.addressing_mode),
            Instructions::SAR(e) => write!(f, "SAR {}", e.addressing_mode),
            Instructions::POPS(e) => write!(f, "POPS {}", e.addressing_mode),
            Instructions::BRK(e) => write!(f, "BRK {}", e.addressing_mode),
            Instructions::CALLN(e) => write!(f, "CALLN {}", e.addressing_mode),
            Instructions::CO(e) => write!(f, "CO {}", e.addressing_mode),
            Instructions::FN(e) => write!(f, "FN {}", e.addressing_mode),
            Instructions::DEA(e) => write!(f, "DEA {}", e.addressing_mode),
        }
    }
}

