use hmlm;
use std::io::prelude::{Read};
use crate::Diagram;
use xml;
use xml::reader::XmlEvent;

type Attributes = Vec<xml::attribute::OwnedAttribute>;

pub enum Error {
    Blob(usize)
}
impl Error {
    fn no_more_events() -> Self {
        Self::Blob(0)
    }
}
impl From<crate::diagram::ValueError> for Error {
    fn from(e: crate::diagram::ValueError) -> Error {
        Error::Blob(0)
    }
}
impl From<hmlm::reader::ParseError> for Error {
    fn from(e: hmlm::reader::ParseError) -> Error {
        Error::Blob(0)
    }
}


pub struct DiagramML<'a> {
    diagram: &'a mut Diagram<'a>,
}

struct MLReader<R:Read> {
    reader: hmlm::reader::EventReader<R>,
}

impl <R:Read> MLReader<R> {
    fn new(reader:hmlm::reader::EventReader<R>) -> Self {
        Self {
            reader,
        }
    }
    fn next_event(&mut self) -> Result<XmlEvent,Error> {
        match self.reader.next() {
            None => Err(Error::no_more_events()),
            Some(Err(e)) => Err(from(e)),
            Some(Ok(x))  => Ok(x),
        }
    }
}
trait MLEvent : Sized {
    /// ml_new is invoked from StartElement(<element type>, <atttributes>, _<namespace>)
    fn ml_new<R:Read> (reader:&mut MLReader<R>, attributes:&Attributes) -> Result<Self, Error>;
    /// ml_event is invoked after an object is created
    fn ml_event<R:Read> (&mut self, reader:&mut MLReader<R>) -> Result<Self, Error>;
}

impl MLEvent for crate::diagram::Shape {
    fn ml_new<R:Read> (reader:&mut MLReader<R>, attributes:&Attributes) -> Result<Self, Error> {
        let shape = crate::diagram::Shape::new(styles, attributes.iter().map(|a| (a.name.local_name, a.value)))?;
        shape.ml_event(reader)
    }
    fn ml_event<R:Read> (&mut self, reader:&mut MLReader<R>) -> Result<Self, Error> {
        match reader.next_event().unwrap() {
            XmlEvent::Comment(_) => (),
            XmlEvent::EndElement{..} => {
                return Ok(self);
            },
            _ => {
                return Err();
            },
        }
        self.ml_event(reader)
    }
}

/*
fn ml_event_text(d:&mut Diagram, ei:Iter<XmlEvent>) -> Result<> {
    match ei.next().unwrap() {
        XmlEvent::Comment(_) => ,
        XmlEvent::CData(s) => append string(),
        XmlEvent::Characters(s) => append string(),
        XmlEvent::EndElement => Ok(()),
        _ => Err(),
    }
    ml_event_text(d, ei)
}

fn ml_event_group(d:&mut Diagram, ei:Iter<XmlEvent>) -> Result<> {
    match ei.next().unwrap() {
        XmlEvent::Comment(_) => ,
        XmlEvent::EndElement => Ok(()),
        XmlEvent::StartElement => {
            
        },
        _ => Err(),
    }
    ml_event_group(d, ei)
}

fn document_handle_event(d:&mut Diagram, ei:Iter<XmlEvent>) -> {
    match ei.next().unwrap() {
        XmlEvent::EndDocument => Ok(()),
        XmlEvent::StartElement(name, attributes, namespace) =>
           {
               match name {
                   "g" => {
                       let g = Group::new(attributes)
                       document_handle_event(d, e.next(), ei),
                   }
               }
           }
    }
}

fn file_handle_event(d:&mut Diagram, e:XmlEvent, ei:Iter<XmlEvent>) -> {
    match ei.next().unwrap() {
        Ok(XmlEvent::StartDocument(_,_,_)) => document_handle_event(d, e.next(), ei),
        _ => ,
    }
}
 */

impl <'a> DiagramML<'a> {
    pub fn new<'b>(d:&'b mut Diagram<'b>) -> DiagramML<'b> {
        DiagramML { diagram:d }
    }
    pub fn read_file<R:Read>(&mut self, f:R) -> Result<(),Error> {
        let event_reader = hmlm::reader::EventReader::new(f); // Can use an xml::reader
        // ml_event_file(&mut d, event_reader.iter())?
    }
}
