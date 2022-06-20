use ellie_bytecode::{
    assembler::{self, PlatformAttributes},
    instructions::Registers,
    transpiler::type_resolver,
};
use ellie_core::{
    definite::types::{self, Types},
    defs::{Cursor, PlatformArchitecture, Version},
    utils::PageExport,
};
use ellie_parser::parser::Module;

fn main() {
    let bool_type = types::bool::BoolType { value: true };

    let string_type = types::string::StringType {
        value: "Ahmedo".to_string(),
        pos: Cursor::default(),
    };

    let int_type = types::integer::IntegerType {
        value: 123,
        pos: Cursor::default(),
    };

    let float_type = types::float::FloatType {
        value: 123.0,
        pos: Cursor::default(),
    };

    let double_type = types::double::DoubleType {
        value: 123.0,
        pos: Cursor::default(),
    };

    let char_type = types::ellie_char::CharType { value: 'a' };

    let byte_type = types::byte::ByteType {
        value: 0x1,
        pos: Cursor::default(),
    };

    let target_type = Types::Bool(bool_type);

    let module = Module {
        hash: 0,
        name: "test".to_string(),
        description: "test".to_string(),
        initial_page: 0,
        is_library: true,
        ellie_version: Version::build_from_string("1.0.0".to_string()),
        pages: PageExport::new(),
        version: Version::build_from_string("1.0.0".to_string()),
        modules: vec![],
    };

    let mut assembler = assembler::Assembler::new(
        module,
        PlatformAttributes {
            architecture: PlatformArchitecture::B64,
            memory_size: 8,
        },
    );

    let res = type_resolver::resolve_type(&mut assembler, &target_type, Registers::A, &0);
    let conv = type_resolver::convert_type(&target_type);
    let bytecodes: Vec<Vec<u8>> = res.iter().map(|x| x.get_arg()).collect();

    panic!("{:?}\n{:?}\n{:#?}", res, conv, bytecodes);
}
