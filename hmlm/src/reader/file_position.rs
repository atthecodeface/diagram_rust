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

@file    file_position.rs
@brief   A file positiion (line/character)
 */

//a Documentation

//a Imports
use super::utils;

//a FilePosition
//tp FilePosition
/// Holds the line number and character position of a character in a file
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FilePosition {
    /// Line, with first line being 0
    pub ln: usize,
    /// Column, starting with 0
    pub ch: usize,
}

//ip std::fmt::Display for FilePosition
impl std::fmt::Display for FilePosition {

    //mp fmt - format a `CharError` for display
    /// Display the `FilePosition` as line and column
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line {} column {}", self.ln+1, self.ch+1)
    }

    //zz All done
}

//ip FilePosition
impl FilePosition {

    //fp new - Create a new file position
    /// Create a new `FilePosition`, at line 0 character 0
    pub fn new() -> FilePosition {
        FilePosition { ln:0, ch:0 }
    }
    
    //mp move_by - Move the position on by a character
    /// Move the file position on by a character, accounting for newlines
    pub fn move_by(&mut self, ch:char) -> () {
        self.ch += 1;
        if utils::is_newline(ch as u32) {
            self.ln += 1;
            self.ch = 0;
        }
    }

    //zz All done
}

