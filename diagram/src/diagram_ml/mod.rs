use hmlm;
use std::io::prelude::{Read};
use crate::Diagram;
use xml;
use xml::reader::XmlEvent;

type Attributes = Vec<xml::attribute::OwnedAttribute>;

fn to_nv(attributes:&Attributes) -> Vec<(String,String)> {
    attributes.iter().map(|a| (a.name.local_name.clone(), a.value.clone())).collect()
}
#[derive(Debug)]
pub enum MLError {
    Blob(usize),
    BadElementName(String), // file position?
    BadMLStructure, // file position?
}

impl MLError {
    fn no_more_events() -> Self {
        Self::Blob(0)
    }
    fn bad_element_name(name:&str) -> Self {
        Self::BadElementName(name.to_string())
    }
    fn bad_ml_structure() -> Self {
        Self::BadMLStructure
    }
}
//ip std::fmt::Display for MLError
impl std::fmt::Display for MLError {
    //mp fmt - format a `MLError` for display
    /// Display the `MLError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MLError::BadElementName(n) => write!(f, "bad element name {}", n),
            MLError::BadMLStructure    => write!(f, "bad structure"),
            _ =>write!(f, "bad MLError"),
        }
    }
}

impl From<crate::diagram::ValueError> for MLError {
    fn from(e: crate::diagram::ValueError) -> MLError {
        MLError::Blob(0)
    }
}
impl From<hmlm::reader::ParseError> for MLError {
    fn from(e: hmlm::reader::ParseError) -> MLError {
        MLError::Blob(0)
    }
}


struct MLReader<'a, 'b, R:Read> {
    pub diagram : &'a mut Diagram<'b>,
    pub reader  : hmlm::reader::EventReader<R>,
}

impl <'a, 'b, R:Read> MLReader<'a, 'b, R> {
    pub fn new<'c, 'd> (diagram:&'c mut Diagram<'d>, reader:hmlm::reader::EventReader<R>) -> MLReader<'c, 'd, R> {
        MLReader {
            diagram,
            reader,
        }
    }
    fn read_file(&mut self) -> Result<(),MLError> {
        match self.next_event()? {
            XmlEvent::StartDocument{..} => self.read_document(),
            _ => Err(MLError::bad_ml_structure()),
        }
    }
    fn read_document(&mut self) -> Result<(),MLError> {
        match self.next_event()? {
            XmlEvent::StartElement{name, ..} => {
                if name.local_name=="diagram" {
                    self.read_diagram()?;
                    match self.next_event()? {
                        XmlEvent::EndDocument => {
                            Ok(())
                        },
                        _ => Err(MLError::bad_ml_structure()),
                    }
                } else {
                    Err(MLError::bad_element_name(&name.local_name))
                }
            },
            _ => Err(MLError::bad_ml_structure()),
        }
    }
    fn read_diagram(&mut self) -> Result<(),MLError> {
        match self.next_event()? {
            XmlEvent::EndElement{..} => {
                return Ok(());
            },
            XmlEvent::StartElement{name, attributes, ..} => {
                // if name.local_name=="defs"
                let element = crate::diagram::Element::ml_new(self, &name.local_name, &attributes)?;
                println!("Added element!");
                self.diagram.elements.push(element)
            },
            _ => {
                return Err(MLError::bad_ml_structure());
            },
        }
        self.read_diagram()
    }
    fn next_event(&mut self) -> Result<XmlEvent,MLError> {
        match self.reader.next() {
            None => Err(MLError::no_more_events()),
            Some(Err(e)) => Err(MLError::from(e)),
            Some(Ok(x))  => Ok(x),
        }
    }
}
trait MLEvent : Sized {
    /// ml_new is invoked from StartElement(<element type>, <atttributes>, _<namespace>)
    fn ml_new<R:Read> (reader:&mut MLReader<R>, name:&str, attributes:&Attributes) -> Result<Self, MLError>;
    /// ml_event is invoked after an object is created
    fn ml_event<R:Read> (self, reader:&mut MLReader<R>) -> Result<Self, MLError>;
}

impl MLEvent for crate::diagram::Shape {
    fn ml_new<R:Read> (reader:&mut MLReader<R>, _name:&str, attributes:&Attributes) -> Result<Self, MLError> {
        let styles = reader.diagram.styles("shape").unwrap();
        let shape = crate::diagram::Shape::new(styles, to_nv(attributes))?;
        shape.ml_event(reader)
    }
    fn ml_event<R:Read> (self, reader:&mut MLReader<R>) -> Result<Self, MLError> {
        match reader.next_event()? {
            XmlEvent::Comment(_) => (),
            XmlEvent::EndElement{..} => {
                return Ok(self);
            },
            _ => {
                return Err(MLError::no_more_events());
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

impl MLEvent for crate::diagram::Element {
    fn ml_new<R:Read> (reader:&mut MLReader<R>, name:&str, attributes:&Attributes) -> Result<Self, MLError> {
        match name {
            "shape" => Ok(crate::diagram::Element::Shape(crate::diagram::Shape::ml_new(reader, name, attributes)?)),
            _ => return Err(MLError::bad_element_name(name))
        }
    }
    fn ml_event<R:Read> (self, reader:&mut MLReader<R>) -> Result<Self, MLError> { Err(MLError::Blob(0)) }
}

pub struct DiagramML<'a, 'b> {
    diagram: &'a mut Diagram<'b>,
}

/// ```
/// extern crate diagram;
/// use diagram::{Diagram, DiagramML};
/// let mut d = Diagram::new();
/// let mut dml = DiagramML::new(&mut d);
/// dml.read_file("#diagram ##shape".as_bytes()).unwrap();
/// assert_eq!(0, d.definitions.len());
/// assert_eq!(1, d.elements.len());
/// ```
impl <'a, 'b> DiagramML<'a, 'b> {
    pub fn new<'c, 'd>(d:&'c mut Diagram<'d>) -> DiagramML<'c, 'd> {
        DiagramML { diagram:d }
    }
    pub fn read_file<R:Read>(&mut self, f:R) -> Result<(),MLError> {
        let event_reader = hmlm::reader::EventReader::new(f); // Can use an xml::reader
        MLReader::new(&mut self.diagram, event_reader).read_file()
    }
}

//a Test
#[cfg(test)]
mod tests {
    use crate::{Diagram, DiagramML};
    #[test]
    fn test_why() {
        let mut d = Diagram::new();
        let mut dml = DiagramML::new(&mut d);
        dml.read_file("#blob".as_bytes());
        assert_eq!(0, d.definitions.len());
        assert_eq!(0, d.elements.len());
    }
}
