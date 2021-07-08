/*
    Provides communication between the core and the parser also runtime
*/

use alloc::string::String;

pub struct Message {
    pub code: u8,
    pub payload: String,
}

pub struct Com<'a> {
    pub on_message: &'a dyn Fn(Message),
}

impl Com<'_> {
    pub fn send(&self, message: Message) {
        (self.on_message)(message);
    }
}
