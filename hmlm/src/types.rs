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
use std::fmt::Write; // for write_char in formatters
// use xml::reader::XmlEvent as XmlReaderEvent;
use xml::writer::XmlEvent as XmlWriterEvent;
use xml::name::Name as XmlName;
use xml::common::XmlVersion as XmlVersion;
use xml::attribute::Attribute as XmlAttribute;
use xml::namespace::Namespace as XmlNamespace;
use xml::namespace::NamespaceStack as XmlNamespaceStack;
use super::utils; // for file position 'next line'

//a Namespace stuff - replace me 
/// Namespace is a map from prefixes to namespace URIs.
///
/// No prefix (i.e. default namespace) is designated by `NS_NO_PREFIX` constant.
pub type Namespace = XmlNamespace;
pub type NamespaceStack = XmlNamespaceStack;
// use std::collections::btree_map::{BTreeMap, Entry};
use std::borrow::Cow;
pub fn as_xml_namespace<'a>(ns:&'a Namespace) -> Cow<'a, XmlNamespace> {
    Cow::Borrowed(ns)
}

//a FilePosition trait
//tt FilePosition
pub trait FilePosition : Clone + Copy + Sized + std::fmt::Debug + std::fmt::Display {
    //fp new - Create a new file position
    /// Create a new object - required for every FilePosition type
    fn new() -> Self;
    fn move_by(&mut self, ch:char);
}

//a Character result and error
//tp Char
/// `Char` represents a unicode character or EOF marker
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Char {
    /// Eof indicates end of stream/file reached; once a reader returns Eof, it should continue to do so
    Eof,
    /// NoData indicates that the stream/file did not supply data, but this is configured to not be EOF
    /// This can only be returned by the reader if `eof_on_no_data` is false
    NoData,
    /// Char indicates a valid Unicode character
    Char(char)
}

//ip std::fmt::Display for Char
impl std::fmt::Display for Char {
    //mp fmt - format a character for display
    /// Display the character as either the character itself, or '<EOF>'
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Char::Eof    => write!(f, "<EOF>"),
            Char::NoData => write!(f, "<NoData>"),
            Char::Char(ch) => f.write_char(*ch),
        }
    }
}

//a CharError, CharResult
//tp CharError
/// `CharError` represents an error from a UTF-8 character reader,
/// either an IO error from the reader or a malformed UTF-8 encoded
/// set of bytes
#[derive(Debug)]
pub enum CharError<F:FilePosition> {
    /// An `IoError` is passed through from a reader as a `CharError`
    IoError(std::io::Error),
    /// A MalformedUtf8 error occurs when a byte stream contains
    /// invalid UTF-8; the Unicode-character position of the error is
    /// recorded, and the number of bytes that form the invalid UTF-8
    /// encoding (which will be from 1 to 3)
    MalformedUtf8(usize, F),
}

//ip From IO Error to CharError
/// Provides an implicit conversion from a std::io::Error to a CharError
impl <F:FilePosition> From<std::io::Error> for CharError<F> {
    fn from(e: std::io::Error) -> Self {
        CharError::IoError(e)
    }
}

//ip std::fmt::Display for CharError
impl <F:FilePosition> std::fmt::Display for CharError<F> {
    //mp fmt - format a `CharError` for display
    /// Display the `CharError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CharError::MalformedUtf8(n, pos) => write!(f, "malformed UTF-8 of {} bytes at {}", n, pos),
            CharError::IoError(ref e) => write!(f, "IO error: {}", e),
        }
    }
}

//tp CharResult
/// `CharResult` represents the result of fetching a character
pub type CharResult<F> = std::result::Result<Char, CharError<F>>;

