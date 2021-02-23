//a Documentation
//! This module provides a tokenizer for HMLH documents. HMLH documents are UTF-8 encoded.
//!
//! An HMLH token can be a tag, such as `###banana`, which may be
//! an explicit boxing open tag e.g. `##fruit{` or the equivalent close
//! tag `##fruit}`
//!
//! The token may be a comment - any HMLH line whose first non-whitespace character is a semicolon
//! makes the rest of the line after the semicolon a comment
//!
//! The token may be characters - a quoted string - which starts with either a single or double quote character.
//! Quoted strings using one quote character to delineate it, in which case the contents are escaped, and must not contain newlines
//! Alternatively quoted strings may start with three quote characters, in which case they can be boxed, and the terminate at the
//! next occurrence of the same three quote characters
//!
//! A token may be a attribute - which is of the form [<name_space>:]<name>=<quoted string>

//a Imports
use std::io::prelude::Read;
use crate::types::*;
use crate::utils::*;
use super::char::*;

//a LexerResult
type LexerResult<T, F> = Result<T,TokenError<F>>;

//a Token
//tp Token
/// `Token` represents a single item in an HMLH document
/// This will be an entity that effects the parse state of the parser
/// Hence it includes all of attr="string with spaces"
#[derive(Debug)]
pub enum Token {
    /// ; stuff up to newline
    Comment(Vec<String>),
    /// ###<tag>[{] Tag open - with depth (number of #) and true if boxed
    TagOpen(MarkupName, usize, bool),
    /// ###<tag>} Tag close - with depth (number of #)
    TagClose(MarkupName, usize),
    /// attribute [<string>:]<string>=<quoted string>
    Attribute(MarkupName, String),
    /// Quoted string - unquoted
    Characters(String),
    /// End of file
    EndOfFile,
}
impl Token {
    pub fn is_eof(&self) -> bool {
        match self {
            Self::EndOfFile => true,
            _ => false,
        }
    }
}

//tp TokenWithPos<F>
pub type TokenWithPos<F> = (F, F, Token);

//ip std::fmt::Display for Token
impl std::fmt::Display for Token {
    //mp fmt - format a `Token` for display
    /// Display the `Token` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::Comment(v) => write!(f, "<comment : {}...>", v[0]),
            Token::TagOpen(ns_name, depth, false) => write!(f, "<{} {}/>", depth, ns_name),
            Token::TagOpen(ns_name, depth, true)  => write!(f, "<{} {}>",  depth, ns_name),
            Token::TagClose(ns_name, depth)       => write!(f, "</{} {}>",  depth, ns_name),
            Token::Attribute(ns_name, value)      => write!(f, "<{}='{}'>",  ns_name, value),
            Token::Characters(value)              => write!(f, "[{}]",  value),
            Token::EndOfFile                      => write!(f, "<!EOF>"),
        }
    }
}

//a Lexer
/// `Lexer` is a tokenizer for HMLH documents
///
/// Main method is `next_token` which accepts an `Read` instance
///
//tp Lexer
pub struct Lexer <F:FilePosition> {
    read_ahead : Option<Char>,
    token_start: F,
}

impl <F:FilePosition> Lexer<F> {

    //fp new - 
    /// Returns a new lexer with default state.
    pub fn new() -> Self {
        Lexer {
            read_ahead: None,
            token_start: F::new(),
        }
    }

    //mi peek_char - peek at the next character
    /// Peek character
    fn peek_char <R:CharReader<F>> (&mut self, reader:&mut R) -> CharResult<F> {
        match self.read_ahead {
            Some(x) => {
                Ok(x)
            },
            None => {
                let ch = reader.next_char()?;
                self.read_ahead = Some(ch);
                Ok(ch)
            },
        }
    }

    //mi peek_char_no_eof - peek at the next character, with an error if it is EOF
    /// Peek character - EOF not permitted
    fn peek_char_no_eof<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<char, F> {
        match self.peek_char(reader)? {
            Char::Char(ch) => { Ok(ch) },
            _              => { Err(self.unexpected_eof(reader)) },
        }
    }

