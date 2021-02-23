// use std::io;
// use std::error::Error;
use std::fs::File;
extern crate xml;

extern crate hmlm;

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
    // let file          = std::io::BufReader::new(file);
    // let parser_config = xml::reader::ParserConfig::new().ignore_comments(false);
    // let event_reader        = xml::reader::EventReader::new_with_config(file, parser_config);
    let event_reader = hmlm::EventReader::new(file);

    let file_out   = File::create(output_file).unwrap();
    let file_out   = std::io::BufWriter::new(file_out);
    // let mut writer = hmlm::writer::EventWriter::new(file_out);
    let mut writer = xml::writer::EventWriter::new(file_out);
    for e in event_reader {
        match e {
            Ok(event) => {
                match event.as_xml_writer_event() {
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
