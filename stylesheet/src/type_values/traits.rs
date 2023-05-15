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
@brief   The basics of a style value/type
 */

//a Imports
use erased_serde::Serialize as ESerialize;
use serde::Serialize;
use std::any::Any;

use crate::utils;
use crate::ValueError;

//a Macros
//mi boiler_plate
macro_rules! boiler_plate {
    {} => {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
    }
}

//mi boiler_plate_serialize
macro_rules! boiler_plate_serialize {
    {} => {
    fn as_serialize(&self) -> Option<&dyn ESerialize> { Some(self) }
    }
}

//mi array_n
macro_rules! array_n {
    {$t:ident, $tname:expr, $n:expr, $parser:path} => {
impl TypeValue for [$t ; $n] {
    boiler_plate!();
    boiler_plate_serialize!();
    fn type_name(&self) -> String {
        format!($tname, $n)
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        let s = Self::default();
        Box::new(s)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        Box::new(*self)
    }
    fn len(&self) -> usize {
        $n
    }
    fn is_none(&self) -> bool {
        false
    }
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        for (i, d) in data.iter_mut().enumerate() {
            if i >= $n {
                return Some(&data[0..$n]);
            }
            *d = self[i] as f64;
        }
        Some(data)
    }
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        for (i, d) in data.iter_mut().enumerate() {
            if i >= $n {
                return Some(&data[0..$n]);
            }
            *d = self[i] as isize;
        }
        Some(data)
    }
    fn parse_string(&mut self, s: &str, _append: bool) -> Result<(), ValueError> {
        for (i, f) in $parser(s, Some($n))?
            .into_iter()
            .enumerate()
        {
            self[i] = f as $t;
        }
        Ok(())
    }
}}
}

