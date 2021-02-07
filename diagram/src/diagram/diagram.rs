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
@brief   Diagram module
 */

//a Imports
use std::collections::HashMap;
use super::types::*;
use super::DiagramDescriptor;
use super::Element;

//a Diagram Definition
//tp Definition - item with a Diagram that is not displayed, but may be 'used'
pub struct Definition {
    name    : String,
    elements : Vec<Element>,
}

//tp Diagram
pub struct Diagram<'a> {
    pub descriptor  : DiagramDescriptor<'a>,
    pub definitions : Vec<Definition>,
    pub elements    : Vec<Element>,
}

//ti Diagram
impl <'a> Diagram <'a> {
    pub fn new() -> Self {
        Self { descriptor: DiagramDescriptor::new(),
               definitions:Vec::new(),
               elements:Vec::new(),
        }
    }
    pub fn styles(&self, tag:&str) -> Option<&StyleDescriptor> {
        self.descriptor.get(tag)
    }
    pub fn uniquify(&mut self) -> Result<(),()> {
        Ok(())
    }
    pub fn style(&mut self) -> Result<(),()> {
        Ok(())
    }
    pub fn layout(&mut self) -> Result<(),()> {
        Ok(())
    }
    pub fn geometry(&mut self) -> Result<(),()> {
        Ok(())
    }
    pub fn iter_elements(&mut self) -> Result<(),()> {
        Ok(())
    }
}
    
