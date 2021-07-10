// #[macro_use]
// extern crate lazy_static;

use std::fs::File;
// extern crate xml;
// extern crate hmlm;
// extern crate clap;
// extern crate geometry;
// extern crate diagram;

const DEBUG_MAIN : bool = 1 == 0;

use clap::{App, Arg};
use diagram::DiagramDescriptor;
use diagram::Diagram;
use diagram::DiagramML;
use geometry::Rectangle;
use diagram::{Svg};
fn exit_on_err<T,U:std::fmt::Display>(result:Result<T,U>) -> T {
    match result {
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
        Ok(v) => v
    }
}

fn exit_on_err_vec<T>(result:Result<T,Vec<String>>) -> T {
    match result {
        Err(vs) => {
            for v in vs {
                eprintln!("{}\n", v);
            }
            std::process::exit(1);
        },
        Ok(v) => v
    }
}

static mut SVG_INDENT_STR :String = String::new();
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
        .arg(Arg::with_name("svg_version")
             .long("svg_version")
             .help("Specify SVG version for output - 2.0 by default")
             .required(false)
             .takes_value(true))
        .arg(Arg::with_name("svg_indent")
             .long("svg_indent")
             .help("Put XML elements in SVG on newlines and indent using this string; use '' for git-friendly svg output")
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
        .arg(Arg::with_name("svg_content")
             .long("svg_content")
             .help("Enable debug showing of content rectangles for elements")
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
             .help("Input files to read; first must be a diagram, others must be library")
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
            let mut is_library = false;
            for filename in vf {
                let file = File::open(filename).unwrap();
                exit_on_err_vec(diagram_ml.read_file(file, is_library));
                is_library = true;
            }
        },
    }
    let svg_version      = {
        match matches.value_of("svg_version") {
            Some("1.0") => 10,
            Some("1.1") => 11,
            _     => 20,
        }
    };
    let svg_indent       = matches.value_of("svg_indent").map(|s| s.to_string());
    let svg_show_grid    = matches.is_present("svg_grid");
    let svg_show_layout  = matches.is_present("svg_layout");
    let svg_show_content = matches.is_present("svg_content");
    let svg_display      = matches.is_present("svg_display");
    let diag_display     = matches.is_present("diag_display");
    let output_file      = matches.value_of("output").unwrap_or("a.svg");
    if DEBUG_MAIN{ println!("Uniqify"); }
    exit_on_err( diagram.uniquify() );
    if DEBUG_MAIN{ println!("Apply stylesheet"); }
    diagram.apply_stylesheet();
    if DEBUG_MAIN{ println!("Style"); }
    exit_on_err( diagram.style() );
    if DEBUG_MAIN{ println!("Lay out"); }
    // exit_on_err( diagram.layout(&Rectangle::new(0.,0.,297.,210.)) );
    exit_on_err( diagram.layout(&Rectangle::none()) );
    if DEBUG_MAIN{ println!("Generate geometry"); }
    exit_on_err( diagram.geometry() );
    if diag_display { diagram.display(); }
    if DEBUG_MAIN{ println!("Create SVG"); }
    let mut svg = Svg::new(&diagram)
        .set_version(svg_version)
        .set_grid(svg_show_grid)
        .set_layout(svg_show_layout)
        .set_display(svg_display)
        .set_content_rectangles(svg_show_content);
    exit_on_err( svg.generate_diagram() );
    if DEBUG_MAIN{ println!("Write SVG"); }
    let file_out   = File::create(output_file).unwrap();
    let file_out   = std::io::BufWriter::new(file_out);
    let mut emitter_config = xml::writer::EmitterConfig::new();
    if let Some(indent) = svg_indent {
        unsafe {
            SVG_INDENT_STR = indent.clone();
            emitter_config = emitter_config.perform_indent(true).indent_string(&SVG_INDENT_STR);
        }
    }
    let mut writer = xml::writer::EventWriter::new_with_config(file_out,
                                                               emitter_config
    );
    for e in svg.iter_events() {
        match e.as_writer_event() {
            None => (),
            Some(we) => writer.write(we).unwrap(),
        }
    }
    if DEBUG_MAIN{ println!("Complete"); }

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