//a TypeValue
//tp TypeValue
/// This is a trait that must be supported by any type that can be used as a StyleTypeValue
///
/// It enables the type
pub trait TypeValue: std::fmt::Debug + 'static {
    /// Return 'self' as an &dyn Any
    ///
    /// This should always be "fn as_any(&self) -> &dyn Any { self }"
    fn as_any(&self) -> &dyn Any;

    /// Return 'self' as an &dyn erased_serde::Serialize
    ///
    /// This should always be "fn as_serialize(&self) -> Option<&dyn erased_serde::Serialize> { Some(self) }" if suppllied
    fn as_serialize(&self) -> Option<&dyn ESerialize> {
        None
    }

    /// Get the name of the type; this may not be static
    fn type_name(&self) -> String;

    /// Return 'self' as an &mut dyn Any
    ///
    /// This should always be "fn as_any_mut(&mut self) -> &mut dyn Any { self }"
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Get a new "empty" value assuming 'self' is the 'type'
    ///
    /// This should be an empty Vec, for example, or zeros, etc
    fn mk_value(&self) -> Box<dyn TypeValue>;

    /// Clone the value
    fn clone_value(&self) -> Box<dyn TypeValue> {
        self.mk_value()
    }

    /// Return the length - if a singleton, then 1; if none then 0
    fn len(&self) -> usize;

    /// Return true if the type is empty (sort of Rust standard)
    ///
    /// A type is usually 'is_none()' if it 'is_empty()'; this is not required though
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return true if this is a 'None' value
    fn is_none(&self) -> bool;

    /// Return the value as a string, using the type-specific 'format'
    fn as_string(&self, _format: usize) -> String {
        format!("{:?}", self)
    }

    /// Get ints from the value, returning the slice that has been
    /// filled in, or None if not possible
    fn get_ints<'a>(&self, _data: &'a mut [isize]) -> Option<&'a [isize]> {
        None
    }

    /// Get floats from the value, returning the slice that has been
    /// filled in, or None if not possible
    fn get_floats<'a>(&self, _data: &'a mut [f64]) -> Option<&'a [f64]> {
        None
    }

    /// Get strs from the value, where possible returning the number
    /// of strs gotten.
    ///
    /// Only really useful for a string or list of strings
    fn get_strs<'ty, 'get>(&'ty self, _data: &'get mut [&'ty str]) -> Option<&'get [&'ty str]> {
        None
    }

    /// Compare a value with another that should be *OF THE SAME TYPE*
    fn cmp(&self, _other: &dyn Any) -> Option<std::cmp::Ordering> {
        None
    }

    /// Determine if this contains a string; for non-string things, this is false
    fn has_string(&self, _s: &str, _as_token: bool) -> bool {
        false
    }

    /// Parse the string and set the value (or add to a list if append is false)
    fn parse_string(&mut self, _s: &str, append: bool) -> Result<(), ValueError>;
}

//a TypeValue implementations
//ip TypeValue for Option<T>
impl<T: TypeValue + Default + Clone + Serialize + ESerialize> TypeValue for Option<T> {
    boiler_plate!();
    boiler_plate_serialize!();
    fn type_name(&self) -> String {
        format!("Option<{}>", T::default().type_name())
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        let s: Self = None;
        Box::new(s)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        let s = (*self).clone();
        Box::new(s)
    }
    fn len(&self) -> usize {
        if let Some(t) = self {
            t.len()
        } else {
            0
        }
    }
    fn is_none(&self) -> bool {
        Option::is_none(self)
    }
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        if let Some(t) = self {
            t.get_floats(data)
        } else {
            None
        }
    }
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        if let Some(t) = self {
            t.get_ints(data)
        } else {
            None
        }
    }
    fn get_strs<'a, 'b>(&'b self, data: &'a mut [&'b str]) -> Option<&'a [&'b str]> {
        if let Some(t) = self {
            t.get_strs(data)
        } else {
            None
        }
    }
    fn cmp(&self, other: &dyn Any) -> Option<std::cmp::Ordering> {
        match other.downcast_ref::<Self>() {
            Some(other) => match (self, other) {
                (Some(t), Some(o)) => t.cmp(o),
                (None, None) => Some(std::cmp::Ordering::Equal),
                (None, _) => Some(std::cmp::Ordering::Less),
                (_, None) => Some(std::cmp::Ordering::Greater),
            },
            _ => None,
        }
    }

    fn parse_string(&mut self, s: &str, append: bool) -> Result<(), ValueError> {
        if utils::parse_str_is_none(s) {
            if !append {
                *self = None;
            }
        } else if let Some(t) = self {
            t.parse_string(s, append)?;
        } else {
            let mut t = T::default();
            t.parse_string(s, append)?;
            *self = Some(t);
        }
        let make_none = self.as_ref().map_or(false, |x| x.is_none());
        if make_none {
            *self = None;
        }
        Ok(())
    }
}

//ip TypeValue for isize
impl TypeValue for isize {
    boiler_plate!();
    boiler_plate_serialize!();
    fn type_name(&self) -> String {
        "isize".into()
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        Box::new(0)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        Box::new(*self)
    }
    fn len(&self) -> usize {
        1
    }
    fn is_none(&self) -> bool {
        false
    }
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        if !data.is_empty() {
            data[0] = *self as f64;
            Some(&data[0..1])
        } else {
            None
        }
    }
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        if !data.is_empty() {
            data[0] = *self;
            Some(&data[0..1])
        } else {
            None
        }
    }
    fn cmp(&self, other: &dyn Any) -> Option<std::cmp::Ordering> {
        match other.downcast_ref::<Self>() {
            Some(other) => self.partial_cmp(other),
            _ => None,
        }
    }
    fn parse_string(&mut self, s: &str, _append: bool) -> Result<(), ValueError> {
        *self = utils::parse_str_as_ints(s, Some(1))?[0];
        Ok(())
    }
}

