use std::fs::File;
extern crate xml;
extern crate hmlm;
extern crate clap;
extern crate diagram;

use clap::{App, Arg};
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

    let mut diagram = Diagram::new();
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
    diagram.uniquify();
    diagram.style();
    diagram.layout(&Rectangle::new(0.,0.,210.,197.));
    diagram.geometry();
    for e in diagram.iter_elements() {
        println!("{:?}", e);
    }

    let mut svg = Svg::new();
    diagram.generate_svg(&mut svg);
    let file_out   = File::create("a.svg").unwrap();
    let file_out   = std::io::BufWriter::new(file_out);
    let mut writer = xml::writer::EventWriter::new(file_out);
    for e in svg.iter_events() {
        println!("{:?}",e);
        match e.as_writer_event() {
            None => (),
            Some(we) => writer.write(we).unwrap(),
        }
    }
    
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

#rect class="structure hardware"   grid="0,0,10,10"   "Cpu Core"
#rect class="structure kernel"     grid="0,20,10,10"  "VFS"
#rect class="structure kernel"     grid="0,40,10,10"  "Block\n(blk-mq)\n& NVME"
#rect class="structure kernel i10" grid="0,60,10,10"  "i10"
#rect class="structure kernel"     grid="0,80,10,10"  "TCP/IP\nStack"
#rect class="structure kernel"     grid="0,100,10,10" "Device"

#rect class="cpu core"             grid="20,0,30,10" "X"

#rect class="app app1"             grid="60,-20,30,10" "X"
#rect class="app app2"             grid="100,-20,10,10" "X"
#rect class="cpu core"             grid="60,0,10,10" "X"
#rect class="cpu core"             grid="80,0,30,10" "X"

#rect class="cpu core"             grid="80,12,50,6" "I/O syscalls"

#rect class="cpu core"             grid="80,0,30,10" ""


#style rounded_rect rx=5
#rule class=structure       style=

   
*/
    
