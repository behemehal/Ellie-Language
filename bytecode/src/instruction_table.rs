//Auto builded from `instructions.csv` by `build.rs` dont modify while language server is running
use alloc::vec;
use lazy_static;
use crate::instructions::{InstructionStruct, AddressingModesStruct};
pub static Revision : i8 = 1;

 pub struct Instruction {
	rtype: &'static str,
	code: i8,
	mode: &'static str,
}

lazy_static! {
	let instructions: [Instruction, 92] = [
		Instruction {
			rtype: "lda",
			code: 1,
			mode: "immediate",
		},
		Instruction {
			rtype: "lda",
			code: 2,
			mode: "absolute",
		},
		Instruction {
			rtype: "lda",
			code: 3,
			mode: "indirect_b",
		},
		Instruction {
			rtype: "lda",
			code: 4,
			mode: "indirect_c",
		},
		Instruction {
			rtype: "lda",
			code: 5,
			mode: "indirect_x",
		},
		Instruction {
			rtype: "lda",
			code: 6,
			mode: "indirect_y",
		},
		Instruction {
			rtype: "lda",
			code: 7,
			mode: "absolute_index",
		},
		Instruction {
			rtype: "lda",
			code: 8,
			mode: "absolute_property",
		},
		Instruction {
			rtype: "ldb",
			code: 9,
			mode: "immediate",
		},
		Instruction {
			rtype: "ldb",
			code: 10,
			mode: "absolute",
		},
		Instruction {
			rtype: "ldb",
			code: 11,
			mode: "indirect_a",
		},
		Instruction {
			rtype: "ldb",
			code: 12,
			mode: "indirect_c",
		},
		Instruction {
			rtype: "ldb",
			code: 13,
			mode: "indirect_x",
		},
		Instruction {
			rtype: "ldb",
			code: 14,
			mode: "indirect_y",
		},
		Instruction {
			rtype: "ldb",
			code: 15,
			mode: "absolute_index",
		},
		Instruction {
			rtype: "ldb",
			code: 16,
			mode: "absolute_property",
		},
		Instruction {
			rtype: "ldc",
			code: 17,
			mode: "immediate",
		},
		Instruction {
			rtype: "ldc",
			code: 18,
			mode: "absolute",
		},
		Instruction {
			rtype: "ldc",
			code: 19,
			mode: "indirect_a",
		},
		Instruction {
			rtype: "ldc",
			code: 20,
			mode: "indirect_b",
		},
		Instruction {
			rtype: "ldc",
			code: 21,
			mode: "indirect_x",
		},
		Instruction {
			rtype: "ldc",
			code: 22,
			mode: "indirect_y",
		},
		Instruction {
			rtype: "ldc",
			code: 23,
			mode: "absolute_index",
		},
		Instruction {
			rtype: "ldc",
			code: 24,
			mode: "absolute_property",
		},
		Instruction {
			rtype: "ldx",
			code: 25,
			mode: "immediate",
		},
		Instruction {
			rtype: "ldx",
			code: 26,
			mode: "absolute",
		},
		Instruction {
			rtype: "ldx",
			code: 27,
			mode: "indirect_a",
		},
		Instruction {
			rtype: "ldx",
			code: 28,
			mode: "indirect_b",
		},
		Instruction {
			rtype: "ldx",
			code: 29,
			mode: "indirect_c",
		},
		Instruction {
			rtype: "ldx",
			code: 30,
			mode: "indirect_y",
		},
		Instruction {
			rtype: "ldx",
			code: 31,
			mode: "absolute_index",
		},
		Instruction {
			rtype: "ldx",
			code: 32,
			mode: "absolute_property",
		},
		Instruction {
			rtype: "ldy",
			code: 33,
			mode: "immediate",
		},
		Instruction {
			rtype: "ldy",
			code: 34,
			mode: "absolute",
		},
		Instruction {
			rtype: "ldy",
			code: 35,
			mode: "indirect_a",
		},
		Instruction {
			rtype: "ldy",
			code: 36,
			mode: "indirect_b",
		},
		Instruction {
			rtype: "ldy",
			code: 38,
			mode: "indirect_c",
		},
		Instruction {
			rtype: "ldy",
			code: 37,
			mode: "indirect_x",
		},
		Instruction {
			rtype: "ldy",
			code: 39,
			mode: "absolute_index",
		},
		Instruction {
			rtype: "ldy",
			code: 40,
			mode: "absolute_property",
		},
		Instruction {
			rtype: "sta",
			code: 41,
			mode: "implict",
		},
		Instruction {
			rtype: "stb",
			code: 42,
			mode: "implict",
		},
		Instruction {
			rtype: "stc",
			code: 43,
			mode: "implict",
		},
		Instruction {
			rtype: "stx",
			code: 44,
			mode: "implict",
		},
		Instruction {
			rtype: "sty",
			code: 45,
			mode: "implict",
		},
		Instruction {
			rtype: "eq",
			code: 46,
			mode: "implict",
		},
		Instruction {
			rtype: "ne",
			code: 47,
			mode: "implict",
		},
		Instruction {
			rtype: "gt",
			code: 48,
			mode: "implict",
		},
		Instruction {
			rtype: "lt",
			code: 49,
			mode: "implict",
		},
		Instruction {
			rtype: "gq",
			code: 50,
			mode: "implict",
		},
		Instruction {
			rtype: "lq",
			code: 51,
			mode: "implict",
		},
		Instruction {
			rtype: "and",
			code: 52,
			mode: "implict",
		},
		Instruction {
			rtype: "or",
			code: 53,
			mode: "implict",
		},
		Instruction {
			rtype: "add",
			code: 54,
			mode: "implict",
		},
		Instruction {
			rtype: "sub",
			code: 55,
			mode: "implict",
		},
		Instruction {
			rtype: "mul",
			code: 56,
			mode: "implict",
		},
		Instruction {
			rtype: "exp",
			code: 57,
			mode: "implict",
		},
		Instruction {
			rtype: "div",
			code: 58,
			mode: "implict",
		},
		Instruction {
			rtype: "mod",
			code: 59,
			mode: "implict",
		},
		Instruction {
			rtype: "inc",
			code: 60,
			mode: "implict",
		},
		Instruction {
			rtype: "dec",
			code: 61,
			mode: "implict",
		},
		Instruction {
			rtype: "jmp",
			code: 62,
			mode: "absolute",
		},
		Instruction {
			rtype: "call",
			code: 63,
			mode: "absolute",
		},
		Instruction {
			rtype: "ret",
			code: 64,
			mode: "implict",
		},
		Instruction {
			rtype: "ret",
			code: 65,
			mode: "immediate",
		},
		Instruction {
			rtype: "ret",
			code: 66,
			mode: "absolute",
		},
		Instruction {
			rtype: "ret",
			code: 67,
			mode: "indirect_a",
		},
		Instruction {
			rtype: "ret",
			code: 68,
			mode: "indirect_b",
		},
		Instruction {
			rtype: "ret",
			code: 69,
			mode: "indirect_c",
		},
		Instruction {
			rtype: "ret",
			code: 70,
			mode: "indirect_x",
		},
		Instruction {
			rtype: "ret",
			code: 71,
			mode: "indirect_y",
		},
		Instruction {
			rtype: "ret",
			code: 72,
			mode: "absolute_index",
		},
		Instruction {
			rtype: "ret",
			code: 73,
			mode: "absolute_property",
		},
		Instruction {
			rtype: "aol",
			code: 74,
			mode: "absolute",
		},
		Instruction {
			rtype: "pusha",
			code: 75,
			mode: "absolute",
		},
		Instruction {
			rtype: "pusha",
			code: 76,
			mode: "indirect_a",
		},
		Instruction {
			rtype: "pusha",
			code: 77,
			mode: "indirect_b",
		},
		Instruction {
			rtype: "pusha",
			code: 78,
			mode: "indirect_c",
		},
		Instruction {
			rtype: "pusha",
			code: 79,
			mode: "indirect_x",
		},
		Instruction {
			rtype: "pusha",
			code: 80,
			mode: "indirect_y",
		},
		Instruction {
			rtype: "pusha",
			code: 81,
			mode: "absolute_index",
		},
		Instruction {
			rtype: "pusha",
			code: 82,
			mode: "absolute_property",
		},
		Instruction {
			rtype: "len",
			code: 83,
			mode: "implict",
		},
		Instruction {
			rtype: "a2i",
			code: 84,
			mode: "implict",
		},
		Instruction {
			rtype: "a2f",
			code: 85,
			mode: "implict",
		},
		Instruction {
			rtype: "a2d",
			code: 86,
			mode: "implict",
		},
		Instruction {
			rtype: "a2b",
			code: 87,
			mode: "implict",
		},
		Instruction {
			rtype: "a2s",
			code: 88,
			mode: "implict",
		},
		Instruction {
			rtype: "a2c",
			code: 89,
			mode: "implict",
		},
		Instruction {
			rtype: "a2o",
			code: 90,
			mode: "implict",
		},
		Instruction {
			rtype: "jmpa",
			code: 91,
			mode: "absolute",
		},
		Instruction {
			rtype: "pops",
			code: 92,
			mode: "implict",
		},
	];
}