//ip TypeValue for [isize; N]
array_n!(isize, "isize[{}]", 1, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 2, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 3, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 4, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 5, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 6, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 7, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 8, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 9, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 10, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 11, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 12, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 13, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 14, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 15, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 16, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 17, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 18, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 19, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 20, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 21, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 22, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 23, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 24, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 25, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 26, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 27, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 28, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 29, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 30, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 31, utils::parse_str_as_ints);
array_n!(isize, "isize[{}]", 32, utils::parse_str_as_ints);

//ip TypeValue for Vec<isize>
impl TypeValue for Vec<isize> {
    boiler_plate!();
    boiler_plate_serialize!();
    fn type_name(&self) -> String {
        "Vec<isize>".into()
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        let v: Vec<isize> = vec![];
        Box::new(v)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        let s = (*self).clone();
        Box::new(s)
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn is_none(&self) -> bool {
        self.is_empty()
    }
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        let n = self.len();
        for (i, d) in data.iter_mut().enumerate() {
            if i >= n {
                return Some(&data[0..n]);
            }
            *d = self[i] as f64;
        }
        Some(data)
    }
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        let n = self.len();
        for (i, d) in data.iter_mut().enumerate() {
            if i >= n {
                return Some(&data[0..n]);
            }
            *d = self[i];
        }
        Some(data)
    }
    fn parse_string(&mut self, s: &str, append: bool) -> Result<(), ValueError> {
        if !append {
            self.clear();
        }
        self.append(&mut utils::parse_str_as_ints(s, None)?);
        Ok(())
    }
}

//ip TypeValue for f64
impl TypeValue for f64 {
    boiler_plate!();
    boiler_plate_serialize!();
    fn type_name(&self) -> String {
        "f64".into()
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        Box::new(0.0_f64)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        Box::new(*self)
    }
    fn len(&self) -> usize {
        1
    }
    fn is_none(&self) -> bool {
        false
    }
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        if !data.is_empty() {
            data[0] = *self;
            Some(&data[0..1])
        } else {
            None
        }
    }
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        if !data.is_empty() {
            data[0] = *self as isize;
            Some(&data[0..1])
        } else {
            None
        }
    }
    fn cmp(&self, other: &dyn Any) -> Option<std::cmp::Ordering> {
        match other.downcast_ref::<Self>() {
            Some(other) => self.partial_cmp(other),
            _ => None,
        }
    }
    fn parse_string(&mut self, s: &str, _append: bool) -> Result<(), ValueError> {
        *self = utils::parse_str_as_floats(s, Some(1))?[0];
        Ok(())
    }
}

//ip TypeValue for [f64; N]
array_n!(f64, "isize[{}]", 1, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 2, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 3, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 4, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 5, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 6, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 7, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 8, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 9, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 10, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 11, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 12, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 13, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 14, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 15, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 16, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 17, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 18, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 19, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 20, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 21, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 22, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 23, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 24, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 25, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 26, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 27, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 28, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 29, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 30, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 31, utils::parse_str_as_floats);
array_n!(f64, "isize[{}]", 32, utils::parse_str_as_floats);

//ip TypeValue for Vec<f64>
impl TypeValue for Vec<f64> {
    boiler_plate!();
    boiler_plate_serialize!();
    fn type_name(&self) -> String {
        "Vec<f64>".into()
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        let v: Vec<f64> = vec![];
        Box::new(v)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        let s = (*self).clone();
        Box::new(s)
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn is_none(&self) -> bool {
        self.is_empty()
    }
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        let n = self.len();
        for (i, d) in data.iter_mut().enumerate() {
            if i >= n {
                return Some(&data[0..n]);
            }
            *d = self[i];
        }
        Some(data)
    }
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        let n = self.len();
        for (i, d) in data.iter_mut().enumerate() {
            if i >= n {
                return Some(&data[0..n]);
            }
            *d = self[i] as isize;
        }
        Some(data)
    }
    fn parse_string(&mut self, s: &str, append: bool) -> Result<(), ValueError> {
        if !append {
            self.clear();
        }
        self.append(&mut utils::parse_str_as_floats(s, None)?);
        Ok(())
    }
}