    //mi get_char - get the next character
    /// Get character
    fn get_char<R:CharReader<F>> (&mut self, reader:&mut R) -> CharResult<F> {
        match self.read_ahead {
            Some(x) => {
                self.read_ahead = None;
                Ok(x)
            }
            None => reader.next_char(),
        }
    }

    //mi get_char - get the next character, with an error if it is EOF
    /// Get character - EOF not permitted
    fn get_char_no_eof<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<char, F> {
        match self.get_char(reader)? {
            Char::Char(ch) => { Ok(ch) },
            _              => { Err(self.unexpected_eof(reader)) },
        }
    }

    //mi unget_char - return a character to the (single char) readahead buffer
    /// Unget a character - put it into the readahead
    fn unget_char(&mut self, char:Char) -> () {
        self.read_ahead = Some(char);
    }

    //mi skip_whitespace - get up to first non-whitespace character
    /// Read characters until EOF or non-whitespace
    /// If non-whitespace, then unget it back into the readahead
    fn skip_whitespace<R:CharReader<F>> (&mut self, reader:&mut R) -> Result<(),CharError<F>> {
        loop {
            let ch = self.get_char(reader)?;
            match ch {
                Char::Char(x) => {
                    if !is_whitespace(x as u32) {
                        self.unget_char(ch);
                        break;
                    }
                },
                _ => {
                    self.unget_char(ch);
                    break;
                }
            }
        }
        Ok(())
    }

    //mi read_line - read up to newline, for (e.g.) comments
    /// Read the string from the current char to a newline, leaving that out
    fn read_line<R:CharReader<F>> (&mut self, reader:&mut R) -> Result<String,CharError<F>> {
        let mut s = String::new();
        loop {
            let ch = self.get_char(reader)?;
            match ch {
                Char::Char(ch) => {
                    if is_newline(ch as u32) {
                       break;
                    }
                    s.push(ch);
                },
                _ => {
                    self.unget_char(ch);
                    break;
                }
            }
        }
        return Ok(s);        
    }

    //mp next_token
    /// Tries to read the next token from the buffer, returning an Ok(Token) on success
    ///
    /// # Errors
    ///
    /// Can return an IO error from the underlying stream, or a UTF-8 encoding error.
    ///
    /// Additionally it may return an error for characters that are illegal within the token stream
    pub fn next_token<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<Token, F> {
        self.skip_whitespace(reader)?;
        self.token_start = reader.pos();
        match self.peek_char(reader)? {
            Char::Char(ch) => {
                if is_semicolon(ch as u32) {
                    self.get_char(reader)?; // drop the semicolon
                    let mut comment_strings = Vec::new();
                    loop {
                        comment_strings.push(self.read_line(reader)?);
                        self.skip_whitespace(reader)?;
                        match self.peek_char(reader)? {
                            Char::Char(ch) => {
                                if !is_semicolon(ch as u32) {
                                    break;
                                }
                            },
                            _ => {break;},
                        }
                        self.get_char(reader)?;
                    }
                    return Ok(Token::Comment(comment_strings));
                } else if is_hash(ch as u32) {
                    return self.read_tag(reader);
                } else if is_single_quote(ch as u32) || is_double_quote(ch as u32) {
                    let s = self.read_quoted_string(reader)?;
                    return Ok(Token::Characters(s));
                } else if is_name_start(ch as u32) {
                    return self.read_attribute(reader);
                }
                return Err(self.unexpected_character(reader,ch));
            }
            _ => Ok(Token::EndOfFile),
        }
    }

    //mp next_token_with_pos
    /// Tries to read the next token from the buffer, returning an Ok(TokenWithPos) on success
    ///
    /// same as next_token, but returns the bounds of the token too, if not an error
    pub fn next_token_with_pos<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<TokenWithPos<F>, F> {
        let t = self.next_token(reader)?;
        Ok( (self.token_start, reader.pos(), t) )
    }

    //mi unexpected_eof
    fn unexpected_eof<R:CharReader<F>> (&self, reader:&R) -> TokenError<F> {
        TokenError::UnexpectedEOF(self.token_start, reader.pos())
    }

