//! Contains an HMLH writer based on the xml-rs EventWriter
//!
//! The most important type in this module is `EventWriter` which allows writing an HMLH format
//! XML document to some output stream.

extern crate xml;
pub use xml::writer::XmlEvent;

use std::io::prelude::*;
mod emitter;
pub use super::{HmlmResult, HmlmError};

/// A wrapper around an `std::io::Write` instance which emits XML document according to provided
/// events.
pub struct EventWriter<W> {
    sink:    W,
    emitter: emitter::Emitter,
}

impl<W: Write> EventWriter<W> {
    /// Creates a new `EventWriter` out of an `std::io::Write` instance using the default
    /// configuration.
    #[inline]
    pub fn new(sink: W) -> EventWriter<W> {
        EventWriter {
            sink,
            emitter: emitter::Emitter::new(),
        }
    }

    /// Writes the next piece of HMLM document according to the provided event.
    ///
    pub fn write<'a, E>(&mut self, event: E) -> HmlmResult<()>
    where E: Into<XmlEvent<'a>> {
        match event.into() {
            XmlEvent::StartDocument { version, encoding, standalone } => {
                Ok(())
            },
            XmlEvent::ProcessingInstruction { name, data } => {
                Ok(())
            }
            XmlEvent::StartElement { name, attributes, namespace } => {
                println!("{:#?}",namespace);
                // self.emitter.namespace_stack_mut().push_empty().checked_target().extend(namespace.as_ref());
                self.emitter.emit_start_element(&mut self.sink, name, &attributes)
            }
            XmlEvent::EndElement { name } => {
                let r = self.emitter.emit_end_element(&mut self.sink, name);
                // self.emitter.namespace_stack_mut().try_pop();
                r
            }
            XmlEvent::Comment(content) => {
                self.emitter.emit_comment(&mut self.sink, content)
            },
            XmlEvent::CData(content) => {
                self.emitter.emit_cdata(&mut self.sink, content)
            },
            XmlEvent::Characters(content) => {
                self.emitter.emit_characters(&mut self.sink, content)
            },
        }
    }

    /// Returns a mutable reference to the underlying `Writer`.
    ///
    /// Note that having a reference to the underlying sink makes it very easy to emit invalid XML
    /// documents. Use this method with care. Valid use cases for this method include accessing
    /// methods like `Write::flush`, which do not emit new data but rather change the state
    /// of the stream itself.
    pub fn inner_mut(&mut self) -> &mut W {
        &mut self.sink
    }

    /// Unwraps this `EventWriter`, returning the underlying writer.
    ///
    /// Note that this is a destructive operation: unwrapping a writer and then wrapping
    /// it again with `EventWriter::new()` will create a fresh writer whose state will be
    /// blank; for example, accumulated namespaces will be reset.
    pub fn into_inner(self) -> W {
        self.sink
    }

}
