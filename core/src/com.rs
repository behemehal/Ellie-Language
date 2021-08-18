use alloc::string::String;

#[derive(Clone, Debug)]
pub enum MessageType {
    ParserLineExec,
    ParserImportItem,
    ParserImportNativeItem,
    ParserVariableItem,
    ParserFunctionItem,
    ParserClassItem,
    ParseComplete,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub id: String,
    pub message_type: MessageType,
    pub from: String,
    pub from_chain: Option<String>,
    pub message_data: String,
}
