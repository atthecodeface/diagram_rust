use hmlm;
use std::io::prelude::{Read};
use crate::{Diagram, DiagramContents, DiagramDescriptor};
use xml;
use xml::reader::XmlEvent;
use hmlm::{XmlEventWithPos, FilePosition};
use crate::diagram::{ValueError, Element, Use, Group, Text, Shape};

type Attributes = Vec<xml::attribute::OwnedAttribute>;

fn to_nv(attributes:&Attributes) -> Vec<(String,String)> {
    attributes.iter().map(|a| (a.name.local_name.clone(), a.value.clone())).collect()
}
#[derive(Debug)]
pub enum MLError {
    EndOfStream,
    BadElementName(FilePosition, String),
    BadMLEvent(String),
    BadValue(String),
    ParseError(String),
}

impl MLError {
    fn unexpected_end_of_stream() -> Self {
        Self::EndOfStream
    }
    fn bad_element_name(fp:&FilePosition, name:&str) -> Self {
        Self::BadElementName(fp.clone(), name.to_string())
    }
}
//ip std::fmt::Display for MLError
impl std::fmt::Display for MLError {
    //mp fmt - format a `MLError` for display
    /// Display the `MLError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MLError::EndOfStream          => write!(f, "Unexpected end of XML event stream - bug in event source"),
            MLError::BadElementName(fp,n) => write!(f, "Bad element '{}' at {}", n, fp),
            MLError::BadMLEvent(s)        => write!(f, "Bad XML event {}", s),
            MLError::BadValue(s)          => f.write_str(s),
            MLError::ParseError(s)        => f.write_str(s),
        }
    }
}

impl From<ValueError> for MLError {
    fn from(e: ValueError) -> MLError {
        match e {
            ValueError::BadValue(s) => MLError::BadValue(s),
            _ => MLError::BadValue(e.to_string()),
        }
    }
}
impl From<hmlm::reader::ParseError> for MLError {
    fn from(e: hmlm::reader::ParseError) -> MLError {
        MLError::ParseError(e.to_string())
    }
}

pub struct MLErrorList {
    errors : Vec<MLError>,
}
impl MLErrorList {
    pub fn new() -> Self {
        Self { errors : Vec::new() }
    }
    pub fn add(&mut self, e:MLError) -> () {
        self.errors.push(e);
    }
    pub fn update_t<T>(&mut self, e:Result<T,MLError>) -> Result<T, ()> {
        match e {
            Err(e) => {
                self.errors.push(e);
                Err(())
                },
            Ok(x) => Ok(x),
        }
    }
    pub fn update<T>(&mut self, e:Result<T,MLError>) {
        match e {
            Err(e) => { self.errors.push(e); }
            _ => (),
        }
    }
    pub fn as_err<T>(&mut self, v:Result<T,MLError>) -> Result<T, MLErrorList> {
        let x = std::mem::replace(&mut self.errors, Vec::new());
        match x.len() {
            0 => {
                match v {
                    Ok(v) => Ok(v),
                    _ => Err(Self{errors:x})
                }
            },
            _ => Err(Self{errors:x})
        }
    }
}

//ip std::fmt::Display for MLErrorList
impl std::fmt::Display for MLErrorList {
    //mp fmt - format a `MLErrorList` for display
    /// Display the `MLError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for e in &self.errors {
            write!(f, "{}\n", *e)?;
        }
        Ok(())
    }
}
//tt MLEvent - internal trait to enable extension of type implementations
trait MLEvent <'a, R:Read, S:Sized> {
    /// ml_new is invoked from StartElement(<element type>, <atttributes>, _<namespace>)
    fn ml_new(reader:&mut MLReader<R>, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<S, MLError>;
    /// ml_event is invoked after an object is created
    fn ml_event (mut s:S, reader:&mut MLReader<R>) -> Result<S, MLError> { Ok(s) }
}

//ti MLEvent for Group
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Group<'a> {
    fn ml_new(reader:&mut MLReader<R>, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<Element<'a>, MLError> {
        let group = Element::new_group(reader.descriptor, name, to_nv(attributes))?;
        Self::ml_event(group, reader)
    }
    fn ml_event (mut s:Element<'a>, reader:&mut MLReader<R>) -> Result<Element<'a>, MLError> {
        match reader.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the group
            (_,_,XmlEvent::Comment(_))     => (), // continue
            (fp,_,XmlEvent::StartElement{name, attributes, ..}) => { // content of group
                match Element::ml_new(reader, &fp, &name.local_name, &attributes) {
                    Ok(element) => {
                // self.add_element(element);
                    },
                    e => { reader.errors.update(e); },
                }
            },
            ewp => { return Err(reader.bad_ml_event(ewp)); },
        }
        Self::ml_event(s, reader)
    }
}