    //mi unexpected_character
    fn unexpected_character<R:CharReader<F>> (&self, reader:&R, ch:char) -> TokenError<F> {
        TokenError::UnexpectedCharacter(ch, self.token_start, reader.pos())
    }

    //mi read_name - read a name, cursor should be pointing at a is_name_start character
    // at end, cursor pointing at first non-name character or EOF
    fn read_name<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<String, F> {
        let mut s = String::new();
        let ch = self.get_char_no_eof(reader)?;
        if !is_name_start(ch as u32) {
            return Err(self.unexpected_character(reader, ch));
        }
        s.push(ch);
        loop {
            match self.get_char(reader)? {
                Char::Char(ch) => {
                    if !is_name(ch as u32) {
                        self.unget_char(Char::Char(ch));
                        break;
                    }
                    s.push(ch);
                },
                ch => {
                    self.unget_char(ch);
                    break;
                }
            }
        }
        return Ok(s);        
    }

    //mi read_namespace_name
    // pointing at first character of name
    fn read_namespace_name<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<MarkupName, F> {
        let name = self.read_name(reader)?;
        match self.peek_char(reader)? {
            Char::Char(':') => {
                self.get_char(reader)?;
                let name2 = self.read_name(reader)?;
                Ok(MarkupName::new(Some(name), name2))
            },
            _ => {
                Ok(MarkupName::new(None, name))
            },
        }
    }

    //mi read_tag - read a tag given cursor is at first #
    /// the stream cursor points at the first # in the tag,
    /// and this method reads the tag from that point
    ///
    /// a tag is #+ <namespace_name> [ { | } ] <whitespace>
    ///
    /// The result is a TagOpen or TagClose, with the depth set to the number of '#'s
    /// at the front of the tag, and the namespace_name set appropriately
    fn read_tag<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<Token, F> {
        let mut hash_count : usize =0;
        loop {
            let ch = self.peek_char_no_eof(reader)?;
            if !is_hash(ch as u32) { break; }
            hash_count += 1;
            self.get_char(reader)?;
        }
        let name = self.read_namespace_name(reader)?;
        let result = {
            match self.peek_char(reader)? {
                Char::Char('{') => {
                    self.get_char(reader)?;
                    Token::TagOpen(name, hash_count, true)
                },
                Char::Char('}') => {
                    self.get_char(reader)?;
                    Token::TagClose(name, hash_count)
                },
                _ => {
                    Token::TagOpen(name, hash_count, false)
                },
            }
        };
        match self.peek_char(reader)? {
            Char::Char(ch) => {
                if is_whitespace(ch as u32) { Ok(result) } else { Err(self.unexpected_character(reader, ch)) }
            },
            _ => Ok(result),
        }
    }

    //mi read_string
    /// Reads a string, possibly a quoted string, given the stream cursor is pointing at the opening character.
    ///
    /// The string must start with a quote character or a different non-whitespace character
    /// If it starts with a non-whitespace character then the string goes up to EOF or or whitespace
    /// If it starts with a quote character then it is a quoted string
    pub fn read_string<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<String, F> {
        let ch = self.peek_char_no_eof(reader)?;
        if is_quote(ch as u32) {
            self.read_quoted_string(reader)
        } else {
            let mut result = String::new();
            loop {
                let ch = self.get_char(reader)?;
                match ch {
                    Char::Char(c) => {
                        if is_whitespace(c as u32) {
                            self.unget_char(ch);
                            break;
                        } else {
                            result.push(c);
                        }
                    }
                    _      => {
                        self.unget_char(ch);
                        break;
                    },
                }
            }
            Ok(result)
        }
    }

