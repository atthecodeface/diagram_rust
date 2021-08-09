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
// use crate::constants::attributes as at;
use crate::constants::elements as el;
use crate::{DiagramDescriptor};
use super::{MLResult, MLError, MLReader};
use super::{KnownName};
use hml_rs::reader::Position  as HmlPosition;
use hml_rs::reader::Error     as HmlError;
use hml_rs::reader::Reader    as HmlReader;
use hml_rs::reader::Span      as HmlSpan;
use hml_rs::names::Tag        as HmlTag;
use hml_rs::markup::EventType as HmlEventType;

//a MLReadElement
//tt MLReadElement - internal trait to enable extension of type implementations
pub trait MLReadElement <'a, P, E, R>
where P:HmlPosition,
      E:HmlError<Position = P>,
      R:HmlReader<Position = P, Error = E>
{

    /// ml_read is invoked from MarkupEvent::StartElement(<element type>, <atttributes>, _<namespace>)
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&HmlSpan<P>, tag:HmlTag) -> MLResult<Element<'a>, P, E>;
}

//ti MLEvent for Use
impl <'a, P, E, R> MLReadElement <'a, P, E, R> for Use<'a>
where P:HmlPosition,
      E:HmlError<Position = P>,
      R:HmlReader<Position = P, Error = E>
{
    //fp ml_read
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&HmlSpan<P>, tag:HmlTag) -> MLResult<Element<'a>, P, E> {
        let attrs = tag.attributes.take();
        let mut attr_values = attrs.iter().map(|a| reader.map_attr(a));
        let mut use_ref = MLError::value_result(span, Element::new(descriptor, el::Typ::Use, &mut attr_values ))?;
        loop {
            let e = reader.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment     => (), // continue
                EndElement  => { return Ok(use_ref); },
                Content     => {
                    MLError::element_result( e.borrow_span(), use_ref.add_string(e.as_content().unwrap().1))?;
                },
                StartElement => { // content of style is not allowed
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    reader.consume_bad_element(&span, &tag)?;
                },
                _ => { return Err(MLError::bad_ml_event(&e)); },
            }
        }
    }
}

//ii MLEvent for Group
impl <'a, P, E, R> MLReadElement <'a, P, E, R> for Group<'a>
where P:HmlPosition,
      E:HmlError<Position = P>,
      R:HmlReader<Position = P, Error = E>
{
    //fp ml_read
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&HmlSpan<P>, tag:HmlTag) -> MLResult<Element<'a>, P, E> {
        let el_type = {
            match reader.known_id(&tag.name) {
                Some(KnownName::Marker)  =>  el::Typ::Marker,
                Some(KnownName::Layout)  =>  el::Typ::Layout,
                _ => el::Typ::Group,
            }
        };

        let attrs = tag.attributes.take();
        let mut attr_values = attrs.iter().map(|a| reader.map_attr(a));
        let mut group = MLError::value_result(span, Element::new(descriptor, el_type, &mut attr_values ))?;
        loop {
            let e = reader.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment      => (), // continue
                EndElement   => { return Ok(group); },
                StartElement => {
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    match Element::ml_read(reader, descriptor, &span, tag) {
                        Ok(element) => {
                            group.add_element(element);
                        },
                        e => { reader.errors.update(e); },
                    }
                },
                _ => { return Err(MLError::bad_ml_event(&e)); },
            }
        }
    }
}

//ii MLEvent for Path
impl <'a, P, E, R> MLReadElement <'a, P, E, R> for Path
where P:HmlPosition,
      E:HmlError<Position = P>,
      R:HmlReader<Position = P, Error = E>
{
    //fp ml_read
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&HmlSpan<P>, tag:HmlTag) -> MLResult<Element<'a>, P, E> {
        let attrs = tag.attributes.take();
        let mut attr_values = attrs.iter().map(|a| reader.map_attr(a));
        let path  = MLError::value_result(span, Element::new(descriptor, el::Typ::Path, &mut attr_values ))?;
        loop {
            let e = reader.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment      => (), // continue
                EndElement   => { return Ok(path); },
                StartElement => { // content of style is not allowed
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    reader.consume_bad_element(&span, &tag)?;
                },
                _ => { return Err(MLError::bad_ml_event(&e)); },
            }
        }
    }
}

