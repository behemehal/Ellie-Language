use std::collections::HashMap;
use ellie_engine::ellie_vm::raw_type::{StaticRawType, TypeId};

#[derive(Debug, Clone)]
pub struct EllieMessage {
    pub r#type: String,
    pub message: String,
    pub code: isize,
    pub variables: Option<HashMap<String, String>>,
}

impl EllieMessage {
    pub fn new(r#type: &str, message: &str, code: isize) -> Self {
        Self {
            r#type: r#type.to_string(),
            message: message.to_string(),
            variables: None,
            code,
        }
    }

    pub fn new_with_variables(
        r#type: &str,
        message: &str,
        code: isize,
        variables: HashMap<String, String>,
    ) -> Self {
        Self {
            r#type: r#type.to_string(),
            message: message.to_string(),
            variables: Some(variables),
            code,
        }
    }

    fn type_color(r#type: &str) -> String {
        let color_id = match r#type {
            "error" => "[31m",
            "info" => "[32m",
            "warning" => "[33m",
            "log" => "[37m",
            _ => panic!("Unknown message type: {}", r#type),
        };
        if r#type == "log" {
            "".to_string()
        } else {
            format!("\u{001b}{}[{}]\u{001b}[0m: ", color_id, r#type)
        }
    }

    fn build_message(&self) -> String {
        match &self.variables {
            Some(variables) => {
                let mut message = self.message.clone();
                for (key, value) in variables {
                    message = message.replace(&format!("{{{key}}}"), value);
                }
                message
            }
            None => self.message.clone(),
        }
    }

    pub fn build_json_message(&self) -> String {
        format!(
            "{{\"type\":\"{}\",\"code\": \"{}\",\"message\":\"{}\", \"variables\": {{{}}}}}",
            self.r#type,
            self.code,
            self.build_message(),
            {
                let mut data = String::new();
                if (&self.variables).is_none() {
                    return data;
                }
                for (idx, variable) in self.variables.as_ref().unwrap().iter().enumerate() {
                    data += &format!("\"{}\": \"{}\"", variable.0, variable.1);
                    if idx != self.variables.as_ref().unwrap().len() - 1 {
                        data += ",";
                    }
                }
                data
            }
        )
    }

    pub fn build_plain_message(&self) -> String {
        format!(
            "{}{}",
            EllieMessage::type_color(&self.r#type),
            self.build_message()
        )
    }
}

pub fn render_type_id(type_id: TypeId) -> HashMap<String, String> {
    let mut result = HashMap::new();
    result.insert("type_id".to_string(), type_id.id.to_string());
    result.insert("type_size".to_string(), type_id.size.to_string());
    result
}

pub fn render_static_raw_type(value: StaticRawType) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.extend(render_type_id(value.type_id));
    map.insert("data".to_string(), format!("{:?}", value.data));

    map.insert(
        "data_text".to_string(),
        format!("{}", {
            let type_id = match value.type_id.id {
                1 => value.to_int().to_string(),
                2 => value.to_float().to_string(),
                3 => value.to_double().to_string(),
                4 => value.to_byte().to_string(),
                5 => (value.data[0] == 1).to_string(),
                6 => {
                    todo!()
                }
                7 => value.to_char().to_string(),
                8 => String::from("void"),
                9 => String::from("arr"),
                10 => String::from("null"),
                11 => String::from("class"),
                12 => String::from("function"),
                13 => String::from("stack_reference"),
                14 => String::from("heap_reference"),
                15 => String::from("static_array"),
                _ => unreachable!("Wrong typeid"),
            };
            format!("{}", type_id)
        }),
    );
    return map;
}

