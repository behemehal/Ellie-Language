use crate::instructions::{InstructionStruct, AddressingModesStruct};
pub static Revision : i8 = 1;

static Instructions: phf::Map<&'static str, Keyword> = phf_map! {
	"0x00" => Keyword {
		op_code: "0x00",
		rtype: "LDA",
		modes: vec![
			Immediate,
			Absolute,
			IndirectB,
			IndirectC,
			IndirectX,
			IndirectY,
		]
	},
	"0x06" => Keyword {
		op_code: "0x06",
		rtype: "LDB",
		modes: vec![
			Immediate,
			Absolute,
			IndirectA,
			IndirectC,
			IndirectX,
			IndirectY,
		]
	},
	"0x0C" => Keyword {
		op_code: "0x0C",
		rtype: "LDC",
		modes: vec![
			Immediate,
			Absolute,
			IndirectA,
			IndirectB,
			IndirectX,
			IndirectY,
		]
	},
	"0x12" => Keyword {
		op_code: "0x12",
		rtype: "LDX",
		modes: vec![
			Immediate,
			Absolute,
			IndirectA,
			IndirectB,
			IndirectC,
			IndirectY,
		]
	},
	"0x18" => Keyword {
		op_code: "0x18",
		rtype: "LDY",
		modes: vec![
			Immediate,
			Absolute,
			IndirectA,
			IndirectB,
			IndirectC,
			IndirectX,
		]
	},
	"0x1E" => Keyword {
		op_code: "0x1E",
		rtype: "STA",
		modes: vec![
			Absolute,
			IndirectB,
			IndirectC,
			IndirectX,
			IndirectY,
		]
	},
	"0x24" => Keyword {
		op_code: "0x24",
		rtype: "STB",
		modes: vec![
			Absolute,
			IndirectA,
			IndirectC,
			IndirectX,
			IndirectY,
		]
	},
	"0x29" => Keyword {
		op_code: "0x29",
		rtype: "STC",
		modes: vec![
			Absolute,
			IndirectA,
			IndirectB,
			IndirectX,
			IndirectY,
		]
	},
	"0x2E" => Keyword {
		op_code: "0x2E",
		rtype: "STX",
		modes: vec![
			Absolute,
			IndirectA,
			IndirectB,
			IndirectC,
			IndirectY,
		]
	},
	"0x33" => Keyword {
		op_code: "0x33",
		rtype: "STY",
		modes: vec![
			Absolute,
			IndirectA,
			IndirectB,
			IndirectC,
			IndirectX,
		]
	},
	"0x38" => Keyword {
		op_code: "0x38",
		rtype: "EQ",
		modes: vec![
			Implicit,
		]
	},
	"0x39" => Keyword {
		op_code: "0x39",
		rtype: "NE",
		modes: vec![
			Implicit,
		]
	},
	"0x3A" => Keyword {
		op_code: "0x3A",
		rtype: "GT",
		modes: vec![
			Implicit,
		]
	},
	"0x3B" => Keyword {
		op_code: "0x3B",
		rtype: "LT",
		modes: vec![
			Implicit,
		]
	},
	"0x3C" => Keyword {
		op_code: "0x3C",
		rtype: "GQ",
		modes: vec![
			Implicit,
		]
	},
	"0x3D" => Keyword {
		op_code: "0x3D",
		rtype: "LQ",
		modes: vec![
			Implicit,
		]
	},
	"0x3E" => Keyword {
		op_code: "0x3E",
		rtype: "AND",
		modes: vec![
			Implicit,
		]
	},
	"0x3F" => Keyword {
		op_code: "0x3F",
		rtype: "OR",
		modes: vec![
			Implicit,
		]
	},
	"0x40" => Keyword {
		op_code: "0x40",
		rtype: "SUB",
		modes: vec![
			Implicit,
			Implicit,
		]
	},
	"0x42" => Keyword {
		op_code: "0x42",
		rtype: "MUL",
		modes: vec![
			Implicit,
		]
	},
	"0x43" => Keyword {
		op_code: "0x43",
		rtype: "EXP",
		modes: vec![
			Implicit,
		]
	},
	"0x44" => Keyword {
		op_code: "0x44",
		rtype: "DIV",
		modes: vec![
			Implicit,
		]
	},
	"0x45" => Keyword {
		op_code: "0x45",
		rtype: "MOD",
		modes: vec![
			Implicit,
		]
	},
	"0x46" => Keyword {
		op_code: "0x46",
		rtype: "INC",
		modes: vec![
			Implicit,
		]
	},
	"0x47" => Keyword {
		op_code: "0x47",
		rtype: "DEC",
		modes: vec![
			Implicit,
		]
	},
	"0x48" => Keyword {
		op_code: "0x48",
		rtype: "JMP",
		modes: vec![
			Implicit,
			Absolute,
		]
	},
	"0x4A" => Keyword {
		op_code: "0x4A",
		rtype: "CALL",
		modes: vec![
			Absolute,
		]
	},
	"0x4B" => Keyword {
		op_code: "0x4B",
		rtype: "RET",
		modes: vec![
			Absolute,
		]
	},
};
