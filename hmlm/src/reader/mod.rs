use std::io::Read;

pub mod utils;
pub mod char;
pub mod lexer;
pub mod parser;

use xml::reader::XmlEvent;
use self::parser::ParseError;

pub struct EventReader<'a, R: Read> {
    lexer : lexer::Lexer<'a, R>,
    parser: parser::Parser,
    finished : bool,
}

impl <'a, R: Read> EventReader<'a, R> {
    /// Creates a new reader, consuming the given stream.
    #[inline]
    pub fn new<'b> (source: &'b mut char::Reader<R>) -> EventReader<'b, R> {
        let lexer       = lexer::LexerOfReader::new(source);
        let parser      = parser::Parser::new();
        EventReader { lexer, parser, finished:false }
    }
}

//ip Iterator for EventReader - iterate over events
impl <'a, R:Read> Iterator for EventReader<'a, R> {
    // we will be counting with usize
    type Item = Result<XmlEvent,ParseError>;

    //mp next - return next character or None if end of file
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let (mut p, mut l) = (&mut self.parser, &mut self.lexer);
             {
            let e = p.next_event(|| l.next_token_with_pos());
            match e {
                Ok(XmlEvent::EndDocument) | Err(_) => self.finished = true,
                _ => (),
            }
                 Some(e)
             }
        }
    }

    //zz All done
}
