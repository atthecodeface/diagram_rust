use std::io::prelude::{Read};
use hmlm;
use super::Diagram;

pub enum Error {
    Blob(usize)
}

pub struct DiagramML<'a> {
    diagram: &'a mut Diagram,
    
}
impl <'a> DiagramML<'a> {
    pub fn new<'b>(d:&'b mut Diagram) -> DiagramML<'b> {
        DiagramML { diagram:d }
    }
    pub fn read_file<R:Read>(&mut self, f:R) -> Result<(),Error> {
        let event_reader = hmlm::reader::EventReader::new(f); // Can use an xml::reader
        for e in event_reader {
            // let e = e.unwrap()?;
        }
        Ok(())
    }
}
