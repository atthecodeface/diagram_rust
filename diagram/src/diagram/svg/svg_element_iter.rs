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

@file    svg_element_iter.rs
@brief   Iterator over the SVG element
 */

//a Imports
use super::SvgElement;
use xml::attribute::Attribute;
use xml::common::XmlVersion;
use xml::name::Name;
use xml::namespace::Namespace;
use xml::reader::XmlEvent;

//a Constants
const DEBUG_SVG_ITERATOR: bool = false;

//a SvgElement iterator
//ti IterState
#[derive(Debug)]
enum IterState {
    PreDocument,
    PreElement,
    PreString,
    PreContent,
    PostContent,
    FindNextElement,
    DocumentEnd,
    Completed,
}

//tp ElementIter
/// An iterator structure to permit iteration over an Svg object's elements
pub struct ElementIter<'a> {
    state: IterState,
    elements: Vec<(&'a SvgElement, usize)>,
}

//ip ElementIter
impl<'a> ElementIter<'a> {
    //fp new
    /// Create a new Svg element iterator
    pub fn new(e: &'a SvgElement) -> Self {
        let mut elements = Vec::new();
        elements.push((e, 0));
        Self {
            state: IterState::PreDocument,
            elements,
        }
    }
}

//ip Iterator for ElementIter
impl<'a> Iterator for ElementIter<'a> {
    type Item = XmlEvent;
    fn next(&mut self) -> Option<Self::Item> {
        // Track the state for debugging
        if DEBUG_SVG_ITERATOR {
            let (ele, n) = self.elements.pop().unwrap();
            println!(
                "State {:?} {}:{} [{}]",
                self.state,
                ele.name,
                n,
                ele.contents.len()
            );
            self.elements.push((ele, n));
        }
        match self.state {
            IterState::PreDocument => {
                self.state = IterState::PreElement;
                Some(XmlEvent::StartDocument {
                    version: XmlVersion::Version10,
                    encoding: "UTF-8".to_string(),
                    standalone: None,
                })
            }
            IterState::PreElement => {
                let (ele, n) = self.elements.pop().unwrap();
                self.state = IterState::PreString;
                let name = Name::local(&ele.name).to_owned();
                let namespace = Namespace::empty();
                let mut attributes = Vec::new();
                for (n, v) in &ele.attributes {
                    let name = Name::local(n);
                    let attribute = Attribute::new(name, v).to_owned();
                    attributes.push(attribute);
                }
                self.elements.push((ele, n));
                Some(XmlEvent::StartElement {
                    name,
                    attributes,
                    namespace,
                })
            }
            IterState::PreString => {
                let (ele, n) = self.elements.pop().unwrap();
                self.state = IterState::PreContent;
                if let Some(s) = &ele.characters {
                    self.elements.push((ele, n));
                    Some(XmlEvent::Characters(s.to_string()))
                } else {
                    self.elements.push((ele, n));
                    self.next()
                }
            }
            IterState::PreContent => {
                let (ele, n) = self.elements.pop().unwrap();
                if n < ele.contents.len() {
                    let next_ele = &ele.contents[n];
                    self.elements.push((ele, n));
                    self.elements.push((next_ele, 0));
                    self.state = IterState::PreElement;
                } else {
                    self.state = IterState::PostContent;
                    self.elements.push((ele, n));
                }
                self.next()
            }
            IterState::PostContent => {
                let (ele, n) = self.elements.pop().unwrap();
                self.state = IterState::FindNextElement;
                let name = Name::local(&ele.name).to_owned();
                self.elements.push((ele, n));
                Some(XmlEvent::EndElement { name })
            }
            IterState::FindNextElement => {
                if self.elements.len() > 1 {
                    let (_ele, _n) = self.elements.pop().unwrap();
                    let (ele, n) = self.elements.pop().unwrap();
                    if n + 1 < ele.contents.len() {
                        let next_ele = &ele.contents[n + 1];
                        self.elements.push((ele, n + 1));
                        self.elements.push((next_ele, 0));
                        self.state = IterState::PreElement;
                    } else {
                        self.elements.push((ele, n + 1));
                        self.state = IterState::PostContent;
                    }
                } else {
                    self.state = IterState::DocumentEnd;
                }
                self.next()
            }
            IterState::DocumentEnd => {
                self.state = IterState::Completed;
                Some(XmlEvent::EndDocument)
            }
            IterState::Completed => None,
        }
    }
}
