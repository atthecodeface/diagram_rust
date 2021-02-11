use std::fs::File;
extern crate xml;
extern crate hmlm;
extern crate clap;
extern crate diagram;

use clap::{App, Arg};
use diagram::DiagramDescriptor;
use diagram::Diagram;
use diagram::DiagramML;
use diagram::Rectangle;
use diagram::{Svg, GenerateSvg};
fn main() {
    let matches = App::new("diagram")
        .about("SVG creator from a diagram descriptor")
        .author( "Gavin J Stark")
        .version( "0.1")
        .arg(Arg::with_name("output")
             .long("output")
             .help("Sets the output file to use")
             .required(false)
             .takes_value(true))
        .arg(Arg::with_name("debug")
             .short("d")
             .multiple(true))
        .arg(Arg::with_name("file")
             .help("Input files to read")
             .multiple(true))
        .get_matches();

    let diagram_descriptor = DiagramDescriptor::new();
    let mut diagram = Diagram::new(&diagram_descriptor);
    match matches.values_of("file") {
        None => {
            println!("Should read stdin");
        },
        Some(vf) => {
            let mut diagram_ml = DiagramML::new(&mut diagram);
            for filename in vf {
                let file = File::open(filename).unwrap();
                match diagram_ml.read_file(file)
                {
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    },
                    _ => (),
                }
            }
        },
    }
    diagram.record_layout();
    diagram.uniquify();
    println!("Style");
    diagram.style();
    println!("Lay out");
    diagram.layout(&Rectangle::new(0.,0.,210.,197.));
    println!("Generate geometry");
    diagram.geometry();
    println!("Create SVG");
    let mut svg = Svg::new().set_grid(false).set_layout(true);
    diagram.generate_svg(&mut svg);
    println!("Write SVG");
    let file_out   = File::create("a.svg").unwrap();
    let file_out   = std::io::BufWriter::new(file_out);
    let mut writer = xml::writer::EventWriter::new(file_out);
    for e in svg.iter_events() {
        match e.as_writer_event() {
            None => (),
            Some(we) => writer.write(we).unwrap(),
        }
    }
    println!("Complete");
    
/*
    let input_file  = 
    // let output_file = matches.value_of("output").unwrap();
    
    
    let event_reader = hmlm::reader::EventReader::new(file); // Can use an xml::reader

    let file_out   = File::create(output_file).unwrap();
    let file_out   = std::io::BufWriter::new(file_out);
    // let mut writer = hmlm::writer::EventWriter::new(file_out);
    let mut writer = xml::writer::EventWriter::new(file_out);
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
*/
}

/*
core attributes: id
class

transform : matrix/translate/scale/rotate/skewX/skewY
stroke-width
stroke (color)
fill (color)
font-family: string list
font-size:


SVG tags:


defs
g
image
marker
text : x,y,dx,dy,rotate,lengthAdjust,textLength
path: d
[d has Moveto, Lineto, Cubicbezierto, Quadraticbezierto, ellipticalArcto, Zclosepath - upper case absolute, lower case relative
line
circle
ellipse
polygon: points
polyline: points
rect : x, y, width, height, rx, ry


pattern

 */
/*

   
*/
    
