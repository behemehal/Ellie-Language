use ellie_bytecode_gen2::operand::{AddressingModes, Registers};
use ellie_core::bytecode::{RawType, TypeId};

use crate::{
    heap::Heap,
    module::{Gen2Value, ModuleManager},
    program::Program,
};

#[derive(Debug)]
pub enum VmExit {
    Ok,
    Panic(String),
}

pub struct Thread {
    // General-purpose registers
    pub reg_a: RawType,
    pub reg_b: RawType,
    pub reg_c: RawType,
    pub reg_x: RawType,
    pub reg_y: RawType,
    // Stack pointer (= stack.len(); next free slot index)
    pub sp: usize,
    // Frame pointer (base of current frame)
    pub fp: usize,
    // Program counter
    pub pc: usize,
    // Data stack
    pub stack: Vec<RawType>,
    // Shadow call stack for return addresses
    pub call_stack: Vec<usize>,
    // Comparison flags
    pub flag_eq: bool,
    pub flag_lt: bool,
    pub flag_gt: bool,
    // Heap
    pub heap: Heap,
}

impl Thread {
    fn zero_raw() -> RawType {
        RawType {
            type_id: TypeId::Int,
            size: 0,
            data: [0; 8],
        }
    }

    pub fn new() -> Self {
        let zero = Self::zero_raw();
        Thread {
            reg_a: zero,
            reg_b: zero,
            reg_c: zero,
            reg_x: zero,
            reg_y: zero,
            sp: 0,
            fp: 0,
            pc: 0,
            stack: Vec::new(),
            call_stack: Vec::new(),
            flag_eq: false,
            flag_lt: false,
            flag_gt: false,
            heap: Heap::new(),
        }
    }

    fn get_reg(&self, reg: Registers) -> RawType {
        match reg {
            Registers::A => self.reg_a,
            Registers::B => self.reg_b,
            Registers::C => self.reg_c,
            Registers::X => self.reg_x,
            Registers::Y => self.reg_y,
            Registers::SP => {
                let mut r = Self::zero_raw();
                let b = self.sp.to_le_bytes();
                r.data[..8].copy_from_slice(&b);
                r.type_id = TypeId::Int;
                r.size = 8;
                r
            }
            Registers::FP => {
                let mut r = Self::zero_raw();
                let b = self.fp.to_le_bytes();
                r.data[..8].copy_from_slice(&b);
                r.type_id = TypeId::Int;
                r.size = 8;
                r
            }
            Registers::PC => {
                let mut r = Self::zero_raw();
                let b = self.pc.to_le_bytes();
                r.data[..8].copy_from_slice(&b);
                r.type_id = TypeId::Int;
                r.size = 8;
                r
            }
        }
    }

    fn set_reg(&mut self, reg: Registers, val: RawType) {
        match reg {
            Registers::A => self.reg_a = val,
            Registers::B => self.reg_b = val,
            Registers::C => self.reg_c = val,
            Registers::X => self.reg_x = val,
            Registers::Y => self.reg_y = val,
            Registers::SP => {
                self.sp = usize::from_le_bytes(val.data);
                // Truncate or extend the stack to match new SP
                self.stack.resize(self.sp, Self::zero_raw());
            }
            Registers::FP => {
                self.fp = usize::from_le_bytes(val.data);
            }
            Registers::PC => {
                self.pc = usize::from_le_bytes(val.data);
            }
        }
    }

    /// Resolve an operand to a RawType value.
    fn resolve(&self, op: &ellie_bytecode_gen2::operand::Operand) -> RawType {
        match op.mode {
            AddressingModes::Immediate => op.immediate.unwrap(),
            AddressingModes::Register => self.get_reg(op.register.unwrap()),
            AddressingModes::Indirect => {
                let reg_val: usize = usize::from_le_bytes(self.get_reg(op.register.unwrap()).data);
                *self.stack.get(reg_val).unwrap_or(&Self::zero_raw())
            }
            AddressingModes::IndirectOffset => {
                let base = usize::from_le_bytes(self.get_reg(op.register.unwrap()).data);
                let offset: isize = op.immediate.unwrap().into();
                let idx = (base as isize + offset) as usize;
                *self.stack.get(idx).unwrap_or(&Self::zero_raw())
            }
            AddressingModes::Direct => {
                let addr: usize = op.immediate.unwrap().into();
                *self.stack.get(addr).unwrap_or(&Self::zero_raw())
            }
            AddressingModes::Implicit => Self::zero_raw(),
        }
    }

