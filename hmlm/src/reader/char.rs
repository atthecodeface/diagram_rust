//a Documentation
//! The `char` module provides a streaming char reader struct that
//! converts a `Read` stream into a stream of `Char` values.
//!
//! If the `Read` stream comes from a file then this is just a
//! streaming version of (e.g.) std::fs::read_to_string, but if the
//! `Read` stream comes from, e.g., a std::net::TcpStream then it has
//! more value: the `Char` value returned can indicate that the stream
//! currently has no data valid, but the reader can continue when it
//! does.

//a Imports
use std::io::prelude::Read;
use crate::types::{Char, CharResult, CharError, FilePosition};

//a Constants
/// `BUFFER_SIZE` is the maximum number of bytes held in the UTF-8
/// character reader from the incoming stream.  The larger the value,
/// the larger the data read requests from the stream. This value must be larger than `BUFFER_SLACK`.
/// For testing purposes this value should be small (such as 8), to catch corner cases in the code where UTF-8 encodings
/// run over the end of a buffer; for performance, this value should be larger (e.g. 2048).
const BUFFER_SIZE  : usize = 2048;
/// `BUFFER_SLACK` must be at least 4 - the maximum number of bytes in
/// a UTF-8 encoding; when fewer than BUFFER_SLACK bytes are in the
/// buffer a read from the buffer stream is performed - attempting to
/// fill the `BUFFER_SIZE` buffer with current data and new read data.
/// There is no reason why `BUFFER_SLACK` should be larger than 4.
const BUFFER_SLACK : usize = 4;

//a CharReader trait
pub trait CharReader<F:FilePosition> {
    fn next_char(&mut self) -> CharResult<F>;
    fn pos(&self) -> F;
}

//a Reader
//tp Reader
/// `Reader` provides a stream of characters by UTF-8 decoding a byte
/// stream provided by something that implements the `Read` trait.
/// It utilizes an internal buffer of bytes that are filled as
/// required from the `Read` object; it maintains a position with
/// the stream (line and character) for the next character, and
/// provides the ability to get a stream of characters from `Read`
/// object with any UTF-8 encoding errors reported by line and
/// character.
/// 
/// If simple files are to be read, using std::fs::read_to_string is
/// a better approach than using the `Reader`
///
/// # Example
///
/// ```
///     extern crate hmlm;
///     use hmlm::reader::char::Reader;    
///     let str = "This is a \u{1f600} string\nWith a newline\n";
///     let mut buf_bytes = str.as_bytes();
///     let reader    = Reader::new(&mut buf_bytes);
///     for x in reader {
///         // use char x
///     }
/// ```
///
/// This example could just as easily use 'for x in str'
///
/// The `Reader`, though, can be used over any object supporting `Read` such
/// as a std::net::TcpStream
///
pub struct Reader<R:Read, F:FilePosition> {
    /// The reader from which data is to be fetched
    buf_reader : R,
    /// Position within the file of the next character to be decoded
    cursor     : F,
    /// `eof_on_no_data` defaults to true; it can be set to false to indicate that
    /// if the stream has no data then the reader should return Char::NoData
    /// when its buffer does not contain a complete UTF-8 character
    eof_on_no_data : bool,
    /// `eof` is set when the stream is complete - any character
    /// requested once `eof` is asserted will be `Char::Eof`.
    eof        : bool,
    /// Internal buffer
    current    : [u8; BUFFER_SIZE],
    /// `start` is first byte within the internal buffer that is valid
    start      : usize,
    /// `end` is last byte + 1 within the internal buffer that is valid
    end        : usize,
    /// `valid_end` is the last byte + 1 within the internal buffer
    /// used by a valid UTF-8 byte stream that begins with `start` As
    /// such `start` <= `valid_end` <= `end` If `start` < `valid_end`
    /// then the bytes in the buffer between the two are a valid UTF-8
    /// byte stream; this should perhaps be kept in a string inside
    /// the structure for performance
    valid_end  : usize,
}

//ip Reader
impl <R:Read, F:FilePosition> Reader<R, F> {

    //fp new
    /// Returns a new UTF-8 character reader, with a file position of the start of the file
    pub fn new (buf_reader: R) -> Reader<R, F> {
        Reader {
            buf_reader,
            cursor: F::new(),
            eof_on_no_data : true,
            eof:    false,
            current : [0; BUFFER_SIZE],
            start     : 0,
            end       : 0,
            valid_end : 0,
        }
    }

    //mp complete
    /// Return `true` if the reader has consumed all the data available on the stream
    pub fn complete(&mut self) -> Result<bool, CharError<F>> {
        if self.eof {
            Ok(true)
        } else if self.start!=self.end {
            Ok(false)
        } else {
            Ok(self.fetch_input()? == 0)
        }
    }
    
