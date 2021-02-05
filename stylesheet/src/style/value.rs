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

@file    value.rs
@brief   Style type and values
 */

//a Imports
use std::str::FromStr;
use std::fmt::Debug;
use regex::Regex;
use super::color;

//a Helper functions and modules 
//vi STRING_IS_NONE - regexp that is true if the string is only whitespace
const STRING_IS_NONE : &str = r"^\s*$";

//vi STRING_AS_FLOAT  - float with optional whitespace / comma in front of it and a 'rest' overflow
/// <whitespace> [, <whitespace>] [-] <0-9>+ [.<0-9>*] [.*]
const STRING_AS_FLOAT : &str = r"^\s*,?\s*(-?\d+\.?\d*)(.*)$";

//vi STRING_AS_INT - decimal or hex with optional whitespace / comma in front of it and a 'rest' overflow
const STRING_AS_INT : &str = r"^\s*,?\s*(0x[0-9a-fA-F]+|\d+)(.*)$";

//vi Static versions thereof
lazy_static!{
    static ref STRING_IS_NONE_REX:  Regex = Regex::new(STRING_IS_NONE).unwrap();
    static ref STRING_AS_FLOAT_REX: Regex = Regex::new(STRING_AS_FLOAT).unwrap();
    static ref STRING_AS_INT_REX:   Regex = Regex::new(STRING_AS_INT).unwrap();
}

//fi extract_first_and_rest
fn extract_first_and_rest<'a> (rex:&Regex, s:&'a str) -> Option<(&'a str, &'a str)> {
    match rex.captures(s) {
        None => None,
        Some(caps) => Some( (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()) )
    }
}

//fi extract_vec_re_first_and_rest
fn extract_vec_first_and_rest<'a, R:FromStr> (rex:&Regex, max_len:usize, v:&'a mut Vec<R>, s:&'a str) -> Result<(usize, &'a str), ValueError> {
    if v.len()>=max_len {
        Ok((v.len(), s))
    } else {
        match rex.captures(s) {
            None => Ok((v.len(), s)),
            Some(caps) => {
                match caps.get(1).unwrap().as_str().parse::<R>() {
                    Ok(value) => {
                        v.push(value);
                        extract_vec_first_and_rest(rex, max_len, v, caps.get(2).unwrap().as_str())
                    },
                    _e => Ok((v.len(), s)),
                }
            }
        }
    }
}
fn parse_str_as_floats(s:&str, len:Option<usize>) -> Result<Vec<f64>, ValueError> {
    let mut v = Vec::new();
    let max_len = len.unwrap_or(10000);
    let (actual_len, _) = extract_vec_first_and_rest(&STRING_AS_FLOAT_REX, max_len, &mut v, s)?;
    match len {
        None => (),
        Some(len) => {
            if actual_len==0 {v.push(0.0);}
            let mut i=0;
            while v.len()<len {
                v.push(v[i]);
                i+=1;
            }
        }
    }
    Ok(v)
}
fn parse_str_as_ints(s:&str, len:Option<usize>) -> Result<Vec<isize>, ValueError> {
    let mut v = Vec::new();
    let max_len = len.unwrap_or(10000);
    let (actual_len, _) = extract_vec_first_and_rest(&STRING_AS_INT_REX, max_len, &mut v, s)?;
    match len {
        None => (),
        Some(len) => {
            if actual_len==0 {v.push(0);}
            let mut i=0;
            while v.len()<len {
                v.push(v[i]);
                i+=1;
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
        assert_eq!(extract_first_and_rest(&rex, "1 2 3"),Some(("1"," 2 3")));
        assert_eq!(extract_first_and_rest(&rex, "0x123 2 3"),Some(("0x123"," 2 3")));
    }
    fn test_extract_vec<R:FromStr+Debug+PartialEq>(rex:&Regex, s:&str, max_len:usize, expected:Vec<R>, rest:&str) {
        let mut v = Vec::new();
        assert_eq!(extract_vec_first_and_rest::<R>(rex, max_len, &mut v, s).unwrap(),(expected.len(),rest));
        assert_eq!(v,expected);
    }
    #[test]
    fn test_extract_vec_int() {
        test_extract_vec::<isize>(&STRING_AS_INT_REX, "1 2 3", 10, vec![1,2,3], "");
        test_extract_vec::<isize>(&STRING_AS_INT_REX, "1 2 3", 1, vec![1], " 2 3");
        test_extract_vec::<usize>(&STRING_AS_INT_REX, "1 2 3", 10, vec![1,2,3], "");
        test_extract_vec::<usize>(&STRING_AS_INT_REX, "1 2 3", 1, vec![1], " 2 3");
    }
    #[test]
    fn test_extract_vec_float() {
        test_extract_vec::<f32>(&STRING_AS_FLOAT_REX, "1 -2 3.14 4.56", 10, vec![1.,-2.,3.14,4.56], "");
        test_extract_vec::<f64>(&STRING_AS_FLOAT_REX, "1 -2 3.14 4.56", 1, vec![1.,], " -2 3.14 4.56");
    }
}

//a Style values
//tp ValueError
#[derive(Debug)]
pub enum ValueError {
    BadValue(String),
}
impl ValueError {
    pub fn bad_value(s:&str) -> Self {
        Self::BadValue(s.to_string())
    }
}

//ti Display for ValueError
//ip std::fmt::Display for ValueError
impl std::fmt::Display for ValueError {
    //mp fmt - format a ValueError for display
    /// Display the ValueError
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::BadValue(s) => write!(f, "Bad value '{}'", s)
        }
    }
}

