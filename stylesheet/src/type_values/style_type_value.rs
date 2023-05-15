//a Imports
// use erased_serde::{Serialize, Serializer};
use serde::{Serialize, Serializer};
use std::any::{Any, TypeId};

use crate::{TypeValue, ValueError};

//a StyleTypeValue
//tp StyleTypeValue
/// This is a type of a style or the value of such a type.
///
/// The normal use is that a style is declared to be of a type such as:
///
///    let t = StyleTypeValue::<[isize; 2]>::mk_type();
///
/// Then new values are created using
///
///    let mut v = t.new_value();
///    v.from_string("73, 46");
///
/// and the contents can be interrogated somewhat type-agnostically using
///
///    assert_eq!(v.as_ints(&mut [0; 2]), Some(&[73,46][..]));
///
#[derive(Debug)]
pub struct StyleTypeValue {
    value: Box<dyn TypeValue>,
}

//ip Display for StyleTypeValue
impl std::fmt::Display for StyleTypeValue {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(&self.value, fmt)
    }
}

//ip Clone for StyleTypeValue
impl std::clone::Clone for StyleTypeValue {
    fn clone(&self) -> Self {
        let value = self.value.clone_value();
        Self { value }
    }
}

//ip PartialEq for StyleTypeValue
impl std::cmp::PartialEq<StyleTypeValue> for StyleTypeValue {
    fn eq(&self, other: &Self) -> bool {
        self.equals(&other.value)
    }
}

//ip PartialOrd for StyleTypeValue
impl std::cmp::PartialOrd<StyleTypeValue> for StyleTypeValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.compare(&other.value)
    }
}

//ip Serialize for StyleTypeValue
impl Serialize for StyleTypeValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(f) = self.value.as_serialize() {
            f.serialize(serializer)
        } else {
            "No serializer".serialize(serializer)
        }
    }
}

//ip StyleTypeValue
impl StyleTypeValue {
    //cp mk_type
    /// Create a [StyleTypeValue] that represents a type (which must
    /// be TypeValue and Default)
    ///
    /// Once the type has been created, values can be created from it
    pub fn mk_type<T: TypeValue + Default>() -> Self {
        Self::new(T::default())
    }

    //mp new_value
    /// Create a [StyleTypeValue] from this that is a type
    pub fn new_value(&self) -> Self {
        let value = self.value.mk_value();
        Self { value }
    }

    //cp new
    /// Create a [StyleTypeValue] from a *value* of a type that must
    /// be TypeValue - but not necessarily Default.
    ///
    /// Once the type has been created, values can be created from it
    pub fn new<T: TypeValue>(value: T) -> Self {
        let value = Box::new(value);
        Self { value }
    }

    //mp of_type
    /// Create a new [StyleTypeValue] of the same type as this one,
    /// with its 'standard' value
    pub fn of_type(&self) -> Self {
        let value = self.value.mk_value();
        Self { value }
    }

    //mp is_t
    /// Returns true if this [StyleTypeValue] contains the type T
    pub fn is_t<T: TypeValue>(&self) -> bool {
        self.value.as_ref().as_any().type_id() == TypeId::of::<T>()
    }

    //mp as_t
    /// Returns Some(&T) if this [StyleTypeValue] contains a value of
    /// the type T; None if it does not.
    ///
    /// This is useful when the type of [StyleTypeValue] is
    /// *explicitly* known - as is usually the case for style values
    pub fn as_t<T: TypeValue>(&self) -> Option<&T> {
        self.value.as_any().downcast_ref::<T>()
    }

    //mp as_mut_t
    /// Returns Some(&mut T) if this [StyleTypeValue] contains a value of
    /// the type T; None if it does not.
    ///
    /// This is useful when the type of [StyleTypeValue] is
    /// *explicitly* known - as is usually the case for style values
    pub fn as_mut_t<T: TypeValue>(&mut self) -> Option<&mut T> {
        self.value.as_any_mut().downcast_mut::<T>()
    }

