// use std::io;
// use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
extern crate xml;

// use xml::namespace::Namespace;
// use xml::name::Name;
// use xml::attribute::Attribute;

mod error;
mod writer;
mod reader;

pub use error::{HmlmError, HmlmResult};

fn main() {
    let file = File::open("file.xml").unwrap();
    let file = BufReader::new(file);

    let parser_config = xml::reader::ParserConfig::new().ignore_comments(false);
    let parser = xml::reader::EventReader::new_with_config(file, parser_config);

    let file_out = File::create("file.hmlm").unwrap();
    let file_out = BufWriter::new(file_out);
    let mut writer = writer::EventWriter::new(file_out);
    for e in parser {
        match e {
            Ok(e) => {
                match e.as_writer_event() {
                    None => (),
                    Some(we) => writer.write(we).unwrap(),
                }
            },
            _ => ()}
    }
}
