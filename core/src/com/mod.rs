/*
    Provides communication with runtime
*/

/*
pub struct Future<T> {
    error: bool,
    has_data: bool,
    data: T
}

impl Future<T> {

}
*/


pub mod types;

pub struct Message {
    pub code: u8,
    pub message_type: types::MessageType,
}

pub struct Com<'a> {
    pub on_message: &'a dyn Fn(Message),
}

impl Com<'_> {
    pub fn send_async(&self, message: Message) {
        (self.on_message)(message);
    }
}
