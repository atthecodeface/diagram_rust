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
use std::fmt;
use std::fmt::Write;
use super::utils;

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

//a Blah
pub trait CharReader {
    fn next_char(&mut self) -> CharResult;
    fn pos(&self) -> FilePosition;
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

//ip fmt::Display for Char
impl fmt::Display for Char {
    //mp fmt - format a character for display
    /// Display the character as either the character itself, or '<EOF>'
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Char::Eof    => write!(f, "<EOF>"),
            Char::NoData => write!(f, "<NoData>"),
            Char::Char(ref ch) => f.write_char(*ch),
        }
    }
}

//tp CharError
/// `CharError` represents an error from a UTF-8 character reader,
/// either an IO error from the reader or a malformed UTF-8 encoded
/// set of bytes
#[derive(Debug)]
pub enum CharError {
    /// An `IoError` is passed through from a reader as a `CharError`
    IoError(std::io::Error),
    /// A MalformedUtf8 error occurs when a byte stream contains
    /// invalid UTF-8; the Unicode-character position of the error is
    /// recorded, and the number of bytes that form the invalid UTF-8
    /// encoding (which will be from 1 to 3)
    MalformedUtf8(usize, FilePosition),
}

//ip From IO Error to CharError
/// Provides an implicit conversion from a std::io::Error to a CharError
impl From<std::io::Error> for CharError {
    fn from(e: std::io::Error) -> CharError {
        CharError::IoError(e)
    }
}

//ip fmt::Display for CharError
impl fmt::Display for CharError {
    //mp fmt - format a `CharError` for display
    /// Display the `CharError` in a human-readable form
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CharError::MalformedUtf8(n, pos) => write!(f, "malformed UTF-8 of {} bytes at {}", n, pos),
            CharError::IoError(ref e) => write!(f, "IO error: {}", e),
        }
    }
}

//tp CharResult
/// `CharResult` represents the result of fetching a character
pub type CharResult = std::result::Result<Char,CharError>;

//a FilePosition
//tp FilePosition
/// Holds the line number and character position of a character in a file
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FilePosition {
    /// Line, with first line being 0
    pub ln: usize,
    /// Column, starting with 0
    pub ch: usize,
}

//ip fmt::Display for FilePosition
impl fmt::Display for FilePosition {

    //mp fmt - format a `CharError` for display
    /// Display the `FilePosition` as line and column
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} column {}", self.ln+1, self.ch+1)
    }

    //zz All done
}

//ip FilePosition
impl FilePosition {

    //fp new - Create a new file position
    /// Create a new `FilePosition`, at line 0 character 0
    pub fn new() -> FilePosition {
        FilePosition { ln:0, ch:0 }
    }
    
    //mp move_by - Move the position on by a character
    /// Move the file position on by a character, accounting for newlines
    pub fn move_by(&mut self, ch:char) -> () {
        self.ch += 1;
        if utils::is_newline(ch as u32) {
            self.ln += 1;
            self.ch = 0;
        }
    }

    //zz All done
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
///     let str = "This is a \u{1f600} string\nWith a newline\n";
///     let mut buf_bytes = buf.as_bytes();
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
pub struct Reader<R:Read> {
    /// The reader from which data is to be fetched
    buf_reader : R,
    /// Position within the file of the next character to be decoded
    cursor     : FilePosition,
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
impl <R:Read> Reader<R> {

    //fp new
    /// Returns a new UTF-8 character reader, with a file position of the start of the file
    pub fn new (buf_reader: R) -> Reader<R> {
        Reader {
            buf_reader,
            cursor: FilePosition::new(),
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
    pub fn complete(&mut self) -> Result<bool, CharError> {
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

impl <R:Read> CharReader for Reader<R> {
    //mp pos
    /// Return the file position of the next character to be decoded from the stream
    fn pos(&self) -> FilePosition {
        self.cursor
    }

    //fm next_char
    /// Get the next character from the stream
    ///
    /// # Errors
    ///
    /// May return CharError::MalformedUtf8 if 
    fn next_char(&mut self) -> CharResult {
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
impl <R:Read> Iterator for Reader<R> {
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
    use super::*;
    #[test]
    fn test_reader() {
        let buf = "This is a \u{2764} string\nWith a newline\n\u{0065}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}";
        let char_list : Vec<char> = buf.chars().collect();
        let mut buf_bytes = buf.as_bytes();
        let reader = Reader::new(&mut buf_bytes);
        for (i,ch) in reader.enumerate() {
            assert_eq!(ch, char_list[i], "Mismatch in characters from string {} {}", char_list[i], ch );//, reader.pos(), );
        }
    }
}
