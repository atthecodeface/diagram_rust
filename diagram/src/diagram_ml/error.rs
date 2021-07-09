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
use hml::reader::{Position, Span};
use hml::reader::Error as ReaderError;

pub type MLResult<T, P, R> = std::result::Result<T,MLError<P, R>>;

//a MLError type
//tp MLError
#[derive(Debug)]
pub enum MLError<P, R>
where P:Position, R:std::error::Error + 'static
{
    EndOfStream,
    BadElementName(Span<P>, String),
    BadAttributeName(Span<P>, String),
    BadElement(Span<P>, String),
    BadMLEvent(Span<P>, String),
    BadValue(Span<P>, String),
    ParseError(ReaderError<P,R>),
    IOError(std::io::Error),
}

//ii MLError
impl <P, R> MLError<P, R>
where P:Position, R:std::error::Error + 'static
{

    //fi unexpected_end_of_stream
    #[allow(dead_code)]
    pub(crate) fn unexpected_end_of_stream() -> Self {
        Self::EndOfStream
    }

    //fi bad_element_name
    pub(crate) fn bad_element_name(namespace_stack:&hml::NamespaceStack<'_>, span:&Span<P>, tag:&Tag) -> Self {
        Self::BadElementName(span.clone(), tag.name.to_string(namespace_stack))
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
    pub(crate) fn value_result<V, E:std::fmt::Display>(span:&Span<P>, result:Result<V,E>) -> Result<V,Self> {
        match result {
            Ok(v) => Ok(v),
            Err(e) => Err(Self::BadValue(span.clone(), e.to_string())),
        }
    }

    //fi element_result
    pub(crate) fn element_result<V, E:std::fmt::Display>(span:&Span<P>, result:Result<V,E>) -> Result<V,Self> {
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

//ip std::fmt::Display for MLError
impl <P, R> std::fmt::Display for MLError<P, R>
where P:Position, R:std::error::Error + 'static
{
    //mp fmt - format a `MLError` for display
    /// Display the `MLError` in a human-readable form
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EndOfStream              => write!(f, "Unexpected end of XML event stream - bug in event source"),
            Self::BadElementName(span,n)   => write!(f, "Bad element '{}' at {}", n, span),
            Self::BadAttributeName(span,n) => write!(f, "Bad attribute '{}' at {}", n, span),
            Self::BadElement(span,s)       => write!(f, "Element error '{}' at {}", s, span),
            Self::BadMLEvent(span,s)       => write!(f, "Bad XML event {} at {}", s, span),
            Self::BadValue(span,s )        => write!(f, "Bad value '{}' at {}", s, span),
            Self::ParseError(s)            => write!(f, "Parse error '{}'", s),
            Self::IOError(e)               => write!(f, "IO error '{}'", e),
        }
    }

    //zz All done
}

//ip From<ReaderError> for MLError
impl <P, R> From<ReaderError<P, R>> for MLError<P, R>
where P:Position, R:std::error::Error + 'static
{
    fn from(e: ReaderError<P, R>) -> Self {
        MLError::ParseError(e)
        // Self::EndOfStream
    }
}

//ip From<IOError> for MLError
impl <P, R> From<std::io::Error> for MLError<P, R>
where P:Position, R:std::error::Error + 'static
{
    fn from(e: std::io::Error) -> Self {
        Self::io_error(e)
    }
}
