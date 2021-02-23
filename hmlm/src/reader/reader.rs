use std::io::Read;

use super::char;
use super::lexer;
use super::parser;

use crate::types::*;

pub struct EventReader<R: Read> {
    reader   : char::Reader<R, HmlFilePosition>,
    lexer    : lexer::Lexer<HmlFilePosition>,
    parser   : parser::Parser<HmlFilePosition>,
    finished : bool,
}

impl <R: Read> EventReader<R> {
    /// Creates a new reader, consuming the given stream.
    #[inline]
    pub fn new (source: R) -> EventReader<R> {
        let reader      = char::Reader::new(source);
        let lexer       = lexer::Lexer::new();
        let parser      = parser::Parser::new();
        EventReader { reader, lexer, parser, finished:false }
    }
}

//ip Iterator for EventReader - iterate over events
impl <R:Read> Iterator for EventReader<R> {
    // we will be counting with usize
    type Item = Result<MarkupEvent<HmlFilePosition>, ParseError<HmlFilePosition>>;

    //mp next - return next character or None if end of file
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            let (p, l, r) = (&mut self.parser, &mut self.lexer, &mut self.reader);
            {
                let e = p.next_event(|| l.next_token_with_pos(r));
                match e {
                    Ok(MarkupEvent::EndDocument{..}) | Err(_) => self.finished = true,
                    _ => (),
                }
                Some(e)
            }
        }
    }

    //zz All done
}