    //mi read_quoted_string
    /// reads a quoted string, given the stream cursor is pointing at the opening quote character
    /// an empty quoted string is two identical quote characters then a different character (or EOF)
    /// a triple quoted string starts with three identical quote characters and continues (including newlines)
    /// until the next three identical quote characters
    /// otherwise it is a single quoted string, which should handle escapes (only \\ => \, \" => ", \' => ', \n => newline?)
    pub fn read_quoted_string<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<String, F> {
        let mut result = String::new();
        let ch = self.get_char_no_eof(reader)?;
        let ch2 = self.get_char_no_eof(reader)?;
        if ch==ch2 {
            match self.peek_char(reader)? {
                Char::Char(ch3) => {
                    if ch3==ch2 {
                        self.get_char(reader)?;
                        self.read_triple_quoted_string(reader,ch)
                    } else {
                        Ok(result) // empty string
                    }
                },
                _ => {
                    Ok(result) // empty string
                },
            }
        } else { // build single quoted string - no newlines permitted, copy raw up to next 'ch' character
            let mut new_ch = ch2;
            while new_ch != ch {
                if is_newline(ch as u32) {
                    return Err(self.unexpected_character(reader, ch));
                }
                result.push(new_ch);
                new_ch = self.get_char_no_eof(reader)?;
            }
            Ok(result)
        }
    }

    //mi read_triple_quoted_string
    /// read a triple quoted string, with the stream cursor pointing
    /// at first character of contents (after the triple quote) keeps
    /// reading characters and pushing them until the three
    /// consecutive quote characters are seen.
    fn read_triple_quoted_string<R:CharReader<F>> (&mut self, reader:&mut R, quote_char:char) -> LexerResult<String, F> {
        let mut result = String::new();
        let mut num_quotes = 0;
        while num_quotes<3 {
            let ch = self.get_char_no_eof(reader)?;
            if ch==quote_char {
                num_quotes += 1;
            } else if num_quotes>0 {
                for _ in 0..num_quotes {
                    result.push(quote_char);
                }
                num_quotes = 0;
                result.push(ch);
            } else {
                result.push(ch);
            }
        }
        Ok(result)
    }

    //mi read_attribute
    // pointing at first character of attribute
    fn read_attribute<R:CharReader<F>> (&mut self, reader:&mut R) -> LexerResult<Token, F> {
        let name = self.read_namespace_name(reader)?;
        let ch = self.get_char_no_eof(reader)?;
        if !is_equals(ch as u32) {return Err(self.unexpected_character(reader, ch)); }
        let value=self.read_string(reader)?;
        Ok(Token::Attribute(name,value))
    }

    //zz All done
}

//a LexerOfReader
/// `Lexer` is a tokenizer for HMLH documents
///
/// Main method is `next_token` which accepts an `Read` instance
///
//tp LexerOfReader
pub struct LexerOfReader<'a, R:Read> {
    reader     : &'a mut Reader<R, HmlFilePosition>,
    lexer      : Lexer<HmlFilePosition>,
}

impl <'a, R:Read> LexerOfReader<'a, R> {

    //fp new - 
    /// Returns a new lexer with default state.
    pub fn new<'b>(reader : &'b mut Reader<R, HmlFilePosition>) -> LexerOfReader<'b, R>  {
        LexerOfReader {
            reader,
            lexer: Lexer::new(),
        }
    }

    //mp next_token
    /// Tries to read the next token from the buffer, returning an Ok(Token) on success
    ///
    /// # Errors
    ///
    /// Can return an IO error from the underlying stream, or a UTF-8 encoding error.
    ///
    /// Additionally it may return an error for characters that are illegal within the token stream
    pub fn next_token(&mut self) -> LexerResult<Token, HmlFilePosition> {
        self.lexer.next_token(self.reader)
    }

    //mp next_token_with_pos
    /// Tries to read the next token from the buffer, returning an Ok(TokenWithPos) on success
    ///
    /// same as next_token, but returns the bounds of the token too, if not an error
    pub fn next_token_with_pos(&mut self) -> LexerResult<TokenWithPos<HmlFilePosition>, HmlFilePosition> {
        self.lexer.next_token_with_pos(self.reader)
    }

    //zz All done
}

//a Test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blah() {
        let mut buf = "; This is a comment\n   ; with more comment\n #banana r='2' \"\"\"Stuff \"\"  and more \"\"\"".as_bytes();
        let mut reader = Reader::new(&mut buf);
        let mut lexer  = LexerOfReader::new(&mut reader);
        loop {
            let t = lexer.next_token();
            assert_eq!( t.is_err(), false, "T should not be an error");
            println!("{:?}", t);
            if t.unwrap().is_eof() {break;}
        }
        // assert_eq!(true, false);
    }
}
