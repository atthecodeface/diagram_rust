//a Documentation
/// 

//a Imports
use std::io::prelude::BufRead;
use std::fmt;
use std::fmt::Write;

//a Constants
/// `BUFFER_SIZE` is the maximum number of bytes held in the UTF-8
/// character reader from the incoming stream.  The larger the value,
/// the larger the data read requests from the stream. This value must be larger than `BUFFER_SLACK`.
/// For testing purposes this value should be small (such as 8), to catch corner cases in the code where UTF-8 encodings
/// run over the end of a buffer; for performance, this value should be larger (e.g. 256).
const BUFFER_SIZE  : usize = 256;
/// `BUFFER_SLACK` must be at least 4 - the maximum number of bytes in
/// a UTF-8 encoding; when fewer than BUFFER_SLACK bytes are in the
/// buffer a read from the buffer stream is performed - attempting to
/// fill the `BUFFER_SIZE` buffer with current data and new read data.
/// There is no reason why `BUFFER_SLACK` should be larger than 4.
const BUFFER_SLACK : usize = 4;

//a Character result and error
//tp Char
/// `Char` represents a unicode character or EOF marker
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Char {
    /// Eof indicates end of stream/file reached; once a reader returns Eof, it should continue to do so
    Eof,
    /// Char indicates a valid Unicode character
    Char(char)
}

//ip fmt::Display for Char
impl fmt::Display for Char {
    //mp fmt - format a character for display
    /// Display the character as either the character itself, or '<EOF>'
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Char::Eof => write!(f, "<EOF>"),
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

//a Character functions - for HMLH
pub fn is_newline(ch:u32) -> bool {(ch==10) || (ch==133)}
pub fn is_whitespace(ch:u32) -> bool {
    (ch==9)  || (ch==10) || (ch==11) ||
    (ch==12) || (ch==13) || (ch==32) ||
    (ch==133) || (ch==160)
}

pub fn is_digit(ch:u32) -> bool { (ch>=48) && (ch<=57) }

pub fn is_semicolon(ch:u32) -> bool { ch==59 }

pub fn is_hash(ch:u32) -> bool { ch==35 }

pub fn is_equals(ch:u32) -> bool { ch==61 }

pub fn is_single_quote(ch:u32) -> bool { ch==39 }
pub fn is_double_quote(ch:u32) -> bool { ch==34 }

pub fn is_name_start(ch:u32) -> bool {
    match ch {
        58 => {true}, // colon
        95 => {true}, // underscore
        _  => { ((ch>=65) && (ch<=90))       ||    // A-Z
                    ((ch>=97) && (ch<=122))     ||   // a-z 
                    ((ch>=0xc0) && (ch<=0xd6)) ||
                    ((ch>=0xd8) && (ch<=0xf6)) ||
                    ((ch>=0xf8) && (ch<=0x2ff)) ||
                    ((ch>=0x370) && (ch<=0x37d)) ||
                    ((ch>=0x37f) && (ch<=0x1fff)) ||
                    ((ch>=0x200c) && (ch<=0x200d)) ||
                    ((ch>=0x2070) && (ch<=0x218f)) ||
                    ((ch>=0x2c00) && (ch<=0x2fef)) ||
                    ((ch>=0x3001) && (ch<=0xd7ff)) ||
                    ((ch>=0xf900) && (ch<=0xfdcf)) ||
                    ((ch>=0xfdf0) && (ch<=0xfffd)) ||
                    ((ch>=0x10000) && (ch<=0xeffff))  }
    }
}

pub fn is_name(ch:u32) -> bool {
  is_name_start(ch) || (
      ((ch==45) || (ch==46) || (ch==0xb7)) || // - .
          ((ch>=48) && (ch<=57)) || // 0-9
          ((ch>=0x399) && (ch<=0x36f)) ||
          ((ch>=0x203f) && (ch<=0x2040)) )
}

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
        if is_newline(ch as u32) {
            self.ln += 1;
            self.ch = 0;
        }
    }

    //zz All done
}

//a Reader
//tp Reader
/// `Reader` provides a stream of characters by UTF-8 decoding a byte
/// stream provided by something that implements the `BufRead` trait.
/// It utilizes an internal buffer of bytes that are filled as
/// required from the `BufRead` object; it maintains a position with
/// the stream (line and character) for the next character, and
/// provides the ability to get a stream of characters from `BufRead`
/// object with any UTF-8 encoding errors reported by line and
/// character.
/// 
/// The `BufRead` object is borrowed for the lifetime of the `Reader`;
/// as the `Reader` maintains its internal read-ahead buffer it does
/// not make sense to permit the `BufRead` object be used until the
/// `Reader` completes.
///
/// Actually it should probably not borrow the reader, but consume it.
///
/// If simple files are to be read, using std::fs::read_to_string is
/// a better approach than using the `Reader`
///
/// # Example
///
/// ```
///     let str = "This is a \u{1f600} string\nWith a newline\n";
///     let mut buf_bytes = buf.as_bytes();
///     let mut reader    = Reader::new(&mut buf_bytes);
///     for x in reader {
/// ```
pub struct Reader<'a, R:BufRead> {
    /// The reader from which data is to be fetched
    buf_reader : &'a mut R,
    /// Position within the file of the next character to be decoded
    cursor     : FilePosition,
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
impl <'a, R:BufRead> Reader<'a, R> {

