//Auto generated from `instructions.json` by `reAssembler.js rev: 3

use crate::{
    heap_memory::HeapMemory,
    instructions::{ExecuterPanic, ExecuterResult, InstructionExecuter, StaticProgram},
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingModes, AddressingValues},
};
use ellie_core::defs::PlatformArchitecture;

#[derive(Clone, Copy, Debug)]
pub struct LDA {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct LDB {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct LDC {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct LDX {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct LDY {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct STA {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct STB {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct STC {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct STX {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct STY {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct EQ {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct NE {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct GT {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct LT {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct GQ {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct LQ {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct AND {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct OR {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct ADD {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct SUB {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct MUL {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct EXP {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct DIV {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct MOD {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct JMP {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct JMPA {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct CALL {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct RET {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct PUSH {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct SPUS {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct LEN {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct A2I {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct A2F {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct A2D {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct A2B {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct A2S {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct A2C {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct A2O {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct ARR {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct STR {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct SAR {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct POPS {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct BRK {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct CALLN {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct CO {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct FN {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub struct DEA {
    pub addressing_mode: AddressingModes,
}

#[derive(Clone, Copy, Debug)]
pub enum Instructions {
    LDA(LDA),
    LDB(LDB),
    LDC(LDC),
    LDX(LDX),
    LDY(LDY),
    STA(STA),
    STB(STB),
    STC(STC),
    STX(STX),
    STY(STY),
    EQ(EQ),
    NE(NE),
    GT(GT),
    LT(LT),
    GQ(GQ),
    LQ(LQ),
    AND(AND),
    OR(OR),
    ADD(ADD),
    SUB(SUB),
    MUL(MUL),
    EXP(EXP),
    DIV(DIV),
    MOD(MOD),
    JMP(JMP),
    JMPA(JMPA),
    CALL(CALL),
    RET(RET),
    PUSH(PUSH),
    SPUS(SPUS),
    LEN(LEN),
    A2I(A2I),
    A2F(A2F),
    A2D(A2D),
    A2B(A2B),
    A2S(A2S),
    A2C(A2C),
    A2O(A2O),
    ARR(ARR),
    STR(STR),
    SAR(SAR),
    POPS(POPS),
    BRK(BRK),
    CALLN(CALLN),
    CO(CO),
    FN(FN),
    DEA(DEA),
}

impl Instructions {
    pub fn from(op_code: &u8) -> Option<Self> {
        match op_code {
            1 => Some(Instructions::LDA(LDA {
                addressing_mode: AddressingModes::Immediate,
            })),
            2 => Some(Instructions::LDA(LDA {
                addressing_mode: AddressingModes::Absolute,
            })),
            3 => Some(Instructions::LDA(LDA {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            4 => Some(Instructions::LDA(LDA {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            5 => Some(Instructions::LDA(LDA {
                addressing_mode: AddressingModes::AbsoluteStatic,
            })),
            6 => Some(Instructions::LDA(LDA {
                addressing_mode: AddressingModes::IndirectB,
            })),
            7 => Some(Instructions::LDA(LDA {
                addressing_mode: AddressingModes::IndirectC,
            })),
            8 => Some(Instructions::LDA(LDA {
                addressing_mode: AddressingModes::IndirectX,
            })),
            9 => Some(Instructions::LDA(LDA {
                addressing_mode: AddressingModes::IndirectY,
            })),
            10 => Some(Instructions::LDB(LDB {
                addressing_mode: AddressingModes::Immediate,
            })),
            11 => Some(Instructions::LDB(LDB {
                addressing_mode: AddressingModes::Absolute,
            })),
            12 => Some(Instructions::LDB(LDB {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            13 => Some(Instructions::LDB(LDB {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            14 => Some(Instructions::LDB(LDB {
                addressing_mode: AddressingModes::AbsoluteStatic,
            })),
            15 => Some(Instructions::LDB(LDB {
                addressing_mode: AddressingModes::IndirectA,
            })),
            16 => Some(Instructions::LDB(LDB {
                addressing_mode: AddressingModes::IndirectC,
            })),
            17 => Some(Instructions::LDB(LDB {
                addressing_mode: AddressingModes::IndirectX,
            })),
            18 => Some(Instructions::LDB(LDB {
                addressing_mode: AddressingModes::IndirectY,
            })),
            19 => Some(Instructions::LDC(LDC {
                addressing_mode: AddressingModes::Immediate,
            })),
            20 => Some(Instructions::LDC(LDC {
                addressing_mode: AddressingModes::Absolute,
            })),
            21 => Some(Instructions::LDC(LDC {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            22 => Some(Instructions::LDC(LDC {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            23 => Some(Instructions::LDC(LDC {
                addressing_mode: AddressingModes::AbsoluteStatic,
            })),
            24 => Some(Instructions::LDC(LDC {
                addressing_mode: AddressingModes::IndirectA,
            })),
            25 => Some(Instructions::LDC(LDC {
                addressing_mode: AddressingModes::IndirectB,
            })),
            26 => Some(Instructions::LDC(LDC {
                addressing_mode: AddressingModes::IndirectX,
            })),
            27 => Some(Instructions::LDC(LDC {
                addressing_mode: AddressingModes::IndirectY,
            })),
            28 => Some(Instructions::LDX(LDX {
                addressing_mode: AddressingModes::Immediate,
            })),
            29 => Some(Instructions::LDX(LDX {
                addressing_mode: AddressingModes::Absolute,
            })),
            30 => Some(Instructions::LDX(LDX {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            31 => Some(Instructions::LDX(LDX {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            32 => Some(Instructions::LDX(LDX {
                addressing_mode: AddressingModes::AbsoluteStatic,
            })),
            33 => Some(Instructions::LDX(LDX {
                addressing_mode: AddressingModes::IndirectA,
            })),
            34 => Some(Instructions::LDX(LDX {
                addressing_mode: AddressingModes::IndirectB,
            })),
            35 => Some(Instructions::LDX(LDX {
                addressing_mode: AddressingModes::IndirectC,
            })),
            36 => Some(Instructions::LDX(LDX {
                addressing_mode: AddressingModes::IndirectY,
            })),
            37 => Some(Instructions::LDY(LDY {
                addressing_mode: AddressingModes::Immediate,
            })),
            38 => Some(Instructions::LDY(LDY {
                addressing_mode: AddressingModes::Absolute,
            })),
            39 => Some(Instructions::LDY(LDY {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            40 => Some(Instructions::LDY(LDY {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            41 => Some(Instructions::LDY(LDY {
                addressing_mode: AddressingModes::AbsoluteStatic,
            })),
            42 => Some(Instructions::LDY(LDY {
                addressing_mode: AddressingModes::IndirectA,
            })),
            43 => Some(Instructions::LDY(LDY {
                addressing_mode: AddressingModes::IndirectB,
            })),
            44 => Some(Instructions::LDY(LDY {
                addressing_mode: AddressingModes::IndirectC,
            })),
            45 => Some(Instructions::LDY(LDY {
                addressing_mode: AddressingModes::IndirectX,
            })),
            46 => Some(Instructions::STA(STA {
                addressing_mode: AddressingModes::Implicit,
            })),
            47 => Some(Instructions::STA(STA {
                addressing_mode: AddressingModes::Immediate,
            })),
            48 => Some(Instructions::STA(STA {
                addressing_mode: AddressingModes::Absolute,
            })),
            49 => Some(Instructions::STA(STA {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            50 => Some(Instructions::STA(STA {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            51 => Some(Instructions::STB(STB {
                addressing_mode: AddressingModes::Implicit,
            })),
            52 => Some(Instructions::STB(STB {
                addressing_mode: AddressingModes::Immediate,
            })),
            53 => Some(Instructions::STB(STB {
                addressing_mode: AddressingModes::Absolute,
            })),
            54 => Some(Instructions::STB(STB {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            55 => Some(Instructions::STB(STB {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            56 => Some(Instructions::STC(STC {
                addressing_mode: AddressingModes::Implicit,
            })),
            57 => Some(Instructions::STC(STC {
                addressing_mode: AddressingModes::Immediate,
            })),
            58 => Some(Instructions::STC(STC {
                addressing_mode: AddressingModes::Absolute,
            })),
            59 => Some(Instructions::STC(STC {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            60 => Some(Instructions::STC(STC {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            61 => Some(Instructions::STX(STX {
                addressing_mode: AddressingModes::Implicit,
            })),
            62 => Some(Instructions::STX(STX {
                addressing_mode: AddressingModes::Immediate,
            })),
            63 => Some(Instructions::STX(STX {
                addressing_mode: AddressingModes::Absolute,
            })),
            64 => Some(Instructions::STX(STX {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            65 => Some(Instructions::STX(STX {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            66 => Some(Instructions::STY(STY {
                addressing_mode: AddressingModes::Implicit,
            })),
            67 => Some(Instructions::STY(STY {
                addressing_mode: AddressingModes::Immediate,
            })),
            68 => Some(Instructions::STY(STY {
                addressing_mode: AddressingModes::Absolute,
            })),
            69 => Some(Instructions::STY(STY {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            70 => Some(Instructions::STY(STY {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            71 => Some(Instructions::EQ(EQ {
                addressing_mode: AddressingModes::Implicit,
            })),
            72 => Some(Instructions::NE(NE {
                addressing_mode: AddressingModes::Implicit,
            })),
            73 => Some(Instructions::GT(GT {
                addressing_mode: AddressingModes::Implicit,
            })),
            74 => Some(Instructions::LT(LT {
                addressing_mode: AddressingModes::Implicit,
            })),
            75 => Some(Instructions::GQ(GQ {
                addressing_mode: AddressingModes::Implicit,
            })),
            76 => Some(Instructions::LQ(LQ {
                addressing_mode: AddressingModes::Implicit,
            })),
            77 => Some(Instructions::AND(AND {
                addressing_mode: AddressingModes::Implicit,
            })),
            78 => Some(Instructions::OR(OR {
                addressing_mode: AddressingModes::Implicit,
            })),
            79 => Some(Instructions::ADD(ADD {
                addressing_mode: AddressingModes::Implicit,
            })),
            80 => Some(Instructions::SUB(SUB {
                addressing_mode: AddressingModes::Implicit,
            })),
            81 => Some(Instructions::MUL(MUL {
                addressing_mode: AddressingModes::Implicit,
            })),
            82 => Some(Instructions::EXP(EXP {
                addressing_mode: AddressingModes::Implicit,
            })),
            83 => Some(Instructions::DIV(DIV {
                addressing_mode: AddressingModes::Implicit,
            })),
            84 => Some(Instructions::MOD(MOD {
                addressing_mode: AddressingModes::Implicit,
            })),
            85 => Some(Instructions::JMP(JMP {
                addressing_mode: AddressingModes::Absolute,
            })),
            86 => Some(Instructions::JMPA(JMPA {
                addressing_mode: AddressingModes::Absolute,
            })),
            87 => Some(Instructions::CALL(CALL {
                addressing_mode: AddressingModes::Absolute,
            })),
            88 => Some(Instructions::RET(RET {
                addressing_mode: AddressingModes::Implicit,
            })),
            89 => Some(Instructions::PUSH(PUSH {
                addressing_mode: AddressingModes::Absolute,
            })),
            90 => Some(Instructions::PUSH(PUSH {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            91 => Some(Instructions::PUSH(PUSH {
                addressing_mode: AddressingModes::IndirectA,
            })),
            92 => Some(Instructions::PUSH(PUSH {
                addressing_mode: AddressingModes::IndirectB,
            })),
            93 => Some(Instructions::PUSH(PUSH {
                addressing_mode: AddressingModes::IndirectC,
            })),
            94 => Some(Instructions::PUSH(PUSH {
                addressing_mode: AddressingModes::IndirectX,
            })),
            95 => Some(Instructions::PUSH(PUSH {
                addressing_mode: AddressingModes::IndirectY,
            })),
            96 => Some(Instructions::SPUS(SPUS {
                addressing_mode: AddressingModes::Absolute,
            })),
            97 => Some(Instructions::SPUS(SPUS {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            98 => Some(Instructions::SPUS(SPUS {
                addressing_mode: AddressingModes::IndirectA,
            })),
            99 => Some(Instructions::SPUS(SPUS {
                addressing_mode: AddressingModes::IndirectB,
            })),
            100 => Some(Instructions::SPUS(SPUS {
                addressing_mode: AddressingModes::IndirectC,
            })),
            101 => Some(Instructions::SPUS(SPUS {
                addressing_mode: AddressingModes::IndirectX,
            })),
            102 => Some(Instructions::SPUS(SPUS {
                addressing_mode: AddressingModes::IndirectY,
            })),
            103 => Some(Instructions::LEN(LEN {
                addressing_mode: AddressingModes::Absolute,
            })),
            104 => Some(Instructions::A2I(A2I {
                addressing_mode: AddressingModes::Implicit,
            })),
            105 => Some(Instructions::A2F(A2F {
                addressing_mode: AddressingModes::Implicit,
            })),
            106 => Some(Instructions::A2D(A2D {
                addressing_mode: AddressingModes::Implicit,
            })),
            107 => Some(Instructions::A2B(A2B {
                addressing_mode: AddressingModes::Implicit,
            })),
            108 => Some(Instructions::A2S(A2S {
                addressing_mode: AddressingModes::Implicit,
            })),
            109 => Some(Instructions::A2C(A2C {
                addressing_mode: AddressingModes::Implicit,
            })),
            110 => Some(Instructions::A2O(A2O {
                addressing_mode: AddressingModes::Implicit,
            })),
            111 => Some(Instructions::ARR(ARR {
                addressing_mode: AddressingModes::Implicit,
            })),
            112 => Some(Instructions::STR(STR {
                addressing_mode: AddressingModes::Implicit,
            })),
            113 => Some(Instructions::SAR(SAR {
                addressing_mode: AddressingModes::Immediate,
            })),
            114 => Some(Instructions::POPS(POPS {
                addressing_mode: AddressingModes::Absolute,
            })),
            115 => Some(Instructions::BRK(BRK {
                addressing_mode: AddressingModes::Implicit,
            })),
            116 => Some(Instructions::CALLN(CALLN {
                addressing_mode: AddressingModes::Absolute,
            })),
            117 => Some(Instructions::CO(CO {
                addressing_mode: AddressingModes::Absolute,
            })),
            118 => Some(Instructions::FN(FN {
                addressing_mode: AddressingModes::Immediate,
            })),
            119 => Some(Instructions::DEA(DEA {
                addressing_mode: AddressingModes::Absolute,
            })),
            _ => None,
        }
    }

    pub fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match &self {
            Instructions::LDA(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::LDB(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::LDC(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::LDX(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::LDY(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::STA(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::STB(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::STC(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::STX(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::STY(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::EQ(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::NE(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::GT(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::LT(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::GQ(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::LQ(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::AND(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::OR(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::ADD(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::SUB(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::MUL(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::EXP(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::DIV(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::MOD(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::JMP(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::JMPA(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::CALL(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::RET(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::PUSH(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::SPUS(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::LEN(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::A2I(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::A2F(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::A2D(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::A2B(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::A2S(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::A2C(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::A2O(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::ARR(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::STR(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::SAR(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::POPS(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::BRK(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::CALLN(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::CO(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::FN(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
            Instructions::DEA(e) => e.execute(
                heap_memory,
                program,
                current_stack,
                stack_memory,
                addressing_value,
                arch,
            ),
        }
    }
    pub fn addressing_mode(&self) -> AddressingModes {
        match &self {
            Instructions::LDA(e) => e.addressing_mode,
            Instructions::LDB(e) => e.addressing_mode,
            Instructions::LDC(e) => e.addressing_mode,
            Instructions::LDX(e) => e.addressing_mode,
            Instructions::LDY(e) => e.addressing_mode,
            Instructions::STA(e) => e.addressing_mode,
            Instructions::STB(e) => e.addressing_mode,
            Instructions::STC(e) => e.addressing_mode,
            Instructions::STX(e) => e.addressing_mode,
            Instructions::STY(e) => e.addressing_mode,
            Instructions::EQ(e) => e.addressing_mode,
            Instructions::NE(e) => e.addressing_mode,
            Instructions::GT(e) => e.addressing_mode,
            Instructions::LT(e) => e.addressing_mode,
            Instructions::GQ(e) => e.addressing_mode,
            Instructions::LQ(e) => e.addressing_mode,
            Instructions::AND(e) => e.addressing_mode,
            Instructions::OR(e) => e.addressing_mode,
            Instructions::ADD(e) => e.addressing_mode,
            Instructions::SUB(e) => e.addressing_mode,
            Instructions::MUL(e) => e.addressing_mode,
            Instructions::EXP(e) => e.addressing_mode,
            Instructions::DIV(e) => e.addressing_mode,
            Instructions::MOD(e) => e.addressing_mode,
            Instructions::JMP(e) => e.addressing_mode,
            Instructions::JMPA(e) => e.addressing_mode,
            Instructions::CALL(e) => e.addressing_mode,
            Instructions::RET(e) => e.addressing_mode,
            Instructions::PUSH(e) => e.addressing_mode,
            Instructions::SPUS(e) => e.addressing_mode,
            Instructions::LEN(e) => e.addressing_mode,
            Instructions::A2I(e) => e.addressing_mode,
            Instructions::A2F(e) => e.addressing_mode,
            Instructions::A2D(e) => e.addressing_mode,
            Instructions::A2B(e) => e.addressing_mode,
            Instructions::A2S(e) => e.addressing_mode,
            Instructions::A2C(e) => e.addressing_mode,
            Instructions::A2O(e) => e.addressing_mode,
            Instructions::ARR(e) => e.addressing_mode,
            Instructions::STR(e) => e.addressing_mode,
            Instructions::SAR(e) => e.addressing_mode,
            Instructions::POPS(e) => e.addressing_mode,
            Instructions::BRK(e) => e.addressing_mode,
            Instructions::CALLN(e) => e.addressing_mode,
            Instructions::CO(e) => e.addressing_mode,
            Instructions::FN(e) => e.addressing_mode,
            Instructions::DEA(e) => e.addressing_mode,
        }
    }
}
