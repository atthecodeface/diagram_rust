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
@brief   Stylable values
 */

//a Imports
use std::str::FromStr;
use std::fmt::Debug;
use regex::Regex;
use super::color;

//a Helper functions and modules 
//vi string_is_none_rex - regexp that is true if the string is only whitespace
const STRING_IS_NONE_REX : &str = r"^\s*$";

//vi string_as_float_rex  - float with optional whitespace / comma in front of it and a 'rest' overflow
/// <whitespace> [, <whitespace>] [-] <0-9>+ [.<0-9>*] [.*]
const STRING_AS_FLOAT_REX : &str = r"^\s*,?\s*(-?\d+\.?\d*)(.*)$";

//f string_as_int_rex - decimal or hex with optional whitespace / comma in front of it and a 'rest' overflow
const STRING_AS_INT_REX : &str = r"^\s*,?\s*(0x[0-9a-fA-F]+|\d+)(.*)$";

//f extract_first_and_rest
fn extract_first_and_rest<'a> (rex:&Regex, s:&'a str) -> Option<(&'a str, &'a str)> {
    match rex.captures(s) {
        None => None,
        Some(caps) => Some( (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()) )
    }
}

//f extract_vec_re_first_and_rest
fn extract_vec_first_and_rest<'a, R:FromStr> (rex:&Regex, max_len:usize, v:&'a mut Vec<R>, s:&'a str) -> (usize, &'a str) {
    if v.len()>=max_len {
        (v.len(), s)
    } else {
        match rex.captures(s) {
            None => (v.len(), s),
            Some(caps) => {
                match caps.get(1).unwrap().as_str().parse::<R>() {
                    Ok(value) => {
                        v.push(value);
                        extract_vec_first_and_rest(rex, max_len, v, caps.get(2).unwrap().as_str())
                    },
                    _e => (v.len(), s),
                }
            }
        }
    }
}

//t Test regular expressions
#[cfg(test)]
mod test_res {
    use super::*;
    #[test]
    fn test_extract_ints() {
        let rex = Regex::new(STRING_AS_INT_REX).unwrap();
        assert_eq!(extract_first_and_rest(&rex, "1 2 3"),Some(("1"," 2 3")));
        assert_eq!(extract_first_and_rest(&rex, "0x123 2 3"),Some(("0x123"," 2 3")));
    }
    fn test_extract_vec<R:FromStr+Debug+PartialEq>(rex_str:&str, s:&str, max_len:usize, expected:Vec<R>, rest:&str) {
        let rex = Regex::new(rex_str).unwrap();
        let mut v = Vec::new();
        assert_eq!(extract_vec_first_and_rest::<R>(&rex, max_len, &mut v, s),(expected.len(),rest));
        assert_eq!(v,expected);
    }
    #[test]
    fn test_extract_vec_int() {
        test_extract_vec::<isize>(STRING_AS_INT_REX, "1 2 3", 10, vec![1,2,3], "");
        test_extract_vec::<isize>(STRING_AS_INT_REX, "1 2 3", 1, vec![1], " 2 3");
        test_extract_vec::<usize>(STRING_AS_INT_REX, "1 2 3", 10, vec![1,2,3], "");
        test_extract_vec::<usize>(STRING_AS_INT_REX, "1 2 3", 1, vec![1], " 2 3");
    }
    #[test]
    fn test_extract_vec_float() {
        test_extract_vec::<f32>(STRING_AS_FLOAT_REX, "1 -2 3.14 4.56", 10, vec![1.,-2.,3.14,4.56], "");
        test_extract_vec::<f64>(STRING_AS_FLOAT_REX, "1 -2 3.14 4.56", 1, vec![1.,], " -2 3.14 4.56");
    }
}

//a Stylesheet values and value types
//tp StylableType
/// `StylableType` is used in descriptors of stylesheets to define
/// the styles that are expected within the stylesheet.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum StylableType {
    /// `Float` indicates that the style should be one float value
    Float,
    /// `FloatArray` indicates that the style should be a list of float values
    FloatArray,
    /// `Floats(n)` indicates that the style should be 'n' float values
    Floats(usize),
    Int,
    IntArray,
    Ints(usize),
    Rgb,
    String,
    TokenList
}

