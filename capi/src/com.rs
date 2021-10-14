use libc::c_char;

#[repr(C)]
pub enum MessageType {
    ParserLineExec,
    ParserImportItem,
    ParserImportNativeItem,
    ParserVariableItem,
    ParserFunctionItem,
    ParserClassItem,
    ParseComplete,
}

#[repr(C)]
pub enum ErrorChainOption {
    Some(*mut c_char),
    None,
}

#[repr(C)]
pub struct Message {
    pub id: *mut c_char,
    pub message_type: MessageType,
    pub from: *mut c_char,
    pub from_chain: ErrorChainOption,
    pub message_data: *mut c_char,
}