    /// Write a value to the location described by an operand.
    fn write_dst(&mut self, op: &ellie_bytecode_gen2::operand::Operand, val: RawType) {
        match op.mode {
            AddressingModes::Register => {
                let reg = op.register.unwrap();
                self.set_reg(reg, val);
            }
            AddressingModes::Indirect => {
                let idx = usize::from_le_bytes(self.get_reg(op.register.unwrap()).data);
                if idx < self.stack.len() {
                    self.stack[idx] = val;
                }
            }
            AddressingModes::IndirectOffset => {
                let base = usize::from_le_bytes(self.get_reg(op.register.unwrap()).data);
                let offset: isize = op.immediate.unwrap().into();
                let idx = (base as isize + offset) as usize;
                if idx < self.stack.len() {
                    self.stack[idx] = val;
                }
            }
            AddressingModes::Direct => {
                let addr: usize = op.immediate.unwrap().into();
                if addr < self.stack.len() {
                    self.stack[addr] = val;
                }
            }
            _ => {}
        }
    }

    fn push_stack(&mut self, val: RawType) {
        if self.stack.len() == self.sp {
            self.stack.push(val);
        } else {
            // SP might be less than stack.len() after a Mov SP, FP truncation
            self.stack.truncate(self.sp);
            self.stack.push(val);
        }
        self.sp += 1;
    }

    fn pop_stack(&mut self) -> Option<RawType> {
        if self.sp == 0 {
            return None;
        }
        self.sp -= 1;
        self.stack.truncate(self.sp + 1);
        self.stack.pop()
    }

    fn add_raw(&mut self, a: RawType, b: RawType) -> Result<RawType, String> {
        // In `Add A, B`, A is the RIGHT operand and B is the LEFT operand
        // (transpiler pushes left first, computes right into A, pops B as left).
        // String concatenation result = LEFT + RIGHT = str(b) + str(a).
        if a.type_id == TypeId::Reference || b.type_id == TypeId::Reference {
            let str_left;
            let str_right;
            if b.type_id == TypeId::Reference {
                let idx: usize = b.into();
                str_left = self
                    .heap
                    .get_string(idx)
                    .ok_or_else(|| format!("Invalid heap ref {}", idx))?
                    .to_string();
                str_right = raw_to_string(a, &self.heap);
            } else {
                // a is reference
                let idx: usize = a.into();
                str_right = self
                    .heap
                    .get_string(idx)
                    .ok_or_else(|| format!("Invalid heap ref {}", idx))?
                    .to_string();
                str_left = raw_to_string(b, &self.heap);
            }
            let new_str = str_left + &str_right;
            let new_idx = self.heap.alloc_string(new_str);
            let mut r = Self::zero_raw();
            r.type_id = TypeId::Reference;
            r.size = 8;
            r.data.copy_from_slice(&new_idx.to_le_bytes());
            return Ok(r);
        }
        // Numeric add
        numeric_binop(a, b, |x, y| x + y)
    }

