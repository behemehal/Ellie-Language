use crate::processors::value_processor;
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error, utils};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_array(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    if let types::Types::Array(ref mut data) = itered_data.data.value {
        /*
            Don't look right to it, it's dangerously complicated
            Here is the story,

            I assume you as a person that doesn't have a programming experience. In a loop you can process a data
            and if a same data applied you can use the same function to process the data, This program uses millions of same pattern,
            I experienced this a million times, Created programs that runs through loops processing big data. But this time I got stuck at this
            function. It took almost 2 months, Thank god I got it.

            A Weird way to stop a letter,

            Sincerely

            Ahmetcan Aksu 🦀
        */

        let last_entry = data.clone().collective.len();
        //let mut value: types::Types = types::Types::Null;

        let is_s_n = last_entry == 0 || data.collective[last_entry - 1].value.is_type_complete();

        if letter_char == "[" && !data.child_start && is_s_n {
            if !data.comma && last_entry != 0 {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/array.rs:0"
                        .to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: 
