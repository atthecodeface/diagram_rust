use hmlm;
use std::io::prelude::{Read};
use crate::{Diagram, DiagramContents, DiagramDescriptor};
use xml;
use xml::reader::XmlEvent;
use hmlm::XmlEventWithPos;
use crate::diagram::{ValueError, Element, Use, Group, Text, Shape};

type Attributes = Vec<xml::attribute::OwnedAttribute>;

fn to_nv(attributes:&Attributes) -> Vec<(String,String)> {
    attributes.iter().map(|a| (a.name.local_name.clone(), a.value.clone())).collect()
}
#[derive(Debug)]
pub enum MLError {
    Blob(usize),
    BadElementName(String), // file position?
    BadMLStructure, // file position?
    BadMLEvent(String),
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
            MLError::BadMLEvent(s)     => write!(f, "bad XML event {}", s),
            _ =>write!(f, "bad MLError"),
        }
    }
}

impl From<ValueError> for MLError {
    fn from(e: ValueError) -> MLError {
        MLError::Blob(0)
    }
}
impl From<hmlm::reader::ParseError> for MLError {
    fn from(e: hmlm::reader::ParseError) -> MLError {
        MLError::Blob(0)
    }
}


struct MLReader<'a, 'b, R:Read> {
    pub descriptor  : &'b DiagramDescriptor<'a>,
    pub contents    : &'b mut DiagramContents<'a>,
    pub reader  : hmlm::reader::EventReader<R>,
}

impl <'a, 'b, R:Read> MLReader<'a, 'b, R> {
    pub fn new<'c, 'd> ( descriptor: &'d DiagramDescriptor<'c>,
                         contents:   &'d mut DiagramContents<'c>,
                         reader:hmlm::reader::EventReader<R> ) -> MLReader<'c, 'd, R> {
        MLReader {
            descriptor,
            contents,
            reader,
        }
    }
    fn bad_ml_event(&self, ewp:XmlEventWithPos) -> MLError {
        MLError::BadMLEvent(format!("{:?} at {}",ewp.2, ewp.0).to_string())
    }
    fn read_file(&mut self) -> Result<(),MLError> {
        match self.next_event()? {
            (_,_,XmlEvent::StartDocument{..}) => self.read_document(),
            ewp => Err(self.bad_ml_event(ewp)),
        }
    }
    fn read_document(&mut self) -> Result<(),MLError> {
        match self.next_event()? {
            (_,_,XmlEvent::StartElement{name, ..}) => {
                if name.local_name=="diagram" {
                    self.read_diagram()?;
                    match self.next_event()? {
                        (_,_,XmlEvent::EndDocument) => { Ok (()) },
                        ewp => Err(self.bad_ml_event(ewp)),
                    }
                } else {
                    Err(MLError::bad_element_name(&name.local_name))
                }
            },
            ewp => Err(self.bad_ml_event(ewp)),
        }
    }
    fn read_diagram(&mut self) -> Result<(),MLError> {
        match self.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(()); },
            (_,_,XmlEvent::StartElement{name, attributes, ..}) => {
                // if name.local_name=="defs"
                let element = Element::ml_new(self, &name.local_name, &attributes)?;
                println!("Added element!");
                self.contents.elements.push(element)
            },
            ewp => { return Err(self.bad_ml_event(ewp)); },
        }
        self.read_diagram()
    }
    fn next_event(&mut self) -> Result<XmlEventWithPos,MLError> {
        match self.reader.next() {
            None => Err(MLError::no_more_events()),
            Some(Err(e)) => Err(MLError::from(e)),
            Some(Ok(x))  => Ok(x),
        }
    }
}

//tt MLEvent - internal trait to enable extension of type implementations
trait MLEvent <'a, R:Read, S:Sized> {
    /// ml_new is invoked from StartElement(<element type>, <atttributes>, _<namespace>)
    fn ml_new(reader:&mut MLReader<R>, name:&str, attributes:&Attributes) -> Result<S, MLError>;
    /// ml_event is invoked after an object is created
    fn ml_event (mut s:S, reader:&mut MLReader<R>) -> Result<S, MLError> { Ok(s) }
}

//ti MLEvent for Group
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Group<'a> {
    fn ml_new(reader:&mut MLReader<R>, name:&str, attributes:&Attributes) -> Result<Element<'a>, MLError> {
        let group = Element::new_group(reader.descriptor, name, to_nv(attributes))?;
        Self::ml_event(group, reader)
    }
    fn ml_event (mut s:Element<'a>, reader:&mut MLReader<R>) -> Result<Element<'a>, MLError> {
        match reader.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the group
            (_,_,XmlEvent::Comment(_))     => (), // continue
            (_,_,XmlEvent::StartElement{name, attributes, ..}) => { // content of group
                let element = Element::ml_new(reader, &name.local_name, &attributes)?;
                // self.add_element(element);
            },
            ewp => { return Err(reader.bad_ml_event(ewp)); },
        }
        Self::ml_event(s, reader)
    }
}

//ti MLEvent for Shape
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Shape {
    fn ml_new (reader:&mut MLReader<R>, name:&str, attributes:&Attributes) -> Result<Element<'a>, MLError> {
        let shape = Element::new_shape(reader.descriptor, name, to_nv(attributes))?;
        Self::ml_event(shape, reader)
    }
    fn ml_event (mut s:Element<'a>, reader:&mut MLReader<R>) -> Result<Element<'a>, MLError> {
        match reader.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the group
            (_,_,XmlEvent::Comment(_))     => (), // continue
            ewp => { return Err(reader.bad_ml_event(ewp)); },
        }
        Self::ml_event(s, reader)
    }
}

//ti MLEvent for Element
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Element<'a> {
    fn ml_new (reader:&mut MLReader<R>, name:&str, attributes:&Attributes) -> Result<Self, MLError> {
        match name {
            "shape" => Ok(Shape::ml_new(reader, name, attributes)?),
            "g"     => Ok(Group::ml_new(reader, name, attributes)?),
            _ => return Err(MLError::bad_element_name(name))
        }
    }
}

pub struct DiagramML<'a, 'b> {
    diagram: &'a mut Diagram<'b>,
}

/// ```
/// extern crate diagram;
/// use diagram::{Diagram, DiagramML};
/// let mut d = Diagram::new();
/// let mut dml = DiagramML::new(&mut d);
/// dml.read_file("#diagram ##shape ##g ###shape ##shape".as_bytes()).unwrap();
/// assert_eq!(0, d.contents.definitions.len());
/// assert_eq!(3, d.contents.elements.len());
/// ```
impl <'a, 'b> DiagramML<'a, 'b> {
    pub fn new<'c, 'd>(d:&'c mut Diagram<'d>) -> DiagramML<'c, 'd> {
        DiagramML { diagram:d }
    }
    pub fn read_file<R:Read>(&mut self, f:R) -> Result<(),MLError> {
        let event_reader = hmlm::reader::EventReader::new(f); // Can use an xml::reader
        MLReader::new(&self.diagram.descriptor, &mut self.diagram.contents, event_reader).read_file()
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
        assert_eq!(0, d.contents.definitions.len());
        assert_eq!(0, d.contents.elements.len());
    }
}
