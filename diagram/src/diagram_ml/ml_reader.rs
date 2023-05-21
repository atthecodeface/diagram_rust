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
use super::{KnownName, NameIds};
use super::{MLError, MLErrorList, MLReadElement, MLResult};
use crate::constants::elements as el;
use crate::diagram::Element;
use crate::{DiagramContents, DiagramDescriptor};
use crate::{StyleRule, StyleSheet};
use hml_rs::markup::Event as HmlEvent;
use hml_rs::markup::EventType as HmlEventType;
use hml_rs::names::Attribute as HmlAttribute;
use hml_rs::names::Name as HmlName;
use hml_rs::names::Tag as HmlTag;
use hml_rs::names::{Namespace, NamespaceStack};
use hml_rs::reader::Error as HmlError;
use hml_rs::reader::Position as HmlPosition;
use hml_rs::reader::Reader as HmlReader;
use hml_rs::reader::Span as HmlSpan;

//a MLReader
//tp MLReader
/// A reader that creates diagram contents
pub struct MLReader<'diag, 'reader, P, E, R>
where
    P: HmlPosition,
    E: HmlError<Position = P>,
    R: HmlReader<Position = P, Error = E>,
{
    pub contents: &'reader mut DiagramContents<'diag>,
    pub stylesheet: &'reader mut StyleSheet<'diag>,
    reader: &'reader mut R,
    name_ids: NameIds,
    namespace_stack: NamespaceStack<'reader>,
    lexer: hml_rs::hml_reader::Lexer<R>,
    parser: hml_rs::hml_reader::Parser<R>,
    pub errors: MLErrorList<P, R::Error>,
}