    //mp compare
    /// Compare the value of this [StyleTypeValue] to another value of
    /// type T; if the [StyleTypeValue] is *not* of type T then it
    /// will should None; on a true success it returns Some(Ordering).
    pub fn compare<T: Any>(&self, other: &T) -> Option<std::cmp::Ordering> {
        self.value.cmp(other as &dyn Any)
    }

    //mp equals
    /// Compare the value of this [StyleTypeValue] to another value of type T.
    ///
    /// It only returns true *IF* the [StyleTypeValue] is of type T
    /// and the value compares as 'Equal'
    pub fn equals<T: Any>(&self, other: &T) -> bool {
        self.compare(other) == Some(std::cmp::Ordering::Equal)
    }

    //mp as_f64
    pub fn as_f64(&self) -> Option<f64> {
        let mut data = [0.];
        if self.value.get_floats(&mut data).is_some() {
            Some(data[0])
        } else {
            None
        }
    }

    //mp as_str
    pub fn as_str(&self) -> Option<&str> {
        let mut data = [""];
        let has_strs = self.value.get_strs(&mut data).is_some();
        if has_strs {
            Some(data[0])
        } else {
            None
        }
    }

    //ap as_isize
    /// Return the value as an isize, if the type provides such access
    /// (for example, f64 does, as does isize)
    ///
    /// If not it returns None
    ///
    /// If the type of the value of the [StyleTypeValue] is known then
    /// this can provide simple access to the data
    pub fn as_isize(&self) -> Option<isize> {
        let mut data = [0];
        if self.value.get_ints(&mut data).is_some() {
            Some(data[0])
        } else {
            None
        }
    }