//ti MLEvent for Shape
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Shape {
    fn ml_new (reader:&mut MLReader<R>, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<Element<'a>, MLError> {
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
    fn ml_new (reader:&mut MLReader<R>, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<Self, MLError> {
        match name {
            "shape" => Ok(Shape::ml_new(reader, fp, name, attributes)?),
            "g"     => Ok(Group::ml_new(reader, fp, name, attributes)?),
            _ => {
                let r = BadXMLElement::ml_new(reader, fp, name, attributes);
                reader.errors.update(r);
                return Err(MLError::bad_element_name(fp,name))
            }
        }
    }
}

//ti MLEvent for BadXMLElement
struct BadXMLElement {
}
impl <'a, R:Read> MLEvent <'a, R, BadXMLElement> for BadXMLElement {
    fn ml_new (reader:&mut MLReader<R>, _fp:&FilePosition, _name:&str, _attributes:&Attributes) -> Result<Self, MLError> {
        let s = Self {};
        Self::ml_event(s, reader)
    }
    fn ml_event (mut s:Self, reader:&mut MLReader<R>) -> Result<Self, MLError> {
        match reader.next_event()? {
            (fp,_,XmlEvent::StartElement{name, attributes, ..}) => {
                let r = BadXMLElement::ml_new(reader, &fp, &name.local_name, &attributes);
                reader.errors.update(r);
            }
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the group
            (_,_,XmlEvent::Comment(_))     => (), // continue
            (_,_,XmlEvent::Whitespace(_))  => (), // continue
            (_,_,XmlEvent::Characters(_))  => (), // continue
            (_,_,XmlEvent::CData(_))       => (), // continue
            ewp => { return Err(reader.bad_ml_event(ewp)); },
        }
        Self::ml_event(s, reader)
    }
}

//tp MLReader
struct MLReader<'a, 'b, R:Read> {
    pub descriptor  : &'b DiagramDescriptor<'a>,
    pub contents    : &'b mut DiagramContents<'a>,
    pub reader      : hmlm::reader::EventReader<R>,
    errors          : MLErrorList,
}

//ti MLReader
impl <'a, 'b, R:Read> MLReader<'a, 'b, R> {
    pub fn new<'c, 'd> ( descriptor: &'d DiagramDescriptor<'c>,
                         contents:   &'d mut DiagramContents<'c>,
                         reader:hmlm::reader::EventReader<R> ) -> MLReader<'c, 'd, R> {
        MLReader {
            descriptor,
            contents,
            reader,
            errors :MLErrorList::new(),
        }
    }
    fn bad_ml_event(&self, ewp:XmlEventWithPos) -> MLError {
        MLError::BadMLEvent(format!("{:?} at {}",ewp.2, ewp.0).to_string())
    }
    fn read_diagram(&mut self) -> Result<(),MLError> {
        match self.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(()); },
            (fp,_,XmlEvent::StartElement{name, attributes, ..}) => {
                // if name.local_name=="defs"
                match Element::ml_new(self, &fp, &name.local_name, &attributes) {
                    Ok(element) => {
                        self.contents.elements.push(element);
                    },
                    e => { self.errors.update(e); },
                }
            },
            ewp => { return Err(self.bad_ml_event(ewp)); },
        }
        self.read_diagram()
    }
    fn read_document(&mut self) -> Result<(),MLError> {
        match self.next_event()? {
            (fp,_,XmlEvent::StartElement{name, ..}) => {
                if name.local_name=="diagram" {
                    self.read_diagram()?;
                    match self.next_event()? {
                        (_,_,XmlEvent::EndDocument) => { Ok (()) },
                        ewp => Err(self.bad_ml_event(ewp)),
                    }
                } else {
                    Err(MLError::bad_element_name(&fp, &name.local_name))
                }
            },
            ewp => Err(self.bad_ml_event(ewp)),
        }
    }
    fn next_event(&mut self) -> Result<XmlEventWithPos,MLError> {
        match self.reader.next() {
            None => Err(MLError::unexpected_end_of_stream()),
            Some(Err(e)) => Err(MLError::from(e)),
            Some(Ok(x))  => Ok(x),
        }
    }
    fn read_file(&mut self) -> Result<(),MLErrorList> {
        match self.next_event() {
            Ok( (_,_,XmlEvent::StartDocument{..}) ) => {
                let x = self.read_document();
                self.errors.update(x);
            },
            Ok(ewp) => { self.errors.add(self.bad_ml_event(ewp)); }
            Err(e) =>  { self.errors.add(e); }
        }
        self.errors.as_err(Ok(()))
    }
}

//tp DiagramML
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
    pub fn read_file<R:Read>(&mut self, f:R) -> Result<(),MLErrorList> {
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