//a TokenError
//tp TokenError
/// `TokenError` represents an error in a token stream; this may be
/// due to a UTF-8 decoding error, an I/O error on the underlying
/// stream, or an unexpected character within a token.
#[derive(Debug)]
pub enum TokenError<F:FilePosition> {
    /// `UnexpectedCharacter` indicates that a character occurred that
    /// did not fit with those required by the current token A
    /// character that is not permitted to start a token (such as a
    /// digit) would be unexpected if the tokenizer is looking for the
    /// start of a token, for example. Tokens are expected to be
    /// separated by whitespace - and so on. Two file positions are
    /// provided in the error - the first is from the start of the
    /// token, the second the position of the error
    UnexpectedCharacter(char, F, F),
    /// `UnexpectedEOF` indicates that an EOF was found during the
    /// decoding of, for example, a quoted string, or within an
    /// attribute. EOF can only occur after a token (or whitespace following a token). Two file positions are
    /// provided in the error - the first is from the start of the
    /// token, the second the position of the error
    UnexpectedEOF(F,F),
    /// `MalformedUtf8` occurs if the underlying char stream indicates a malformed UTF-8 encoding
    MalformedUtf8(usize, F),
    /// `IoError`s from the underlying stream get passed through
    IoError(std::io::Error),
}

//ip From CharError for TokenError
impl <F:FilePosition> From<CharError<F>> for TokenError<F> {
    //mp from CharError
    /// Render a CharError as a TokenError for implicit conversions
    fn from(e: CharError<F>) -> TokenError<F> {
        match e {
            CharError::IoError(e    )       => TokenError::IoError(e),
            CharError::MalformedUtf8(n,pos) => TokenError::MalformedUtf8(n,pos),
        }
    }
}

//ip From std::io::Error for TokenError
impl <F:FilePosition> From<std::io::Error> for TokenError<F> {
    //mp from std::io::Error
    /// Render an IO Error as a TokenError for implicit conversions
    fn from(e: std::io::Error) -> Self {
        TokenError::IoError(e)
    }
}

//ip std::fmt::Display for TokenError
impl <F:FilePosition> std::fmt::Display for TokenError<F> {
    //mp fmt - format a `TokenError` for display
    /// Display the `TokenError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TokenError::UnexpectedEOF(pos1, pos2) => write!(f, "unexpected EOF at {} in token starting at {}", pos2, pos1),
            TokenError::UnexpectedCharacter(ch, pos1, pos2) => write!(f, "unexpected character {} at {} in token starting at {}", ch, pos2, pos1),
            TokenError::MalformedUtf8(n, pos) => write!(f, "malformed UTF-8 of {} bytes at {}", n, pos),
            TokenError::IoError(ref e) => write!(f, "IO error: {}", e),
        }
    }
}

//a ParseError type
#[derive(Debug)]
/// `ParseError` provides for all the errors the parser can return
pub enum ParseError <F:FilePosition> {
    /// `UnexpectedTagIndent` occurs when a tag is provided in the stream
    /// with too many '#' as an indent
    UnexpectedTagIndent(F, F, usize),
    /// `UnexpectedAttribute` occurs when an attribute token is in the
    /// stream but it does not follow an open tag or another attribute
    /// token
    UnexpectedAttribute(F, F, MarkupName),
    /// `EventAfterEnd` occurs when the client polls for an event after an EndDocument event has been provide
    EventAfterEnd,
    /// `TokenError` occurs when there is an underlying token decode error, or IO error
    Token(TokenError<F>)
}

//ip From TokenError for ParseError
impl <F:FilePosition> From<TokenError<F>> for ParseError<F> {
    //mp from TokenError
    /// Render a TokenError as a ParseError for implicit conversions
    fn from(e: TokenError<F>) -> Self {
        ParseError::Token(e)
    }
}

//ip ParseError
impl <F:FilePosition> ParseError<F> {
    pub fn no_more_events() -> Self {
        ParseError::EventAfterEnd
    }
    pub fn unexpected_attribute(pos1: F, pos2: F, ns_name:MarkupName) -> Self {
        ParseError::UnexpectedAttribute(pos1, pos2, ns_name.clone())
    }
    pub fn unexpected_tag_indent(pos1: F, pos2: F, depth:usize) -> Self {
        ParseError::UnexpectedTagIndent(pos1, pos2, depth)
    }
}

//ip std::fmt::Display for ParseError
impl <F:FilePosition> std::fmt::Display for ParseError<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedAttribute(pos1, _pos2, ns_name) => write!(f, "Unexpected HMLH attribute {} at {}", ns_name, pos1),
            ParseError::UnexpectedTagIndent(pos1, _pos2, depth) => write!(f, "Unexpected tag indent {} in tag at {}", depth, pos1),
            ParseError::EventAfterEnd            => write!(f, "Client request event after EndDocument was reported"),
            ParseError::Token(ref e) => write!(f, "{}", e),
        }
    }
}

