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

@file    utils.rs
@brief   Utility functions for HML reader
 */

//a Imports

//a Character functions - for HMLH
//fp is_newline
/// Returns true if the UTF character is a newline
#[inline]
pub fn is_newline(ch:u32) -> bool {(ch==10) || (ch==133)}

//fp is_whitespace
/// Returns true if the UTF character is whitespace
///
/// Whitespace is tab, cr, lf, ff, space, 
#[inline]
pub fn is_whitespace(ch:u32) -> bool {
    (ch==9)  || (ch==10) || (ch==11) ||
    (ch==12) || (ch==13) || (ch==32) ||
    (ch==133) || (ch==160)
}

//fp is_digit
/// Returns true if the UTF character is a numeric digit
#[inline]
pub fn is_digit(ch:u32) -> bool { (ch>=48) && (ch<=57) }

//fp is_semicolon
/// Returns true if the UTF character is a semicolon
#[inline]
pub fn is_semicolon(ch:u32) -> bool { ch==59 }

//fp is_newline
/// Returns true if the UTF character is a '#' character
#[inline]
pub fn is_hash(ch:u32) -> bool { ch==35 }

//fp is_equals
/// Returns true if the UTF character is an 'equals' character
#[inline]
pub fn is_equals(ch:u32) -> bool { ch==61 }

//fp is_single_quote
/// Returns true if the UTF character is a single quote '
#[inline]
pub fn is_single_quote(ch:u32) -> bool { ch==39 }

//fp is_double_quote
/// Returns true if the UTF character is a double quote "
#[inline]
pub fn is_double_quote(ch:u32) -> bool { ch==34 }

//fp is_quote
/// Returns true if the UTF character is either a single or double quote
#[inline]
pub fn is_quote(ch:u32) -> bool { ch==34 || ch==39 }

//fp is_name_start
/// Returns true if the UTF character is a colon, underscore, alphabetic, or UTF name character
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

//fp is_name
/// Returns true if the UTF character is a name character or a
/// continuation of a name character that adds -, ., digits, and other
/// UTF characters
pub fn is_name(ch:u32) -> bool {
  is_name_start(ch) || (
      ((ch==45) || (ch==46) || (ch==0xb7)) || // - .
          ((ch>=48) && (ch<=57)) || // 0-9
          ((ch>=0x399) && (ch<=0x36f)) ||
          ((ch>=0x203f) && (ch<=0x2040)) )
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
}
