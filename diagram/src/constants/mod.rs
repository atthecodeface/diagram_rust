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

@file    constants/mod.rs
@brief   Constants for names of elements and attributes
 */
macro_rules! att {
    ( $n:ident , $v:literal ) => {
        /// Attribute
        pub const $n : & str = $v ;
    };
}

macro_rules! tag {
    ( $n:ident , $v:literal ) => {
        /// Tag
        pub const $n : & str = $v ;
    };
}

pub mod attributes {
    att!(DEBUG,       "debug");
    att!(BBOX,        "bbox");

    att!(GRID,        "grid");
    att!(GRIDX,       "gridx");
    att!(GRIDY,       "gridy");
    att!(MINX,        "minx");
    att!(MINY,        "miny");
    att!(GROWX,       "growx");
    att!(GROWY,       "growy");

    att!(PLACE,       "place");

    att!(ANCHOR,      "anchor");
    att!(EXPAND,      "expand");

    att!(PAD,         "pad");
    att!(MARGIN,      "margin");
    att!(BG,          "bg");
    att!(BORDERWIDTH, "border-width");
    att!(BORDERROUND, "border-round");
    att!(BORDERCOLOR, "border-color");
    att!(SCALE,       "scale");
    att!(ROTATE,      "rotate");
    att!(TRANSLATE,   "translate");
    att!(FILL,        "fill-color");
    att!(STROKE,      "stroke-color");
    att!(STROKEWIDTH, "stroke-width");
    att!(WIDTH,       "width");
    att!(HEIGHT,      "height");
    att!(COORDS,      "coords");
    att!(ROUND,       "round");
    att!(STELLATE,    "stellate");
    att!(MARKERS,     "markers");
    att!(FONT,        "font-family");
    att!(FONTSIZE,    "font-size");
    att!(FONTWEIGHT,  "font-weight");
    att!(FONTSTYLE,   "font-style");
    att!(VERTICES,    "vertices");
    att!(FLAGS,       "flags");
    att!(REF,         "ref");
    att!(POINT,       "point");
    att!(RELIEF,      "relief");
}
pub mod elements {
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
}
