use std::io::{ BufRead };

pub struct BufReadIter<r>
    where r: BufRead {

    bufreader: r
}

impl<r: BufRead> From<r> for BufReadIter<r> {
    fn from(other: r) -> Self {
        BufReadIter {
            bufreader: other
        }
    }
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