/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    mod.rs
@brief   Diagram Markup Reader module
 */

//a Imports
use hmlm;
use std::io::prelude::{Read};
use crate::{StyleSheet, StyleRule, Diagram, DiagramContents, DiagramDescriptor};
use crate::constants::elements   as el;
use xml;
use xml::reader::XmlEvent;
use hmlm::{XmlEventWithPos, FilePosition};
use crate::diagram::{Element, Use, Group, Text, Shape, Path};

type Attributes = Vec<xml::attribute::OwnedAttribute>;

fn to_nv(attributes:&Attributes) -> Vec<(String,String)> {
    attributes.iter().map(|a| (a.name.local_name.clone(), a.value.clone())).collect()
}

//a MLError type
//tp MLError
#[derive(Debug)]
pub enum MLError {
    EndOfStream,
    BadElementName(FilePosition, String),
    BadAttributeName(FilePosition, String),
    BadElement(FilePosition, String),
    BadMLEvent(String),
    BadValue(FilePosition, String),
    ParseError(String),
}

//ii MLError
impl MLError {
    //fi unexpected_end_of_stream
    fn unexpected_end_of_stream() -> Self {
        Self::EndOfStream
    }
    //fi bad_element_name
    fn bad_element_name(fp:&FilePosition, name:&str) -> Self {
        Self::BadElementName(fp.clone(), name.to_string())
    }

    //fi bad_attribute_name
    // fn bad_attribute_name(fp:&FilePosition, name:&str) -> Self {
    // Self::BadAttributeName(fp.clone(), name.to_string())
    // }

    //mp bad_ml_event
    fn bad_ml_event(ewp:&XmlEventWithPos) -> Self {
        Self::BadMLEvent(format!("{:?} at {}",ewp.2, ewp.0))
    }

    //fi value_result
    fn value_result<V, E:std::fmt::Display>(fp:&FilePosition, result:Result<V,E>) -> Result<V,Self> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(MLError::BadValue(fp.clone(), e.to_string())),
        }
    }

    //fi element_result
    fn element_result<V, E:std::fmt::Display>(fp:&FilePosition, result:Result<V,E>) -> Result<V,Self> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(MLError::BadElement(fp.clone(), e.to_string())),
        }
    }

    //zz All done
}

//ip std::fmt::Display for MLError
impl std::fmt::Display for MLError {
    //mp fmt - format a `MLError` for display
    /// Display the `MLError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MLError::EndOfStream          => write!(f, "Unexpected end of XML event stream - bug in event source"),
            MLError::BadElementName(fp,n) => write!(f, "Bad element '{}' at {}", n, fp),
            MLError::BadAttributeName(fp,n) => write!(f, "Bad attribute '{}' at {}", n, fp),
            MLError::BadElement(fp,s)     => write!(f, "Element error '{}' at {}", s, fp),
            MLError::BadMLEvent(s)        => write!(f, "Bad XML event {}", s),
            MLError::BadValue(fp,s)       => write!(f, "Bad value '{}' at {}", s, fp),
            MLError::ParseError(s)        => f.write_str(s),
        }
    }

    //zz All done
}

//ip From<hmlm::reader::ParseError> for MLError
impl From<hmlm::reader::ParseError> for MLError {
    fn from(e: hmlm::reader::ParseError) -> MLError {
        MLError::ParseError(e.to_string())
    }
}

//a MLErrorList
//tp MLErrorList
/// An array of errors; if the array is zero-length after reading a
/// file, then there is no error.
///
/// Otherwise it is an accumulation of errors from reading the
/// file. Since some errors may indicate failure to correctly parse a
/// file, only the first error is guaranteed to be valid, but it is
/// useful to get a list of errors for when only minor attribute value
/// errors are returned.
#[derive(Debug)]
pub struct MLErrorList {
    errors : Vec<MLError>,
}

//ip MLErrorList
impl MLErrorList {
    //fp new
    /// Create a new MLErrorList
    pub(self) fn new() -> Self {
        Self { errors : Vec::new() }
    }

    //mp add
    /// Add an error to the list
    pub(self) fn add(&mut self, e:MLError) -> () {
        self.errors.push(e);
    }

