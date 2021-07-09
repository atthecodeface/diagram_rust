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

@file    mod.rs
@brief   Diagram Markup Reader module
 */

//a Imports
use super::{MLError, MLResult};
use hml::reader::Position;

//a MLErrorList
//tp MLErrorList
/// An array of errors; if the array is zero-length after reading a
/// file, then there is no error.
///
/// Otherwise it is an accumulation of errors from reading the
/// file. Since some errors may indicate failure to correctly parse a
/// file, only the first error is guaranteed to be valid, but it is
/// useful to get a list of errors for when only minor attribute value
/// errors are returned.
#[derive(Debug)]
pub struct MLErrorList <P, R>
where P:Position, R:std::error::Error + 'static
{
    errors : Vec<MLError<P, R>>,
}

//ip MLErrorList
impl <P, R> MLErrorList<P, R>
where P:Position, R:std::error::Error + 'static
{
    //fp new
    /// Create a new MLErrorList
    pub fn new() -> Self {
        Self { errors : Vec::new() }
    }

    //mp add
    /// Add an error to the list
    pub fn add(&mut self, e:MLError<P, R>) -> () {
        self.errors.push(e);
    }

    //mp update
    /// Update the MLErrorList from a result; this returns () so the
    /// error is effectively caught and recorded. Subsequent errors
    /// are therefore less reliable.
    pub fn update<T>(&mut self, e:MLResult<T, P, R>) {
        match e {
            Err(e) => { self.errors.push(e); }
            _ => (),
        }
    }

    //mp take
    fn take(&mut self) -> Vec<MLError<P, R>> {
        std::mem::replace(&mut self.errors, Vec::new())
    }

    //mp as_err
    /// Return a result of 'Ok(x)' if this error list is empty, or
    /// 'Err(MLErrorList)' if the error list has contents. It cleans
    /// the current error list.
    pub fn as_err<T>(&mut self, v:T) -> Result<T, MLErrorList<P, R>> {
        let x = self.take();
        match x.len() {
            0 => { Ok(v) },
            _ => Err(Self{errors:x})
        }
    }

    //zz All done
}

//ip std::fmt::Display for MLErrorList
impl <P, R> std::fmt::Display for MLErrorList<P, R>
where P:Position, R:std::error::Error + 'static
{
    //mp fmt
    /// Display the [MLErrorList] for humans
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for e in &self.errors {
            write!(f, "{}\n", *e)?;
        }
        Ok(())
    }
}

//ip From<ReaderError> for MLError
impl <P, R> From<std::io::Error> for MLErrorList<P, R>
where P:Position, R:std::error::Error + 'static
{
    fn from(e: std::io::Error) -> Self {
        let mut el = Self::new();
        el.add(e.into());
        el
    }
}
