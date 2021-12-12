use crate::alloc::borrow::ToOwned;
use crate::warning;

lazy_static! {
    pub static ref WARNING_S1: warning::Warning = warning::Warning {
        code: 0x00,
        title: "ClassNameRule".to_owned(),
        message: "Class names should be high camel case. Found '$current', expected '$correct'".to_owned(),
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
        title: "FunctionParameterRule".to_owned(),
        message: "Function parameter names names should be low camel case. Found '$current', expected '$correct'".to_owned(),
        full_assist: true,
        ..Default::default()
    };
}
