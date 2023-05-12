//a Imports
use std::any::{Any, TypeId};

use crate::style::utils;
use crate::ValueError;

//a NVBTrait
//tp NVBTrait
pub trait NVBTrait: std::fmt::Debug + 'static {
    /// Return 'self' as an &dyn Any
    ///
    /// This should always be "fn as_any(&self) -> &dyn Any { self }"
    fn as_any(&self) -> &dyn Any;

    /// Get a new "empty" value assuming 'self' is the 'type'
    ///
    /// This should be an empty Vec, for example, or zeros, etc
    fn new_nvb(&self) -> Box<dyn NVBTrait>;

    /// Clone the value
    fn clone(&self) -> Box<dyn NVBTrait> {
        self.new_nvb()
    }

    /// Return the length - if a singleton, then 1; if none then 0
    fn len(&self) -> usize;

    /// Return true if this is a 'None' value
    fn is_none(&self) -> bool;

    /// Return the value as a string, using the type-specific 'format'
    fn as_string(&self, _format: usize) -> String {
        format!("{:?}", self)
    }

    /// Get ints from the value, returning the slice that has been
    /// filled in, or None if not possible
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        None
    }

    /// Get floats from the value, returning the slice that has been
    /// filled in, or None if not possible
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        None
    }

    /// Get strs from the value, where possible returning the number
    /// of strs gotten.
    ///
    /// Only really useful for a string or list of strings
    fn get_strs<'a>(&'a self, _data: &'a mut [&'a str]) -> Option<&'a [&'a str]> {
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

//a NVBTrait implementations
//ip NVBTrait for Option<T>
impl<T: NVBTrait + Default> NVBTrait for Option<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_nvb(&self) -> Box<dyn NVBTrait> {
        let s: Self = None;
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

//ip NVBTrait for isize
impl NVBTrait for isize {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_nvb(&self) -> Box<dyn NVBTrait> {
        Box::new(0)
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

//ip NVBTrait for [isize; N]
impl<const N: usize> NVBTrait for [isize; N] {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_nvb(&self) -> Box<dyn NVBTrait> {
        Box::new([0; N])
    }
    fn len(&self) -> usize {
        N
    }
    fn is_none(&self) -> bool {
        false
    }
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        for (i, d) in data.iter_mut().enumerate() {
            if i >= N {
                return Some(&data[0..N]);
            }
            *d = self[i] as f64;
        }
        Some(data)
    }
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        for (i, d) in data.iter_mut().enumerate() {
            if i >= N {
                return Some(&data[0..N]);
            }
            *d = self[i];
        }
        Some(data)
    }
    fn parse_string(&mut self, s: &str, _append: bool) -> Result<(), ValueError> {
        for (i, f) in utils::parse_str_as_ints(s, Some(N))?
            .into_iter()
            .enumerate()
        {
            self[i] = f;
        }
        Ok(())
    }
}

//ip NVBTrait for Vec<isize>
impl NVBTrait for Vec<isize> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_nvb(&self) -> Box<dyn NVBTrait> {
        let v: Vec<isize> = vec![];
        Box::new(v)
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

//ip NVBTrait for f64
impl NVBTrait for f64 {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_nvb(&self) -> Box<dyn NVBTrait> {
        Box::new(0.0_f64)
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

//ip NVBTrait for [f64; N]
impl<const N: usize> NVBTrait for [f64; N] {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_nvb(&self) -> Box<dyn NVBTrait> {
        Box::new([0.0_f64; N])
    }
    fn len(&self) -> usize {
        N
    }
    fn is_none(&self) -> bool {
        false
    }
    fn get_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        for (i, d) in data.iter_mut().enumerate() {
            if i >= N {
                return Some(&data[0..N]);
            }
            *d = self[i];
        }
        Some(data)
    }
    fn get_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        for (i, d) in data.iter_mut().enumerate() {
            if i >= N {
                return Some(&data[0..N]);
            }
            *d = self[i] as isize;
        }
        Some(data)
    }
    fn parse_string(&mut self, s: &str, _append: bool) -> Result<(), ValueError> {
        for (i, f) in utils::parse_str_as_floats(s, Some(N))?
            .into_iter()
            .enumerate()
        {
            self[i] = f;
        }
        Ok(())
    }
}

//ip NVBTrait for Vec<f64>
impl NVBTrait for Vec<f64> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_nvb(&self) -> Box<dyn NVBTrait> {
        let v: Vec<f64> = vec![];
        Box::new(v)
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

//ip NVBTrait for String
impl NVBTrait for String {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_nvb(&self) -> Box<dyn NVBTrait> {
        let s: String = Self::new();
        Box::new(s)
    }
    fn len(&self) -> usize {
        1
    }
    fn is_none(&self) -> bool {
        self.is_empty()
    }
    fn get_strs<'a>(&'a self, data: &'a mut [&'a str]) -> Option<&'a [&'a str]> {
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

//ip NVBTrait for Vec<String>
impl NVBTrait for Vec<String> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn new_nvb(&self) -> Box<dyn NVBTrait> {
        let v: Vec<f64> = vec![];
        Box::new(v)
    }
    fn len(&self) -> usize {
        self.len()
    }
    fn is_none(&self) -> bool {
        self.is_empty()
    }
    fn get_strs<'a>(&'a self, data: &'a mut [&'a str]) -> Option<&'a [&'a str]> {
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
        self.iter().fold(false, |acc, x| acc || (x == s))
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

//a StyleTypeValue
#[derive(Debug)]
pub struct StyleTypeValue {
    value: Box<dyn NVBTrait>,
}
impl std::fmt::Display for StyleTypeValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(&self.value, fmt)
    }
}
impl std::clone::Clone for StyleTypeValue {
    fn clone(&self) -> Self {
        let value = self.value.clone();
        Self { value }
    }
}
impl std::cmp::PartialEq<StyleTypeValue> for StyleTypeValue {
    fn eq(&self, other: &Self) -> bool {
        self.value.cmp(other.value.as_any()) == Some(std::cmp::Ordering::Equal)
    }
}
impl std::cmp::PartialOrd<StyleTypeValue> for StyleTypeValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.cmp(other.value.as_any())
    }
}
impl StyleTypeValue {
    pub fn mk_type<T: NVBTrait + Default>() -> Self {
        Self::new(T::default())
    }
    pub fn new<T: NVBTrait>(value: T) -> Self {
        let value = Box::new(value);
        Self { value }
    }
    pub fn of_type(&self) -> Self {
        let value = self.value.new_nvb();
        Self { value }
    }
    pub fn is_type<T: NVBTrait>(&self) -> bool {
        self.value.as_ref().as_any().type_id() == TypeId::of::<T>()
    }
    pub fn as_t<T: NVBTrait>(&self) -> &T {
        self.value.as_any().downcast_ref::<T>().unwrap()
    }
    pub fn cmp<T: Any>(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.value.cmp(other as &dyn Any)
    }
    pub fn eq<T: Any>(&self, other: &T) -> bool {
        self.cmp(other) == Some(std::cmp::Ordering::Equal)
    }
    pub fn as_f64(&self) -> Option<f64> {
        let mut data = [0.];
        if self.value.get_floats(&mut data).is_some() {
            Some(data[0])
        } else {
            None
        }
    }
    pub fn as_isize(&self) -> Option<isize> {
        let mut data = [0];
        if self.value.get_ints(&mut data).is_some() {
            Some(data[0])
        } else {
            None
        }
    }
    fn as_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        self.value.get_ints(data)
    }
    fn as_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        self.value.get_floats(data)
    }
    // From TypeValue
    fn new_value(&self) -> Self {
        self.clone()
    }
    fn as_type(&self) -> Self {
        self.clone()
    }
    //mp from_string
    /// Set the value from a string
    fn from_string<'a>(&'a mut self, s: &str) -> Result<&'a mut Self, ValueError> {
        self.value.parse_string(s, false)?;
        Ok(self)
    }
    fn eq_string(&self, s: &str) -> bool {
        self.value.has_string(s, false)
    }
}

