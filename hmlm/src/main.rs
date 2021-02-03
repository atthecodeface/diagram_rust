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

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Gavin J Stark")
        (about: "Converts XML to/from HMLH")
        (@arg INPUT: +required "Sets the input file to use")
        (@arg OUTPUT: +required "Sets the output file to use")
    ).get_matches();

    let input_file = matches.value_of("INPUT").unwrap();
    let output_file = matches.value_of("OUTPUT").unwrap();
    println!("{}, {}",input_file, output_file);

    let file          = File::open(input_file).unwrap();
    // let file          = BufReader::new(file);
    // let parser_config = xml::reader::ParserConfig::new().ignore_comments(false);
    // let parser        = xml::reader::EventReader::new_with_config(file, parser_config);
    let mut reader = reader::char::Reader::new(file);
    let event_reader = reader::EventReader::new(&mut reader);
    //let mut lexer  = reader::lexer::Lexer::new(&mut reader);
    //let mut parser     = reader::parser::Parser::new();

    let file_out   = File::create(output_file).unwrap();
    let file_out   = BufWriter::new(file_out);
    // let mut writer = writer::EventWriter::new(file_out);
    let mut writer = xml::writer::EventWriter::new(file_out);
    //    for e in parser {
//    loop {
    //        let e = parser.next_event(|| lexer.next_token_with_pos());a
    for e in event_reader {
        match e {
            Ok(e) => {
                match e.as_writer_event() {
                    None => (),
                    Some(we) => writer.write(we).unwrap(),
                }
            },
            Err(e) => {
                println!("Error {:?}",e);
                break;
            },
        }
    }
}
