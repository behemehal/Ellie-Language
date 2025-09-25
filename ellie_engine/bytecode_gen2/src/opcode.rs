use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(u8)]
#[derive(IntoPrimitive, TryFromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    Mov,
    Add,
    Sub,
    Mul,
    Div,
    Load,
    Store,
    Push,
    Pop,
    Call,
    Ret,
    Jmp, // Unconditional jump
    Je,  // Jump if equal (A == B)
    Jne, // Jump if not equal (A != B)
    Jg,  // Jump if greater (A > B)
    Jge, // Jump if greater or equal (A >= B)
    Jl,  // Jump if less (A < B)
    Jle, // Jump if less or equal (A <= B)
}

impl OpCode {
    pub fn to_string(&self) -> &'static str {
        match self {
            OpCode::Mov => "MOV",
            OpCode::Add => "ADD",
            OpCode::Sub => "SUB",
            OpCode::Mul => "MUL",
            OpCode::Div => "DIV",
            OpCode::Load => "LOAD",
            OpCode::Store => "STORE",
            OpCode::Push => "PUSH",
            OpCode::Pop => "POP",
            OpCode::Call => "CALL",
            OpCode::Ret => "RET",
            OpCode::Jmp => "JMP",
            OpCode::Je => "JE",
            OpCode::Jne => "JNE",
            OpCode::Jg => "JG",
            OpCode::Jge => "JGE",
            OpCode::Jl => "JL",
            OpCode::Jle => "JLE",
        }
    }
}