    //mp update
    /// Update the MLErrorList from a result; this returns () so the
    /// error is effectively caught and recorded. Subsequent errors
    /// are therefore less reliable.
    pub fn update<T>(&mut self, e:Result<T,MLError>) {
        match e {
            Err(e) => { self.errors.push(e); }
            _ => (),
        }
    }

    //mp as_err
    /// Return a result of 'Ok(x)' if this error list is empty, or
    /// 'Err(MLErrorList)' if the error list has contents. It cleans
    /// the current error list.
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

    //zz All done
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
//a MLEvent
//tt MLEvent - internal trait to enable extension of type implementations
trait MLEvent <'a, R:Read, S:Sized> {
    /// ml_new is invoked from StartElement(<element type>, <atttributes>, _<namespace>)
    fn ml_new(reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<S, MLError>;
    /// ml_event is invoked after an object is created
    fn ml_event (s:S, _reader:&mut MLReader<R>, _descriptor:&'a DiagramDescriptor) -> Result<S, MLError> { Ok(s) }
}

//ti MLEvent for Use
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Use<'a> {
    fn ml_new(reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<Element<'a>, MLError> {
        let use_ref = MLError::value_result(fp, Element::new(descriptor, name, to_nv(attributes)))?;
        Self::ml_event(use_ref, reader, descriptor)
    }
    fn ml_event (mut s:Element<'a>, reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor) -> Result<Element<'a>, MLError> {
        match reader.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the use
            (fp,_,XmlEvent::Characters(c))  => { MLError::element_result(&fp, s.add_string(&c))?; },
            (_,_,XmlEvent::Comment(_))     => (), // continue
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

//ii MLEvent for Group
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Group<'a> {
    //fp ml_new
    fn ml_new(reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<Element<'a>, MLError> {
        let group = MLError::value_result(fp, Element::new(descriptor, name, to_nv(attributes)))?;
        Self::ml_event(group, reader, descriptor)
    }
    fn ml_event (mut s:Element<'a>, reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor) -> Result<Element<'a>, MLError> {
        match reader.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the group
            (_,_,XmlEvent::Comment(_))     => (), // continue
            (fp,_,XmlEvent::StartElement{name, attributes, ..}) => { // content of group
                match Element::ml_new(reader, descriptor, &fp, &name.local_name, &attributes) {
                    Ok(element) => {
                        s.add_element(element);
                    },
                    e => { reader.errors.update(e); },
                }
            },
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

//ii MLEvent for Path
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Path {
    fn ml_new (reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<Element<'a>, MLError> {
        let path = MLError::value_result(fp, Element::new(descriptor, name, to_nv(attributes)))?;
        Self::ml_event(path, reader, descriptor)
    }
    fn ml_event (s:Element<'a>, reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor) -> Result<Element<'a>, MLError> {
        match reader.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the group
            (_,_,XmlEvent::Comment(_))     => (), // continue
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

//ii MLEvent for Shape
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Shape {
    fn ml_new (reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<Element<'a>, MLError> {
        let shape = MLError::value_result(fp, Element::new(descriptor, name, to_nv(attributes)))?;
        Self::ml_event(shape, reader, descriptor)
    }
    fn ml_event (s:Element<'a>, reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor) -> Result<Element<'a>, MLError> {
        match reader.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the group
            (_,_,XmlEvent::Comment(_))     => (), // continue
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

//ii MLEvent for Text
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Text {
    fn ml_new (reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<Element<'a>, MLError> {
        let text = MLError::value_result(fp, Element::new(descriptor, name, to_nv(attributes)))?;
        Self::ml_event(text, reader, descriptor)
    }
    fn ml_event (mut s:Element<'a>, reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor) -> Result<Element<'a>, MLError> {
        match reader.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the group
            (fp,_,XmlEvent::Characters(c))  => { MLError::element_result(&fp, s.add_string(&c))?; },
            (_,_,XmlEvent::Comment(_))     => (), // continue
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

//ii MLEvent for Element
impl <'a, R:Read> MLEvent <'a, R, Element<'a>> for Element<'a> {
    fn ml_new (reader:&mut MLReader<R>, descriptor:&'a DiagramDescriptor, fp:&FilePosition, name:&str, attributes:&Attributes) -> Result<Self, MLError> {
        match name {
            el::PATH     => Ok(Path::ml_new(reader, descriptor, fp, name, attributes)?),
            el::RECT     => Ok(Shape::ml_new(reader, descriptor, fp, name, attributes)?),
            el::CIRCLE   => Ok(Shape::ml_new(reader, descriptor, fp, name, attributes)?),
            el::POLYGON  => Ok(Shape::ml_new(reader, descriptor, fp, name, attributes)?),
            el::TEXT     => Ok(Text::ml_new(reader, descriptor, fp, name, attributes)?),
            el::GROUP    => Ok(Group::ml_new(reader, descriptor, fp, name, attributes)?),
            el::LAYOUT   => Ok(Group::ml_new(reader, descriptor, fp, name, attributes)?),
            el::USE      => Ok(Use::ml_new(reader, descriptor, fp, name, attributes)?),
            _ => {
                let r = BadXMLElement::ml_new(reader, descriptor, fp, name, attributes);
                reader.errors.update(r);
                return Err(MLError::bad_element_name(fp,name))
            }
        }
    }
}

//ti MLEvent for BadXMLElement
struct BadXMLElement {
}

//ii MLEvent for BadXMLElement
impl <'a, R:Read> MLEvent <'a, R, BadXMLElement> for BadXMLElement {
    fn ml_new (reader:&mut MLReader<R>, descriptor:&DiagramDescriptor, _fp:&FilePosition, _name:&str, _attributes:&Attributes) -> Result<Self, MLError> {
        let s = Self {};
        Self::ml_event(s, reader, descriptor)
    }
    fn ml_event (s:Self, reader:&mut MLReader<R>, descriptor:&DiagramDescriptor) -> Result<Self, MLError> {
        match reader.next_event()? {
            (fp,_,XmlEvent::StartElement{name, attributes, ..}) => {
                let r = BadXMLElement::ml_new(reader, descriptor, &fp, &name.local_name, &attributes);
                reader.errors.update(r);
            }
            (_,_,XmlEvent::EndElement{..}) => { return Ok(s); } // end the group
            (_,_,XmlEvent::Comment(_))     => (), // continue
            (_,_,XmlEvent::Whitespace(_))  => (), // continue
            (_,_,XmlEvent::Characters(_))  => (), // continue
            (_,_,XmlEvent::CData(_))       => (), // continue
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

//a MLReader
//tp MLReader
/// A reader that creates diagram contents
struct MLReader<'a, 'b, R:Read> {
    pub contents    : &'b mut DiagramContents<'a>,
    pub stylesheet  : &'b mut StyleSheet<'a>,
    pub reader      : hmlm::reader::EventReader<R>,
    errors          : MLErrorList,
}

//ti MLReader
impl <'a, 'b, R:Read> MLReader<'a, 'b, R> {
    //fp new
    pub fn new<'c, 'd> ( // descriptor: &'d DiagramDescriptor<'c>,
                         contents:    &'d mut DiagramContents<'c>,
                         stylesheet:  &'d mut StyleSheet<'c>,
                         reader:hmlm::reader::EventReader<R> ) -> MLReader<'c, 'd, R> {
        MLReader {
            // descriptor,
            contents,
            reader,
            stylesheet,
            errors :MLErrorList::new(),
        }
    }

    //mp next_event
    fn next_event(&mut self) -> Result<XmlEventWithPos,MLError> {
        match self.reader.next() {
            None => Err(MLError::unexpected_end_of_stream()),
            Some(Err(e)) => Err(MLError::from(e)),
            Some(Ok(x))  => Ok(x),
        }
    }

    //mp read_definitions
    fn read_definitions (&mut self, descriptor:&'a DiagramDescriptor) -> Result<(),MLError> {
        match self.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => { return Ok(()); },
            (_,_,XmlEvent::Comment(_))     => (), // continue
            (fp,_,XmlEvent::StartElement{name, attributes, ..}) => {
                match Element::ml_new(self, descriptor, &fp, &name.local_name, &attributes) {
                    Ok(element) => {
                        self.contents.definitions.push(element);
                    },
                    e => { self.errors.update(e); },
                }
            },
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        self.read_definitions(descriptor)
    }

    //mp read_rule
    fn read_rule (&mut self, descriptor:&'a DiagramDescriptor, parent:Option<usize>, fp:&FilePosition, _name:&str, attributes:&Attributes) -> Result<(), MLError> {
        let mut rule = StyleRule::new();
        let mut action = None;
        let mut blob = Vec::new();
        for attr in attributes {
            match attr.name.local_name.as_str() {
                "style" => {
                    if let Some(a) = self.stylesheet.get_action_index(&attr.value) {
                        action = Some(*a);
                    } else {
                        return Err(MLError::BadValue(fp.clone(),format!("unknown style id '{}' in rule", attr.value)));
                    }
                }
                "id"    => {
                    rule = rule.has_id(&attr.value);
                }
                "class"  => {
                    rule = rule.has_class(&attr.value);
                }
                "depth"  => {
                    // rule = rule.has_class(attr.value);
                }
                name => {
                    blob.push( (name.to_string(), attr.value.clone()) );
                }
            }
        }
        if blob.len() > 0 {
            assert!(action.is_none());
            action = Some(MLError::value_result(fp, self.stylesheet.add_action_from_name_values(&blob))?);
        }
        let rule_index = self.stylesheet.add_rule(parent, rule, action);
        loop {
            // should support an 'apply' subrule
            match self.next_event()? {
                (fp,_,XmlEvent::StartElement{name, attributes, ..}) => {
                    match name.local_name.as_str() {
                        "rule"  => {
                            let e = self.read_rule(descriptor, Some(rule_index), &fp, &name.local_name, &attributes);
                            self.errors.update(e);
                        }
                        _ => {
                            let r = BadXMLElement::ml_new(self, descriptor, &fp, &name.local_name, &attributes);
                            self.errors.update(r);
                            return Err(MLError::bad_element_name(&fp,&name.local_name))
                        }
                    }
                },
                (_,_,XmlEvent::EndElement{..}) => { return Ok(()); },
                (_,_,XmlEvent::Comment(_))     => (), // continue
                ewp => { return Err(MLError::bad_ml_event(&ewp)); },
            }
        }
    }

    //mp read_style
    fn read_style (&mut self, descriptor:&'a DiagramDescriptor, fp:&FilePosition, _name:&str, attributes:&Attributes) -> Result<(), MLError> {
        MLError::value_result(fp, self.stylesheet.add_action_from_name_values(&to_nv(attributes)))?;
        loop {
            match self.next_event()? {
                (fp,_,XmlEvent::StartElement{name, attributes, ..}) => {
                    let r = BadXMLElement::ml_new(self, descriptor, &fp, &name.local_name, &attributes);
                    self.errors.update(r);
                    return Err(MLError::bad_element_name(&fp,&name.local_name))
                },
                (_,_,XmlEvent::EndElement{..}) => { return Ok(()); },
                (_,_,XmlEvent::Comment(_))     => (), // continue
                ewp => { return Err(MLError::bad_ml_event(&ewp)); },
            }
        }
    }

    //mp read_diagram
    fn read_diagram(&mut self, descriptor:&'a DiagramDescriptor, mut layout:Element<'a>) -> Result<(),MLError> {
        match self.next_event()? {
            (_,_,XmlEvent::EndElement{..}) => {
                self.contents.set_root_element(layout);
                return Ok(());
            },
            (_,_,XmlEvent::Comment(_))     => (), // continue
            (fp,_,XmlEvent::StartElement{name, attributes, ..}) => {
                match name.local_name.as_str() {
                    "style"  => {
                        let e = self.read_style(descriptor, &fp, &name.local_name, &attributes);
                        self.errors.update(e);
                    }
                    "rule"  => {
                        let e = self.read_rule(descriptor, None, &fp, &name.local_name, &attributes);
                        self.errors.update(e);
                    }
                    "defs" => {
                        let e = self.read_definitions(descriptor);
                        self.errors.update(e);
                    }
                    _ => {
                        match Element::ml_new(self, descriptor, &fp, &name.local_name, &attributes) {
                            Ok(element) => {
                                layout.add_element(element);
                            },
                            e => { self.errors.update(e); },
                        }
                    }
                }
            },
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        self.read_diagram(descriptor, layout)
    }

    //mp read_document
    fn read_document(&mut self, descriptor:&'a DiagramDescriptor) -> Result<(),MLError> {
        match self.next_event()? {
            (fp,_,XmlEvent::StartElement{name, attributes, ..}) => {
                if name.local_name=="diagram" {
                    let layout = MLError::value_result(&fp, Element::new(descriptor, &name.local_name, to_nv(&attributes)))?;
                    self.read_diagram(descriptor, layout)?;
                    match self.next_event()? {
                        (_,_,XmlEvent::EndDocument) => { Ok (()) },
                        ewp => Err(MLError::bad_ml_event(&ewp)),
                    }
                } else {
                    Err(MLError::bad_element_name(&fp, &name.local_name))
                }
            },
            ewp => Err(MLError::bad_ml_event(&ewp)),
        }
    }

    //mp read_file
    fn read_file(&mut self, descriptor:&'a DiagramDescriptor) -> Result<(),MLErrorList> {
        match self.next_event() {
            Ok( (_,_,XmlEvent::StartDocument{..}) ) => {
                let x = self.read_document(descriptor);
                self.errors.update(x);
            },
            Ok(ewp) => { self.errors.add(MLError::bad_ml_event(&ewp)); }
            Err(e) =>  { self.errors.add(e); }
        }
        self.errors.as_err(Ok(()))
    }

    //zz All done
}

//a DiagramML
//tp DiagramML
/// The `DiagramML` structure is used to construct a diagram from
/// mark-up, be that XML or HML.
///
/// # Example
///
/// ```
/// extern crate diagram;
/// use diagram::{Diagram, DiagramDescriptor, DiagramML};
/// let style_set = DiagramDescriptor::create_style_set();
/// let diagram_descriptor = DiagramDescriptor::new(&style_set);
/// let mut diagram  = Diagram::new(&diagram_descriptor);
/// let mut dml      = DiagramML::new(&mut diagram);
/// dml.read_file("#diagram ##defs ###rect id=a ##rect ##group ###rect ##rect".as_bytes()).unwrap();
/// let (_, contents, _) = diagram.borrow_contents_descriptor();
/// assert_eq!(1, contents.definitions.len(), "One definition expected from this");
/// // assert_eq!(3, contents.root.elements.len(), "Three elements (rect, group, rect) expected from this");
/// ```
pub struct DiagramML<'a, 'b> {
    diagram: &'a mut Diagram<'b>,
}

//ip DiagramML
impl <'a, 'b> DiagramML<'a, 'b> {
    //fp new
    /// Create a new mark-up diagram reader `DiagramML`, for the provided diagram.
    ///
    /// The diagram is borrowed mutably, and is obviously then held
    /// until the reader has completed reading the file.
    ///
    /// It is possible that the reader will support including other
    /// files within a file being read; this will require the reader
    /// to invoke a new reader with the new file.
    pub fn new(d:&'a mut Diagram<'b>) -> Self {
        Self { diagram:d }
    }

    //mp read_file
    /// Read a file as HML (currently), using its contents to build
    /// the `Diagram` that this reader is constructing.
    pub fn read_file<R:Read>(&mut self, f:R) -> Result<(),MLErrorList> {
        let event_reader = hmlm::reader::EventReader::new(f); // Can use an xml::reader
        let (descriptor, contents, stylesheet) = self.diagram.borrow_contents_descriptor();
        MLReader::new(contents, stylesheet, event_reader).read_file(descriptor)
    }
    
    //zz All done
}

//a Test
#[cfg(test)]
mod tests {
    use crate::{Diagram, DiagramDescriptor, DiagramML};
    #[test]
    fn test_why() {
        let style_set = DiagramDescriptor::create_style_set();
        let diagram_descriptor = DiagramDescriptor::new(&style_set);
        let mut diagram = Diagram::new(&diagram_descriptor);
        let mut dml     = DiagramML::new(&mut diagram);
        dml.read_file("#diagram".as_bytes()).unwrap();
        let (_, contents, _) = diagram.borrow_contents_descriptor();
        assert_eq!(0, contents.definitions.len());
    }
}
