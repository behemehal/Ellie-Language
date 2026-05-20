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
    Jmp,   // Unconditional jump
    Je,    // Jump if equal (A == B)
    Jne,   // Jump if not equal (A != B)
    Jg,    // Jump if greater (A > B)
    Jge,   // Jump if greater or equal (A >= B)
    Jl,    // Jump if less (A < B)
    Jle,   // Jump if less or equal (A <= B)
    Cmp,   // Compare operands, set flags
    CallN, // Call native function by hash
    Fn,    // Function declaration marker with arg count
    Cast,  // Type cast: operand0=src register, operand1=target type id
    Str,   // Allocate string on heap; operand0=dest register
    Spus,  // Push char to string accumulator
    Res,   // Reserve N stack slots: advance SP by operand0 (no writes)
    Co,    // Allocate class instance: Co #n_fields → heap ref in A
    Mod,   // Modulo (A % B)
    Exp,   // Exponentiation (A ** B)
    Gfld,  // Get class field: Gfld A, #field_idx → A = heap[A].fields[field_idx]
    Sfld,  // Set class field: Sfld A, #field_idx → heap[A].fields[field_idx] = B
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
            OpCode::Cmp => "CMP",
            OpCode::CallN => "CALLN",
            OpCode::Fn => "FN",
            OpCode::Cast => "CAST",
            OpCode::Str => "STR",
            OpCode::Spus => "SPUS",
            OpCode::Res => "RES",
            OpCode::Co => "CO",
            OpCode::Mod => "MOD",
            OpCode::Exp => "EXP",
            OpCode::Gfld => "GFLD",
            OpCode::Sfld => "SFLD",
        }
    }
}