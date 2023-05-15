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
@brief   Utilities for parsing values and other things
 */

//a Imports
use regex::Regex;
use std::str::FromStr;

use crate::ValueError;

//a Helper functions and modules
//vi STRING_IS_NONE - regexp that is true if the string is only whitespace
const STRING_IS_NONE: &str = r"^\s*$";

//vi STRING_AS_FLOAT  - float with optional whitespace / comma in front of it and a 'rest' overflow
/// <whitespace> [, <whitespace>] [-] <0-9>+ [.<0-9>*] [.*]
const STRING_AS_FLOAT: &str = r"^\s*,?\s*(-?\d+\.?\d*)(.*)$";

//vi STRING_AS_INT - decimal or hex with optional whitespace / comma in front of it and a 'rest' overflow
const STRING_AS_INT: &str = r"^\s*,?\s*(0x[0-9a-fA-F]+|-?\d+)(.*)$";

//vi Static versions thereof
lazy_static! {
    static ref STRING_IS_NONE_REX: Regex = Regex::new(STRING_IS_NONE).unwrap();
    static ref STRING_AS_FLOAT_REX: Regex = Regex::new(STRING_AS_FLOAT).unwrap();
    static ref STRING_AS_INT_REX: Regex = Regex::new(STRING_AS_INT).unwrap();
}

//fi extract_first_and_rest
// only used in test at present
#[allow(dead_code)]
fn extract_first_and_rest<'a>(rex: &Regex, s: &'a str) -> Option<(&'a str, &'a str)> {
    rex.captures(s)
        .map(|caps| (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()))
}
#[warn(dead_code)]

//fi extract_vec_re_first_and_rest
fn extract_vec_first_and_rest<'a, R: FromStr>(
    rex: &Regex,
    max_len: usize,
    v: &'a mut Vec<R>,
    s: &'a str,
) -> Result<(usize, &'a str), ValueError> {
    if v.len() >= max_len {
        Ok((v.len(), s))
    } else {
        match rex.captures(s) {
            None => Ok((v.len(), s)),
            Some(caps) => match caps.get(1).unwrap().as_str().parse::<R>() {
                Ok(value) => {
                    v.push(value);
                    extract_vec_first_and_rest(rex, max_len, v, caps.get(2).unwrap().as_str())
                }
                _e => Ok((v.len(), s)),
            },
        }
    }
}

//fp parse_str_is_none
pub fn parse_str_is_none(s: &str) -> bool {
    STRING_IS_NONE_REX.is_match(s)
}

//fp parse_str_as_floats
/// Parse a string into a Vec<f64> with an optional required length
///
/// Error if the string cannot be parsed at all
///
/// Replicate the values given
///
/// Ignore any trailing data that cannot be parsed
pub fn parse_str_as_floats(s: &str, len: Option<usize>) -> Result<Vec<f64>, ValueError> {
    let mut v = Vec::new();
    let max_len = len.unwrap_or(10000);
    let (actual_len, _) = extract_vec_first_and_rest(&STRING_AS_FLOAT_REX, max_len, &mut v, s)?;
    match len {
        None => (),
        Some(len) => {
            if actual_len == 0 {
                v.push(0.0);
            }
            let mut i = 0;
            while v.len() < len {
                v.push(v[i]);
                i += 1;
            }
        }
    }
    Ok(v)
}

//fp parse_str_as_ints
/// Parse a string into a Vec<isize> with an optional required length
///
/// Error if the string cannot be parsed at all
///
/// Replicate the values given
///
/// Ignore any trailing data that cannot be parsed
pub fn parse_str_as_ints(s: &str, len: Option<usize>) -> Result<Vec<isize>, ValueError> {
    let mut v = Vec::new();
    let max_len = len.unwrap_or(10000);
    let (actual_len, _) = extract_vec_first_and_rest(&STRING_AS_INT_REX, max_len, &mut v, s)?;
    match len {
        None => (),
        Some(len) => {
            if actual_len == 0 {
                v.push(0);
            }
            let mut i = 0;
            while v.len() < len {
                v.push(v[i]);
                i += 1;
            }
        }
    }
    Ok(v)
}

//t Test regular expressions
#[cfg(test)]
mod test_res {
    use super::*;
    #[test]
    fn test_extract_ints() {
        let rex = Regex::new(STRING_AS_INT).unwrap();
        assert_eq!(extract_first_and_rest(&rex, "1 2 3"), Some(("1", " 2 3")));
        assert_eq!(
            extract_first_and_rest(&rex, "0x123 2 3"),
            Some(("0x123", " 2 3"))
        );
    }
    fn test_extract_vec<R: FromStr + std::fmt::Debug + PartialEq>(
        rex: &Regex,
        s: &str,
        max_len: usize,
        expected: Vec<R>,
        rest: &str,
    ) {
        let mut v = Vec::new();
        println!("Test string {}", s);
        assert_eq!(
            extract_vec_first_and_rest::<R>(rex, max_len, &mut v, s).unwrap(),
            (expected.len(), rest)
        );
        assert_eq!(v, expected);
    }
    #[test]
    fn test_extract_vec_int() {
        test_extract_vec::<isize>(&STRING_AS_INT_REX, "1 2 3", 10, vec![1, 2, 3], "");
        test_extract_vec::<isize>(&STRING_AS_INT_REX, "1 2 3", 1, vec![1], " 2 3");
        test_extract_vec::<usize>(&STRING_AS_INT_REX, "1 2 3", 10, vec![1, 2, 3], "");
        test_extract_vec::<usize>(&STRING_AS_INT_REX, "1 2 3", 1, vec![1], " 2 3");
        test_extract_vec::<isize>(&STRING_AS_INT_REX, "1 -2 3", 10, vec![1, -2, 3], "");
        test_extract_vec::<isize>(&STRING_AS_INT_REX, "1 -2 3", 1, vec![1], " -2 3");
        test_extract_vec::<usize>(&STRING_AS_INT_REX, "1 -2 3", 10, vec![1], " -2 3");
        test_extract_vec::<usize>(&STRING_AS_INT_REX, "1 -2 3", 1, vec![1], " -2 3");
    }
    #[test]
    fn test_extract_vec_float() {
        test_extract_vec::<f32>(
            &STRING_AS_FLOAT_REX,
            "1 -2 3.14 4.56",
            10,
            vec![1., -2., 3.14, 4.56],
            "",
        );
        test_extract_vec::<f64>(
            &STRING_AS_FLOAT_REX,
            "1 -2 3.14 4.56",
            1,
            vec![1.],
            " -2 3.14 4.56",
        );
    }
}