//ip TypeValue for String
impl TypeValue for String {
    boiler_plate!();
    boiler_plate_serialize!();
    fn type_name(&self) -> String {
        "String".into()
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        let s: String = Self::new();
        Box::new(s)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        let s = (*self).clone();
        Box::new(s)
    }
    fn len(&self) -> usize {
        1
    }
    fn is_none(&self) -> bool {
        self.is_empty()
    }
    fn get_strs<'a, 'b>(&'b self, data: &'a mut [&'b str]) -> Option<&'a [&'b str]> {
        if data.is_empty() {
            None
        } else {
            data[0] = self;
            Some(&data[0..1])
        }
    }
    fn cmp(&self, other: &dyn Any) -> Option<std::cmp::Ordering> {
        match other.downcast_ref::<Self>() {
            Some(other) => self.partial_cmp(other),
            _ => None,
        }
    }
    fn has_string(&self, s: &str, as_token: bool) -> bool {
        if as_token {
            false
        } else {
            self == s
        }
    }
    fn parse_string(&mut self, s: &str, append: bool) -> Result<(), ValueError> {
        if !append {
            self.clear();
        }
        self.push_str(s);
        Ok(())
    }
}

//ip TypeValue for Vec<String>
impl TypeValue for Vec<String> {
    boiler_plate!();
    boiler_plate_serialize!();
    fn type_name(&self) -> String {
        "Vec<String>".into()
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        let v: Vec<f64> = vec![];
        Box::new(v)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        let s = (*self).clone();
        Box::new(s)
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn is_none(&self) -> bool {
        self.is_empty()
    }
    fn get_strs<'a, 'b>(&'b self, data: &'a mut [&'b str]) -> Option<&'a [&'b str]> {
        let n = self.len();
        for (i, d) in data.iter_mut().enumerate() {
            if i >= n {
                return Some(&data[0..n]);
            }
            *d = &self[i];
        }
        Some(data)
    }
    fn has_string(&self, s: &str, _as_token: bool) -> bool {
        self.iter().any(|x| (x == s))
    }
    fn parse_string(&mut self, s: &str, append: bool) -> Result<(), ValueError> {
        if !append {
            self.clear();
        }
        if !s.is_empty() {
            self.push(s.into());
        }
        Ok(())
    }
}
//ip TypeValue for (&str, bool, Vec<String>)
// (separator, allow_empty, data)
impl TypeValue for (&'static str, bool, Vec<String>) {
    boiler_plate!();
    fn type_name(&self) -> String {
        format!("{}:{}:Vec<String>", self.0, self.1)
    }
    fn as_serialize(&self) -> Option<&dyn ESerialize> {
        Some(&self.2)
    }
    fn mk_value(&self) -> Box<dyn TypeValue> {
        let v = (self.0, self.1, Vec::<String>::new());
        Box::new(v)
    }
    fn clone_value(&self) -> Box<dyn TypeValue> {
        let s = (*self).clone();
        Box::new(s)
    }
    fn len(&self) -> usize {
        self.2.len()
    }
    fn is_none(&self) -> bool {
        self.2.is_empty()
    }
    fn get_strs<'a, 'b>(&'b self, data: &'a mut [&'b str]) -> Option<&'a [&'b str]> {
        let n = self.2.len();
        for (i, d) in data.iter_mut().enumerate() {
            if i >= n {
                return Some(&data[0..n]);
            }
            *d = &self.2[i];
        }
        Some(data)
    }
    fn has_string(&self, s: &str, _as_token: bool) -> bool {
        self.2.iter().any(|x| (x == s))
    }
    fn parse_string(&mut self, s: &str, append: bool) -> Result<(), ValueError> {
        if !append {
            self.2.clear();
        }
        if !s.is_empty() {
            for fs in s.split(self.0) {
                if !fs.is_empty() || self.1 {
                    self.2.push(fs.into());
                }
            }
        }
        Ok(())
    }
}