//a Test suite
//tf test_isize
#[test]
fn test_isize() {
    let t_isize = StyleTypeValue::mk_type::<isize>();
    let mut v_isize = t_isize.new_value();
    v_isize.from_string("73").unwrap();
    assert!(
        v_isize.is_type::<isize>(),
        "Expect isize to be of type isize"
    );
    assert_eq!(v_isize.as_t::<isize>(), &73, "Expect value to be isize 73");
    assert!(v_isize.eq(&73_isize));
    assert!(
        !v_isize.eq(&73_usize),
        "73 *isize* should not equal 73 *usize*"
    );
    assert_eq!(v_isize.as_isize().unwrap(), 73);
    assert_eq!(v_isize.as_f64().unwrap(), 73.0);

    v_isize.from_string("1").unwrap();
    assert_eq!(v_isize.as_t::<isize>(), &1, "Expect value to be isize 1");
    assert!(v_isize.eq(&1_isize));
    assert!(
        !v_isize.eq(&1_usize),
        "1 *isize* should not equal 1 *usize*"
    );
    assert_eq!(v_isize.as_isize().unwrap(), 1);
    assert_eq!(v_isize.as_f64().unwrap(), 1.0);

    assert_eq!(v_isize.as_ints(&mut [0]), Some(&[1][..]));
}

//tf test_isize_array
#[test]
fn test_isize_array() {
    let t = StyleTypeValue::mk_type::<[isize; 2]>();
    let mut v = t.new_value();
    assert!(
        v.is_type::<[isize; 2]>(),
        "Expect v to be of type [isize; 2]"
    );
    v.from_string("73").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1]), Some(&[73, 73][..]));
    v.from_string("-6, 3").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1]), Some(&[-6, 3][..]));

    let t = StyleTypeValue::mk_type::<[isize; 4]>();
    let mut v = t.new_value();
    assert!(
        v.is_type::<[isize; 4]>(),
        "Expect v to be of type [isize; 4]"
    );
    v.from_string("1").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1, 2, 3]), Some(&[1, 1, 1, 1][..]));
    v.from_string("-6, 3").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1, 2, 3]), Some(&[-6, 3, -6, 3][..]));
    v.from_string("1 2 3 4").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1, 2, 3]), Some(&[1, 2, 3, 4][..]));
    v.from_string("5 6 7").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1, 2, 3]), Some(&[5, 6, 7, 5][..]));
}