    pub fn run(mut self, program: &Program, modules: &ModuleManager) -> (VmExit, Thread) {
        self.pc = program.main_start;

        loop {
            if self.pc >= program.instructions.len() {
                return (VmExit::Ok, self);
            }
            let instr = program.instructions[self.pc].clone();
            let op0 = instr.operand_0.as_ref();
            let op1 = instr.operand_1.as_ref();

            use ellie_bytecode_gen2::opcode::OpCode;

            match instr.op_code {
                // -----------------------------------------------------------------
                // Control flow
                // -----------------------------------------------------------------
                OpCode::Fn => {
                    // Just a marker — skip it (never reached via Call, but main starts at it)
                    self.pc += 1;
                }
                OpCode::Ret => {
                    match self.call_stack.pop() {
                        Some(ret_pc) => self.pc = ret_pc,
                        None => return (VmExit::Ok, self),
                    }
                }
                OpCode::Call => {
                    let hash: usize = self.resolve(op0.unwrap()).into();
                    let target = match program.fn_table.get(&hash) {
                        Some(&t) => t,
                        None => {
                            return (
                                VmExit::Panic(format!("Call to unknown function hash {}", hash)),
                                self,
                            )
                        }
                    };
                    self.call_stack.push(self.pc + 1);
                    self.pc = target;
                }
                OpCode::CallN => {
                    let hash: usize = self.resolve(op0.unwrap()).into();
                    let arg_count: usize = self.resolve(op1.unwrap()).into();

                    // Collect top arg_count items from stack (they were pushed left→right)
                    let stack_len = self.sp;
                    let args_start = if stack_len >= arg_count {
                        stack_len - arg_count
                    } else {
                        0
                    };
                    let args: Vec<RawType> = self.stack[args_start..self.sp].to_vec();

                    match modules.call(hash, &args, &mut self.heap) {
                        Some(result) => {
                            self.reg_a = gen2_value_to_raw(result, &mut self.heap);
                        }
                        None => {
                            // Look up trace to give a useful error
                            let name = program
                                .native_traces
                                .iter()
                                .find(|t| t.function_hash == hash)
                                .map(|t| t.function_name.as_str())
                                .unwrap_or("<unknown>");
                            return (
                                VmExit::Panic(format!(
                                    "Missing native function '{}' (hash {})",
                                    name, hash
                                )),
                                self,
                            );
                        }
                    }
                    self.pc += 1;
                }
                OpCode::Jmp => {
                    let target: usize = self.resolve(op0.unwrap()).into();
                    self.pc = target;
                }
                OpCode::Je => {
                    let target: usize = self.resolve(op0.unwrap()).into();
                    if self.flag_eq {
                        self.pc = target;
                    } else {
                        self.pc += 1;
                    }
                }
                OpCode::Jne => {
                    let target: usize = self.resolve(op0.unwrap()).into();
                    if !self.flag_eq {
                        self.pc = target;
                    } else {
                        self.pc += 1;
                    }
                }
                OpCode::Jg => {
                    let target: usize = self.resolve(op0.unwrap()).into();
                    if self.flag_gt {
                        self.pc = target;
                    } else {
                        self.pc += 1;
                    }
                }
                OpCode::Jge => {
                    let target: usize = self.resolve(op0.unwrap()).into();
                    if self.flag_gt || self.flag_eq {
                        self.pc = target;
                    } else {
                        self.pc += 1;
                    }
                }
                OpCode::Jl => {
                    let target: usize = self.resolve(op0.unwrap()).into();
                    if self.flag_lt {
                        self.pc = target;
                    } else {
                        self.pc += 1;
                    }
                }
                OpCode::Jle => {
                    let target: usize = self.resolve(op0.unwrap()).into();
                    if self.flag_lt || self.flag_eq {
                        self.pc = target;
                    } else {
                        self.pc += 1;
                    }
                }
                // -----------------------------------------------------------------
                // Data movement
                // -----------------------------------------------------------------
                OpCode::Mov => {
                    match (op0, op1) {
                        (Some(dst), Some(src_op)) => {
                            // Special case: Mov SP, <src> → truncate stack
                            if dst.mode == AddressingModes::Register
                                && dst.register == Some(Registers::SP)
                            {
                                let src = self.resolve(src_op);
                                let new_sp: usize = usize::from_le_bytes(src.data);
                                self.sp = new_sp;
                                self.stack.truncate(new_sp);
                                self.pc += 1;
                                continue;
                            }
                            let src = self.resolve(src_op);
                            self.write_dst(dst, src);
                        }
                        (Some(imm_op), None) => {
                            // 1-operand form: load immediate into A
                            let val = self.resolve(imm_op);
                            self.reg_a = val;
                        }
                        _ => {}
                    }
                    self.pc += 1;
                }
                OpCode::Push => {
                    let val = self.resolve(op0.unwrap());
                    self.push_stack(val);
                    self.pc += 1;
                }
                OpCode::Pop => {
                    let val = self.pop_stack().unwrap_or(Self::zero_raw());
                    if let Some(op) = op0 {
                        self.write_dst(op, val);
                    }
                    self.pc += 1;
                }
                OpCode::Res => {
                    // Reserve N stack slots by advancing SP (no writes)
                    let n: usize = self.resolve(op0.unwrap()).into();
                    let zero = Self::zero_raw();
                    for _ in 0..n {
                        self.push_stack(zero);
                    }
                    self.pc += 1;
                }
                OpCode::Load => {
                    // Load from memory address into register — treat as Direct
                    let addr: usize = self.resolve(op1.unwrap()).into();
                    let val = *self.stack.get(addr).unwrap_or(&Self::zero_raw());
                    if let Some(op) = op0 {
                        self.write_dst(op, val);
                    }
                    self.pc += 1;
                }
                OpCode::Store => {
                    // Store register value into memory address
                    let val = self.resolve(op0.unwrap());
                    let addr: usize = self.resolve(op1.unwrap()).into();
                    if addr < self.stack.len() {
                        self.stack[addr] = val;
                    }
                    self.pc += 1;
                }
                // -----------------------------------------------------------------
                // Arithmetic
                // -----------------------------------------------------------------
                OpCode::Add => {
                    let a = self.resolve(op0.unwrap());
                    let b = self.resolve(op1.unwrap());
                    match self.add_raw(a, b) {
                        Ok(r) => self.write_dst(op0.unwrap(), r),
                        Err(e) => return (VmExit::Panic(e), self),
                    }
                    self.pc += 1;
                }
                OpCode::Sub => {
                    // Special case: Sub SP, #n → pop n items
                    if let (Some(op_dst), Some(op_src)) = (op0, op1) {
                        if op_dst.mode == AddressingModes::Register
                            && op_dst.register == Some(Registers::SP)
                        {
                            let n: usize = self.resolve(op_src).into();
                            self.sp = self.sp.saturating_sub(n);
                            self.stack.truncate(self.sp);
                            self.pc += 1;
                            continue;
                        }
                        let a = self.resolve(op_dst);
                        let b = self.resolve(op_src);
                        match numeric_binop(a, b, |x, y| x - y) {
                            Ok(r) => self.write_dst(op_dst, r),
                            Err(e) => return (VmExit::Panic(e), self),
                        }
                    }
                    self.pc += 1;
                }
                OpCode::Mul => {
                    let a = self.resolve(op0.unwrap());
                    let b = self.resolve(op1.unwrap());
                    match numeric_binop(a, b, |x, y| x * y) {
                        Ok(r) => self.write_dst(op0.unwrap(), r),
                        Err(e) => return (VmExit::Panic(e), self),
                    }
                    self.pc += 1;
                }
                OpCode::Div => {
                    let a = self.resolve(op0.unwrap());
                    let b = self.resolve(op1.unwrap());
                    let bv: isize = b.into();
                    if bv == 0 {
                        return (VmExit::Panic("Division by zero".to_string()), self);
                    }
                    match numeric_binop(a, b, |x, y| x / y) {
                        Ok(r) => self.write_dst(op0.unwrap(), r),
                        Err(e) => return (VmExit::Panic(e), self),
                    }
                    self.pc += 1;
                }
                OpCode::Mod => {
                    let a = self.resolve(op0.unwrap());
                    let b = self.resolve(op1.unwrap());
                    let bv: isize = b.into();
                    if bv == 0 {
                        return (VmExit::Panic("Modulo by zero".to_string()), self);
                    }
                    match numeric_binop(a, b, |x, y| x % y) {
                        Ok(r) => self.write_dst(op0.unwrap(), r),
                        Err(e) => return (VmExit::Panic(e), self),
                    }
                    self.pc += 1;
                }
                OpCode::Exp => {
                    let a = self.resolve(op0.unwrap());
                    let b = self.resolve(op1.unwrap());
                    let base: isize = a.into();
                    let exp: isize = b.into();
                    let result = if exp >= 0 {
                        base.pow(exp as u32)
                    } else {
                        0
                    };
                    let r: RawType = result.into();
                    self.write_dst(op0.unwrap(), r);
                    self.pc += 1;
                }
                // -----------------------------------------------------------------
                // Comparison
                // -----------------------------------------------------------------
                OpCode::Cmp => {
                    let a = self.resolve(op0.unwrap());
                    let b = self.resolve(op1.unwrap());
                    let av: isize = a.into();
                    let bv: isize = b.into();
                    self.flag_eq = av == bv;
                    self.flag_lt = av < bv;
                    self.flag_gt = av > bv;
                    self.pc += 1;
                }
                // -----------------------------------------------------------------
                // String operations
                // -----------------------------------------------------------------
                OpCode::Str => {
                    // Allocate empty string on heap, put heap index in destination register
                    let idx = self.heap.alloc_string(String::new());
                    let mut r = Self::zero_raw();
                    r.type_id = TypeId::Reference;
                    r.size = 8;
                    r.data.copy_from_slice(&idx.to_le_bytes());
                    if let Some(op) = op0 {
                        self.write_dst(op, r);
                    }
                    self.pc += 1;
                }
                OpCode::Spus => {
                    // Push a char onto a heap string.
                    // operand_0: register holding heap ref (destination string)
                    // operand_1: immediate char value
                    let heap_ref = self.resolve(op0.unwrap());
                    let idx: usize = heap_ref.into();
                    let char_raw = self.resolve(op1.unwrap());
                    let ch_code = u32::from_le_bytes(char_raw.data[..4].try_into().unwrap());
                    if let Some(ch) = char::from_u32(ch_code) {
                        if let Some(s) = self.heap.get_string_mut(idx) {
                            s.push(ch);
                        }
                    }
                    self.pc += 1;
                }
                // -----------------------------------------------------------------
                // Cast / misc
                // -----------------------------------------------------------------
                OpCode::Cast => {
                    let val = self.resolve(op0.unwrap());
                    let type_id_byte: u8 = {
                        let raw = self.resolve(op1.unwrap());
                        raw.data[0]
                    };
                    let target_type =
                        TypeId::try_from(type_id_byte).unwrap_or(TypeId::Int);
                    let casted = cast_raw(val, target_type);
                    if let Some(op) = op0 {
                        self.write_dst(op, casted);
                    }
                    self.pc += 1;
                }
                OpCode::Co => {
                    // Allocate class instance with n_fields slots; result in A
                    let n_fields: usize = self.resolve(op0.unwrap()).into();
                    let idx = self.heap.alloc_class_inst(n_fields);
                    let mut r = Self::zero_raw();
                    r.type_id = TypeId::Reference;
                    r.size = 8;
                    r.data.copy_from_slice(&idx.to_le_bytes());
                    self.reg_a = r;
                    self.pc += 1;
                }
                OpCode::Gfld => {
                    // Gfld A, #field_idx → A = heap[A].fields[field_idx]
                    let heap_ref: usize = self.resolve(op0.unwrap()).into();
                    let field_idx: usize = self.resolve(op1.unwrap()).into();
                    let val = self.heap.get_field(heap_ref, field_idx)
                        .unwrap_or(Self::zero_raw());
                    self.write_dst(op0.unwrap(), val);
                    self.pc += 1;
                }
                OpCode::Sfld => {
                    // Sfld A, #field_idx → heap[A].fields[field_idx] = B
                    let heap_ref: usize = self.resolve(op0.unwrap()).into();
                    let field_idx: usize = self.resolve(op1.unwrap()).into();
                    self.heap.set_field(heap_ref, field_idx, self.reg_b);
                    self.pc += 1;
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn numeric_binop(a: RawType, b: RawType, f: impl Fn(isize, isize) -> isize) -> Result<RawType, String> {
    let av: isize = a.into();
    let bv: isize = b.into();
    Ok(f(av, bv).into())
}

fn raw_to_string(r: RawType, heap: &Heap) -> String {
    match r.type_id {
        TypeId::Reference => {
            let idx: usize = r.into();
            heap.get_string(idx).unwrap_or("").to_string()
        }
        TypeId::Int => {
            let v: isize = r.into();
            v.to_string()
        }
        TypeId::Bool => {
            let v: u8 = r.into();
            if v != 0 { "true".to_string() } else { "false".to_string() }
        }
        TypeId::Byte => {
            let v: u8 = r.into();
            v.to_string()
        }
        TypeId::Char => {
            let code = u32::from_le_bytes(r.data[..4].try_into().unwrap());
            char::from_u32(code)
                .map(|c| c.to_string())
                .unwrap_or_default()
        }
        TypeId::Float => {
            let bits = u32::from_le_bytes(r.data[..4].try_into().unwrap());
            f32::from_bits(bits).to_string()
        }
        TypeId::Double => {
            let bits = u64::from_le_bytes(r.data);
            f64::from_bits(bits).to_string()
        }
        TypeId::Array => "[array]".to_string(),
    }
}

fn gen2_value_to_raw(val: Gen2Value, heap: &mut Heap) -> RawType {
    match val {
        Gen2Value::Raw(r) => r,
        Gen2Value::Str(s) => {
            let idx = heap.alloc_string(s);
            let mut r = RawType {
                type_id: TypeId::Reference,
                size: 8,
                data: [0; 8],
            };
            r.data.copy_from_slice(&idx.to_le_bytes());
            r
        }
        Gen2Value::Void => RawType {
            type_id: TypeId::Int,
            size: 0,
            data: [0; 8],
        },
    }
}

fn cast_raw(val: RawType, target: TypeId) -> RawType {
    if val.type_id == target {
        return val;
    }
    match (val.type_id, target) {
        (TypeId::Int, TypeId::Float) => {
            let v: isize = val.into();
            let f = v as f32;
            let mut r = RawType { type_id: TypeId::Float, size: 4, data: [0; 8] };
            r.data[..4].copy_from_slice(&f.to_bits().to_le_bytes());
            r
        }
        (TypeId::Int, TypeId::Double) => {
            let v: isize = val.into();
            let f = v as f64;
            let mut r = RawType { type_id: TypeId::Double, size: 8, data: [0; 8] };
            r.data.copy_from_slice(&f.to_bits().to_le_bytes());
            r
        }
        (TypeId::Float, TypeId::Int) => {
            let bits = u32::from_le_bytes(val.data[..4].try_into().unwrap());
            let v = f32::from_bits(bits) as isize;
            v.into()
        }
        (TypeId::Double, TypeId::Int) => {
            let bits = u64::from_le_bytes(val.data);
            let v = f64::from_bits(bits) as isize;
            v.into()
        }
        _ => val,
    }
}
