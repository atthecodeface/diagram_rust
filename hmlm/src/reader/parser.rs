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

@file    types.rs
@brief   Types used throughout the HML library
 */

//a Imports
use crate::types::*;
use super::lexer::{Token, TokenWithPos};

//a Internal types
//ti StackElement
/// A stack element is
struct StackElement <F:FilePosition> {
    parent_depth : usize,
    depth        : usize, // all elements inside of >depth are in this element, of ==depth are siblings
    boxed        : bool, // if true, expects a close
    name         : MarkupName,
    attributes   : MarkupAttributes,
    token_start  : F,
    token_end    : F,
}

//ii StackElement
impl <F:FilePosition> StackElement<F> {
    pub fn new(parent_depth:usize, depth:usize, boxed:bool, name:MarkupName,
               token_start:F, token_end:F) -> StackElement<F> {
        StackElement {
            parent_depth, depth, boxed,
            name         : name,
            attributes   : MarkupAttributes::new(),
            token_start, token_end,
        }
    }
    pub fn as_start_element(&mut self, _namespace:Namespace) -> MarkupEvent<F> {
        // Move attributes from self to new vector
        let mut attributes = MarkupAttributes::new();
        attributes.steal(&mut self.attributes);
        MarkupEvent::start_element( self.token_start, self.token_end, self.name.clone(), attributes )
        // namespace  : namespace,
    }        
    pub fn as_end_element(&self) -> (MarkupEvent<F>, usize) {
        ( MarkupEvent::end_element( self.token_start, self.name.clone() ),
          self.parent_depth )
    }
}

//a Public types: Parser and TokenFn
//tp Parser
/// A parser, usingg a file position provided
///
pub struct Parser <F:FilePosition>{
    start_emitted     : bool,
    end_emitted       : bool,
    finished          : bool,
    tag_depth         : usize,
    tag_stack         : Vec<StackElement<F>>,
    ns_stack          : NamespaceStack,
    pending_eof       : bool,
    pending_open_tag  : Vec<(usize, MarkupName, bool)>, // at most 1 deep
    pending_close_tag : Vec<(usize, MarkupName)>, // at most 1 deep
    pending_token     : Vec<TokenWithPos<F>>, // at most 1 deep
    start_element_building : bool,
    token_start       : F,
    token_end         : F,
}

//ip Parser
impl <F:FilePosition> Parser<F> {

    //fp new
    /// Returns a new lexer with default state.
    pub fn new() -> Self {
        Parser {
            start_emitted: false,
            end_emitted: false,
            finished: false,
            tag_depth : 0,
            tag_stack : Vec::new(),
            ns_stack : NamespaceStack::empty(),
            pending_eof : false,
            pending_open_tag  : Vec::new(),
            pending_close_tag : Vec::new(),
            pending_token     : Vec::new(),
            start_element_building : false,
            token_start : F::new(),
            token_end   : F::new(),
        }
    }

    //mi pop_tag_stack
    /// pop_tag_stack
    // Pops the tag stack and returns an XmlEvent of an end of that element
    fn pop_tag_stack(&mut self) -> Result<MarkupEvent<F>,ParseError<F>> {
        assert!(self.tag_stack.len()>0);
        self.ns_stack.pop();
        let (e, depth) = self.tag_stack.pop().unwrap().as_end_element();
        self.tag_depth = depth;
        Ok(e)
    }
    
    //mi push_tag_stack
    /// only invoked when self.pending_open_tag is not None, and presumably
    /// when the indent depth is appropriate for the tag (i.e. self.tag_depth+1==depth)
    /// pushes self.pending_open_tag on to stack
    fn push_tag_stack(&mut self) -> () {
        let (depth, ns_name, boxed) = self.pending_open_tag.pop().unwrap();
        self.tag_stack.push(StackElement::new(self.tag_depth, depth, boxed, ns_name, self.token_start, self.token_end));
        self.ns_stack.push_empty();
        self.start_element_building = true;
        self.tag_depth += 1;
        if boxed {
            self.tag_depth = 0;
        }
    }
    
    //mi start_element_event
    /// If no start_element being built then return None, otherwise the StartElement
    /// for the top of the tag stack
    fn start_element_event(&mut self) -> MarkupEvent<F> {
        let n = self.tag_stack.len()-1;
        self.tag_stack[n].as_start_element(self.ns_stack.squash())
    }
    
