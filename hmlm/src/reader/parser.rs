//! A stream parser for HMLH producing xml-rs reader::XmlEvent's
//!

//a Imports
use std::fmt;
use std::io::prelude::BufRead;
use std::io::Read;
use std::result;
use xml::attribute::{Attribute, OwnedAttribute};
use xml::name::{Name, OwnedName};
use xml::namespace::{Namespace, NamespaceStack};
use xml::reader::XmlEvent;
use xml::common::XmlVersion;

use super::char::FilePosition;
use super::lexer::{Token, TokenWithPos, TokenError, Lexer, NamespaceName};

//a Conversion functions
fn owned_of_ns_name (ns_name:&NamespaceName) -> OwnedName {
    match ns_name.namespace {
        Some(ref prefix) => Name::prefixed(&ns_name.name, prefix).to_owned(),
        None             => Name::local(&ns_name.name).to_owned(),
    }
}

//a ParseError type
#[derive(Debug)]
/// `ParseError` provides for all the errors the parser can return
pub enum ParseError {
    /// `UnexpectedTagIndent` occurs when a tag is provided in the stream
    /// with too many '#' as an indent
    UnexpectedTagIndent(FilePosition, FilePosition),
    /// `UnexpectedAttribute` occurs when an attribute token is in the
    /// stream but it does not follow an open tag or another attribute
    /// token
    UnexpectedAttribute(FilePosition, FilePosition),
    /// `EventAfterEnd` occurs when the client polls for an event after an EndDocument event has been provide
    EventAfterEnd,
    /// `TokenError` occurs when there is an underlying token decode error, or IO error
    Token(TokenError)
}

//ip From TokenError for ParseError
impl From<TokenError> for ParseError {
    //mp from TokenError
    /// Render a TokenError as a ParseError for implicit conversions
    fn from(e: TokenError) -> ParseError {
        ParseError::Token(e)
    }
}

impl ParseError {
    pub fn no_more_events() -> ParseError {
        ParseError::EventAfterEnd
    }
    pub fn unexpected_attribute(pos1: FilePosition, pos2: FilePosition) -> ParseError {
        ParseError::UnexpectedAttribute(pos1, pos2)
    }
    pub fn unexpected_tag_indent(pos1: FilePosition, pos2: FilePosition) -> ParseError {
        ParseError::UnexpectedTagIndent(pos1, pos2)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::UnexpectedAttribute(pos1, pos2) => write!(f, "Unexpected HMLH attribute at {}", pos1),
            ParseError::UnexpectedTagIndent(pos1, pos2) => write!(f, "Unexpected tag indent in tag at {}", pos1),
            ParseError::EventAfterEnd            => write!(f, "Client request event after EndDocument was reported"),
            ParseError::Token(ref e) => write!(f, "{}", e),
        }
    }
}

//a Internal types: StackElement
#[derive(Clone, PartialEq, Eq)]
struct StackElement {
    parent_depth : usize,
    depth        : usize, // all elements inside of >depth are in this element, of ==depth are siblings
    boxed        : bool, // if true, expects a close
    ns_name      : OwnedName,
    attributes   : Vec<OwnedAttribute>,
}

impl StackElement {
    pub fn new(parent_depth:usize, depth:usize, boxed:bool, ns_name:&NamespaceName) -> StackElement {
        StackElement {
            parent_depth, depth, boxed,
            ns_name      : owned_of_ns_name(ns_name),
            attributes   : Vec::new(),
        }
    }
    pub fn as_start_element(&mut self, namespace:Namespace) -> XmlEvent {
        let mut attributes = Vec::new();
        attributes.append(&mut self.attributes);
        XmlEvent::StartElement{name       : self.ns_name.clone(),
                               attributes : attributes,
                               namespace  : namespace,
        }
    }        
    pub fn as_end_element(&self) -> (XmlEvent, usize) {
        (XmlEvent::EndElement{name:self.ns_name.clone()}, self.parent_depth)
    }
}

//a Public types: Parser and TokenFn
//tp Parser
/// `Parser`
///
pub struct Parser {
    start_emitted     : bool,
    end_emitted       : bool,
    finished          : bool,
    tag_depth         : usize,
    tag_stack         : Vec<StackElement>,    // 
    ns_stack          : NamespaceStack,
    pending_eof       : bool,
    pending_open_tag  : Vec<(usize, NamespaceName, bool)>, // at most 1 deep
    pending_close_tag : Vec<(usize, NamespaceName)>, // at most 1 deep
    pending_token     : Vec<TokenWithPos>, // at most 1 deep
    start_element_building : bool,
    token_start       : FilePosition,
    token_end         : FilePosition,
}

