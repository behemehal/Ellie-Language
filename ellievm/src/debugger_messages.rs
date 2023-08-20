use std::collections::HashMap;


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
                    message = message.replace(key, value);
                }
                message
            }
            None => self.message.clone(),
        }
    }

    pub fn build_json_message(&self) -> String {
        format!(
            "{{\"type\":\"{}\",\"code\": \"{}\",\"message\":\"{}\"}}",
            self.r#type,
            self.code,
            self.build_message()
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

lazy_static! {
    pub static ref READY: EllieMessage = EllieMessage::new("info", "Ready", 0);

    // Error
    pub static ref UNKNOWN_COMMAND: EllieMessage = EllieMessage::new("error", "Unknown command", 1);
    pub static ref INVALID_ARGUMENT_LENGTH: EllieMessage = EllieMessage::new("error", "Invalid argument length, check syntax of the command", 2);
    pub static ref INVALID_ARGUMENT_TYPE: EllieMessage = EllieMessage::new("error", "Invalid argument type, check syntax of the command", 3);
    pub static ref CANT_RENDER_INFO: EllieMessage = EllieMessage::new("error", "Output is not a standard text, this might be a help text which is not feasible to be json logged.", 4);
    pub static ref FILE_READ_ERROR: EllieMessage = EllieMessage::new_with_variables("error", "Failed to read the file: {error}", 5, HashMap::new());
    pub static ref PROGRAM_READ_ERROR: EllieMessage = EllieMessage::new_with_variables("error", "Failed to read program: {error}", 6, HashMap::new());
    pub static ref DEBUG_FILE_PARSE_ERROR: EllieMessage = EllieMessage::new_with_variables("error", "Failed to parse debug fie: {error}", 7, HashMap::new());
    
    pub static ref EXIT_MESSAGE: EllieMessage = EllieMessage::new("info", "Debugger exited", 8);
    pub static ref PROGRAM_LOADED: EllieMessage = EllieMessage::new("info", "Program Loaded", 8);
}