    //fi fetch_input
    /// Fetch input from the `Read` into the internal buffer,
    /// moving valid data to the start of the buffer first if
    /// required.  This method should only be invoked if more data is
    /// required; it is relatively code-heavy
    /// 
    /// To reduce the frequency
    fn fetch_input(&mut self) -> Result<usize, std::io::Error> {
        if self.start>BUFFER_SIZE-BUFFER_SLACK {
            // Move everything down by self.start
            let n = self.end - self.start;
            if n>0 {
                for i in 0..n {
                    self.current[i] = self.current[self.start+i];
                }
            }
            self.valid_end -= self.start;
            self.start      = 0; // == self.start - self.start
            self.end        = n; // == self.end   - self.start
        }
        let n = self.buf_reader.read( &mut self.current[self.end..BUFFER_SIZE] )?;
        self.end += n;
        if n==0 && self.eof_on_no_data {
            self.eof = true;
        }
        Ok(n)
    }

    //fm buffered_data
    /// Get the unconsumed buffered data - useful if the Reader has completed its task,
    /// and the `Read` stream is to be used for something further. This call returns the
    /// buffered data that has already been read from the `Read` stream, and which should be
    /// consumed before any more `read` calls to the stream.
    pub fn buffered_data(&self) -> &[u8] {
        &self.current[self.start..self.end]
    }

    //zz All done
}

impl <R:Read, F:FilePosition> CharReader<F> for Reader<R, F> {
    //mp pos
    /// Return the file position of the next character to be decoded from the stream
    fn pos(&self) -> F {
        self.cursor
    }

    //fm next_char
    /// Get the next character from the stream
    ///
    /// # Errors
    ///
    /// May return CharError::MalformedUtf8 if 
    fn next_char(&mut self) -> CharResult<F> {
        if self.eof {
            Ok(Char::Eof)
        } else if self.start == self.end { // no data present, try reading data
            self.fetch_input()?;
            self.next_char()
        } else if self.start < self.valid_end { // there is valid UTF-8 data at buffer+self.start
            let s = std::str::from_utf8(&self.current[self.start..self.valid_end]).unwrap();
            let mut chars = s.chars();
            let ch = chars.next().unwrap();
            let n = s.len() - chars.as_str().len();
            self.start += n;
            self.cursor.move_by(ch);
            Ok(Char::Char(ch))
        } else { // there is data but it may or may not be valid
            match std::str::from_utf8(&self.current[self.start..self.end]) {
                Ok(_) => { // the data is valid, mark it and the return from there
                    self.valid_end = self.end;
                    self.next_char()
                }
                Err(e) => { // the data is not all valid
                    if e.valid_up_to()>0 { // some bytes form valid UTF-8 - mark them and return that data
                        self.valid_end = self.start+e.valid_up_to();
                        self.next_char()
                    } else { // no valid data - check it is just incomplete, or an actual error
                        match e.error_len() {
                            None => { // incomplete UTF-8 fetch more
                                match self.fetch_input()? {
                                    0 => { // ... and eof reached when incomplete UTF8 is present
                                        if self.eof {
                                            Err(CharError::MalformedUtf8(self.end-self.start, self.cursor))
                                        } else {
                                            Ok(Char::NoData)
                                        }
                                    }
                                    _ => { // ... but got more data so try that!
                                        self.next_char()
                                    }
                                }
                            }
                            Some(n) => { // Bad UTF-8 with n bytes used
                                Err(CharError::MalformedUtf8(n, self.cursor))
                            },
                        }
                    }
                },
            }
        }
    }
}

//ip Iterator for Reader - iterate over characters
impl <R:Read, F:FilePosition> Iterator for Reader<R, F> {
    // we will be counting with usize
    type Item = char;

    //mp next - return next character or None if end of file
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_char() {
            Ok(Char::Char(ch)) => Some(ch),
            Ok(Char::Eof) => None,
            _ => {panic!("Error in stream");},
        }
    }

    //zz All done
}

//a Test
#[cfg(test)]
mod tests {
    use crate::types::*;
    use super::*;
    #[test]
    fn test_reader() {
        let buf = "This is a \u{2764} string\nWith a newline\n\u{0065}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}";
        let char_list : Vec<char> = buf.chars().collect();
        let mut buf_bytes = buf.as_bytes();
        let reader = Reader::<_, HmlFilePosition>::new(&mut buf_bytes);
        for (i,ch) in reader.enumerate() {
            assert_eq!(ch, char_list[i], "Mismatch in characters from string {} {}", char_list[i], ch );//, reader.pos(), );
        }
    }
}