    //mi start_element_add_attribute
    /// If no start_element being built then return ParseError, otherwise add it to the top of the stack
    fn start_element_add_attribute(&mut self, ns_name:MarkupName, value:String) -> Result<(),ParseError<F>> {
        if !self.start_element_building {
            Err(ParseError::unexpected_attribute(self.token_start, self.token_end, ns_name))
        } else {
            let n = self.tag_stack.len()-1;
            self.tag_stack[n].attributes.add(ns_name, value);
            Ok(())
        }
    }
    
    //mi push_token
    fn push_token(&mut self, t:TokenWithPos<F>) -> () {
        self.pending_token.push(t);
    }
    
    //mi add_pending_open_tag
    fn add_pending_open_tag(&mut self, name:MarkupName, depth:usize, boxed:bool) -> () {
        self.pending_open_tag.push((depth, name, boxed));
    }
    
    //mi add_pending_close_tag
    fn add_pending_close_tag(&mut self, name:MarkupName, depth:usize) -> () {
        self.pending_close_tag.push((depth, name));
    }
    
    /// next_event
    pub fn next_event<T> (&mut self, mut get_token:T) -> Result<MarkupEvent<F>,ParseError<F>>
        where T: FnMut () -> Result<TokenWithPos<F>, TokenError<F>>
    {
        if !self.start_emitted {
            self.start_emitted = true;
            Ok(MarkupEvent::StartDocument { file_pos:self.token_start, version:"1.0".to_string() })
        } else if self.finished {
            Err(ParseError::no_more_events())
        } else if self.end_emitted {
            self.finished = true;
            Ok(MarkupEvent::EndDocument { file_pos:self.token_start })
        } else if self.pending_eof {
            if self.tag_stack.len()>0 {
                self.pop_tag_stack()
            } else {
                self.end_emitted = true;
                self.next_event(get_token)
            }
        } else if self.pending_close_tag.len()>0 { // will close something!
            if self.tag_depth>0 { // close the current element at the top of the stack
                self.pop_tag_stack()
            } else { // token should match, open should be boxed, and we can close the element this involves dropping the pending_tag too
                let (_depth, _ns_name) = self.pending_close_tag.pop().unwrap();
                self.pop_tag_stack()
            }
        } else if self.pending_open_tag.len()>0 { // will close something or open this!
            let (depth,_,_) = self.pending_open_tag[0];
            if depth<=self.tag_depth { // close the current element at the top of the stack
                self.pop_tag_stack()
            } else if depth==self.tag_depth+1 { // open the new element
                self.push_tag_stack();
                self.next_event(get_token)
            } else { // too far down!
                Err(ParseError::unexpected_tag_indent(self.token_start, self.token_end, depth))
            }
        } else { // read a token and do something!
            let token = {
                match self.pending_token.pop() {
                    Some(t) => t,
                    None    => get_token()?,
                }
            };
            let should_emit_start_element = {
                match token {
                    (_,_,Token::Attribute(_,_))  => false,
                    _ => true,
                }
            };
            if should_emit_start_element && self.start_element_building {
                self.start_element_building = false;
                self.push_token(token); 
                Ok(self.start_element_event())
            } else {
                self.token_start = token.0;
                self.token_end   = token.1;
                match token.2 {
                    Token::Comment(string_list)  => {
                        let mut comment = String::new();
                        let mut first = true;
                        for s in &string_list {
                            if !first {comment.push('\n');}
                            first = false;
                            comment.push_str(s);
                        }
                        Ok(MarkupEvent::Comment{bounds:(token.0, token.1), data:MarkupContent::Text(comment)})
                    },
                    Token::TagOpen(ns_name, depth, boxed)  => {
                        self.add_pending_open_tag(ns_name, depth, boxed);
                        self.next_event(get_token)
                    },
                    Token::TagClose(ns_name, depth)  => {
                        self.add_pending_close_tag(ns_name, depth);
                        self.next_event(get_token)
                    },
                    Token::Attribute(ns_name, value)  => {
                        self.start_element_add_attribute(ns_name, value)?;
                        self.next_event(get_token)
                    },
                    Token::Characters(s)  => {
                        Ok(MarkupEvent::Content{bounds:(token.0, token.1), data:MarkupContent::Text(s)})
                    },
                    Token::EndOfFile  => {
                        self.pending_eof = true;
                        self.next_event(get_token)
                    },
                }
            }
        }
    }
}

//a Test
#[cfg(test)]
mod tests {
    use super::Parser;
    use super::super::char::Reader;
    use xml::attribute::{Attribute, OwnedAttribute};
    use xml::name::{Name, OwnedName};
    use xml::namespace::{Namespace};
    use xml::reader::XmlEvent;
    use xml::common::XmlVersion;

