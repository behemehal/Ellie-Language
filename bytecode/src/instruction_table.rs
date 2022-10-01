//Auto builded from `instructions.csv` by `build.rs`
use lazy_static;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Instruction {
    pub rtype: &'static str,
    pub code: u8,
    pub mode: &'static str,
}

lazy_static! {
    pub static ref INSTRUCTIONS: HashMap<&'static str, Instruction> = {
        let mut i = HashMap::new();
        i.insert(
            "lda_immediate",
            Instruction {
                rtype: "lda",
                code: 1,
                mode: "immediate",
            },
        );
        i.insert(
            "lda_absolute",
            Instruction {
                rtype: "lda",
                code: 2,
                mode: "absolute",
            },
        );
        i.insert(
            "lda_indirect_b",
            Instruction {
                rtype: "lda",
                code: 3,
                mode: "indirect_b",
            },
        );
        i.insert(
            "lda_indirect_c",
            Instruction {
                rtype: "lda",
                code: 4,
                mode: "indirect_c",
            },
        );
        i.insert(
            "lda_indirect_x",
            Instruction {
                rtype: "lda",
                code: 5,
                mode: "indirect_x",
            },
        );
        i.insert(
            "lda_indirect_y",
            Instruction {
                rtype: "lda",
                code: 6,
                mode: "indirect_y",
            },
        );
        i.insert(
            "lda_absolute_index",
            Instruction {
                rtype: "lda",
                code: 7,
                mode: "absolute_index",
            },
        );
        i.insert(
            "lda_absolute_property",
            Instruction {
                rtype: "lda",
                code: 8,
                mode: "absolute_property",
            },
        );
        i.insert(
            "lda_parameter",
            Instruction {
                rtype: "lda",
                code: 9,
                mode: "parameter",
            },
        );
        i.insert(
            "ldb_immediate",
            Instruction {
                rtype: "ldb",
                code: 10,
                mode: "immediate",
            },
        );
        i.insert(
            "ldb_absolute",
            Instruction {
                rtype: "ldb",
                code: 11,
                mode: "absolute",
            },
        );
        i.insert(
            "ldb_indirect_a",
            Instruction {
                rtype: "ldb",
                code: 12,
                mode: "indirect_a",
            },
        );
        i.insert(
            "ldb_indirect_c",
            Instruction {
                rtype: "ldb",
                code: 13,
                mode: "indirect_c",
            },
        );
        i.insert(
            "ldb_indirect_x",
            Instruction {
                rtype: "ldb",
                code: 14,
                mode: "indirect_x",
            },
        );
        i.insert(
            "ldb_indirect_y",
            Instruction {
                rtype: "ldb",
                code: 15,
                mode: "indirect_y",
            },
        );
        i.insert(
            "ldb_absolute_index",
            Instruction {
                rtype: "ldb",
                code: 16,
                mode: "absolute_index",
            },
        );
        i.insert(
            "ldb_absolute_property",
            Instruction {
                rtype: "ldb",
                code: 17,
                mode: "absolute_property",
            },
        );
        i.insert(
            "ldb_parameter",
            Instruction {
                rtype: "ldb",
                code: 18,
                mode: "parameter",
            },
        );
        i.insert(
            "ldc_immediate",
            Instruction {
                rtype: "ldc",
                code: 19,
                mode: "immediate",
            },
        );
        i.insert(
            "ldc_absolute",
            Instruction {
                rtype: "ldc",
                code: 20,
                mode: "absolute",
            },
        );
        i.insert(
            "ldc_indirect_a",
            Instruction {
                rtype: "ldc",
                code: 21,
                mode: "indirect_a",
            },
        );
        i.insert(
            "ldc_indirect_b",
            Instruction {
                rtype: "ldc",
                code: 22,
                mode: "indirect_b",
            },
        );
        i.insert(
            "ldc_indirect_x",
            Instruction {
                rtype: "ldc",
                code: 23,
                mode: "indirect_x",
            },
        );
        i.insert(
            "ldc_indirect_y",
            Instruction {
                rtype: "ldc",
                code: 24,
                mode: "indirect_y",
            },
        );
        i.insert(
            "ldc_absolute_index",
            Instruction {
                rtype: "ldc",
                code: 25,
                mode: "absolute_index",
            },
        );
        i.insert(
            "ldc_absolute_property",
            Instruction {
                rtype: "ldc",
                code: 26,
                mode: "absolute_property",
            },
        );
        i.insert(
            "ldc_parameter",
            Instruction {
                rtype: "ldc",
                code: 27,
                mode: "parameter",
            },
        );
        i.insert(
            "ldx_immediate",
            Instruction {
                rtype: "ldx",
                code: 28,
                mode: "immediate",
            },
        );
        i.insert(
            "ldx_absolute",
            Instruction {
                rtype: "ldx",
                code: 29,
                mode: "absolute",
            },
        );
        i.insert(
            "ldx_indirect_a",
            Instruction {
                rtype: "ldx",
                code: 30,
                mode: "indirect_a",
            },
        );
        i.insert(
            "ldx_indirect_b",
            Instruction {
                rtype: "ldx",
                code: 31,
                mode: "indirect_b",
            },
        );
        i.insert(
            "ldx_indirect_c",
            Instruction {
                rtype: "ldx",
                code: 32,
                mode: "indirect_c",
            },
        );
        i.insert(
            "ldx_indirect_y",
            Instruction {
                rtype: "ldx",
                code: 33,
                mode: "indirect_y",
            },
        );
        i.insert(
            "ldx_absolute_index",
            Instruction {
                rtype: "ldx",
                code: 34,
                mode: "absolute_index",
            },
        );
        i.insert(
            "ldx_absolute_property",
            Instruction {
                rtype: "ldx",
                code: 35,
                mode: "absolute_property",
            },
        );
        i.insert(
            "ldx_parameter",
            Instruction {
                rtype: "ldx",
                code: 36,
                mode: "parameter",
            },
        );
        i.insert(
            "ldy_immediate",
            Instruction {
                rtype: "ldy",
                code: 37,
                mode: "immediate",
            },
        );
        i.insert(
            "ldy_absolute",
            Instruction {
                rtype: "ldy",
                code: 38,
                mode: "absolute",
            },
        );
        i.insert(
            "ldy_indirect_a",
            Instruction {
                rtype: "ldy",
                code: 39,
                mode: "indirect_a",
            },
        );
        i.insert(
            "ldy_indirect_b",
            Instruction {
                rtype: "ldy",
                code: 40,
                mode: "indirect_b",
            },
        );
        i.insert(
            "ldy_indirect_c",
            Instruction {
                rtype: "ldy",
                code: 41,
                mode: "indirect_c",
            },
        );
        i.insert(
            "ldy_indirect_x",
            Instruction {
                rtype: "ldy",
                code: 42,
                mode: "indirect_x",
            },
        );
        i.insert(
            "ldy_absolute_index",
            Instruction {
                rtype: "ldy",
                code: 43,
                mode: "absolute_index",
            },
        );
        i.insert(
            "ldy_absolute_property",
            Instruction {
                rtype: "ldy",
                code: 44,
                mode: "absolute_property",
            },
        );
        i.insert(
            "ldy_parameter",
            Instruction {
                rtype: "ldy",
                code: 45,
                mode: "parameter",
            },
        );
        i.insert(
            "sta_implicit",
            Instruction {
                rtype: "sta",
                code: 46,
                mode: "implicit",
            },
        );
        i.insert(
            "sta_immediate",
            Instruction {
                rtype: "sta",
                code: 47,
                mode: "immediate",
            },
        );
        i.insert(
            "sta_absolute",
            Instruction {
                rtype: "sta",
                code: 48,
                mode: "absolute",
            },
        );
        i.insert(
            "sta_absolute_index",
            Instruction {
                rtype: "sta",
                code: 49,
                mode: "absolute_index",
            },
        );
        i.insert(
            "sta_absolute_property",
            Instruction {
                rtype: "sta",
                code: 50,
                mode: "absolute_property",
            },
        );
        i.insert(
            "sta_parameter",
            Instruction {
                rtype: "sta",
                code: 51,
                mode: "parameter",
            },
        );
        i.insert(
            "stb_implicit",
            Instruction {
                rtype: "stb",
                code: 52,
                mode: "implicit",
            },
        );
        i.insert(
            "stb_immediate",
            Instruction {
                rtype: "stb",
                code: 53,
                mode: "immediate",
            },
        );
        i.insert(
            "stb_absolute",
            Instruction {
                rtype: "stb",
                code: 54,
                mode: "absolute",
            },
        );
        i.insert(
            "stb_absolute_index",
            Instruction {
                rtype: "stb",
                code: 55,
                mode: "absolute_index",
            },
        );
        i.insert(
            "stb_absolute_property",
            Instruction {
                rtype: "stb",
                code: 56,
                mode: "absolute_property",
            },
        );
        i.insert(
            "stb_parameter",
            Instruction {
                rtype: "stb",
                code: 57,
                mode: "parameter",
            },
        );
        i.insert(
            "stc_implicit",
            Instruction {
                rtype: "stc",
                code: 58,
                mode: "implicit",
            },
        );
        i.insert(
            "stc_immediate",
            Instruction {
                rtype: "stc",
                code: 59,
                mode: "immediate",
            },
        );
        i.insert(
            "stc_absolute",
            Instruction {
                rtype: "stc",
                code: 60,
                mode: "absolute",
            },
        );
        i.insert(
            "stc_absolute_index",
            Instruction {
                rtype: "stc",
                code: 61,
                mode: "absolute_index",
            },
        );
        i.insert(
            "stc_absolute_property",
            Instruction {
                rtype: "stc",
                code: 62,
                mode: "absolute_property",
            },
        );
        i.insert(
            "stc_parameter",
            Instruction {
                rtype: "stc",
                code: 63,
                mode: "parameter",
            },
        );
        i.insert(
            "stx_implicit",
            Instruction {
                rtype: "stx",
                code: 64,
                mode: "implicit",
            },
        );
        i.insert(
            "stx_immediate",
            Instruction {
                rtype: "stx",
                code: 65,
                mode: "immediate",
            },
        );
        i.insert(
            "stx_absolute",
            Instruction {
                rtype: "stx",
                code: 66,
                mode: "absolute",
            },
        );
        i.insert(
            "stx_absolute_index",
            Instruction {
                rtype: "stx",
                code: 67,
                mode: "absolute_index",
            },
        );
        i.insert(
            "stx_absolute_property",
            Instruction {
                rtype: "stx",
                code: 68,
                mode: "absolute_property",
            },
        );
        i.insert(
            "sty_implicit",
            Instruction {
                rtype: "sty",
                code: 69,
                mode: "implicit",
            },
        );
        i.insert(
            "sty_immediate",
            Instruction {
                rtype: "sty",
                code: 70,
                mode: "immediate",
            },
        );
        i.insert(
            "sty_absolute",
            Instruction {
                rtype: "sty",
                code: 71,
                mode: "absolute",
            },
        );
        i.insert(
            "sty_absolute_index",
            Instruction {
                rtype: "sty",
                code: 72,
                mode: "absolute_index",
            },
        );
        i.insert(
            "sty_absolute_property",
            Instruction {
                rtype: "sty",
                code: 73,
                mode: "absolute_property",
            },
        );
        i.insert(
            "sty_parameter",
            Instruction {
                rtype: "sty",
                code: 74,
                mode: "parameter",
            },
        );
        i.insert(
            "eq_implicit",
            Instruction {
                rtype: "eq",
                code: 75,
                mode: "implicit",
            },
        );
        i.insert(
            "ne_implicit",
            Instruction {
                rtype: "ne",
                code: 76,
                mode: "implicit",
            },
        );
        i.insert(
            "gt_implicit",
            Instruction {
                rtype: "gt",
                code: 77,
                mode: "implicit",
            },
        );
        i.insert(
            "lt_implicit",
            Instruction {
                rtype: "lt",
                code: 78,
                mode: "implicit",
            },
        );
        i.insert(
            "gq_implicit",
            Instruction {
                rtype: "gq",
                code: 79,
                mode: "implicit",
            },
        );
        i.insert(
            "lq_implicit",
            Instruction {
                rtype: "lq",
                code: 80,
                mode: "implicit",
            },
        );
        i.insert(
            "and_implicit",
            Instruction {
                rtype: "and",
                code: 81,
                mode: "implicit",
            },
        );
        i.insert(
            "or_implicit",
            Instruction {
                rtype: "or",
                code: 82,
                mode: "implicit",
            },
        );
        i.insert(
            "add_implicit",
            Instruction {
                rtype: "add",
                code: 83,
                mode: "implicit",
            },
        );
        i.insert(
            "sub_implicit",
            Instruction {
                rtype: "sub",
                code: 84,
                mode: "implicit",
            },
        );
        i.insert(
            "mul_implicit",
            Instruction {
                rtype: "mul",
                code: 85,
                mode: "implicit",
            },
        );
        i.insert(
            "exp_implicit",
            Instruction {
                rtype: "exp",
                code: 86,
                mode: "implicit",
            },
        );
        i.insert(
            "div_implicit",
            Instruction {
                rtype: "div",
                code: 87,
                mode: "implicit",
            },
        );
        i.insert(
            "mod_implicit",
            Instruction {
                rtype: "mod",
                code: 88,
                mode: "implicit",
            },
        );
        i.insert(
            "inc_implicit",
            Instruction {
                rtype: "inc",
                code: 89,
                mode: "implicit",
            },
        );
        i.insert(
            "dec_implicit",
            Instruction {
                rtype: "dec",
                code: 90,
                mode: "implicit",
            },
        );
        i.insert(
            "jmp_absolute",
            Instruction {
                rtype: "jmp",
                code: 91,
                mode: "absolute",
            },
        );
        i.insert(
            "call_absolute",
            Instruction {
                rtype: "call",
                code: 92,
                mode: "absolute",
            },
        );
        i.insert(
            "ret_implicit",
            Instruction {
                rtype: "ret",
                code: 93,
                mode: "implicit",
            },
        );
        i.insert(
            "ret_immediate",
            Instruction {
                rtype: "ret",
                code: 94,
                mode: "immediate",
            },
        );
        i.insert(
            "ret_absolute",
            Instruction {
                rtype: "ret",
                code: 95,
                mode: "absolute",
            },
        );
        i.insert(
            "ret_indirect_a",
            Instruction {
                rtype: "ret",
                code: 96,
                mode: "indirect_a",
            },
        );
        i.insert(
            "ret_indirect_b",
            Instruction {
                rtype: "ret",
                code: 97,
                mode: "indirect_b",
            },
        );
        i.insert(
            "ret_indirect_c",
            Instruction {
                rtype: "ret",
                code: 98,
                mode: "indirect_c",
            },
        );
        i.insert(
            "ret_indirect_x",
            Instruction {
                rtype: "ret",
                code: 99,
                mode: "indirect_x",
            },
        );
        i.insert(
            "ret_indirect_y",
            Instruction {
                rtype: "ret",
                code: 100,
                mode: "indirect_y",
            },
        );
        i.insert(
            "ret_absolute_index",
            Instruction {
                rtype: "ret",
                code: 101,
                mode: "absolute_index",
            },
        );
        i.insert(
            "ret_absolute_property",
            Instruction {
                rtype: "ret",
                code: 102,
                mode: "absolute_property",
            },
        );
        i.insert(
            "ugr_implicit",
            Instruction {
                rtype: "ugr",
                code: 103,
                mode: "implicit",
            },
        );
        i.insert(
            "ulr_implicit",
            Instruction {
                rtype: "ulr",
                code: 104,
                mode: "implicit",
            },
        );
        i.insert(
            "ulr_absolute",
            Instruction {
                rtype: "ulr",
                code: 105,
                mode: "absolute",
            },
        );
        i.insert(
            "push_absolute",
            Instruction {
                rtype: "push",
                code: 106,
                mode: "absolute",
            },
        );
        i.insert(
            "push_indirect_a",
            Instruction {
                rtype: "push",
                code: 107,
                mode: "indirect_a",
            },
        );
        i.insert(
            "push_indirect_b",
            Instruction {
                rtype: "push",
                code: 108,
                mode: "indirect_b",
            },
        );
        i.insert(
            "push_indirect_c",
            Instruction {
                rtype: "push",
                code: 109,
                mode: "indirect_c",
            },
        );
        i.insert(
            "push_indirect_x",
            Instruction {
                rtype: "push",
                code: 110,
                mode: "indirect_x",
            },
        );
        i.insert(
            "push_indirect_y",
            Instruction {
                rtype: "push",
                code: 111,
                mode: "indirect_y",
            },
        );
        i.insert(
            "push_absolute_index",
            Instruction {
                rtype: "push",
                code: 112,
                mode: "absolute_index",
            },
        );
        i.insert(
            "push_absolute_property",
            Instruction {
                rtype: "push",
                code: 113,
                mode: "absolute_property",
            },
        );
        i.insert(
            "len_implicit",
            Instruction {
                rtype: "len",
                code: 114,
                mode: "implicit",
            },
        );
        i.insert(
            "a2i_implicit",
            Instruction {
                rtype: "a2i",
                code: 115,
                mode: "implicit",
            },
        );
        i.insert(
            "a2f_implicit",
            Instruction {
                rtype: "a2f",
                code: 116,
                mode: "implicit",
            },
        );
        i.insert(
            "a2d_implicit",
            Instruction {
                rtype: "a2d",
                code: 117,
                mode: "implicit",
            },
        );
        i.insert(
            "a2b_implicit",
            Instruction {
                rtype: "a2b",
                code: 118,
                mode: "implicit",
            },
        );
        i.insert(
            "a2s_implicit",
            Instruction {
                rtype: "a2s",
                code: 119,
                mode: "implicit",
            },
        );
        i.insert(
            "a2c_implicit",
            Instruction {
                rtype: "a2c",
                code: 120,
                mode: "implicit",
            },
        );
        i.insert(
            "a2o_implicit",
            Instruction {
                rtype: "a2o",
                code: 121,
                mode: "implicit",
            },
        );
        i.insert(
            "jmpa_absolute",
            Instruction {
                rtype: "jmpa",
                code: 122,
                mode: "absolute",
            },
        );
        i.insert(
            "pops_implicit",
            Instruction {
                rtype: "pops",
                code: 123,
                mode: "implicit",
            },
        );
        i.insert(
            "acp_absolute",
            Instruction {
                rtype: "acp",
                code: 124,
                mode: "absolute",
            },
        );
        i.insert(
            "brk_implicit",
            Instruction {
                rtype: "brk",
                code: 125,
                mode: "implicit",
            },
        );
        i.insert(
            "calln_immediate",
            Instruction {
                rtype: "calln",
                code: 126,
                mode: "immediate",
            },
        );
        i.insert(
            "co_absolute",
            Instruction {
                rtype: "co",
                code: 127,
                mode: "absolute",
            },
        );
        i.insert(
            "fn_immediate",
            Instruction {
                rtype: "fn",
                code: 128,
                mode: "immediate",
            },
        );
        i
    };
}
