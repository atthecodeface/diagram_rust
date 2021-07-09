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
use crate::diagram::{Element, Use, Group, Text, Shape, Path};
use crate::{Diagram, DiagramDescriptor, DiagramML};
use super::{MLResult, MLError, MLReader};
use super::{NameIds, KnownName};
use hml::reader::{Position, Reader, Span};

//a MLReadElement
//tt MLReadElement - internal trait to enable extension of type implementations
pub trait MLReadElement <'a, P, E, R>
where P:hml::reader::Position,
      E:std::error::Error + 'static,
      R:hml::reader::Reader<Position = P, Error = E>
{

    /// ml_read is invoked from MarkupEvent::StartElement(<element type>, <atttributes>, _<namespace>)
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&Span<P>, tag:hml::Tag) -> MLResult<Element<'a>, P, E>;
}

//ti MLEvent for Use
impl <'a, P, E, R> MLReadElement <'a, P, E, R> for Use<'a>
where P:hml::reader::Position,
      E:std::error::Error + 'static,
      R:hml::reader::Reader<Position = P, Error = E>
{
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&Span<P>, tag:hml::Tag) -> MLResult<Element<'a>, P, E> {
        let v : Vec<(String,&str)> = vec![];
        let mut use_ref = MLError::value_result(span, Element::new(descriptor, "use", &mut v.into_iter() ))?; // attributes.to_name_values() ))?;
        loop {
            let e = reader.next_event()?;
            use hml::EventType::*;
            match e.get_type() {
                Comment     => (), // continue
                EndElement  => { return Ok(use_ref); },
                Content     => {
                    MLError::element_result( e.borrow_span(), use_ref.add_string(e.as_content().unwrap().1))?;
                },
                _ => { return Err(MLError::bad_ml_event(&e)); },
            }
        }
    }
}

/*
//ii MLEvent for Group
impl <'a, P:Position, R:Reader<Position = P>> MLEvent <'a, R, Element<'a>> for Group<'a> {
    //fp ml_new
    fn ml_new(reader:&mut MLReader<P, R>, descriptor:&'a DiagramDescriptor, bounds:&FileBounds, name:&str, attributes:&MarkupAttributes) -> Result<Element<'a>, MLError<R>> {
        let group = MLError::value_result(bounds, Element::new(descriptor, name, attributes.to_name_values()))?;
        Self::ml_event(group, reader, descriptor)
    }
    fn ml_event (mut s:Element<'a>, reader:&mut MLReader<P, R>, descriptor:&'a DiagramDescriptor) -> Result<Element<'a>, MLError<R>> {
        match reader.next_event()? {
            MarkupEvent::EndElement{..}         => { return Ok(s); } // end the use
            MarkupEvent::Comment{..}            => (), // continue
            MarkupEvent::StartElement{bounds, name, attributes, ..} => { // content of group
                match Element::ml_new(reader, descriptor, &bounds, &name.name, &attributes) {
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
impl <'a, P:Position, R:Reader<Position = P>> MLEvent <'a, R, Element<'a>> for Path {
    fn ml_new(reader:&mut MLReader<P, R>, descriptor:&'a DiagramDescriptor, bounds:&FileBounds, name:&str, attributes:&MarkupAttributes) -> Result<Element<'a>, MLError<R>> {
        let path = MLError::value_result(bounds, Element::new(descriptor, name, attributes.to_name_values()))?;
        Self::ml_event(path, reader, descriptor)
    }
    fn ml_event (s:Element<'a>, reader:&mut MLReader<P, R>, descriptor:&'a DiagramDescriptor) -> Result<Element<'a>, MLError<R>> {
        match reader.next_event()? {
            MarkupEvent::EndElement{..}         => { return Ok(s); } // end the use
            MarkupEvent::Comment{..}            => (), // continue
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

//ii MLEvent for Shape
impl <'a, P:Position, R:Reader<Position = P>> MLEvent <'a, R, Element<'a>> for Shape {
    fn ml_new(reader:&mut MLReader<P, R>, descriptor:&'a DiagramDescriptor, bounds:&FileBounds, name:&str, attributes:&MarkupAttributes) -> Result<Element<'a>, MLError<R>> {
        let shape = MLError::value_result(bounds, Element::new(descriptor, name, attributes.to_name_values()))?;
        Self::ml_event(shape, reader, descriptor)
    }
    fn ml_event (s:Element<'a>, reader:&mut MLReader<P, R>, descriptor:&'a DiagramDescriptor) -> Result<Element<'a>, MLError<R>> {
        match reader.next_event()? {
            MarkupEvent::EndElement{..}         => { return Ok(s); } // end the use
            MarkupEvent::Comment{..}            => (), // continue
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

//ii MLEvent for Text
impl <'a, P:Position, R:Reader<Position = P>> MLEvent <'a, R, Element<'a>> for Text {
    fn ml_new(reader:&mut MLReader<P, R>, descriptor:&'a DiagramDescriptor, bounds:&FileBounds, name:&str, attributes:&MarkupAttributes) -> Result<Element<'a>, MLError<R>> {
        let text = MLError::value_result(bounds, Element::new(descriptor, name, attributes.to_name_values()))?;
        Self::ml_event(text, reader, descriptor)
    }
    fn ml_event (mut s:Element<'a>, reader:&mut MLReader<P, R>, descriptor:&'a DiagramDescriptor) -> Result<Element<'a>, MLError<R>> {
        match reader.next_event()? {
            MarkupEvent::EndElement{..}         => { return Ok(s); } // end the use
            MarkupEvent::Comment{..}            => (), // continue
            MarkupEvent::Content{bounds, data}  => { MLError::element_result(&bounds, s.add_string(data.borrow_str()))?; },
            ewp => { return Err(MLError::bad_ml_event(&ewp)); },
        }
        Self::ml_event(s, reader, descriptor)
    }
}

*/
//ii MLReadElement for Element
impl <'a, P, E, R> MLReadElement <'a, P, E, R> for Element<'a>
where P:hml::reader::Position,
      E:std::error::Error + 'static,
      R:hml::reader::Reader<Position = P, Error = E>
{
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&Span<P>, tag:hml::Tag) -> MLResult<Element<'a>, P, E> {
        match reader.known_id(&tag.name) {
            Some(KnownName::Use) => Use::ml_read(reader, descriptor, span, tag),
/*        match name {
            el::PATH     => Ok(Path  ::ml_new(reader, descriptor, bounds, name, attributes)?),
            el::RECT     => Ok(Shape ::ml_new(reader, descriptor, bounds, name, attributes)?),
            el::CIRCLE   => Ok(Shape ::ml_new(reader, descriptor, bounds, name, attributes)?),
            el::POLYGON  => Ok(Shape ::ml_new(reader, descriptor, bounds, name, attributes)?),
            el::TEXT     => Ok(Text  ::ml_new(reader, descriptor, bounds, name, attributes)?),
            el::GROUP    => Ok(Group ::ml_new(reader, descriptor, bounds, name, attributes)?),
            el::MARKER   => Ok(Group ::ml_new(reader, descriptor, bounds, name, attributes)?),
            el::LAYOUT   => Ok(Group ::ml_new(reader, descriptor, bounds, name, attributes)?),
*/
            _ => Err(reader.return_bad_element(span, &tag))
        }
    }
}