//ti std::fmt::Display for StylableType
impl std::fmt::Display for StylableType {
    //mp fmt - format a character for display
    /// Display the character as either the character itself, or '<EOF>'
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Float      => write!(f, "flt"),
            Self::FloatArray => write!(f, "fa"),
            Self::Floats(n)  => write!(f, "f{}", n),
            Self::Int        => write!(f, "int"),
            Self::IntArray   => write!(f, "ia"),
            Self::Ints(n)    => write!(f, "i{}", n),
            Self::Rgb        => write!(f, "rgb"),
            Self::String     => write!(f, "str"),
            Self::TokenList  => write!(f, "tkn"),
        }
    }
}

//tp StylableValue
#[derive(Debug, Clone, PartialEq)]
pub enum StylableValue {
    FloatArray (Vec<f64>),
    Floats     (usize, Vec<f64>),
    Float      (Option<f64>),
    IntArray   (Vec<isize>),
    Ints       (usize, Vec<isize>),
    Int        (Option<isize>),
    /// Rgb should be a 3-element vector; if it has 0 size then it is 'None'
    Rgb        (Vec<f64>),
    String     (Option<String>),
    TokenList  (Vec<String>)
}    

//ti StylableValue
impl StylableValue {

    //fp floats
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
    
    //fp token_list
    pub fn token_list() -> Self { Self::TokenList(Vec::new()) }
    
    //mp get_type
    /// Get the type of the `StylableValue`
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StylableValue;
    /// ```
    pub fn get_type(&self) -> StylableType {
        match self {
            StylableValue::FloatArray(_) => StylableType::FloatArray,
            StylableValue::Floats(n, _)  => StylableType::Floats(*n),
            StylableValue::Float(_)      => StylableType::Float,
            StylableValue::IntArray(_)   => StylableType::IntArray,
            StylableValue::Ints(n, _)    => StylableType::Ints(*n),
            StylableValue::Int(_)        => StylableType::Int,
            StylableValue::Rgb(_)        => StylableType::Rgb,
            StylableValue::String(_)     => StylableType::String,
            StylableValue::TokenList(_)  => StylableType::TokenList,
        }
    }

    //mp is_none
    /// Determine if the StylableValue is not set
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StylableValue;
    /// ```
    pub fn is_none(&self) -> bool {
        match self {
            StylableValue::FloatArray(v) => v.len()==0,
            StylableValue::Floats(_, v)  => v.len()==0,
            StylableValue::IntArray(v)   => v.len()==0,
            StylableValue::Ints(_, v)    => v.len()==0,
            StylableValue::Rgb(v)        => v.len()==0,
            StylableValue::Float(None)   => true,
            StylableValue::Int(None)     => true,
            StylableValue::String(None)  => true,
            StylableValue::TokenList(v)  => v.len()==0,
            _ => false,
        }
    }

    //mp as_int
    /// Try to get an int from the `StylableValue` - the first of an array,
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StylableValue;
    /// ```
    pub fn as_int(&self, default:Option<isize>) -> Option<isize> {
        match self {
            StylableValue::FloatArray(v)  => { if v.len()==0 {default} else {Some(v[0] as isize)} },
            StylableValue::Floats(_, v)   => { if v.len()==0 {default} else {Some(v[0] as isize)} },
            StylableValue::Float(Some(n)) => Some(*n as isize),
            StylableValue::IntArray(v)    => { if v.len()==0 {default} else {Some(v[0])} },
            StylableValue::Ints(_, v)     => { if v.len()==0 {default} else {Some(v[0])} },
            StylableValue::Int(Some(n))   => Some(*n),
            StylableValue::Rgb(v)         => { if v.len()==0 {default} else {Some(v[0] as isize)} },
            _ => default,
        }
    }

    //mp as_ints
    /// Borrow a reference to Vec<isize>, using a default if the `StylableValue` is not set or is of the incorrect type
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StylableValue;
    ///  assert_eq!(true,  StylableValue::Ints(3,vec![]).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true,  StylableValue::Ints(3,vec![0,1]).as_ints(Some(&vec![2,3])).unwrap() == &vec![0,1]);
    ///  assert_eq!(false, StylableValue::Ints(3,vec![2,3]).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true,  StylableValue::IntArray(vec![]).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true,  StylableValue::IntArray(vec![0,1]).as_ints(Some(&vec![2,3])).unwrap() == &vec![0,1]);
    ///  assert_eq!(false, StylableValue::IntArray(vec![2,3]).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true, StylableValue::String(Some("banana".to_string())).as_ints(Some(&vec![0,1])).unwrap() == &vec![0,1]);
    ///  assert_eq!(true, StylableValue::String(Some("banana".to_string())).as_ints(None).is_none());
    ///  assert_eq!(true, StylableValue::Ints(3,vec![]).as_ints(None).is_none());
    ///  assert_eq!(true, StylableValue::IntArray(vec![]).as_ints(None).is_none());
    /// ```
    pub fn as_ints<'a> (&'a self, default:Option<&'a Vec<isize>>) -> Option<&'a Vec<isize>> {
        match &self {
            StylableValue::IntArray(ref v) => { if v.len()==0 {default} else {Some(v)} },
            StylableValue::Ints(_, ref v)  => { if v.len()==0 {default} else {Some(v)} },
            _ => default,
        }
    }

