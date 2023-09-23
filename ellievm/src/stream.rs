use std::io::{BufRead, Read, Result};

pub struct InputStream<'a, T>
where
    T: Read + BufRead + ?Sized,
{
    pub stream: &'a mut T,
    pub external_lines: Vec<String>,
}

impl<T> InputStream<'_, T>
where
    T: Read + BufRead + ?Sized,
{
    pub fn new(stream: &mut T) -> InputStream<'_, T> {
        InputStream {
            stream,
            external_lines: Vec::new(),
        }
    }

    pub fn read_line(&mut self, buf: &mut String) -> Result<(usize, bool)> {
        if self.external_lines.is_empty() {
            self.stream.read_line(buf).map(|x| (x, false))
        } else {
            let line = self.external_lines.remove(0);
            *buf += line.to_string().as_str();
            Result::Ok((line.len(), true))
        }
    }
}