//a HmlFilePosition
//tp HmlFilePosition
/// Holds the line number and character position of a character in a file
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HmlFilePosition {
    /// Line, with first line being 0
    pub ln: usize,
    /// Column, starting with 0
    pub ch: usize,
}

//ip std::fmt::Display for HmlFilePosition
impl std::fmt::Display for HmlFilePosition {

    //mp fmt - format for display
    /// Display the `FilePosition` as line and column
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line {} column {}", self.ln+1, self.ch+1)
    }

    //zz All done
}

//ip FilePosition for HmlFilePosition
impl FilePosition for HmlFilePosition {
    //fp new - Create a new file position
    /// Create a new object at line 0 character 0
    fn new() -> Self {
        Self { ln:0, ch:0 }
    }

    //mp move_by - Move the position on by a character
    /// Move the file position on by a character, accounting for newlines
    fn move_by(&mut self, ch:char) -> () {
        self.ch += 1;
        if utils::is_newline(ch as u32) {
            self.ln += 1;
            self.ch = 0;
        }
    }

    //zz All done
}
    
//ip HmlFilePosition
impl HmlFilePosition {

    //zz All done
}

//tp XmlEvent
/// This is derived from the XmlEvent used in the xml-rs rust library.
/// 
/// An element of an XML input stream.
///
/// Items of this enum are emitted by `reader::EventReader`. They correspond to different
/// elements of an XML document.
//a Markup
//tp MarkupContent
#[derive(Clone, Debug)]
pub enum MarkupContent {
    Text(String),
    Whitespace(String),
}

impl MarkupContent {
    pub fn borrow_str(&self) -> &str {
        match self {
            Self::Text(s)       => s,
            Self::Whitespace(s) => s,
        }
    }
}

//tp MarkupName
#[derive(Clone, Debug)]
pub struct MarkupName {
    pub ns   : Option<String>,
    pub name : String,
}
impl MarkupName {
    pub fn new(ns:Option<String>, name:String) -> Self {
        Self { ns, name }
    }
    #[inline]
    pub fn set_ns(mut self, ns:String) -> Self {
        self.ns = Some(ns);
        self
    }
    pub fn as_xml_name(&self) -> XmlName {
        match self.ns.as_ref() {
            None     => XmlName::local(&self.name),
            Some(ns) => XmlName::prefixed(&self.name, ns),
        }
    }
}

//ip std::fmt::Display for MarkupName
impl std::fmt::Display for MarkupName {
    //mp fmt - format a `MarkupName` for display
    /// Display the `MarkupName` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self.ns {
            None     => write!(f, "{}", self.name),
            Some(ns) => write!(f, "{}:{}", ns, self.name),
        }
    }
}

//tp MarkupAttributes
pub struct MarkupAttributes {
    name_values: Vec<(MarkupName,String)>
}
impl MarkupAttributes {
    pub fn new() -> Self {
        Self { name_values : Vec::new() }
    }
    pub fn is_empty(&self) -> bool {
        self.name_values.is_empty()
    }
    pub fn add(&mut self, name:MarkupName, value:String) {
        self.name_values.push((name,value));
    }
    pub fn steal(&mut self, v:&mut Self) {
        self.name_values.append(&mut v.name_values);
    }
    pub fn to_name_values(&self) -> Vec<(String,String)> {
        self.name_values
            .iter()
            .map( |(n,v)| (n.name.clone(),v.clone()) )
            .collect()
    }
    pub fn as_xml_attributes<'a> (&'a self) -> Cow<'a, [XmlAttribute<'a>]> {
        self.name_values
            .iter()
            .map( |(n,v)| XmlAttribute::new(n.as_xml_name(), v) )
            .collect()
    }
    pub fn iter_name_values(&self) -> impl Iterator<Item = (&str, &str)> {
        self.name_values
            .iter()
            .map( |(n,v)| (n.name.as_str(), v.as_str()) )
    }
}
impl std::fmt::Debug for MarkupAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.name_values.is_empty() {
            write!(f, "[]")
        } else {
            let attributes : Vec<String> = self.name_values.iter()
                .map( |(n,v)| format!("{} -> {}", n, v) )
                .collect();
            write!(f, "[{}]", attributes.join(", "))
        }
    }
}

