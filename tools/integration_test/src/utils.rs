use std::{io::{Write, Read}, fs::File};

use ellie_engine::ellie_bytecode::assembler::AssembleResult;

pub struct VecReader<'a> {
    data: &'a mut Vec<u8>,
    pos: usize,
}

impl<'a> VecReader<'a> {
    pub fn new(data: &'a mut Vec<u8>) -> VecReader<'a> {
        VecReader { data: data, pos: 0 }
    }
}

impl<'a> Read for VecReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = std::cmp::min(buf.len(), self.data.len() - self.pos);
        let slice = &self.data[self.pos..self.pos + len];
        buf[..len].copy_from_slice(slice);
        self.pos += len;
        Ok(len)
    }
}

pub struct StringWrite {
    pub data: String,
}

impl StringWrite {
    pub fn new() -> StringWrite {
        StringWrite {
            data: String::new(),
        }
    }
}

impl Write for StringWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.data.push_str(std::str::from_utf8(buf).unwrap());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn render_text(assembler_result: &AssembleResult) {
    let mut assembly_code_string = StringWrite::new();
    assembler_result.alternate_render(&mut assembly_code_string);
    println!("{}", assembly_code_string.data);
}

pub fn save_file(assembler_result: &AssembleResult) {
    let mut assembly_code_string = StringWrite::new();
    assembler_result.alternate_render(&mut assembly_code_string);
    let mut file = File::create(
        ".(test.eic",
    )
    .unwrap();
    let mut dfile = File::create("./test.eig").unwrap();
    assembler_result.render_binary(&mut file, &mut dfile);
}