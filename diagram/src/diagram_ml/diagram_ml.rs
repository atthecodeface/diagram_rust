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
use crate::{Diagram};
use super::{MLReader};
use hml_rs::names::{Namespace};
use hml_rs::reader::Reader    as HmlReader;
use hml_rs::string::Reader    as StringReader;

//a DiagramML
//tp DiagramML
/// The `DiagramML` structure is used to construct a diagram from
/// mark-up, be that XML or HML.
///
/// # Example
///
/// ```
/// extern crate diagram;
/// use diagram::{Diagram, DiagramDescriptor, DiagramML};
/// let style_set = DiagramDescriptor::create_style_set();
/// let diagram_descriptor = DiagramDescriptor::new(&style_set);
/// let mut diagram  = Diagram::new(&diagram_descriptor);
/// let mut dml      = DiagramML::new(&mut diagram);
/// dml.read_file("#diagram ##defs ###rect id=a ##rect ##group ###rect ##rect".as_bytes(), false).unwrap();
/// let (_, contents, _) = diagram.borrow_contents_descriptor();
/// assert_eq!(1, contents.definitions.len(), "One definition expected from this");
/// // assert_eq!(3, contents.root.elements.len(), "Three elements (rect, group, rect) expected from this");
/// ```
pub struct DiagramML<'a, 'diag> {
    diagram: &'a mut Diagram<'diag>,
}

//ip DiagramML
impl <'a, 'diag> DiagramML<'a, 'diag> {
    //fp new
    /// Create a new mark-up diagram reader `DiagramML`, for the provided diagram.
    ///
    /// The diagram is borrowed mutably, and is obviously then held
    /// until the reader has completed reading the file.
    ///
    /// It is possible that the reader will support including other
    /// files within a file being read; this will require the reader
    /// to invoke a new reader with the new file.
    pub fn new(d:&'a mut Diagram<'diag>) -> Self {
        Self { diagram:d }
    }

    //mp read_file
    /// Read a file as HML (currently), using its contents to build
    /// the `Diagram` that this reader is constructing.
    pub fn read_file<F:std::io::Read>(&mut self, mut f:F, is_library:bool) -> Result<(), Vec<String>> {
        let mut namespace = Namespace::new(true);
        let mut contents = String::new();
        let mut reader = {
            match StringReader::of_file(&mut f, &mut contents) {
                Err(e) => { let mut r=Vec::new(); r.push(format!("{}",e)); return Err(r);},
                Ok(x) => x,
            }
        };
        let (descriptor, contents, stylesheet) = self.diagram.borrow_contents_descriptor();
        let mut ml_reader = MLReader::new(contents, stylesheet, &mut namespace, &mut reader);
        match ml_reader.read_file(descriptor, is_library) {
            Err(mut err_list) => {
                let mut r = Vec::new();
                for e in err_list.take() {
                    let mut s = String::new();
                    use std::fmt::Write;
                    if let Some(span) = e.borrow_span() {
                        reader.fmt_context(&mut s, span.start(), span.end()).unwrap();
                        e.write_without_span(&mut s).unwrap();
                        write!(&mut s, " at {}", span.start() ).unwrap();
                    } else {
                        write!(&mut s, "{}", e).unwrap();
                    }
                    r.push(s);
                }
                Err(r)
            },
            _ => Ok(())
        }
    }

    //zz All done
}

//a Test
#[cfg(test)]
mod tests {
    use crate::{Diagram, DiagramDescriptor, DiagramML};
    #[test]
    fn test_why() {
        let style_set = DiagramDescriptor::create_style_set();
        let diagram_descriptor = DiagramDescriptor::new(&style_set);
        let mut diagram = Diagram::new(&diagram_descriptor);
        let mut dml     = DiagramML::new(&mut diagram);
        dml.read_file("#diagram".as_bytes(),false).unwrap();
        let (_, contents, _) = diagram.borrow_contents_descriptor();
        assert_eq!(0, contents.definitions.len());
    }
}
