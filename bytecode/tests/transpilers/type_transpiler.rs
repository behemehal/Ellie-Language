#[cfg(test)]
mod type_transpiler {
    use ellie_bytecode::{
        assembler::{self, PlatformAttributes},
        instructions::Registers,
        transpiler::type_resolver,
    };
    use ellie_core::{
        definite::types::{self, Types},
        defs::Version,
        utils::PageExport,
    };
    use ellie_parser::parser::Module;

    #[test]
    fn bool_type() {
        let bool_type = types::bool::BoolType { value: true };

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
                architecture: assembler::PlatformArchitecture::B64,
                memory_size: 8,
            },
        );

        let res =
            type_resolver::resolve_type(&mut assembler, &Types::Bool(bool_type), Registers::A, &0);

        panic!("{:?}", res);
        assert!(true);
    }
}