    //mp as_float
    /// Try to get a float from the `StylableValue` - the first of an array,
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StylableValue;
    /// ```
    pub fn as_float(&self, default:Option<f64>) -> Option<f64> {
        match self {
            StylableValue::FloatArray(v)  => { if v.len()==0 {default} else {Some(v[0])} },
            StylableValue::Floats(_, v)   => { if v.len()==0 {default} else {Some(v[0])} },
            StylableValue::Float(Some(n)) => Some(*n),
            StylableValue::IntArray(v)    => { if v.len()==0 {default} else {Some(v[0] as f64)} },
            StylableValue::Ints(_, v)     => { if v.len()==0 {default} else {Some(v[0] as f64)} },
            StylableValue::Int(Some(n))   => Some(*n as f64),
            StylableValue::Rgb(v)         => { if v.len()==0 {default} else {Some(v[0])} },
            _ => default,
        }
    }

    //mp as_floats
    /// Borrow a reference to Vec<f64>, using a default if the `StylableValue` is not set or is of the incorrect type
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StylableValue;
    ///  assert_eq!(true,  StylableValue::Floats(3,vec![]).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true,  StylableValue::Floats(3,vec![0.,1.]).as_floats(Some(&vec![2.,3.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(false, StylableValue::Floats(3,vec![2.,3.]).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true,  StylableValue::FloatArray(vec![]).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true,  StylableValue::FloatArray(vec![0.,1.]).as_floats(Some(&vec![2.,3.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(false, StylableValue::FloatArray(vec![2.,3.]).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true, StylableValue::String(Some("banana".to_string())).as_floats(Some(&vec![0.,1.])).unwrap() == &vec![0.,1.]);
    ///  assert_eq!(true, StylableValue::String(Some("banana".to_string())).as_floats(None).is_none());
    ///  assert_eq!(true, StylableValue::Floats(3,vec![]).as_floats(None).is_none());
    ///  assert_eq!(true, StylableValue::FloatArray(vec![]).as_floats(None).is_none());
    /// ```
    pub fn as_floats<'a> (&'a self, default:Option<&'a Vec<f64>>) -> Option<&'a Vec<f64>> {
        match &self {
            StylableValue::FloatArray(ref v) => { if v.len()==0 {default} else {Some(v)} },
            StylableValue::Floats(_, ref v)  => { if v.len()==0 {default} else {Some(v)} },
            _ => default,
        }
    }

    //mp as_color_string
    /// Generate a color string from an RGB 
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StylableValue;
    ///  assert_eq!(true,  StylableValue::rgb(Some((1.,1.,1.))).as_color_string(None).unwrap() == "white");
    ///  assert_eq!(true,  StylableValue::rgb(Some((0.,0.,0.))).as_color_string(None).unwrap() == "black");
    ///  assert_eq!(true,  StylableValue::rgb(None).as_color_string(None).is_none());
    ///  assert_eq!(true,  StylableValue::rgb(None).as_color_string(Some("None".to_string())).unwrap() == "None");
    ///  assert_eq!(true,  StylableValue::int(None).as_color_string(Some("None".to_string())).unwrap() == "None");
    /// ```
    pub fn as_color_string (&self, default:Option<String>) -> Option<String> {
        match &self {
            StylableValue::Rgb(v) => {
                if v.len()<3 {
                    default
                } else {
                    let rgb:u32 = (((v[0]*255.) as u32) << 0) | (((v[1]*255.) as u32) << 8) | (((v[2]*255.) as u32) << 16);
                    match color::name_of_rgb(rgb) {
                        Some(s) => Some(s.to_string()),
                        None    => Some(format!("#{:06x}",rgb)),
                    }
                }
            },
            _ => default,
        }
    }