//tf test_f64
#[test]
fn test_f64() {
    let t_f64 = StyleTypeValue::mk_type::<f64>();
    let mut v_f64 = t_f64.new_value();
    v_f64.from_string("73").unwrap();
    assert!(v_f64.is_type::<f64>(), "Expect f64 to be of type f64");
    assert_eq!(v_f64.as_t::<f64>(), &73.0, "Expect value to be f64 73");
    assert!(v_f64.eq(&73_f64));
    assert!(!v_f64.eq(&73_usize), "73 *f64* should not equal 73 *usize*");

    assert_eq!(v_f64.as_isize().unwrap(), 73);
    assert_eq!(v_f64.as_f64().unwrap(), 73.0);

    v_f64.from_string("1").unwrap();
    assert_eq!(v_f64.as_t::<f64>(), &1.0, "Expect value to be f64 1");
    assert!(v_f64.eq(&1_f64));
    assert!(!v_f64.eq(&1_usize), "1 *f64* should not equal 1 *usize*");

    assert_eq!(v_f64.as_isize().unwrap(), 1);
    assert_eq!(v_f64.as_f64().unwrap(), 1.0);
}

//tf test_f64_array
#[test]
fn test_f64_array() {
    let t = StyleTypeValue::mk_type::<[f64; 2]>();
    let mut v = t.new_value();
    assert!(v.is_type::<[f64; 2]>(), "Expect v to be of type [f64; 2]");
    v.from_string("73").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1]), Some(&[73, 73][..]));
    assert_eq!(v.as_floats(&mut [0., 1.]), Some(&[73., 73.][..]));
    v.from_string("-6, 3").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1]), Some(&[-6, 3][..]));
    assert_eq!(v.as_floats(&mut [0., 1.]), Some(&[-6., 3.][..]));

    let t = StyleTypeValue::mk_type::<[f64; 4]>();
    let mut v = t.new_value();
    assert!(v.is_type::<[f64; 4]>(), "Expect v to be of type [f64; 4]");
    v.from_string("1").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1, 2, 3]), Some(&[1, 1, 1, 1][..]));
    v.from_string("-6, 3").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1, 2, 3]), Some(&[-6, 3, -6, 3][..]));
    v.from_string("1 2 3 4").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1, 2, 3]), Some(&[1, 2, 3, 4][..]));
    v.from_string("5 6 7").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1, 2, 3]), Some(&[5, 6, 7, 5][..]));

    v.from_string("1").unwrap();
    assert_eq!(
        v.as_floats(&mut [0., 1., 2., 3.]),
        Some(&[1., 1., 1., 1.][..])
    );
    v.from_string("-6, 3").unwrap();
    assert_eq!(
        v.as_floats(&mut [0., 1., 2., 3.]),
        Some(&[-6., 3., -6., 3.][..])
    );
    v.from_string("1 2 3 4").unwrap();
    assert_eq!(
        v.as_floats(&mut [0., 1., 2., 3.]),
        Some(&[1., 2., 3., 4.][..])
    );
    v.from_string("5 6 7").unwrap();
    assert_eq!(
        v.as_floats(&mut [0., 1., 2., 3.]),
        Some(&[5., 6., 7., 5.][..])
    );
}

//tf test_option
/// Test an Option<T> - it should not care what T is
#[test]
fn test_option() {
    let t_opt_isize = StyleTypeValue::mk_type::<Option<isize>>();
    let mut v_opt_isize = t_opt_isize.new_value();

    assert_eq!(v_opt_isize.as_isize(), None,);

    assert_eq!(v_opt_isize.as_f64(), None,);

    v_opt_isize.from_string("73").unwrap();
    assert!(
        v_opt_isize.is_type::<Option<isize>>(),
        "Expect Option<isize> to be of type Option<isize>"
    );
    assert_eq!(
        v_opt_isize.as_t::<Option<isize>>(),
        &Some(73_isize),
        "Expect value to be Some(isize 73)"
    );
    assert_eq!(
        v_opt_isize.as_isize(),
        Some(73),
        "Expect value to be Some(isize 73)"
    );
    assert!(v_opt_isize.eq(&Some(73_isize)));
    assert!(
        !v_opt_isize.eq(&Some(73_usize)),
        "Some(73 *isize*) should not equal Some(73 *usize*)"
    );
    assert!(
        !v_opt_isize.eq(&Option::<isize>::None),
        "Some(73 *isize*) should not equal None",
    );

    v_opt_isize.from_string("").unwrap();
    assert_eq!(v_opt_isize.as_isize(), None,);
    assert_eq!(v_opt_isize.as_f64(), None,);
}