//tp MarkupEvent
pub enum MarkupEvent<F:FilePosition> {
    /// The start of the document
    StartDocument {
        /// File position of start of the document
        file_pos : F,
        /// If the XML document has a version tag, this will be it - it defaults to "1.0"
        version: String,
    },

    EndDocument {
        /// File position of end of the document
        file_pos : F,
    },

    /// Denotes a beginning of an XML element.
    StartElement {
        bounds     : (F,F),
        name       : MarkupName,
        attributes : MarkupAttributes,
        namespace  : Namespace,
    },

    /// Denotes an end of an XML element.
    EndElement {
        name       : MarkupName,
        /// File position of end of the document
        file_pos : F,
    },

    /// Denotes content of an element
    Content {
        bounds  : (F,F),
        data    : MarkupContent,
    },
    
    /// Denotes an XML processing instruction.
    ProcessingInstruction {
        bounds  : (F,F),
        name: String,
        data: Option<String>
    },

    /// Denotes a comment.
    Comment {
        bounds  : (F,F),
        data    : MarkupContent,
    },
}

//ip MarkupEvent
impl <F:FilePosition> MarkupEvent<F> {
    pub fn start_element(start:F, end:F, name:MarkupName, attributes:MarkupAttributes) -> Self {
        let namespace = Namespace::empty();
        Self::StartElement { bounds : (start,end), name, attributes, namespace }
    }
    pub fn end_element(file_pos:F, name:MarkupName) -> Self {
        Self::EndElement { file_pos, name }
    }
}

//ip Debug for MarkupEvent
impl <F:FilePosition> std::fmt::Debug for MarkupEvent<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::StartDocument { file_pos, version } =>
                write!(f, "StartDocument({:?},{})", file_pos, version),
            Self::EndDocument { file_pos } =>
                write!(f, "EndDocument({:?})", file_pos),
            Self::StartElement { bounds, name, attributes, .. /* namespace */ } =>
                write!(f, "StartElement({:?}->{:?}, {:?} {:?})", bounds.0, bounds.1, name, attributes,
                       // namespace,
                       ),
            Self::EndElement { file_pos, name } =>
                write!(f, "EndElement({:?} {:?})", file_pos, name ),
            Self::Content { bounds, data } =>
                write!(f, "Content({:?}->{:?} {:?})", bounds.0, bounds.1, data),
            Self::ProcessingInstruction { bounds, name, data } =>
                write!(f, "ProcessingInstruction({:?}->{:?} {}{:?})", bounds.0, bounds.1, name, data),
            Self::Comment { bounds, data } =>
                write!(f, "Comment({:?}->{:?} {:?})", bounds.0, bounds.1, data),
        }
    }
}

//tp Markupevent continued
impl <F:FilePosition> MarkupEvent<F> {
    pub fn as_xml_writer_event<'a>(&'a self) -> Option<XmlWriterEvent<'a>> {
        match self {
            Self::StartDocument { .. } =>
                Some(XmlWriterEvent::StartDocument {
                    version:    XmlVersion::Version10,
                    encoding:   Some("UTF-8"),
                    standalone: None,
                }),
            Self::ProcessingInstruction { name, data, .. } =>
                Some(XmlWriterEvent::ProcessingInstruction {
                    name: name,
                    data: data.as_ref().map(|s| s.as_str()),
                }),
            Self::StartElement {name, attributes, namespace, .. } =>
                Some(XmlWriterEvent::StartElement {
                    name       : name.as_xml_name(),
                    attributes : attributes.as_xml_attributes(),
                    namespace  : as_xml_namespace(namespace),
                }),
            Self::EndElement { name, .. } =>
                Some(XmlWriterEvent::EndElement {
                    name: Some(name.as_xml_name())
                }),
            Self::Comment{ data, ..} =>
                Some(XmlWriterEvent::Comment(
                    data.borrow_str(),
                )),
            Self::Content{ data, ..} => Some(
                XmlWriterEvent::Characters(
                    data.borrow_str(),
                )),
            _ => None
        }
    }
}