//ii MLEvent for Shape
impl <'a, P, E, R> MLReadElement <'a, P, E, R> for Shape
where P:HmlPosition,
      E:HmlError<Position = P>,
      R:HmlReader<Position = P, Error = E>
{
    //fp ml_read
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&HmlSpan<P>, tag:HmlTag) -> MLResult<Element<'a>, P, E> {
        let el_type = {
            match reader.known_id(&tag.name) {
                Some(KnownName::Rect)   =>  el::Typ::Rect,
                Some(KnownName::Circle) =>  el::Typ::Circle,
                _ => el::Typ::Polygon,
            }
        };

        let attrs = tag.attributes.take();
        let mut attr_values = attrs.iter().map(|a| reader.map_attr(a));
        let shape = MLError::value_result(span, Element::new(descriptor, el_type, &mut attr_values ))?;
        loop {
            let e = reader.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment      => (), // continue
                EndElement   => { return Ok(shape); },
                StartElement => { // content of style is not allowed
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    reader.consume_bad_element(&span, &tag)?;
                },
                _ => { return Err(MLError::bad_ml_event(&e)); },
            }
        }
    }
}

//ii MLEvent for Text
impl <'a, P, E, R> MLReadElement <'a, P, E, R> for Text
where P:HmlPosition,
      E:HmlError<Position = P>,
      R:HmlReader<Position = P, Error = E>
{
    //fp ml_read
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&HmlSpan<P>, tag:HmlTag) -> MLResult<Element<'a>, P, E> {
        let attrs = tag.attributes.take();
        let mut attr_values = attrs.iter().map(|a| reader.map_attr(a));
        let mut text  = MLError::value_result(span, Element::new(descriptor, el::Typ::Text, &mut attr_values ))?;
        loop {
            let e = reader.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment     => (), // continue
                EndElement  => { return Ok(text); },
                Content     => {
                    MLError::element_result( e.borrow_span(), text.add_string(e.as_content().unwrap().1))?;
                },
                StartElement => { // content of style is not allowed
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    reader.consume_bad_element(&span, &tag)?;
                },
                _ => { return Err(MLError::bad_ml_event(&e)); },
            }
        }
    }
}

//ii MLReadElement for Element
impl <'a, P, E, R> MLReadElement <'a, P, E, R> for Element<'a>
where P:HmlPosition,
      E:HmlError<Position = P>,
      R:HmlReader<Position = P, Error = E>
{
    fn ml_read(reader:&mut MLReader<P, E, R>, descriptor:&'a DiagramDescriptor, span:&HmlSpan<P>, tag:HmlTag) -> MLResult<Element<'a>, P, E> {
        match reader.known_id(&tag.name) {
            Some(KnownName::Use)      => Use::ml_read(reader, descriptor, span, tag),
            Some(KnownName::Group)    => Group::ml_read(reader, descriptor, span, tag),
            Some(KnownName::Marker)   => Group::ml_read(reader, descriptor, span, tag),
            Some(KnownName::Layout)   => Group::ml_read(reader, descriptor, span, tag),
            Some(KnownName::Path)     => Path::ml_read(reader, descriptor, span, tag),
            Some(KnownName::Text)     => Text::ml_read(reader, descriptor, span, tag),
            Some(KnownName::Rect)     => Shape::ml_read(reader, descriptor, span, tag),
            Some(KnownName::Circle)   => Shape::ml_read(reader, descriptor, span, tag),
            Some(KnownName::Polygon)  => Shape::ml_read(reader, descriptor, span, tag),
            _ => Err(reader.return_bad_element(span, &tag,
                                               &[KnownName::Use,
                                                 KnownName::Group,
                                                 KnownName::Marker,
                                                 KnownName::Layout,
                                                 KnownName::Path,
                                                 KnownName::Text,
                                                 KnownName::Rect,
                                                 KnownName::Circle,
                                                 KnownName::Polygon,
                                               ]
            ))
        }
    }
}