//tp StyleValue
/// `StyleValue` is used in descriptors of stylesheets to define the
/// styles that are expected within the stylesheet. This is an
/// enumeration that provides for single or sets of ints or floats,
/// colors, strings or lists of strings.
///
/// Instances of a `StyleValue` may be undefined - these are used both as not-yet-set values, and also as 'type descriptors'.
///
#[derive(Debug, Clone, PartialEq)]
pub enum StyleValue {
    /// A `Float` value contains either a single float value or an
    /// indication that the value is not set. As an unset value it may be used to indicate style value should be a single float value, such as for a line width
    Float      (Option<f64>),
    /// A `FloatArray` value contains a number of double-precision
    /// floats; if the value is not set then the length of the vec is
    /// 0
    /// indicates that the style value should be a list of float values, of any size; this could be a list of element weights, for example
    FloatArray (Vec<f64>),
    /// A `Floats(n)` value contains a vector with either 0 or n
    /// double-precision floats
    /// indicates that the style value should be 'n' float values; this might be used for points (two floats for a 2-dimensional point), for example.
    Floats     (usize, Vec<f64>),
    /// An `Int` value contains either a single signed integer value
    /// or an indication that the value is not set
    /// `Int` makes the style value a single signed integer (`isize`), such as may be used to indicate a row or column that an element may be placed within a table.
    Int        (Option<isize>),
    /// An `IntArray` value contains a number of signed integers; if
    /// the value is not set then the length of the vec is 0
    /// `IntArray` makes the style value a list of signed integers.
    IntArray   (Vec<isize>),
    /// An `Ints(n)` value contains a vector with either 0 or n signed
    /// integers
    /// `Ints(n)` requires the style value to be 'n' signed integers; this could indicate a width-height pair or rows/columns that a cell may be spread out over within a table.
    Ints       (usize, Vec<isize>),
    /// An `Rgb` value will be either a 3-element vector or a
    /// 0-element vector if it is not set. When set from a string it
    /// will utilize the colors database to permit a named color to be
    /// used to produce an RGB value.
    /// `Rgb` requires the style value to be three floats with a value of 0 to 1 (inclusive); this corresponds to an RGB value. 
    Rgb        (Vec<f64>),
    /// A `String` value contains either a single string or an
    /// indication that the value is not set
    /// `String` requires the style value to be a single string.
    String     (Option<String>),
    /// A `StringArray` value contains a number of strings; if the
    /// value is not set then the length of the vec is 0
    /// `StringArray` requires the style value to be a list of strings.
    StringArray  (Vec<String>)
}    

//ti StyleValue
impl StyleValue {

    //fp new_value
    /// Create a new value from a current value - which likely will be
    /// unset, and hence is basically used as a 'type' of that value
    ///
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::{StyleValue};
    ///  let type_int = StyleValue::int(None);
    ///  let mut x = type_int.new_value();
    ///  assert!(x.is_none(), "Value of X must be none before it is set");
    ///  x.from_string("2");
    ///  assert_eq!(2, x.as_int(None).unwrap());
    ///  x.from_string("17 5");
    ///  assert_eq!(17, x.as_int(None).unwrap());
    /// ```
    pub fn new_value(&self) -> Self {
        match self {
            StyleValue::Float(_)       => Self::float(None),
            StyleValue::Floats(n,_)    => Self::floats(*n),
            StyleValue::FloatArray(_)  => Self::float_array(),
            StyleValue::Int(_)         => Self::int(None),
            StyleValue::Ints(n, _)     => Self::ints(*n),
            StyleValue::IntArray(_)    => Self::int_array(),
            StyleValue::Rgb(_)         => Self::rgb(None),
            StyleValue::String(_)      => Self::string(None),
            StyleValue::StringArray(_) => Self::string_array(),
        }
    }