    //ap as_ints
    /// Fill a slice with the contents of the [StyleTypeValue] if it
    /// can be represented as one or more isize
    ///
    /// It returns the slice that was filled (so the length of the slice indicates the number of valid values).
    ///
    /// It returns None if the type does not support access as isize (e.g. a String)
    ///
    /// If the type of the value of the [StyleTypeValue] is known then
    /// this can provide simple access to the data
    pub fn as_ints<'a>(&self, data: &'a mut [isize]) -> Option<&'a [isize]> {
        self.value.get_ints(data)
    }

    //ap as_floats
    /// Fill a slice with the contents of the [StyleTypeValue] if it
    /// can be represented as one or more f64
    ///
    /// It returns the slice that was filled (so the length of the slice indicates the number of valid values).
    ///
    /// It returns None if the type does not support access as f64 (e.g. a String)
    ///
    /// If the type of the value of the [StyleTypeValue] is known then
    /// this can provide simple access to the data
    pub fn as_floats<'a>(&self, data: &'a mut [f64]) -> Option<&'a [f64]> {
        self.value.get_floats(data)
    }
    //ap as_strs
    pub fn as_strs<'a, 'b>(&'b self, data: &'a mut [&'b str]) -> Option<&'a [&'b str]> {
        self.value.get_strs(data)
    }
    //ap as_vec_int
    /// Create a Vec with the contents of the [StyleTypeValue] if it
    /// can be represented as one or more isize
    ///
    /// It returns a Vec if the type provides an isize method
    ///
    /// It returns None if the type does not support access as isize (e.g. a String)
    pub fn as_vec_int(&self) -> Option<Vec<isize>> {
        if self.value.is_none() {
            None
        } else {
            let n = self.value.len();
            let mut data = vec![0_isize; n];
            let n = self.value.get_ints(&mut data).map(|x| x.len()).unwrap_or(0);
            data.truncate(n);
            Some(data)
        }
    }

    //ap as_vec_float
    /// Create a Vec with the contents of the [StyleTypeValue] if it
    /// can be represented as one or more isize
    ///
    /// It returns a Vec if the type provides an isize method
    ///
    /// It returns None if the type does not support access as isize (e.g. a String)
    pub fn as_vec_float(&self) -> Option<Vec<f64>> {
        if self.value.is_none() {
            None
        } else {
            let n = self.value.len();
            let mut data = vec![0.0_f64; n];
            let n = self
                .value
                .get_floats(&mut data)
                .map(|x| x.len())
                .unwrap_or(0);
            data.truncate(n);
            Some(data)
        }
    }

    //ap as_vec_str
    /// Create a Vec with the contents of the [StyleTypeValue] if it
    /// can be represented as one or more isize
    ///
    /// It returns a Vec if the type provides an isize method
    ///
    /// It returns None if the type does not support access as isize (e.g. a String)
    pub fn as_vec_str(&self) -> Option<Vec<&str>> {
        if self.value.is_none() {
            None
        } else {
            let n = self.value.len();
            let mut data = vec![""; n];
            let n = self.value.get_strs(&mut data).map(|x| x.len()).unwrap_or(0);
            data.truncate(n);
            Some(data)
        }
    }

    //ap type_name
    /// Get the type name as a [String]
    pub fn type_name(&self) -> String {
        self.value.type_name()
    }

    //cp value_of_string
    /// Get a value from a string
    #[inline]
    pub fn value_of_string(&self, s: &str) -> Result<Self, ValueError> {
        let mut v = self.new_value();
        v.value.parse_string(s, false)?;
        Ok(v)
    }

    //zz OLLD
    // From TypeValue
    //mp as_type
    pub fn as_type(&self) -> Self {
        self.clone()
    }
    //mp from_string
    /// Set the value from a string
    #[inline]
    pub fn from_string<'a>(&'a mut self, s: &str) -> Result<&'a mut Self, ValueError> {
        self.value.parse_string(s, false)?;
        Ok(self)
    }

    //ap eq_string
    pub fn eq_string(&self, s: &str) -> bool {
        self.value.has_string(s, false)
    }
    //fp rgb
    pub fn rgb(rgb: Option<(f64, f64, f64)>) -> Self {
        let mut t = Self::new::<Option<[f64; 3]>>(None);
        if let Some((r, g, b)) = rgb {
            *(t.as_mut_t::<Option<[f64; 3]>>().unwrap().as_mut().unwrap()) = [r, g, b];
        }
        t
    }
}

//a Test suite
//tf test_isize
#[test]
fn test_isize() {
    let t_isize = StyleTypeValue::mk_type::<isize>();
    let mut v_isize = t_isize.new_value();
    v_isize.from_string("73").unwrap();
    assert!(v_isize.is_t::<isize>(), "Expect isize to be of type isize");
    assert_eq!(
        v_isize.as_t::<isize>(),
        Some(&73_isize),
        "Expect value to be isize 73"
    );
    assert!(v_isize.equals(&73_isize));
    assert!(
        !v_isize.equals(&73_usize),
        "73 *isize* should not equal 73 *usize*"
    );
    assert_eq!(v_isize.as_isize().unwrap(), 73);
    assert_eq!(v_isize.as_f64().unwrap(), 73.0);

    v_isize.from_string("1").unwrap();
    assert_eq!(
        v_isize.as_t::<isize>(),
        Some(&1_isize),
        "Expect value to be isize 1"
    );
    assert!(v_isize.equals(&1_isize));
    assert!(
        !v_isize.equals(&1_usize),
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
    assert!(v.is_t::<[isize; 2]>(), "Expect v to be of type [isize; 2]");
    v.from_string("73").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1]), Some(&[73, 73][..]));
    v.from_string("-6, 3").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1]), Some(&[-6, 3][..]));

    let t = StyleTypeValue::mk_type::<[isize; 4]>();
    let mut v = t.new_value();
    assert!(v.is_t::<[isize; 4]>(), "Expect v to be of type [isize; 4]");
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
    assert!(v_f64.is_t::<f64>(), "Expect f64 to be of type f64");
    assert_eq!(
        v_f64.as_t::<f64>(),
        Some(&73.0_f64),
        "Expect value to be f64 73"
    );
    assert!(v_f64.equals(&73_f64));
    assert!(
        !v_f64.equals(&73_usize),
        "73 *f64* should not equal 73 *usize*"
    );

    assert_eq!(v_f64.as_isize().unwrap(), 73);
    assert_eq!(v_f64.as_f64().unwrap(), 73.0);

    v_f64.from_string("1").unwrap();
    assert_eq!(
        v_f64.as_t::<f64>(),
        Some(&1.0_f64),
        "Expect value to be f64 1"
    );
    assert!(v_f64.equals(&1_f64));
    assert!(
        !v_f64.equals(&1_usize),
        "1 *f64* should not equal 1 *usize*"
    );

    assert_eq!(v_f64.as_isize().unwrap(), 1);
    assert_eq!(v_f64.as_f64().unwrap(), 1.0);
}

