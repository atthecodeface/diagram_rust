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
use hml::{Tag, Attribute};
use hml::reader::{Position, Span, ReaderError, Error};
use super::{NameIds, KnownName};

pub type MLResult<T, P, E> = std::result::Result<T,MLError<P, E>>;

//a MLError type
//tp MLError
#[derive(Debug)]
pub enum MLError<P, E>
where P:Position, E:Error<Position = P>
{
    EndOfStream,
    BadElementName(Span<P>, String, String),
    BadAttributeName(Span<P>, String),
    BadElement(Span<P>, String),
    BadMLEvent(Span<P>, String),
    BadValue(Span<P>, String),
    ParseError(ReaderError<P,E>),
    IOError(std::io::Error),
}

//ii MLError
impl <P, E> MLError<P, E>
where P:Position, E:Error<Position = P>
{

    //fi unexpected_end_of_stream
    #[allow(dead_code)]
    pub(crate) fn unexpected_end_of_stream() -> Self {
        Self::EndOfStream
    }

    //fi bad_element_name
    pub(crate) fn bad_element_name(namespace_stack:&hml::NamespaceStack<'_>, span:&Span<P>, tag:&Tag) -> Self {
        Self::BadElementName(span.clone(), tag.name.to_string(namespace_stack), String::new())
    }

    //fi bad_element_name_expected
    pub(crate) fn bad_element_name_expected(namespace_stack:&hml::NamespaceStack<'_>, span:&Span<P>, tag:&Tag, name_ids:&NameIds, expected:&'static [KnownName]) -> Self {
        let mut expectation = String::new();
        use std::fmt::Write;
        if expected.len() == 1 {
            let _ = write!(&mut expectation, ", expected '{}'",name_ids.str_of_name(&expected[0]));
        } else {
            let _ = write!(&mut expectation, ", expected one of ");
            for e in expected {
                let _ = write!(&mut expectation, " '{}'", name_ids.str_of_name(e));
            }
        }
        Self::BadElementName(span.clone(), tag.name.to_string(namespace_stack), expectation)
    }

    //fi bad_attribute_name
    pub(crate) fn bad_attribute_name(namespace_stack:&hml::NamespaceStack<'_>, span:&Span<P>, attr:&Attribute) -> Self {
        Self::BadAttributeName(span.clone(), attr.name.to_string(namespace_stack))
    }

    //fi bad_value
    pub(crate) fn bad_value(span:&Span<P>, reason:&str, value:&str) -> Self {
        Self::BadValue( span.clone(), format!("{}: '{}'", reason, value) )
    }

    //fi bad_attribute_name
    // fn bad_attribute_name(fp:&HmlFilePosition, name:&str) -> Self {
    // Self::BadAttributeName(fp.clone(), name.to_string())
    // }

    //mp bad_ml_event
    pub(crate) fn bad_ml_event(ewp:&hml::Event<Span<P>>) -> Self {
        Self::BadMLEvent(ewp.borrow_span().clone(), format!("{:?}", ewp))
    }

    //fi value_result
    pub(crate) fn value_result<V, Err:std::fmt::Display>(span:&Span<P>, result:Result<V,Err>) -> Result<V,Self> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(Self::BadValue(span.clone(), e.to_string())),
        }
    }

    //fi element_result
    pub(crate) fn element_result<V, Err:std::fmt::Display>(span:&Span<P>, result:Result<V,Err>) -> Result<V,Self> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(Self::BadElement(span.clone(), e.to_string())),
        }
    }

    //fi io_error
    pub(crate) fn io_error(e:std::io::Error) -> Self {
        Self::IOError(e)
    }

    //zz All done
}

//ip hml::reader::Error for MLError
impl <P, E> MLError<P, E>
where P:Position, E:Error<Position = P>
{
    //mp write_without_span
    /// Write the error without the span
    pub fn write_without_span(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        match self {
            Self::EndOfStream              => write!(f, "Unexpected end of XML event stream - bug in event source"),
            Self::BadElementName(span,n,e) => write!(f, "Bad element '{}'{}", n, e),
            Self::BadAttributeName(span,n) => write!(f, "Bad attribute '{}'", n),
            Self::BadElement(span,s)       => write!(f, "Element error '{}'", s),
            Self::BadMLEvent(span,s)       => write!(f, "Bad XML event {}", s),
            Self::BadValue(span,s )        => write!(f, "Bad value '{}'", s),
            Self::ParseError(e)            => e.write_without_span(f),
            Self::IOError(e)               => write!(f, "IO error '{}'", e),
        }
    }

    //mp borrow_span
    /// Borrow a span if it has one
    pub fn borrow_span(&self) -> Option<&Span<P>> {
        match self {
            Self::BadElementName(span,_,_)  => Some(span),
            Self::BadAttributeName(span,_)  => Some(span),
            Self::BadElement(span,_)        => Some(span),
            Self::BadMLEvent(span,_)        => Some(span),
            Self::BadValue(span,_)          => Some(span),
            Self::ParseError(e)             => e.borrow_span(),
            _ => None,
        }
    }

}

//ip std::fmt::Display for MLError
impl <P, E> std::fmt::Display for MLError<P, E>
where P:Position, E:Error<Position = P>
{
    //mp fmt - format a `MLError` for display
    /// Display the `MLError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.write_without_span(f)?;
        if let Some(span) = self.borrow_span() {
            write!(f, " at {}", span)
        } else {
            Ok(())
        }
    }

    //zz All done
}

//ip From<ReaderError> for MLError
impl <P, E> From<ReaderError<P, E>> for MLError<P, E>
where P:Position, E:Error<Position = P>
{
    fn from(e: ReaderError<P, E>) -> Self {
        MLError::ParseError(e)
        // Self::EndOfStream
    }
}

//ip From<IOError> for MLError
impl <P, E> From<std::io::Error> for MLError<P, E>
where P:Position, E:Error<Position = P>
{
    fn from(e: std::io::Error) -> Self {
        Self::io_error(e)
    }
}
