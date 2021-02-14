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
fn exit_on_err<T,U:std::fmt::Display>(result:Result<T,U>) -> T {
    match result {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
        Ok(v) => v
    }
}

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
        .arg(Arg::with_name("svg_grid")
             .long("svg_grid")
             .help("If provided then a grid is added to the SVG file (blue in region -100 to 100, spacing of 10; grey outside)")
             .multiple(false))
        .arg(Arg::with_name("svg_layout")
             .long("svg_layout")
             .help("Enable debug grids in green for every layout element")
             .multiple(false))
        .arg(Arg::with_name("svg_display")
             .long("svg_display")
             .help("Display SVG hierarchy")
             .multiple(false))
        .arg(Arg::with_name("diag_display")
             .long("diag_display")
             .help("Display diagram hierarchy")
             .multiple(false))
        .arg(Arg::with_name("file")
             .help("Input files to read")
             .multiple(true))
        .get_matches();

    let style_set = DiagramDescriptor::create_style_set();
    let diagram_descriptor = DiagramDescriptor::new(&style_set);
    let mut diagram = Diagram::new(&diagram_descriptor);
    match matches.values_of("file") {
        None => {
            println!("Should read stdin");
        },
        Some(vf) => {
            let mut diagram_ml = DiagramML::new(&mut diagram);
            for filename in vf {
                let file = File::open(filename).unwrap();
                exit_on_err( diagram_ml.read_file(file) );
            }
        },
    }
    let svg_show_grid   = matches.is_present("svg_grid");
    let svg_show_layout = matches.is_present("svg_layout");
    let svg_display     = matches.is_present("svg_display");
    let diag_display    = matches.is_present("diag_display");
    diagram.record_layout();
    println!("Uniqify");
    exit_on_err( diagram.uniquify() );
    println!("Style");
    exit_on_err( diagram.style() );
    println!("Lay out");
    // exit_on_err( diagram.layout(&Rectangle::new(0.,0.,297.,210.)) );
    exit_on_err( diagram.layout(&Rectangle::none()) );
    println!("Generate geometry");
    exit_on_err( diagram.geometry() );
    if diag_display { diagram.display(); }
    println!("Create SVG");
    let mut svg = Svg::new().set_grid(svg_show_grid).set_layout(svg_show_layout).set_display(svg_display);
    exit_on_err( diagram.generate_svg(&mut svg) );
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
    