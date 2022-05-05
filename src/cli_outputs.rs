use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum CliOuputType {
    Warning,
    Error,
    Info,
}

#[derive(Serialize, Deserialize, Clone)]

pub struct CliOuputExtraData<K, V> {
    pub key: K,
    pub value: V,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CliOuput<K, V> {
    pub code: u8,
    pub rtype: CliOuputType,
    pub message: String,
    pub extra: Vec<CliOuputExtraData<K, V>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CliInnerModuleOutput {
    pub name: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CliModuleOutput {
    pub name: String,
    pub description: String,
    pub version: String,
    pub modules: Vec<CliInnerModuleOutput>,
}

lazy_static! {
    pub static ref COMPILE_SUCCESS_WITH_NO_WARNINGS: CliOuput<String, String> = CliOuput {
        code: 0,
        rtype: CliOuputType::Info,
        message: "Compiling succeeded with no warnings".to_string(),
        extra: Vec::new(),
    };
    pub static ref COMPILE_SUCCESS_WITH_WARNINGS: CliOuput<String, String> = CliOuput {
        code: 1,
        rtype: CliOuputType::Info,
        message: "Compiling succeeded with warnings".to_string(),
        extra: Vec::new(),
    };
    pub static ref COMPILE_FAILED_WITH_ERRORS_WITH_NO_WARNINGS: CliOuput<String, String> =
        CliOuput {
            code: 2,
            rtype: CliOuputType::Error,
            message: "Compiling failed with errors".to_string(),
            extra: Vec::new(),
        };
    pub static ref COMPILE_FAILED_WITH_ERRORS_WITH_WARNINGS: CliOuput<String, String> = CliOuput {
        code: 3,
        rtype: CliOuputType::Error,
        message: "Compiling failed with errors and warnings".to_string(),
        extra: Vec::new(),
    };
    pub static ref COMPILER_ERRORS: CliOuput<String, Vec<ellie_core::error::Error>> = CliOuput {
        code: 4,
        rtype: CliOuputType::Error,
        message: String::new(),
        extra: Vec::new(),
    };
    pub static ref COMPILER_WARNINGS: CliOuput<String, Vec<ellie_core::warning::Warning>> =
        CliOuput {
            code: 5,
            rtype: CliOuputType::Warning,
            message: String::new(),
            extra: Vec::new(),
        };
    pub static ref WRITE_FILE_ERROR: CliOuput<String, String> = CliOuput {
        code: 6,
        rtype: CliOuputType::Error,
        message: "Failed to write output file".to_string(),
        extra: Vec::new(),
    };
    pub static ref PATH_ERROR: CliOuput<String, String> = CliOuput {
        code: 7,
        rtype: CliOuputType::Error,
        message: "Failed to resolve path".to_string(),
        extra: Vec::new(),
    };
    pub static ref READ_BINARY_MODULE_ERROR: CliOuput<String, String> = CliOuput {
        code: 8,
        rtype: CliOuputType::Error,
        message: "Failed to read binary module".to_string(),
        extra: Vec::new(),
    };
    pub static ref READ_BINARY_MODULE_SUCCEDED: CliOuput<i8, CliModuleOutput> = CliOuput {
        code: 9,
        rtype: CliOuputType::Info,
        message: "Binary read completed successfully".to_string(),
        extra: Vec::new(),
    };
    pub static ref READ_FILE_ERROR: CliOuput<i8, String> = CliOuput {
        code: 10,
        rtype: CliOuputType::Error,
        message: "Failed to read file".to_string(),
        extra: Vec::new(),
    };
    pub static ref WRITE_BINARY_SUCCEDED: CliOuput<i8, String> = CliOuput {
        code: 11,
        rtype: CliOuputType::Info,
        message: "Output write succeded as binary".to_string(),
        extra: Vec::new(),
    };
    pub static ref WRITE_JSON_SUCCEDED: CliOuput<i8, String> = CliOuput {
        code: 12,
        rtype: CliOuputType::Info,
        message: "Output write succeded as json".to_string(),
        extra: Vec::new(),
    };
    pub static ref VERSION: CliOuput<String, String> = CliOuput {
        code: 13,
        rtype: CliOuputType::Info,
        message: "".to_string(),
        extra: Vec::new(),
    };
    pub static ref VERSION_DETAILED: CliOuput<String, String> = CliOuput {
        code: 14,
        rtype: CliOuputType::Info,
        message: "".to_string(),
        extra: Vec::new(),
    };
    pub static ref LEGACY_MODULE: CliOuput<i8, ellie_core::defs::Version> = CliOuput {
        code: 15,
        rtype: CliOuputType::Warning,
        message: "This module is legacy but used anyway".to_string(),
        extra: Vec::new(),
    };
    pub static ref WRITE_BYTE_CODE_SUCCEDED: CliOuput<i8, String> = CliOuput {
        code: 16,
        rtype: CliOuputType::Info,
        message: "Output write succeded as byte code".to_string(),
        extra: Vec::new(),
    };
}