lazy_static! {
    // Error
    pub static ref UNKNOWN_COMMAND: EllieMessage = EllieMessage::new("error", "Unknown command", 1);
    pub static ref INVALID_ARGUMENT_LENGTH: EllieMessage = EllieMessage::new("error", "Invalid argument length, check syntax of the command", 2);
    pub static ref INVALID_ARGUMENT_TYPE: EllieMessage = EllieMessage::new("error", "Invalid argument type, check syntax of the command", 3);
    pub static ref CANT_RENDER_INFO: EllieMessage = EllieMessage::new("error", "Output is not a standard text, this might be a help text which is not feasible to be json logged.", 4);
    pub static ref FILE_READ_ERROR: EllieMessage = EllieMessage::new_with_variables("error", "Failed to read the file: {error}", 5, HashMap::new());
    pub static ref PROGRAM_READ_ERROR: EllieMessage = EllieMessage::new_with_variables("error", "Failed to read program: {error}", 6, HashMap::new());
    pub static ref DEBUG_FILE_PARSE_ERROR: EllieMessage = EllieMessage::new_with_variables("error", "Failed to parse debug fie: {error}", 7, HashMap::new());
    pub static ref PROGRAM_NOT_LOADED: EllieMessage = EllieMessage::new_with_variables("error", "Program not loaded", 8, HashMap::new());
    pub static ref CANT_FIND_ELEMENT_AT_LOCATION: EllieMessage = EllieMessage::new_with_variables("error", "Can't find a element on given position to wait", 9, HashMap::new());
    pub static ref NOT_IN_BREAKPOINT: EllieMessage = EllieMessage::new("error", "Debugger is not on wait state", 25);
    pub static ref DEBUGER_IS_NOT_ON_EXPECTED_STATE: EllieMessage = EllieMessage::new_with_variables("error", "Debugger is on '{current_state}' state but expected to be on '{expected_state}' state", 33, HashMap::new());

    // Info
    pub static ref READY: EllieMessage = EllieMessage::new("info", "Ready", 0);
    pub static ref EXIT_MESSAGE: EllieMessage = EllieMessage::new("info", "Debugger exited", 10);
    pub static ref PROGRAM_LOADED: EllieMessage = EllieMessage::new("info", "Program Loaded", 11);
    pub static ref BREAKPOINT_ADDED: EllieMessage = EllieMessage::new("info", "Breakpoint added", 12);
    pub static ref GET_PATHS_START: EllieMessage = EllieMessage::new("info", "Listing available paths", 13);
    pub static ref GET_PATHS_END: EllieMessage = EllieMessage::new("info", "Path listing complete", 15);
    pub static ref GET_BREAKPOINTS_START: EllieMessage = EllieMessage::new("info", "Listing breakpoints", 16);
    pub static ref GET_BREAKPOINTS_END: EllieMessage = EllieMessage::new("info", "Breakpoint listing complete", 18);
    pub static ref HIT_BREAKPOINT: EllieMessage = EllieMessage::new("info", "Breakpoint hit", 19);
    pub static ref THREAD_EXITED_GRACEFULLY: EllieMessage = EllieMessage::new("info", "Thread exited gracefully", 20);
    pub static ref GET_REGISTERS_START: EllieMessage = EllieMessage::new("info", "Listing registers", 22);
    pub static ref GET_REGISTERS_END: EllieMessage = EllieMessage::new("info", "Register listing complete", 24);
    pub static ref GET_STACK_MEMORY_START: EllieMessage = EllieMessage::new("info", "Listing stack memory", 26);
    pub static ref GET_STACK_MEMORY_END: EllieMessage = EllieMessage::new("info", "Stack memory listing complete", 28);
    pub static ref GET_HEAP_MEMORY_START: EllieMessage = EllieMessage::new("info", "Listing Heap memory", 29);
    pub static ref GET_HEAP_MEMORY_END: EllieMessage = EllieMessage::new("info", "Heap memory listing complete", 31);
    pub static ref STEP_FORWARD : EllieMessage = EllieMessage::new("info", "Stepping forward", 32);

    // Data Feed
    pub static ref GET_PATHS_ENTRY : EllieMessage = EllieMessage::new("log", "Module Name: {module_name}, File Path: {module_path}, Module File Path: {module_file_path}", 14);
    pub static ref GET_BREAKPOINTS_ENTRY : EllieMessage = EllieMessage::new("log", "Module File Path: {module_file_path}, Code Location: {code_location}, Stack Location: {stack_location}", 17);
    pub static ref THREAD_PANIC : EllieMessage = EllieMessage::new("error", "Thread panic, reason: {panic_reason}, code location: {panic_code_location}", 21);
    pub static ref GET_REGISTERS_ENTRY : EllieMessage = EllieMessage::new("log", "Register {register_name}, TypeID {type_id}, Type Size: {type_size}, Data: {data} Data Text: {data_text}", 23);
    pub static ref GET_STACK_MEMORY_ENTRY : EllieMessage = EllieMessage::new("log", "Stack Location: {stack_location}, TypeID: {type_id}, Type Size: {type_size}, Data: {data}, Data Text: {data_text}", 27);
    pub static ref GET_HEAP_MEMORY_ENTRY : EllieMessage = EllieMessage::new("log", "Heap Location: {heap_location}, Data: {data}", 30);
}
