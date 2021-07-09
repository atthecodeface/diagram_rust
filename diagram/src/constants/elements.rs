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

@file    elements.rs
@brief   Constants for elements in a diagram
 */
macro_rules! tag {
    ( $n:ident , $v:literal ) => {
        /// Tag
        pub const $n : & str = $v ;
    };
}

    tag!(MARKER,   "marker");
    tag!(USE,      "use");
    tag!(DIAGRAM,  "diagram");
    tag!(GROUP,    "group");
    tag!(LAYOUT,   "layout");
    tag!(RECT,     "rect");
    tag!(CIRCLE,   "circle");
    tag!(POLYGON,  "polygon");
    tag!(TEXT,     "text");
    tag!(PATH,     "path");

pub enum Typ {
    Marker,
    Use,
    Diagram,
    Group,
    Layout,
    Rect,
    Circle,
    Polygon,
    Text,
    Path,
}

