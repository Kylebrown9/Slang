use std::io::{ Result, Write, Read, stdin };
use std::fs::{ File };

pub fn file_to_string(file: File) -> Result<String> {
    let mut file_m = file;
    let mut data = String::new();

    file_m.read_to_string(&mut data)?;

    Ok(data)
}

pub fn stdio_to_string() -> Result<String> {
    let mut data = String::new();

    stdin().read_to_string(&mut data)?;

    Ok(data)
}

pub struct SimpleOutput {
    contents: Box<Write>
}

impl SimpleOutput {
    pub fn write(&mut self, val: &str) -> Result<()> {
        self.contents.write(val.as_bytes())?;

        Ok(())
    }
}

pub fn simplify_output(writer: Box<Write>) -> SimpleOutput {
    SimpleOutput {
        contents: writer
    }
}