    use super::super::lexer::{LexerOfReader};
    fn onos(s:&str) -> OwnedName {
        Name::from(s).to_owned()
    }
    fn oaos(s:&str, v:&str) -> OwnedAttribute {
        Attribute::new(Name::from(s), v).to_owned()
    }
    fn test_string(s:&str, exp:Vec<XmlEvent>) {
        let mut bytes  = s.as_bytes();
        let mut reader = Reader::new(&mut bytes);
        let mut lexer  = LexerOfReader::new(&mut reader);
        let mut parser  = Parser::new();
        for i in 0..exp.len() {
            let t = parser.next_event(|| lexer.next_token_with_pos());
            assert_eq!( t.is_err(), false, "T should not be an error {:?}", t);
            let t = t.unwrap();
            println!("{:?}, {:?}", t, exp[i]);
            // assert_eq!( t.2, exp[i] );
        }
    }
    #[test]
    fn test_blah() {
        test_string( "#svg ##line ##text",
                       vec![XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None },
                            XmlEvent::StartElement  { name:onos("svg"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("line"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("line") },
                            XmlEvent::StartElement  { name:onos("text"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("text") },
                            XmlEvent::EndElement    { name:onos("svg") },
                            XmlEvent::EndDocument,
                       ]
                       );
    }
    #[test]
    fn test_blah2() {
        test_string( "#svg ##box{ ##box}",
                       vec![XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None },
                            XmlEvent::StartElement  { name:onos("svg"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("box"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("box") },
                            XmlEvent::EndElement    { name:onos("svg") },
                            XmlEvent::EndDocument,
                       ]
        );
    }
    #[test]
    fn test_blah3() {
        test_string( "#svg ##box{ #line ##box}",
                       vec![XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None },
                            XmlEvent::StartElement  { name:onos("svg"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("box"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("line"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("line") },
                            XmlEvent::EndElement    { name:onos("box") },
                            XmlEvent::EndElement    { name:onos("svg") },
                            XmlEvent::EndDocument,
                       ]
        );
    }
    #[test]
    fn test_blah4() {
        test_string( "#svg ##box{ #line #line ##box}",
                       vec![XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None },
                            XmlEvent::StartElement  { name:onos("svg"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("box"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("line"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("line") },
                            XmlEvent::StartElement  { name:onos("line"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("line") },
                            XmlEvent::EndElement    { name:onos("box") },
                            XmlEvent::EndElement    { name:onos("svg") },
                            XmlEvent::EndDocument,
                       ]
        );
    }
    #[test]
    fn test_blah5() {
        test_string( "#svg ##box{ #innerbox{ #line #innerbox} ##box}",
                       vec![XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None },
                            XmlEvent::StartElement  { name:onos("svg"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("box"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("innerbox"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("line"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("line") },
                            XmlEvent::EndElement    { name:onos("innerbox") },
                            XmlEvent::EndElement    { name:onos("box") },
                            XmlEvent::EndElement    { name:onos("svg") },
                            XmlEvent::EndDocument,
                       ]
        );
    }
    #[test]
    fn test_attr1() {
        test_string( "#svg a='1' ##line b='2'",
                       vec![XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None },
                            XmlEvent::StartElement  { name:onos("svg"),  attributes:vec![oaos("a","1"),], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("line"), attributes:vec![oaos("b","2"),], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("line") },
                            XmlEvent::EndElement    { name:onos("svg") },
                            XmlEvent::EndDocument,
                       ]
        );
    }
    #[test]
    fn test_attr2() {
        test_string( "#svg a='1' b='2' ##line ",
                       vec![XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None },
                            XmlEvent::StartElement  { name:onos("svg"),  attributes:vec![oaos("a","1"), oaos("b","2"),], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("line"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("line") },
                            XmlEvent::EndElement    { name:onos("svg") },
                            XmlEvent::EndDocument,
                       ]
        );
    }
    #[test]
    fn test_attr3() {
        test_string( "#svg ##box{ a='1' b='2' ##box} ##line ",
                       vec![XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None },
                            XmlEvent::StartElement  { name:onos("svg"),  attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::StartElement  { name:onos("box"),  attributes:vec![oaos("a","1"), oaos("b","2"),], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("box") },
                            XmlEvent::StartElement  { name:onos("line"), attributes:vec![], namespace:Namespace::empty() },
                            XmlEvent::EndElement    { name:onos("line") },
                            XmlEvent::EndElement    { name:onos("svg") },
                            XmlEvent::EndDocument,
                       ]
        );
    }
}