    //mp has_token
    /// Test if the value contains a particular string. This can only return `true` if the value is a TokenList
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StylableValue;
    ///  assert_eq!(true, StylableValue::TokenList(vec!["string".to_string(),"another_string".to_string()]).has_token("string"));
    ///  assert_eq!(true, StylableValue::TokenList(vec!["string".to_string(),"another_string".to_string()]).has_token("another_string"));
    ///  assert_eq!(false, StylableValue::TokenList(vec!["string".to_string(),"another_string".to_string()]).has_token("not one of the strings"));
    ///  assert_eq!(false, StylableValue::Int(Some(0)).has_token("another_string"));
    /// ```
    pub fn has_token(&self, value:&str) -> bool {
        match self {
            StylableValue::TokenList(sv) => {
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
    ///  use stylesheet::StylableValue;
    ///  assert_eq!(true, StylableValue::String(Some("string".to_string())).equals_string("string"));
    ///  assert_eq!(false, StylableValue::Int(Some(0)).equals_string("string"));
    ///  assert_eq!(false, StylableValue::String(Some("not that string".to_string())).equals_string("string"));
    /// ```
    pub fn equals_string(&self, value:&str) -> bool {
        match self {
            StylableValue::String(Some(s)) => (s==value),
            _ => false,
        }
    }

    //mp as_string - get a string representation
    /// Display the character as either the character itself, or '<EOF>'
    /// ```
    ///  extern crate stylesheet;
    ///  use stylesheet::StylableValue;
    ///  assert_eq!(true,  StylableValue::rgb(Some((1.,1.,1.))).as_string().unwrap() == "white");
    ///  assert_eq!(true,  StylableValue::rgb(Some((0.,0.,0.))).as_string().unwrap() == "black");
    ///  assert_eq!(true,  StylableValue::rgb(None).as_string().is_none());
    ///  assert_eq!(true,  StylableValue::int(None).as_string().is_none());
    ///  assert_eq!(true,  StylableValue::int(Some(1)).as_string().unwrap() == "1");
    ///  assert_eq!(true,  StylableValue::string(Some("banana".to_string())).as_string().unwrap() == "banana");
    /// ```
    pub fn as_string(&self) -> Option<String> {
        if self.is_none() {
            None
        } else {
            match self {
                StylableValue::FloatArray(v)   => Some(format!("{:?}",v)),
                StylableValue::Floats(_, v)    => Some(format!("{:?}",v)),
                StylableValue::Float(Some(v))  => Some(format!("{}",v)),
                StylableValue::IntArray(v)     => Some(format!("{:?}",v)),
                StylableValue::Ints(_, v)      => Some(format!("{:?}",v)),
                StylableValue::Int(Some(v))    => Some(format!("{}",v)),
                StylableValue::Rgb(_)          => self.as_color_string(None),
                StylableValue::String(Some(v)) => Some(v.clone()),
                StylableValue::TokenList(v)    => Some(format!("{:?}",v)),
                _ => None,
            }
        }
    }
    //zz All done
}

//ti Display for StylableValue
impl std::fmt::Display for StylableValue {
    //mp fmt - format a character for display
    /// Display the character as either the character itself, or '<EOF>'
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}:{}", self.get_type(), self.as_string().unwrap_or("<None>".to_string()))
    }

    //zz All done
}

/*
//f fill_out_array : 'a array -> size -> source_valid -> index_to_fill -> index_source -> 'a array
//  Fills out the array so that elements 0 to size-1 contain valid data, assuming that 0 to n-1 initially
//  contain valid data, replicating this data across the whole array as required.
 
let rec fill_out_array a num n i j =
  if (i=num) then
    a
  else
    (
      a.(i) <- a.(j);
      let next_j = if (j+1=n) then 0 else (j+1) in
      fill_out_array a num n (i+1) next_j
    )

//f read_floats_from_n : float_array -> max -> string -> number -> (number, remaining_string)
//  the float array will be completely valid, even if the string supplies fewer values
 
let rec read_floats_from_n floats max string = function
  | n when (n=max) -> (max, string)
  | n -> (
    match extract_first_and_rest string_as_float_rex string with
    | None -> (n, "")
    | Some (s1, s2) -> 
       (
         floats.(n) <- float_of_string s1;
         read_floats_from_n floats max s2 (n+1)
       )
  )

//f read_floats : string -> number -> float array
  the float array will be completely valid, even if the string supplies fewer values
 
let read_floats string num = 
  let floats = Array.make num 0. in
  let (n,_) = read_floats_from_n floats num string 0 in
  fill_out_array floats num (max n 1) n 0

//f read_float_arr : string  -> float array
  read a float array from the string, as many floats as possible
 
let read_float_arr string = 
  let rec acc_float_arrays acc s =
    let max = 10 in
    if (String.length s)=0 then acc else (
      let floats = Array.make max 0. in
      let (n,s) = read_floats_from_n floats max s 0 in
      let (total,arrs) = acc in
      let acc = (total+n,floats::arrs) in
      acc_float_arrays acc s
    )
  in
  let (total, arrs) = acc_float_arrays (0,[]) string in
  (total, Array.(sub (concat arrs) 0 total))

//f read_ints_from_n : int array -> max -> string -> number -> (number,remaining_string)   
let rec read_ints_from_n ints max string = function
  | n when (n=max) -> (max, string)
  | n -> (
    match extract_first_and_rest string_as_int_rex string with
    | None -> (n,"")
    | Some (s1,s2) -> 
       (
         ints.(n) <- int_of_string s1;
         read_ints_from_n ints max s2 (n+1)
       )
  )

//f read_int_arr : string  -> int array
  read a int array from the string, as many ints as possible
 
let read_int_arr string = 
  let rec acc_int_arrays acc s =
    let max = 10 in
    if (String.length s)=0 then acc else (
      let ints = Array.make max 0 in
      let (n,s) = read_ints_from_n ints max s 0 in
      let (total,arrs) = acc in
      let acc = (total+n,ints::arrs) in
      acc_int_arrays acc s
    )
  in
  let (total, arrs) = acc_int_arrays (0,[]) string in
  (total, Array.(sub (concat arrs) 0 total))

//f read_ints 
let read_ints string num = 
  let ints = Array.make num 0 in
  let (n,_) = read_ints_from_n ints num string 0 in
  fill_out_array ints num (max n 1) n 0

//f read_color 
let read_color string = 
    match Color.from_name string with
    | Some f -> f
    | None -> read_floats string 3

//f string_is_none - return True if string is none 
let string_is_none string =
  match (Re.exec_opt string_is_none_rex string) with
  | None -> true
  | _ -> false

//f read_tokens 
let read_tokens string =
  let n = String.length string in
  let rec read_next_token rtl i t nt =
    if (i>=n) then (rtl, t, nt) else (
      let ch = String.get string i in
      if (ch==' ') then (
        if nt then (
          read_next_token rtl (i+1) t nt
        ) else (
          read_next_token (t::rtl) (i+1) "" true
        )
      ) else (
        let s = String.make 1 ch in
        if nt then (
          read_next_token rtl (i+1) s false
        ) else (
          read_next_token rtl (i+1) (t^s) false
        )
      )
    )
  in
  let (rtl, last_token, no_last_token) = read_next_token [] 0 "" true in
  let rtl = if no_last_token then rtl else last_token::rtl in
  List.rev rtl

//f from_string 
let rec from_string stype value =
  if (string_is_none value) then (
    match stype with
    | St_ints n     -> sv_none_ints
    | St_floats n   -> sv_none_floats
    | St_float_arr  -> sv_none_float_arr
    | St_int_arr    -> sv_none_int_arr
    | St_rgb        -> sv_none_rgb
    | St_int        -> sv_none_int
    | St_float      -> sv_none_float
    | St_string     -> sv_none_string
    | St_token_list -> sv_none_token_list
  ) else (
    match stype with
    | St_ints n     -> ( let ints   = read_ints   value n in Sv_ints (n,ints) )
    | St_floats n   -> ( let floats = read_floats value n in Sv_floats (n,floats) )
    | St_float_arr  -> ( let (n,floats) = read_float_arr value in Sv_float_arr (n,floats) )
    | St_int_arr    -> ( let (n,ints)   = read_int_arr   value in Sv_int_arr   (n,ints) )
    | St_rgb        -> Sv_rgb (read_color value)
    | St_int        -> ( let ints   = read_ints   value 1 in Sv_int (Some ints.(0)) )
    | St_float      -> ( let floats = read_floats value 1 in Sv_float (Some floats.(0)) )
    | St_string     -> Sv_string (Some value)
    | St_token_list -> ( let tokens = read_tokens value in Sv_token_list tokens)
  )

//f as_string - get a string an svalue 
let as_string ?default svalue =
  if (is_none svalue) then (
    match default with | Some f -> f | None -> raise (Bad_value "No default value provided when getting value as_string")
  ) else (
  match svalue with
  | Sv_string (Some s) -> s
  | Sv_token_list l -> String.concat " " l
  | _ -> str svalue
  )


 */