//ip Parser
impl Parser {

    //fp new
    /// Returns a new lexer with default state.
    pub fn new() -> Parser {
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
            token_start : FilePosition::new(),
            token_end   : FilePosition::new(),
        }
    }

    //mi pop_tag_stack
    /// pop_tag_stack
    // Pops the tag stack and returns an XmlEvent of an end of that element
    fn pop_tag_stack(&mut self) -> Result<XmlEvent,ParseError> {
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
        self.tag_stack.push(StackElement::new(self.tag_depth, depth, boxed, &ns_name));
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
    fn start_element_event(&mut self) -> XmlEvent {
        let n = self.tag_stack.len()-1;
        self.tag_stack[n].as_start_element(self.ns_stack.squash())
    }
    
    //mi start_element_add_attribute
    /// If no start_element being built then return ParseError, otherwise add it to the top of the stack
    fn start_element_add_attribute(&mut self, ns_name:NamespaceName, value:String) -> Result<(),ParseError> {
        if !self.start_element_building {
            Err(ParseError::unexpected_attribute(self.token_start, self.token_end))
        } else {
            let attr = OwnedAttribute::new(owned_of_ns_name(&ns_name),value);
            let n = self.tag_stack.len()-1;
            self.tag_stack[n].attributes.push(attr);
            Ok(())
        }
    }
    
    //mi push_token
    fn push_token(&mut self, t:TokenWithPos) -> () {
        self.pending_token.push(t);
    }
    
    //mi add_pending_open_tag
    fn add_pending_open_tag(&mut self, name:NamespaceName, depth:usize, boxed:bool) -> () {
        self.pending_open_tag.push((depth, name, boxed));
    }
    
    //mi add_pending_close_tag
    fn add_pending_close_tag(&mut self, name:NamespaceName, depth:usize) -> () {
        self.pending_close_tag.push((depth, name));
    }
    
    /// next_event
    pub fn next_event<T> (&mut self, mut get_token:T) -> Result<XmlEvent,ParseError>
        where T: FnMut () -> Result<TokenWithPos, TokenError>
    {
        if !self.start_emitted {
            self.start_emitted = true;
            Ok(XmlEvent::StartDocument { version:XmlVersion::Version10, encoding:"UTF-8".to_string(), standalone:None })
        } else if self.finished {
            Err(ParseError::no_more_events())
        } else if self.end_emitted {
            self.finished = true;
            Ok(XmlEvent::EndDocument)
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
                let (depth, ns_name) = self.pending_close_tag.pop().unwrap();
                self.pop_tag_stack()
            }
        } else if self.pending_open_tag.len()>0 { // will close something or open this!
            let (depth,_,_) = self.pending_open_tag[0];
            if depth<=self.tag_depth { // close the current element at the top of the stack
                self.pop_tag_stack()
            } else if (depth==self.tag_depth+1) { // open the new element
                self.push_tag_stack();
                self.next_event(get_token)
            } else { // too far down!
                Err(ParseError::unexpected_tag_indent(self.token_start, self.token_end))
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
                        Ok(XmlEvent::Comment(string_list[0].clone())) // BUG - must concat the strings
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
                        Ok(XmlEvent::Characters(s))
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
    use super::*;
    use super::super::char::Reader;
    fn onos(s:&str) -> OwnedName {
        Name::from(s).to_owned()
    }
    fn oaos(s:&str, v:&str) -> OwnedAttribute {
        Attribute::new(Name::from(s), v).to_owned()
    }
    fn test_string(s:&str, exp:Vec<XmlEvent>) {
        let mut bytes  = s.as_bytes();
        let mut reader = Reader::new(&mut bytes);
        let mut lexer  = Lexer::new(&mut reader);
        let mut parser  = Parser::new();
        for i in 0..exp.len() {
            let t = parser.next_event(|| lexer.next_token_with_pos());
            assert_eq!( t.is_err(), false, "T should not be an error {:?}", t);
            let t = t.unwrap();
            println!("{:?}, {:?}", t, exp[i]);
            assert_eq!( t, exp[i] );
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