    //fp new
    /// Returns a new UTF-8 character reader, with a file position of the start of the file
    pub fn new<'b> (buf_reader : &'b mut R) -> Reader<'b, R> {
        Reader {
            buf_reader,
            cursor: FilePosition::new(),
            eof:    false,
            current : [0; BUFFER_SIZE],
            start     : 0,
            end       : 0,
            valid_end : 0,
        }
    }

    //mp pos
    /// Return the file position of the next character to be decoded from the stream
    pub fn pos(&self) -> FilePosition {
        self.cursor
    }
    
    //fp fetch_input
    /// Fetch input from the `BufRead` into the internal buffer,
    /// moving valid data to the start of the buffer first if
    /// required.  This method should only be invoked if more data is
    /// required; it is relatively code-heavy
    /// 
    /// To reduce the frequency
    fn fetch_input(&mut self) -> Result<usize, std::io::Error> {
        match self.buf_reader.fill_buf() {
            Err(e) => { Err(e) },
            Ok(b) => {
                let bl = b.len();
                if bl==0 {
                    self.eof = true;
                    Ok(0)
                } else {
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
                    let room = BUFFER_SIZE - self.end;
                    let n = {if bl>room {room} else {bl}};
                    for i in 0..n {
                        self.current[self.end+i] = b[i];
                    }
                    self.buf_reader.consume(n);
                    self.end += n;
                    Ok(n)
                }
            }
        }
    }

    //fm next_char
    /// Get the next character
    pub fn next_char(&mut self) -> CharResult {
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
                                        Err(CharError::MalformedUtf8(self.end-self.start, self.cursor))
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
    //zz All done
}

//ip Iterator for Reader
impl <'a, R:BufRead> Iterator for Reader<'a, R> {
    // we will be counting with usize
    type Item = char;

    //mp next - return next character or None if end of file
    fn next(&mut self) -> Option<Self::Item> {
        match self.next_char() {
            Ok(Char::Eof) => None,
            Ok(Char::Char(ch)) => Some(ch),
            _ => {panic!("Error in stream");},
        }
    }

    //zz All done
}

//a Test
#[cfg(test)]
const TEST_CHARS : [(u32,u32);15] = [ (10, 0b_00_011),
                                    (133, 0b_00_011),
                                     (' ' as u32,  0b_00000_00_010),
                                     ('0' as u32,  0b_00000_10_100),
                                     ('9' as u32,  0b_00000_10_100),
                                     ('A' as u32,  0b_00000_11_000),
                                     ('Z' as u32,  0b_00000_11_000),
                                     ('a' as u32,  0b_00000_11_000),
                                     ('z' as u32,  0b_00000_11_000),
                                     ('_' as u32,  0b_00000_11_000),
                                     ('=' as u32,  0b_00100_00_000),
                                     (';' as u32,  0b_00001_00_000),
                                     ('#' as u32,  0b_00010_00_000),
                                     ('"' as u32,  0b_10000_00_000),
                                     ('\'' as u32, 0b_01000_00_000),
                                        ];
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chars() {
        for (ch, mask) in TEST_CHARS.iter() {
            assert_eq!( ((mask>>0)&1) == 1, is_newline(*ch)     , "is_newline {} {}", ch, std::char::from_u32(*ch).unwrap()     );
            assert_eq!( ((mask>>1)&1) == 1, is_whitespace(*ch)  , "is_whitespace {} {}", ch, std::char::from_u32(*ch).unwrap()  );
            assert_eq!( ((mask>>2)&1) == 1, is_digit(*ch)       , "is_digit {} {}", ch, std::char::from_u32(*ch).unwrap()       );
            assert_eq!( ((mask>>3)&1) == 1, is_name_start(*ch)  , "is_name_start {} {}", ch, std::char::from_u32(*ch).unwrap()  );
            assert_eq!( ((mask>>4)&1) == 1, is_name(*ch)        , "is_name {} {}", ch, std::char::from_u32(*ch).unwrap()        );
            assert_eq!( ((mask>>5)&1) == 1, is_semicolon(*ch)   , "is_semicolon {} {}", ch, std::char::from_u32(*ch).unwrap()   );
            assert_eq!( ((mask>>6)&1) == 1, is_hash(*ch)        , "is_hash {} {}", ch, std::char::from_u32(*ch).unwrap()        );
            assert_eq!( ((mask>>7)&1) == 1, is_equals(*ch)      , "is_equals {} {}", ch, std::char::from_u32(*ch).unwrap()      );
            assert_eq!( ((mask>>8)&1) == 1, is_single_quote(*ch), "is_single_quote {} {}", ch, std::char::from_u32(*ch).unwrap());
            assert_eq!( ((mask>>9)&1) == 1, is_double_quote(*ch), "is_double_quote {} {}", ch, std::char::from_u32(*ch).unwrap());
        }
    }
    #[test]
    fn test_reader() {
        let buf = "This is a \u{2764} string\nWith a newline\n\u{0065}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}\u{1f600}";
        let char_list : Vec<char> = buf.chars().collect();
        let mut buf_bytes = buf.as_bytes();
        let mut reader = Reader::new(&mut buf_bytes);
        for (i,ch) in reader.enumerate() {
            assert_eq!(ch, char_list[i], "Mismatch in characters from string {} {}", char_list[i], ch );//, reader.pos(), );
        }
    }
}
