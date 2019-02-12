use std::io::{ Write, Read, stdin };
use std::fs::{ File };

pub fn file_to_string(file: File) -> String {
    let mut file_m = file;
    let mut data = String::new();

    file_m.read_to_string(&mut data);

    data
}

pub fn stdio_to_string() -> String {
    let mut data = String::new();

    stdin().read_to_string(&mut data);

    data
}

pub struct SimpleOutput {
    contents: Box<Write>
}

impl SimpleOutput {

    pub fn write(&mut self, val: &str) {
        self.contents.write(val.as_bytes());
    }
}

pub fn simplify_output(writer: Box<Write>) -> SimpleOutput {
    SimpleOutput {
        contents: writer
    }
}