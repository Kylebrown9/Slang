use std::io::{ Read, BufRead, BufReader, Write };

pub type SimpleInput = Box<Iterator<Item=String>>;

pub struct BufReadIter<r>
    where r: BufRead {

    bufreader: r
}

pub fn simplify_input(reader: Box<Read>) -> SimpleInput {
    Box::new(BufReadIter {
        bufreader: BufReader::new(reader)
    })
}

impl<r: BufRead> Iterator for BufReadIter<r> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut result = String::new();

        match self.bufreader.read_line(&mut result) {
            Ok(_) => Some(result),
            Err(_) => None
        }
    }
}

pub struct SimpleOutput {
    contents: Box<Write>
}

impl SimpleOutput {

    pub fn write(&mut self, line: String) {
        let with_ln = line + "\n";

        self.contents.write(with_ln.as_bytes());
    }
}

pub fn simplify_output(writer: Box<Write>) -> SimpleOutput {
    SimpleOutput {
        contents: writer
    }
}