//tf test_f64_array
#[test]
fn test_f64_array() {
    let t = StyleTypeValue::mk_type::<[f64; 2]>();
    let mut v = t.new_value();
    assert!(v.is_t::<[f64; 2]>(), "Expect v to be of type [f64; 2]");
    v.from_string("73").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1]), Some(&[73, 73][..]));
    assert_eq!(v.as_floats(&mut [0., 1.]), Some(&[73., 73.][..]));
    v.from_string("-6, 3").unwrap();
    assert_eq!(v.as_ints(&mut [0, 1]), Some(&[-6, 3][..]));
    assert_eq!(v.as_floats(&mut [0., 1.]), Some(&[-6., 3.][..]));

    let t = StyleTypeValue::mk_type::<[f64; 4]>();
    let mut v = t.new_value();
    assert!(v.is_t::<[f64; 4]>(), "Expect v to be of type [f64; 4]");
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
        v_opt_isize.is_t::<Option<isize>>(),
        "Expect Option<isize> to be of type Option<isize>"
    );
    assert_eq!(
        v_opt_isize.as_t::<Option<isize>>(),
        Some(&Some(73_isize)),
        "Expect value to be Some(isize 73)"
    );
    assert_eq!(
        v_opt_isize.as_isize(),
        Some(73),
        "Expect value to be Some(isize 73)"
    );
    assert!(v_opt_isize.equals(&Some(73_isize)));
    assert!(
        !v_opt_isize.equals(&Some(73_usize)),
        "Some(73 *isize*) should not equal Some(73 *usize*)"
    );
    assert!(
        !v_opt_isize.equals(&Option::<isize>::None),
        "Some(73 *isize*) should not equal None",
    );

    v_opt_isize.from_string("").unwrap();
    assert_eq!(v_opt_isize.as_isize(), None,);
    assert_eq!(v_opt_isize.as_f64(), None,);
}

//tf test_serialize
/// Test Serialize
#[test]
fn test_serialize() {
    let t_opt_isize = StyleTypeValue::mk_type::<Option<isize>>();
    let t_i4 = StyleTypeValue::mk_type::<[isize; 4]>();
    assert_eq!(serde_json::to_string(&t_opt_isize).unwrap(), "null");
    assert_eq!(
        serde_json::to_string(&t_opt_isize.value_of_string("1").unwrap()).unwrap(),
        "1"
    );
    assert_eq!(
        serde_json::to_string(&t_opt_isize.value_of_string("-100").unwrap()).unwrap(),
        "-100"
    );
    assert_eq!(
        serde_json::to_string(&t_i4.value_of_string("-100").unwrap()).unwrap(),
        "[-100,-100,-100,-100]"
    );
    assert_eq!(
        serde_json::to_string(&t_i4.value_of_string("-100,2").unwrap()).unwrap(),
        "[-100,2,-100,2]"
    );
}
