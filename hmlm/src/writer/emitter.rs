use std::io::prelude::{Write};
// use std::io;
// use std::fmt;
// use std::result;
// use std::borrow::Cow;
// use std::error::Error;

use xml::name::{Name, OwnedName};
use xml::attribute::{Attribute};
// use xml::namespace::*;

use super::HmlmResult;
// use super::HmlmError;

pub fn write_str<W>(target: &mut W, s:&str) -> HmlmResult<()>
    where W:Write
    { 
    if s.contains("\n") { // output """<string>""", but replace any "" in the string with \"\"
        target.write(b"\"\"\"")?;
        target.write(s.as_bytes())?; // should map "" to \"\"
        target.write(b"\"\"\"")?;
        target.write(b"\n")?;
    } else if !s.contains('"') {
        target.write(b"\"")?;
        target.write(s.as_bytes())?;
        target.write(b"\"")?;
        target.write(b"\n")?;
    } else {
        target.write(b"'")?;
        let mut needs_quote = false;
        for subs in s.split("'") {
            if needs_quote {target.write(b"\'")?;}
            target.write(subs.as_bytes())?;
            needs_quote = true;
        }
        target.write(b"'")?;
        target.write(b"\n")?;
    }
    Ok (())
}

struct ElementStack {
    name:  OwnedName,
    attrs: Vec<String>,
    is_multiline: bool, // initially false, but it maybe_multiline and there is content then becomes true
    maybe_multiline: bool, // true initially if
    parent_indent_level : usize, // after popping this element, what is the indent level
    indent_level : usize, // if multiline then 0, else parent_indent_level+1
}

const INDENT_STRING : &str = "################################";
fn indent<'a> (n:usize) -> &'a str{
    let l:usize = INDENT_STRING.len();
    if n>=l {INDENT_STRING} else {&INDENT_STRING[l-n..l]}
}

impl ElementStack {
    fn new(indent_level:usize,
           name:Name,
           attributes: &[Attribute],
           maybe_multiline:bool) -> ElementStack {
        let mut e = ElementStack {
            name : name.to_owned(),
            attrs: Vec::new(),
            is_multiline : false,
            maybe_multiline: maybe_multiline,
            parent_indent_level: indent_level,
            indent_level : indent_level+1,
        };
        for attr in attributes.iter() {
            e.attrs.push(attr.to_string());
        }
        e
    }
    fn emit<W>(&mut self, target: &mut W, has_content:bool)
               -> HmlmResult<()>
        where W: Write
    {
        write!(target, "{}", indent(self.parent_indent_level+1))?;
        if has_content && self.maybe_multiline {
            self.is_multiline = true;
            self.indent_level = 0;
        }
        write!(target, "{}", self.name.borrow().repr_display())?;
        if self.is_multiline {
            target.write(b"{")?;
        }
        for s in &self.attrs {
            target.write(b" ")?;
            target.write(s.as_bytes())?;
        }
        target.write(b"\n")?;
        Ok(())
    }
    fn close<W>(&mut self, target: &mut W)
               -> HmlmResult<usize>
        where W: Write
    {
        if self.is_multiline {
            write!(target, "{}", indent(self.parent_indent_level+1))?;
            write!(target, "{}}}\n", self.name.borrow().repr_display())?;
        }
        Ok(self.parent_indent_level)
    }
}
pub struct Emitter {
    // config: EmitterConfig,
    // nst: NamespaceStack,
    indent_level : usize, // number of '#' symbols minus 1 used by current element
    indent_stack: Vec<ElementStack>,
    element_pending : bool,
}

impl Emitter {
    // pub fn new(config: EmitterConfig) -> Emitter {
    pub fn new() -> Emitter {
        Emitter {
            // config,
            // nst: NamespaceStack::empty(),
            indent_level : 0,
            indent_stack : Vec::new(),
            element_pending : false,
        }
    }
}

impl Emitter {
    fn emit_pending_element<W>(&mut self, target: &mut W, has_content: bool)
                                        -> HmlmResult<()>
        where W: Write
    {
        if self.element_pending {
            let l = self.indent_stack.len();
            self.indent_stack[l-1].emit(target, has_content)?;
            self.indent_level = self.indent_stack[l-1].indent_level;
            self.element_pending = false;
        }
        Ok(())
    }

    pub fn emit_start_element<W>(&mut self, target: &mut W,
                                 name: Name,
                                 attributes: &[Attribute]) -> HmlmResult<()>
        where W: Write
    {
        self.emit_pending_element(target, true)?;
        self.element_pending = true;
        let maybe_multiline = {
            if self.indent_level>2 { true }
            else {false}
        };
        let e = ElementStack::new(self.indent_level, name, attributes, maybe_multiline);
        self.indent_level = e.indent_level;
        self.indent_stack.push(e);
        Ok(())
    }

    pub fn emit_current_namespace_attributes<W>(&mut self, target: &mut W) -> HmlmResult<()>
        where W: Write
    {
        /*
        for (prefix, uri) in self.nst.peek() {
            match prefix {
                // internal namespaces are not emitted
                NS_XMLNS_PREFIX | NS_XML_PREFIX => Ok(()),
                //// there is already a namespace binding with this prefix in scope
                //prefix if self.nst.get(prefix) == Some(uri) => Ok(()),
                // emit xmlns only if it is overridden
                NS_NO_PREFIX => if uri != NS_EMPTY_URI {
                    write!(target, " xmlns=\"{}\"", uri)
                } else { Ok(()) },
                // everything else
                prefix => write!(target, " xmlns:{}=\"{}\"", prefix, uri)
            }?;
        }
*/
        Ok(())
    }

    pub fn emit_end_element<W: Write>(&mut self, target: &mut W,
                                      name: Option<Name>) -> HmlmResult<()> {
        self.emit_pending_element(target, false)?;
        self.indent_level = self.indent_stack.pop().unwrap().close(target)?;
        Ok(())
    }

    pub fn emit_cdata<W: Write>(&mut self, target: &mut W, content: &str) -> HmlmResult<()> {
        self.emit_pending_element(target, true)?;
        write_str(target, content)
    }

    pub fn emit_characters<W: Write>(&mut self, target: &mut W,
                                      content: &str) -> HmlmResult<()> {
        self.emit_pending_element(target, true)?;
        let is_whitespace = content.chars().all(|c| c.is_ascii_whitespace());
        if !is_whitespace {
            write_str(target, content)?;
        }
        Ok(())
    }

    pub fn emit_comment<W: Write>(&mut self, target: &mut W, content: &str) -> HmlmResult<()> {
        self.emit_pending_element(target, true)?;
        if content.contains("\n") {
            for subs in content.split("\n") {
                target.write(b";")?;
                target.write(subs.as_bytes())?;
                target.write(b"\n")?;
            }
        } else {
            target.write(b";")?;
            target.write(content.as_bytes())?;
            target.write(b"\n")?;
        }
        Ok(())
    }
}
