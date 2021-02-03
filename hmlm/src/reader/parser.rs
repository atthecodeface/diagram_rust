//! Contains simple lexer for HMLM documents.
//!
//! This module is for internal use.

use std::fmt;
use std::io::prelude::BufRead;
use std::io::Read;
use std::result;
use super::lexer::*;
use xml::attribute::{OwnedAttribute};
use xml::name::{OwnedName};

#[derive(Debug)]
pub enum ParserError {
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::IoError(ref e) => write!(f, "IO error: {}", e),
        }
    }
}

/// `Parser`
///
struct StackElement {
    parent_depth : usize,
    depth        : usize, // all elements inside of >depth are in this element, of ==depth are siblings
    multiline    : bool, // if true, expects a close
    ns_name      : OwnedName,
    attributes   : Vec<OwnedAttribute>,
}
pub struct Parser {
    start_emitted     : bool,
    end_emitted       : bool,
    tag_depth         : usize,
    tag_stack         : Vec<StackElement>,    // 
    ns_stack          : NamespaceStack,
    pending_eof       : bool,
    pending_open_tag  : Option<usize, NamespaceName, bool>,
    pending_close_tag : Option<usize, NamespaceName>,
    pending_token     : Option<Token>,
    start_element_building : bool,
}

pub type TokenFn = fn () -> Result<Token, TokenError>;

impl Parser {
    /// Returns a new lexer with default state.
    pub fn new() -> Parser {
        Parser {
            start_emitted: false,
            end_emitted: false,
            tag_depth : 0,
            tag_stack : Vec::new(),
            ns_stack : NamespaceStack::empty(),
            pending_eof : false,
            pending_open_tag : None,
            pending_close_tag : None,
            pending_token : None,
            start_element_building : false,
        }
    }

    /// pop_tag_stack
    // Pops the tag stack and returns an XmlEvent of an end of that element
    fn pop_tag_stack(&mut self) -> Result<XmlEvent,ParseError> {
        assert!(self.tag_stack.len()>0);
        self.ns_stack.pop();
        let se = self.tag_stack.pop().unwrap();
        self.tag_depth = se.parent_depth;
        Ok(XmlEvent::EndElement{name:se.ns_name})
    }
    
    /// push_tag_stack
    // push pending_open_tag on to stack
    pub fn push_tag_stack(&mut self) -> ()Result<(),ParseError> {
        let (depth, ns_name, multiline) = self.pending_open_tag.unwrap();
        self.pending_open_tag = None;
        let se = StackElement {
            parent_depth : self.tag_depth,
            depth        : depth,
            multiline    : multiline,
            ns_name      : owned_of_ns_name(ns_name),
            attributes   : Vec::new(),
        };
        self.tag_stack.push(se);
        self.ns_stack.push_empty();
        self.start_element_building = true;
        self.tag_depth = 0;
    }
    
    /// start_element_event
    // If no start_element being built then return None, otherwise the StartElement
    pub fn start_element_event(&mut self) -> Option<XmlEvent> {
        if !self.start_element_building {
            None
        } else {
            let se = self.tag_stack[self.tag_stack.len()-1];
            let ns = self.namespace_stack[self.tag_stack.len()-1];
            Some(XmlEvent::StartElement{name:owned_of_ns_name(se.ns_name),
                                        attributes:se.attributes
                                        namespace : self.ns.squash()})
        }
    }
    
    /// start_element_add_attribute
    // If no start_element being built then return ParseError, otherwise add it to the top of the stack
    pub fn start_element_add_attribute(&mut self, ns_name:NamespaceName, value:String) -> Result<(),ParseError> {
        if !self.start_element_building {
            Err()
        } else {
            let se = self.tag_stack[self.tag_stack.len()-1];
            let attr = OwnedAttribute::new(owned_of_ns_name(ns_name),value);
            se.attributes.push(attr);
            Ok()
        }
    }
    
    /// push_token_and_return
    pub fn push_token_and_return(&mut self, t:Token) -> Option<XmlEvent> {
        self.pending_token = Some(t);
    }
    
    /// get_next_token
    pub fn get_next_token(&mut self, get_token:TokenFn) -> Result<Token,TokenError> {
        match self.pending_token {
            None => get_token(),
            Some(t) => { self.pending_token=None;
                         t
            },
        }

    /// next_event
    pub fn next_event(&mut self, get_token:TokenFn) -> Result<XmlEvent,ParseError> {
        if !self.start_emitted {
            self.start_emitted = true;
            Ok(XmlEvent::StartDocument { version:Version10, encoding:"UTF-8", standalone:None })
        } else if self.end_emitted {
            Ok(XmlEvent::EndDocument)
        } else if self.pending_eof {
            if self.tag_stack.len()>0 {
                self.pop_tag_stack()
            } else {
                self.end_emitted = true;
                self.next_event(get_token)
            }
        } else if self.pending_close_tag.is_some() { // will close something!
            if self.tag_depth>0 { // close the current element at the top of the stack
                self.pop_tag_stack()
            } else { // token should match, open should be multiline, and we can close the element this involves dropping the pending_tag too
                let (depth, ns_name) = self.pending_close_tag.unwrap();
                self.pending_close_tag = None;
                self.pop_tag_stack()
            }
        } else if self.pending_open_tag.is_some() { // will close something or open this!
            let (depth,token) = self.pending_open_tag.unwrap();
            if depth<=self.tag_depth { // close the current element at the top of the stack
                self.pop_tag_stack()
            } else if (depth==self.tag_depth+1) { // open the new element
                self.push_tag_stack();
                self.next_event(get_token)
            } else { // too far down!
                ParserError;
            }
        } else { // read a token and do something!
            match self.get_next_token(get_token)? {
                Token::Comment(string_list)  => {
                    match self.start_element_event() {
                        Some(e) => {self.push_token_and_return(token); e},
                        None    => XmlEvent::Comment(s),
                    }
                }
                Token::TagOpen(ns_name, depth, multiline)  => {
                    match self.start_element_event() {
                        Some(e) => {self.push_token_and_return(token); e},
                        None    => {self.add_pending_open_tag(ns_name, depth, multiline); self.next_event(get_token)},
                    }
                }
                Token::TagClose(ns_name, depth)  => {
                    match self.start_element_event() {
                        Some(e) => {self.push_token_and_return(token); e},
                        None    => {self.add_pending_close_tag(ns_name, depth); self.next_event(get_token)},
                    }
                }
                Token::Attribute(ns_name, value)  => {
                    self.start_element_add_attribute(ns_name, value)?;
                    self.next_event(get_token)
                }
                Token::Characters(s)  => {
                    match self.start_element_event() {
                        Some(e) => {self.push_token_and_return(token); e},
                        None    => XmlEvent::Characters(s),
                    }
                }
                Token::EndOfFile  => {
                    match self.start_element_event() {
                        Some(e) => {self.push_token_and_return(token); e},
                        None    => {self.pending_eof = true;
                                    self.next_event(get_token)
                        },
                    }
                }
            }
        }
    }
}

//a Test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blah() {
        let mut buf = "; This is a comment\n   ; with more comment\n #banana r='2' \"\"\"Stuff \"\"  and more \"\"\"".as_bytes();
        let mut reader = Reader::new(&mut buf);
        let mut lexer  = Lexer::new(&mut reader);
        loop {
            let t = lexer.next_token();
            assert_eq!( t.is_err(), false, "T should not be an error");
            println!("{:?}", t);
            if (t.unwrap() == Token::EndOfFile) {break;}
        }
        assert_eq!(true, false);
    }
}