//ti MLReader
impl<'diag, 'reader, P, E, R> MLReader<'diag, 'reader, P, E, R>
where
    P: HmlPosition,
    E: HmlError<Position = P>,
    R: HmlReader<Position = P, Error = E>,
{
    //fp new
    pub fn new(
        contents: &'reader mut DiagramContents<'diag>,
        stylesheet: &'reader mut StyleSheet<'diag>,
        namespace: &'reader mut Namespace,
        reader: &'reader mut R,
    ) -> Self {
        let mut namespace_stack = NamespaceStack::new(namespace);
        let name_ids = NameIds::create(&mut namespace_stack);
        let lexer = hml_rs::hml_reader::Lexer::new();
        let parser = hml_rs::hml_reader::Parser::new();
        Self {
            // descriptor,
            contents,
            stylesheet,
            reader,
            name_ids,
            namespace_stack,
            lexer,
            parser,
            errors: MLErrorList::new(),
        }
    }

    //mp known_id
    pub fn known_id(&self, name: &HmlName) -> Option<KnownName> {
        self.name_ids.known_id(name)
    }

    //mp map_attr
    pub fn map_attr<'a>(&self, attr: &'a HmlAttribute) -> (String, &'a str) {
        (
            attr.name.to_string(&self.namespace_stack),
            attr.value.as_str(),
        )
    }

    //mp next_event
    pub fn next_event(&mut self) -> MLResult<HmlEvent<HmlSpan<P>>, P, E> {
        let (parser, namespace_stack, lexer, reader) = (
            &mut self.parser,
            &mut self.namespace_stack,
            &mut self.lexer,
            &mut self.reader,
        );
        let e = parser.next_event(namespace_stack, || lexer.next_token(reader))?;
        Ok(e)
    }

    //mp consume_element
    /// Returns an error only if fatal
    fn consume_element(&mut self) -> MLResult<(), P, E> {
        loop {
            let e = self.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                EndElement => {
                    return Ok(());
                }
                StartElement => {
                    self.consume_element()?;
                }
                _ => (),
            }
        }
    }

    //mp return_bad_element
    /// Returns an error
    pub fn return_bad_element(
        &mut self,
        span: &HmlSpan<P>,
        tag: &HmlTag,
        expected: &'static [KnownName],
    ) -> MLError<P, E> {
        drop(self.consume_element());
        MLError::bad_element_name_expected(
            &self.namespace_stack,
            span,
            tag,
            &self.name_ids,
            expected,
        )
    }

    //mp consume_bad_element
    /// Returns an error only if fatal
    pub fn consume_bad_element(&mut self, span: &HmlSpan<P>, tag: &HmlTag) -> MLResult<(), P, E> {
        self.errors
            .add(MLError::bad_element_name(&self.namespace_stack, span, tag));
        self.consume_element()
    }

    //mp read_definitions
    fn read_definitions(&mut self, descriptor: &'diag DiagramDescriptor) -> MLResult<(), P, E> {
        loop {
            let e = self.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment => (), // continue
                EndElement => {
                    return Ok(());
                }
                StartElement => {
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    match self.known_id(&tag.name) {
                        Some(KnownName::Marker) => {
                            match Element::ml_read(self, descriptor, &span, tag) {
                                Ok(element) => {
                                    self.contents.markers.push(element);
                                }
                                e => {
                                    self.errors.update(e);
                                }
                            }
                        }
                        _ => match Element::ml_read(self, descriptor, &span, tag) {
                            Ok(element) => {
                                self.contents.definitions.push(element);
                            }
                            e => {
                                self.errors.update(e);
                            }
                        },
                    }
                }
                _ => {
                    return Err(MLError::bad_ml_event(&e));
                }
            }
        }
    }

    //mp read_rule
    fn read_rule(
        &mut self,
        _descriptor: &'diag DiagramDescriptor,
        parent: Option<usize>,
        span: &HmlSpan<P>,
        tag: HmlTag,
    ) -> MLResult<(), P, E> {
        let mut rule = StyleRule::new();
        let mut action = None;
        let mut attrs = Vec::new();
        for attr in tag.attributes.borrow() {
            match self.known_id(&attr.name) {
                Some(KnownName::Style) => {
                    if let Some(a) = self.stylesheet.get_action_index(&attr.value) {
                        action = Some(*a);
                    } else {
                        return Err(MLError::bad_value(
                            span,
                            "unknown style id in rule",
                            &attr.value,
                        ));
                    }
                }
                Some(KnownName::Id) => {
                    rule = rule.has_id(&attr.value);
                }
                Some(KnownName::Class) => {
                    rule = rule.has_class(&attr.value);
                }
                Some(KnownName::Depth) => {
                    // rule = rule.has_class(attr.value);
                }
                Some(_) => {
                    attrs.push((
                        attr.name.to_string(&self.namespace_stack),
                        attr.value.as_str(),
                    ));
                }
                _ => {
                    self.errors.add(MLError::bad_attribute_name(
                        &self.namespace_stack,
                        span,
                        attr,
                    ));
                }
            }
        }
        if !attrs.is_empty() {
            assert!(action.is_none());
            let mut attr_values = attrs.into_iter().map(|(n, v)| (n, v));
            action = Some(MLError::value_result(
                span,
                self.stylesheet
                    .add_action_from_name_values(&mut attr_values),
            )?);
        }
        let rule_index = self.stylesheet.add_rule(parent, rule, action);
        loop {
            // should support an 'apply' subrule
            let e = self.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment => (), // continue
                EndElement => {
                    return Ok(());
                }
                StartElement => {
                    // content of rule must be rules
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    match self.known_id(&tag.name) {
                        Some(KnownName::Rule) => {
                            let e = self.read_rule(_descriptor, Some(rule_index), &span, tag);
                            self.errors.update(e);
                        }
                        _ => {
                            self.consume_bad_element(&span, &tag)?;
                        }
                    }
                }
                _ => {
                    return Err(MLError::bad_ml_event(&e));
                }
            }
        }
    }

    //mp read_style
    fn read_style(
        &mut self,
        _descriptor: &'diag DiagramDescriptor,
        span: &HmlSpan<P>,
        tag: HmlTag,
    ) -> MLResult<(), P, E> {
        let attrs = tag.attributes.take();
        let (namespace_stack, stylesheet) = (&self.namespace_stack, &mut self.stylesheet);
        let mut attr_values = attrs
            .iter()
            .map(|a| (a.name.to_string(namespace_stack), a.value.as_str()));
        MLError::value_result(
            span,
            stylesheet.add_action_from_name_values(&mut attr_values),
        )?;
        loop {
            let e = self.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment => (), // continue
                EndElement => {
                    return Ok(());
                }
                StartElement => {
                    // content of style is not allowed
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    self.consume_bad_element(&span, &tag)?;
                }
                _ => {
                    return Err(MLError::bad_ml_event(&e));
                }
            }
        }
    }

    //mp read_library
    fn read_library(&mut self, descriptor: &'diag DiagramDescriptor) -> MLResult<(), P, E> {
        loop {
            let e = self.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment => (), // continue
                EndElement => {
                    return Ok(());
                }
                StartElement => {
                    // content of style is not allowed
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    match self.known_id(&tag.name) {
                        Some(KnownName::Style) => {
                            let e = self.read_style(descriptor, &span, tag);
                            self.errors.update(e);
                        }
                        Some(KnownName::Rule) => {
                            let e = self.read_rule(descriptor, None, &span, tag);
                            self.errors.update(e);
                        }
                        Some(KnownName::Defs) => {
                            let e = self.read_definitions(descriptor);
                            self.errors.update(e);
                        }
                        _ => {
                            return Err(MLError::bad_element_name_expected(
                                &self.namespace_stack,
                                &span,
                                &tag,
                                &self.name_ids,
                                &[KnownName::Style, KnownName::Rule, KnownName::Defs],
                            ));
                        }
                    }
                }
                _ => {
                    return Err(MLError::bad_ml_event(&e));
                }
            }
        }
    }

    //mp read_diagram
    fn read_diagram(
        &mut self,
        descriptor: &'diag DiagramDescriptor,
        mut layout: Element<'diag>,
    ) -> MLResult<(), P, E> {
        loop {
            let e = self.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment => (), // continue
                EndElement => {
                    self.contents.set_root_element(layout);
                    return Ok(());
                }
                StartElement => {
                    // content of style is not allowed
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    match self.known_id(&tag.name) {
                        Some(KnownName::Style) => {
                            let e = self.read_style(descriptor, &span, tag);
                            self.errors.update(e);
                        }
                        Some(KnownName::Rule) => {
                            let e = self.read_rule(descriptor, None, &span, tag);
                            self.errors.update(e);
                        }
                        Some(KnownName::Defs) => {
                            let e = self.read_definitions(descriptor);
                            self.errors.update(e);
                        }
                        _ => match Element::ml_read(self, descriptor, &span, tag) {
                            Ok(element) => {
                                layout.add_element(element);
                            }
                            e => {
                                self.errors.update(e);
                            }
                        },
                    }
                }
                _ => {
                    return Err(MLError::bad_ml_event(&e));
                }
            }
        }
    }

    //mp read_library_document
    fn read_library_document(
        &mut self,
        descriptor: &'diag DiagramDescriptor,
    ) -> MLResult<(), P, E> {
        let mut library_read = false;
        loop {
            let e = self.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment => (), // continue
                EndDocument => {
                    if !library_read {
                        return Err(MLError::bad_ml_event(&e));
                    } else {
                        return Ok(());
                    }
                }
                StartElement => {
                    // content of style is not allowed
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    match self.known_id(&tag.name) {
                        Some(KnownName::Library) => {
                            self.read_library(descriptor)?;
                            library_read = true;
                        }
                        _ => {
                            return Err(MLError::bad_element_name_expected(
                                &self.namespace_stack,
                                &span,
                                &tag,
                                &self.name_ids,
                                &[KnownName::Library],
                            ));
                        }
                    }
                }
                _ => {
                    return Err(MLError::bad_ml_event(&e));
                }
            }
        }
    }

    //mp read_diagram_document
    fn read_diagram_document(
        &mut self,
        descriptor: &'diag DiagramDescriptor,
    ) -> MLResult<(), P, E> {
        let mut diagram_read = false;
        loop {
            let e = self.next_event()?;
            use HmlEventType::*;
            match e.get_type() {
                Comment => (), // continue
                EndDocument => {
                    if !diagram_read {
                        return Err(MLError::bad_ml_event(&e));
                    } else {
                        return Ok(());
                    }
                }
                StartElement => {
                    // content of style is not allowed
                    let span = *e.borrow_span();
                    let tag = e.as_start_element().unwrap();
                    match self.known_id(&tag.name) {
                        Some(KnownName::Diagram) => {
                            let attrs = tag.attributes.take();
                            let mut attr_values = attrs.iter().map(|a| {
                                (a.name.to_string(&self.namespace_stack), a.value.as_str())
                            });
                            let layout = MLError::value_result(
                                &span,
                                Element::new(descriptor, el::Typ::Diagram, &mut attr_values),
                            )?;
                            self.read_diagram(descriptor, layout)?;
                            diagram_read = true;
                        }
                        _ => {
                            return Err(MLError::bad_element_name_expected(
                                &self.namespace_stack,
                                &span,
                                &tag,
                                &self.name_ids,
                                &[KnownName::Diagram],
                            ));
                        }
                    }
                }
                _ => {
                    return Err(MLError::bad_ml_event(&e));
                }
            }
        }
    }

    //mp read_file
    pub fn read_file(
        &mut self,
        descriptor: &'diag DiagramDescriptor,
        is_library: bool,
    ) -> Result<(), MLErrorList<P, E>> {
        match self.next_event() {
            Ok(e) => {
                if e.is_start_document() {
                    if is_library {
                        let x = self.read_library_document(descriptor);
                        self.errors.update(x);
                    } else {
                        let x = self.read_diagram_document(descriptor);
                        self.errors.update(x);
                    }
                } else {
                    self.errors.add(MLError::bad_ml_event(&e));
                }
            }
            Err(e) => {
                self.errors.add(e);
            }
        }
        self.errors.as_err(())
    }

    //zz All done
}