    //mp as_type
    /// Create an unset `StyleValue` of the same type
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::{StyleValue};
    ///  let type_int = StyleValue::int(None);
    ///  let x = type_int.new_value();
    ///  let type_x = x.as_type();
    ///  assert_eq!(type_int, type_x);
    /// 
    /// ```
    pub fn as_type(&self) -> Self {
        self.new_value()
    }

    //mp from_string
    /// Set the value from a string
    pub fn from_string<'a>(&'a mut self, s:&str) -> Result<&'a mut Self,ValueError> {
        match self {
            StyleValue::Float(ref mut f)       => {
                *f = Some( parse_str_as_floats(s, Some(1))? [0] );
            },
            StyleValue::Floats(n, ref mut f)       => {
                *f = parse_str_as_floats(s, Some(*n))?;
            },
            StyleValue::FloatArray(ref mut f)       => {
                *f = parse_str_as_floats(s, None)?;
            },
            StyleValue::Int(ref mut f)       => {
                let fa = parse_str_as_ints(s, Some(1))?;
                *f=Some(fa[0]);
            },
            StyleValue::Ints(n, ref mut f)       => {
                *f = parse_str_as_ints(s, Some(*n))?;
            },
            StyleValue::IntArray(ref mut f)       => {
                *f = parse_str_as_ints(s, None)?;
            },
            StyleValue::Rgb(ref mut f)       => {
                match color::of_string(s) {
                    Some(rgb) => { color::as_floats(rgb, f); },
                    None      => { *f = parse_str_as_floats(s, Some(3))?; },
                }
            },
            StyleValue::String(ref mut f)       => {
                *f = Some(s.to_string());
            },
            StyleValue::StringArray(ref mut f)       => {
                *f = s.split_whitespace().map(|s| s.to_string()).collect();
            },
        }
        Ok(self)
    }
    
    //fp floats
    /// Create a new floats value
    pub fn floats(n:usize) -> Self { Self::Floats(n,Vec::new()) }
    
    //fp float_array
    pub fn float_array() -> Self { Self::FloatArray(Vec::new()) }
    
    //fp float
    pub fn float(f:Option<f64>) -> Self { Self::Float(f) }
    
    //fp ints
    pub fn ints(n:usize) -> Self { Self::Ints(n,Vec::new()) }
    
    //fp int_array
    pub fn int_array() -> Self { Self::IntArray(Vec::new()) }
    
    //fp int
    pub fn int(n:Option<isize>) -> Self { Self::Int(n) }
    
    //fp rgb
    pub fn rgb(rgb:Option<(f64,f64,f64)>) -> Self {
        let mut v = Vec::new();
        match rgb {
            Some((r,g,b)) => {v.push(r); v.push(g); v.push(b);},
            _ => (),
        }
        Self::Rgb(v)
    }
    
    //fp string
    pub fn string(s:Option<String>) -> Self { Self::String(s) }
    
    //fp string_array
    pub fn string_array() -> Self { Self::StringArray(Vec::new()) }
    
    //mp is_none
    /// Determine if the StyleValue is not set
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StyleValue;
    /// ```
    pub fn is_none(&self) -> bool {
        match self {
            StyleValue::FloatArray(v) => v.len()==0,
            StyleValue::Floats(_, v)  => v.len()==0,
            StyleValue::IntArray(v)   => v.len()==0,
            StyleValue::Ints(_, v)    => v.len()==0,
            StyleValue::Rgb(v)        => v.len()==0,
            StyleValue::Float(None)   => true,
            StyleValue::Int(None)     => true,
            StyleValue::String(None)  => true,
            StyleValue::StringArray(v)  => v.len()==0,
            _ => false,
        }
    }

    //mp as_int
    /// Try to get an int from the `StyleValue` - the first of an array,
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StyleValue;
    /// ```
    pub fn as_int(&self, default:Option<isize>) -> Option<isize> {
        match self {
            StyleValue::FloatArray(v)  => { if v.len()==0 {default} else {Some(v[0] as isize)} },
            StyleValue::Floats(_, v)   => { if v.len()==0 {default} else {Some(v[0] as isize)} },
            StyleValue::Float(Some(n)) => Some(*n as isize),
            StyleValue::IntArray(v)    => { if v.len()==0 {default} else {Some(v[0])} },
            StyleValue::Ints(_, v)     => { if v.len()==0 {default} else {Some(v[0])} },
            StyleValue::Int(Some(n))   => Some(*n),
            StyleValue::Rgb(v)         => { if v.len()==0 {default} else {Some(v[0] as isize)} },
            _ => default,
        }
    }

    //mp as_ints
    /// Borrow a reference to Vec<isize>, using a default if the `StyleValue` is not set or is of the incorrect type
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StyleValue;
    ///  assert_eq!(true,  StyleValue::Ints(3,vec![]).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true,  StyleValue::Ints(3,vec![0,1]).as_ints(Some(&vec![2,3])).unwrap() == &vec![0,1]);
    ///  assert_eq!(false, StyleValue::Ints(3,vec![2,3]).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true,  StyleValue::IntArray(vec![]).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true,  StyleValue::IntArray(vec![0,1]).as_ints(Some(&vec![2,3])).unwrap() == &vec![0,1]);
    ///  assert_eq!(false, StyleValue::IntArray(vec![2,3]).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true, StyleValue::String(Some("banana".to_string())).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true, StyleValue::String(Some("banana".to_string())).as_ints(None).is_none());
    ///  assert_eq!(true, StyleValue::Ints(3,vec![]).as_ints(None).is_none());
    ///  assert_eq!(true, StyleValue::IntArray(vec![]).as_ints(None).is_none());
    /// ```
    pub fn as_ints<'a> (&'a self, default:Option<&'a Vec<isize>>) -> Option<&'a Vec<isize>> {
        match &self {
            StyleValue::IntArray(ref v) => { if v.len()==0 {default} else {Some(v)} },
            StyleValue::Ints(_, ref v)  => { if v.len()==0 {default} else {Some(v)} },
            _ => default,
        }
    }

    //mp as_float
    /// Try to get a float from the `StyleValue` - the first of an array,
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StyleValue;
    /// ```
    pub fn as_float(&self, default:Option<f64>) -> Option<f64> {
        match self {
            StyleValue::FloatArray(v)  => { if v.len()==0 {default} else {Some(v[0])} },
            StyleValue::Floats(_, v)   => { if v.len()==0 {default} else {Some(v[0])} },
            StyleValue::Float(Some(n)) => Some(*n),
            StyleValue::IntArray(v)    => { if v.len()==0 {default} else {Some(v[0] as f64)} },
            StyleValue::Ints(_, v)     => { if v.len()==0 {default} else {Some(v[0] as f64)} },
            StyleValue::Int(Some(n))   => Some(*n as f64),
            StyleValue::Rgb(v)         => { if v.len()==0 {default} else {Some(v[0])} },
            _ => default,
        }
    }

    //mp as_floats
    /// Borrow a reference to Vec<f64>, using a default if the `StyleValue` is not set or is of the incorrect type
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StyleValue;
    ///  assert_eq!(true,  StyleValue::Floats(3,vec![]).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true,  StyleValue::Floats(3,vec![0.,1.]).as_floats(Some(&vec![2.,3.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(false, StyleValue::Floats(3,vec![2.,3.]).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true,  StyleValue::FloatArray(vec![]).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true,  StyleValue::FloatArray(vec![0.,1.]).as_floats(Some(&vec![2.,3.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(false, StyleValue::FloatArray(vec![2.,3.]).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true, StyleValue::String(Some("banana".to_string())).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true, StyleValue::String(Some("banana".to_string())).as_floats(None).is_none());
    ///  assert_eq!(true, StyleValue::Floats(3,vec![]).as_floats(None).is_none());
    ///  assert_eq!(true, StyleValue::FloatArray(vec![]).as_floats(None).is_none());
    /// ```
    pub fn as_floats<'a> (&'a self, default:Option<&'a Vec<f64>>) -> Option<&'a Vec<f64>> {
        match &self {
            StyleValue::FloatArray(ref v) => { if v.len()==0 {default} else {Some(v)} },
            StyleValue::Floats(_, ref v)  => { if v.len()==0 {default} else {Some(v)} },
            _ => default,
        }
    }

    //mp as_color_string
    /// Generate a color string from an RGB 
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StyleValue;
    ///  assert_eq!(true,  StyleValue::rgb(Some((1.,1.,1.))).as_color_string(None).unwrap() == "white");
    ///  assert_eq!(true,  StyleValue::rgb(Some((0.,0.,0.))).as_color_string(None).unwrap() == "black");
    ///  assert_eq!(true,  StyleValue::rgb(None).as_color_string(None).is_none());
    ///  assert_eq!(true,  StyleValue::rgb(None).as_color_string(Some("None".to_string())).unwrap() == "None");
    ///  assert_eq!(true,  StyleValue::int(None).as_color_string(Some("None".to_string())).unwrap() == "None");
    /// ```
    pub fn as_color_string (&self, default:Option<String>) -> Option<String> {
        match &self {
            StyleValue::Rgb(v) => {
                if v.len()<3 {
                    default
                } else {
                    Some(color::as_string(color::as_u32(v)))
                }
            },
            _ => default,
        }
    }

    //mp has_token
    /// Test if the value contains a particular string. This can only return `true` if the value is a StringArray
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StyleValue;
    ///  assert_eq!(true, StyleValue::StringArray(vec!["string".to_string(),"another_string".to_string()]).has_token("string"));
    ///  assert_eq!(true, StyleValue::StringArray(vec!["string".to_string(),"another_string".to_string()]).has_token("another_string"));
    ///  assert_eq!(false, StyleValue::StringArray(vec!["string".to_string(),"another_string".to_string()]).has_token("not one of the strings"));
    ///  assert_eq!(false, StyleValue::Int(Some(0)).has_token("another_string"));
    /// ```
    pub fn has_token(&self, value:&str) -> bool {
        match self {
            StyleValue::StringArray(sv) => {
                for s in sv { if s==value {return true;} }
                false
            }
            _ => false,
        }
    }

    //mp equals_string
    /// Test if the value is the same as a string. This can only return `true` if the value is a String
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StyleValue;
    ///  assert_eq!(true, StyleValue::String(Some("string".to_string())).equals_string("string"));
    ///  assert_eq!(false, StyleValue::Int(Some(0)).equals_string("string"));
    ///  assert_eq!(false, StyleValue::String(Some("not that string".to_string())).equals_string("string"));
    /// ```
    pub fn equals_string(&self, value:&str) -> bool {
        match self {
            StyleValue::String(Some(s)) => (s==value),
            _ => false,
        }
    }

    //mp as_string - get a string representation
    /// Display the character as either the character itself, or '<EOF>'
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StyleValue;
    ///  assert_eq!(true,  StyleValue::rgb(Some((1.,1.,1.))).as_string().unwrap() == "white");
    ///  assert_eq!(true,  StyleValue::rgb(Some((0.,0.,0.))).as_string().unwrap() == "black");
    ///  assert_eq!(true,  StyleValue::rgb(None).as_string().is_none());
    ///  assert_eq!(true,  StyleValue::int(None).as_string().is_none());
    ///  assert_eq!(true,  StyleValue::int(Some(1)).as_string().unwrap() == "1");
    ///  assert_eq!(true,  StyleValue::string(Some("banana".to_string())).as_string().unwrap() == "banana");
    /// ```
    pub fn as_string(&self) -> Option<String> {
        if self.is_none() {
            None
        } else {
            match self {
                StyleValue::FloatArray(v)   => Some(format!("{:?}",v)),
                StyleValue::Floats(_, v)    => Some(format!("{:?}",v)),
                StyleValue::Float(Some(v))  => Some(format!("{}",v)),
                StyleValue::IntArray(v)     => Some(format!("{:?}",v)),
                StyleValue::Ints(_, v)      => Some(format!("{:?}",v)),
                StyleValue::Int(Some(v))    => Some(format!("{}",v)),
                StyleValue::Rgb(_)          => self.as_color_string(None),
                StyleValue::String(Some(v)) => Some(v.clone()),
                StyleValue::StringArray(v)  => Some(format!("{:?}",v)),
                _ => None,
            }
        }
    }
    //mp fmt_type - format the type of the `StyleValue`
    /// Display the character as either the character itself, or '<EOF>'
    fn fmt_type(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Float(_)       => write!(f, "flt"),
            Self::FloatArray(_)  => write!(f, "fa"),
            Self::Floats(n, _)   => write!(f, "f{}", n),
            Self::Int(_)         => write!(f, "int"),
            Self::IntArray(_ )   => write!(f, "ia"),
            Self::Ints(n, _)     => write!(f, "i{}", n),
            Self::Rgb(_)         => write!(f, "rgb"),
            Self::String(_)      => write!(f, "str"),
            Self::StringArray(_) => write!(f, "tkn"),
        }
    }
    //zz All done
}

//ti Display for StyleValue
impl std::fmt::Display for StyleValue {
    //mp fmt - format a character for display
    /// Display the character as either the character itself, or '<EOF>'
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.fmt_type(f)?;
        write!(f, ":{}", self.as_string().unwrap_or("<None>".to_string()))
    }

    //zz All done
}

