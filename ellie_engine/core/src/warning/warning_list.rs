use crate::{alloc::borrow::ToOwned, warning};

lazy_static! {
    pub static ref WARNING_S1: warning::Warning = warning::Warning {
        code: 0x00,
        title: "ClassNameRule".to_owned(),
        message: "Class names should be low camel case. Found '$current', expected '$correct'".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref WARNING_S2: warning::Warning = warning::Warning {
        code: 0x01,
        title: "VariableNameRule".to_owned(),
        message: "Variable names should be low camel case. Found '$current', expected '$correct'".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref WARNING_S3: warning::Warning = warning::Warning {
        code: 0x02,
        title: "FunctionParameterNameRule".to_owned(),
        message: "Function parameter names names should be low camel case. Found '$current', expected '$correct'".to_owned(),
        semi_assist: true,
        ..Default::default()
    };


    pub static ref WARNING_S4: warning::Warning = warning::Warning {
        code: 0x03,
        title: "DeadCode".to_owned(),
        message: "This stage of code is unreachable".to_owned(),
        full_assist: true,
        ..Default::default()
    };

    pub static ref WARNING_S5: warning::Warning = warning::Warning {
        code: 0x04,
        title: "FunctionNameRule".to_owned(),
        message: "Function names should be low camel case. Found '$current', expected '$correct'".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref WARNING_S6: warning::Warning = warning::Warning {
        code: 0x05,
        title: "EnumNameRule".to_owned(),
        message: "Enum names should be low camel case. Found '$current', expected '$correct'".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

    pub static ref WARNING_S7: warning::Warning = warning::Warning {
        code: 0x06,
        title: "EnumItemNameRule".to_owned(),
        message: "Enum item's names should be low camel case. Found '$current', expected '$correct'".to_owned(),
        semi_assist: true,
        ..Default::default()
    };

}
