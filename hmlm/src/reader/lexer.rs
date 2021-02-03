//! Contains simple lexer for HMLM documents.
//!
//! This module is for internal use.

use std::fmt;
use std::io::prelude::BufRead;
use super::char::*;

#[derive(Debug)]
pub enum TokenError {
    UnexpectedCharacter(char, FilePosition),
    UnexpectedEOF(FilePosition),
    MalformedUtf8(usize, FilePosition),
    IoError(std::io::Error),
}

impl From<CharError> for TokenError {
    fn from(e: CharError) -> TokenError {
        match e {
            CharError::IoError(e    )       => TokenError::IoError(e),
            CharError::MalformedUtf8(n,pos) => TokenError::MalformedUtf8(n,pos),
        }
    }
}

impl From<std::io::Error> for TokenError {
    fn from(e: std::io::Error) -> TokenError {
        TokenError::IoError(e)
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenError::UnexpectedEOF(pos) => write!(f, "unexpected EOF in token at {}", pos),
            TokenError::UnexpectedCharacter(ch, pos) => write!(f, "unexpected character {} at {}", ch, pos),
            TokenError::MalformedUtf8(n, pos) => write!(f, "malformed UTF-8 of {} bytes at {}", n, pos),
            TokenError::IoError(ref e) => write!(f, "IO error: {}", e),
        }
    }
}



/// `Token` represents a single item in an HMLH document
/// This will be an entity that effects the parse state of the parser
/// Hence it includes all of attr="string with spaces"
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct NamespaceName {
    namespace: Option<String>,
    name : String,
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token {
    /// ; stuff up to newline
    Comment(Vec<String>),
    /// ###<tag>[{] Tag open - with depth (number of #) and true if multiline
    TagOpen(NamespaceName, usize, bool),
    /// ###<tag>} Tag close - with depth (number of #)
    TagClose(NamespaceName, usize),
    /// attribute [<string>:]<string>=<quoted string>
    Attribute(NamespaceName, String),
    /// Quoted string - unquoted
    Characters(String),
    /// End of file
    EndOfFile,
}

/// `Lexer` is a lexer for HMLH documents
///
/// Main method is `next_token` which accepts an `std::io::Read` instance and
/// tries to read the next lexeme from it.
///
/// When `skip_errors` flag is set, invalid lexemes will be returned as `Chunk`s.
/// When it is not set, errors will be reported as `Err` objects with a string message.
/// By default this flag is not set. Use `enable_errors` and `disable_errors` methods
/// to toggle the behavior.
pub struct Lexer<'a, R:BufRead> {
    reader     : &'a mut Reader<R>,
    read_ahead : Option<Char>,
}

impl <'a, R:BufRead> Lexer<'a, R> {
    /// Returns a new lexer with default state.
    pub fn new<'b>(reader : &'b mut Reader<R>) -> Lexer<'b, R>  {
        Lexer {
            reader,
            read_ahead: None,
        }
    }

    /// Peek character
    pub fn peek_char(&mut self) -> CharResult {
        match self.read_ahead {
            Some(x) => {
                Ok(x)
            },
            None => {
                let ch = self.reader.next_char()?;
                self.read_ahead = Some(ch);
                Ok(ch)
            },
        }
    }

    /// Peek character - EOF not permitted
    pub fn peek_char_no_eof(&mut self) -> Result<char, TokenError> {
        match self.peek_char()? {
            Char::Eof      => { Err(self.unexpected_eof()) },
            Char::Char(ch) => { Ok(ch) },
        }
    }

    /// Get character
    pub fn get_char(&mut self) -> CharResult {
        match self.read_ahead {
            Some(x) => {
                self.read_ahead = None;
                Ok(x)
            }
            None => self.reader.next_char(),
        }
    }

    /// Get character - EOF not permitted
    pub fn get_char_no_eof(&mut self) -> Result<char, TokenError> {
        match self.get_char()? {
            Char::Eof      => { Err(self.unexpected_eof()) },
            Char::Char(ch) => { Ok(ch) },
        }
    }

    /// Unget character
    pub fn unget_char(&mut self, char:Char) -> () {
        self.read_ahead = Some(char);
    }
    
    /// Returns a new lexer with default state.
    pub fn skip_whitespace(&mut self) -> Result<(),CharError> {
        loop {
            let ch = self.get_char()?;
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
    
    /// Read line
    pub fn read_line(&mut self) -> Result<String,CharError> {
        let mut s = String::new();
        loop {
            let ch = self.get_char()?;
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

    /// Tries to read the next token from the buffer.
    ///
    /// It is possible to pass different instaces of `BufReader` each time
    /// this method is called, but the resulting behavior is undefined in this case.
    ///
    /// Return value:
    /// * `Err(reason) where reason: reader::Error` - when an error occurs;
    /// * `Ok(None)` - upon end of stream is reached;
    /// * `Ok(Some(token)) where token: Token` - in case a complete-token has been read from the stream.
    pub fn next_token(&mut self) -> Result<Token,TokenError> {
        println!("Next token");
        self.skip_whitespace()?;
        match self.peek_char()? {
            Char::Eof => Ok(Token::EndOfFile),
            Char::Char(ch) => {
                if is_semicolon(ch as u32) {
                    self.get_char()?; // drop the semicolon
                    let mut comment_strings = Vec::new();
                    loop {
                        comment_strings.push(self.read_line()?);
                        self.skip_whitespace()?;
                        match self.peek_char()? {
                            Char::Eof => {break;},
                            Char::Char(ch) => {
                                if !is_semicolon(ch as u32) {
                                    break;
                                }
                            },
                        }
                        self.get_char()?;
                    }
                    return Ok(Token::Comment(comment_strings));
                } else if is_hash(ch as u32) {
                    return self.read_tag();
                } else if is_single_quote(ch as u32) || is_double_quote(ch as u32) {
                    let s = self.read_quoted_string()?;
                    return Ok(Token::Characters(s));
                } else if is_name_start(ch as u32) {
                    return self.read_attribute();
                }
                return Err(TokenError::UnexpectedCharacter(ch, self.reader.pos()));
            }
        }
    }

    //mi unexpected_eof
    fn unexpected_eof(&self) -> TokenError {
        TokenError::UnexpectedEOF(self.reader.pos())
    }

    //mi unexpected_character
    fn unexpected_character(&self, ch:char) -> TokenError {
        TokenError::UnexpectedCharacter(ch, self.reader.pos())
    }

    //mi read_name - read a name, cursor should be pointing at a is_name_start character
    // at end, cursor pointing at first non-name character or EOF
    fn read_name(&mut self) -> Result<String,TokenError> {
        let mut s = String::new();
        let ch = self.get_char_no_eof()?;
        if !is_name_start(ch as u32) {
            return Err(self.unexpected_character(ch));
        }
        s.push(ch);
        loop {
            match self.get_char()? {
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
    fn read_namespace_name(&mut self) -> Result<NamespaceName,TokenError> {
        let name = self.read_name()?;
        match self.peek_char()? {
            Char::Char(':') => {
                self.get_char()?;
                let name2 = self.read_name()?;
                Ok(NamespaceName { namespace:Some(name),  name:name2 })
            },
            _ => {
                Ok(NamespaceName { namespace:None,  name:name })
            },
        }
    }

    //mi read_tag - read a tag given cursor is at first #
    // a tag is #+ <name> [ { | } ] <whitespace>
    fn read_tag(&mut self) -> Result<Token,TokenError> {
        let mut hash_count : usize =0;
        loop {
            let ch = self.peek_char_no_eof()?;
            if !is_hash(ch as u32) { break; }
            hash_count += 1;
            self.get_char()?;
        }
        let name = self.read_namespace_name()?;
        let result = {
            match self.peek_char()? {
                Char::Char('{') => {
                    self.get_char()?;
                    Token::TagOpen(name, hash_count, true)
                },
                Char::Char('}') => {
                    self.get_char()?;
                    Token::TagClose(name, hash_count)
                },
                _ => {
                    Token::TagOpen(name, hash_count, false)
                },
            }
        };
        match self.peek_char()? {
            Char::Eof => Ok(result),
            Char::Char(ch) => {
                if is_whitespace(ch as u32) { Ok(result) } else { Err(self.unexpected_character(ch)) }
            },
        }
    }

    //mi read_quoted_string
    // pointing at single or double quote
    pub fn read_quoted_string(&mut self) -> Result<String,TokenError> {
        let mut result = String::new();
        let ch = self.get_char_no_eof()?;
        let ch2 = self.get_char_no_eof()?;
        if ch==ch2 {
            match self.peek_char()? {
                Char::Char(ch3) => {
                    if ch3==ch2 {
                        self.get_char()?;
                        self.read_triple_quoted_string(ch)
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
                    return Err(self.unexpected_character(ch));
                }
                result.push(new_ch);
                new_ch = self.get_char_no_eof()?;
            }
            Ok(result)
        }
    }

    //mi read_triple_quoted_string
    // pointing at first character of contents (after the triple)
    fn read_triple_quoted_string(&mut self, quote_char:char) -> Result<String,TokenError> {
        let mut result = String::new();
        let mut num_quotes = 0;
        while num_quotes<3 {
            let ch = self.get_char_no_eof()?;
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
    fn read_attribute(&mut self) -> Result<Token,TokenError> {
        let name = self.read_namespace_name()?;
        let ch = self.get_char_no_eof()?;
        if ch!='=' {return Err(self.unexpected_character(ch)); }
        let value=self.read_quoted_string()?;
        Ok(Token::Attribute(name,value))
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
            if t.unwrap() == Token::EndOfFile {break;}
        }
        assert_eq!(true, false);
    }
}
