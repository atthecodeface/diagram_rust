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

@file    color.rs
@brief   Colors and its dictionary

Arial (sans-serif)
Verdana (sans-serif)
Helvetica (sans-serif)
Tahoma (sans-serif)
Trebuchet MS (sans-serif)
Times New Roman (serif)
Georgia (serif)
Garamond (serif)
Courier New (monospace)
Brush Script MT (cursive)

      

 */

use std::collections::HashMap;
const COLOR_DICTIONARY : [(u32, &str);147] = [
    (0xfff8f0, "aliceblue"),
    (0xd7ebfa, "antiquewhite"),
    (0xffff00, "aqua"),
    (0xd4ff7f, "aquamarine"),
    (0xfffff0, "azure"),
    (0xdcf5f5, "beige"),
    (0xc4e4ff, "bisque"),
    (0x000000, "black"),
    (0xcdebff, "blanchedalmond"),
    (0xff0000, "blue"),
    (0xe22b8a, "blueviolet"),
    (0x2a2aa5, "brown"),
    (0x87b8de, "burlywood"),
    (0xa09e5f, "cadetblue"),
    (0x00ff7f, "chartreuse"),
    (0x1e69d2, "chocolate"),
    (0x507fff, "coral"),
    (0xed9564, "cornflowerblue"),
    (0xdcf8ff, "cornsilk"),
    (0x3c14dc, "crimson"),
    (0xffff00, "cyan"),
    (0x8b0000, "darkblue"),
    (0x8b8b00, "darkcyan"),
    (0x0b86b8, "darkgoldenrod"),
    (0xa9a9a9, "darkgray"),
    (0x006400, "darkgreen"),
    (0xa9a9a9, "darkgrey"),
    (0x6bb7bd, "darkkhaki"),
    (0x8b008b, "darkmagenta"),
    (0x2f6b55, "darkolivegreen"),
    (0x008cff, "darkorange"),
    (0xcc3299, "darkorchid"),
    (0x00008b, "darkred"),
    (0x7a96e9, "darksalmon"),
    (0x8fbc8f, "darkseagreen"),
    (0x8b3d48, "darkslateblue"),
    (0x4f4f2f, "darkslategray"),
    (0x4f4f2f, "darkslategrey"),
    (0xd1ce00, "darkturquoise"),
    (0xd30094, "darkviolet"),
    (0x9314ff, "deeppink"),
    (0xffbf00, "deepskyblue"),
    (0x696969, "dimgray"),
    (0x696969, "dimgrey"),
    (0xff901e, "dodgerblue"),
    (0x2222b2, "firebrick"),
    (0xf0faff, "floralwhite"),
    (0x228b22, "forestgreen"),
    (0xff00ff, "fuchsia"),
    (0xdcdcdc, "gainsboro"),
    (0xfff8f8, "ghostwhite"),
    (0x00d7ff, "gold"),
    (0x20a5da, "goldenrod"),
    (0x808080, "gray"),
    (0x808080, "grey"),
    (0x008000, "green"),
    (0x2fffad, "greenyellow"),
    (0xf0fff0, "honeydew"),
    (0xb469ff, "hotpink"),
    (0x5c5ccd, "indianred"),
    (0x82004b, "indigo"),
    (0xf0ffff, "ivory"),
    (0x8ce6f0, "khaki"),
    (0xfae6e6, "lavender"),
    (0xf5f0ff, "lavenderblush"),
    (0x00fc7c, "lawngreen"),
    (0xcdfaff, "lemonchiffon"),
    (0xe6d8ad, "lightblue"),
    (0x8080f0, "lightcoral"),
    (0xffffe0, "lightcyan"),
    (0xd2fafa, "lightgoldenrodyellow"),
    (0xd3d3d3, "lightgray"),
    (0x90ee90, "lightgreen"),
    (0xd3d3d3, "lightgrey"),
    (0xc1b6ff, "lightpink"),
    (0x7aa0ff, "lightsalmon"),
    (0xaab220, "lightseagreen"),
    (0xface87, "lightskyblue"),
    (0x998877, "lightslategray"),
    (0x998877, "lightslategrey"),
    (0xdec4b0, "lightsteelblue"),
    (0xe0ffff, "lightyellow"),
    (0x00ff00, "lime"),
    (0x32cd32, "limegreen"),
    (0xe6f0fa, "linen"),
    (0xff00ff, "magenta"),
    (0x000080, "maroon"),
    (0xaacd66, "mediumaquamarine"),
    (0xcd0000, "mediumblue"),
    (0xd355ba, "mediumorchid"),
    (0xdb7093, "mediumpurple"),
    (0x71b33c, "mediumseagreen"),
    (0xee687b, "mediumslateblue"),
    (0x9afa00, "mediumspringgreen"),
    (0xccd148, "mediumturquoise"),
    (0x8515c7, "mediumvioletred"),
    (0x701919, "midnightblue"),
    (0xfafff5, "mintcream"),
    (0xe1e4ff, "mistyrose"),
    (0xb5e4ff, "moccasin"),
    (0xaddeff, "navajowhite"),
    (0x800000, "navy"),
    (0xe6f5fd, "oldlace"),
    (0x008080, "olive"),
    (0x238e6b, "olivedrab"),
    (0x00a5ff, "orange"),
    (0x0045ff, "orangered"),
    (0xd670da, "orchid"),
    (0xaae8ee, "palegoldenrod"),
    (0x98fb98, "palegreen"),
    (0xeeeeaf, "paleturquoise"),
    (0x9370db, "palevioletred"),
    (0xd5efff, "papayawhip"),
    (0xb9daff, "peachpuff"),
    (0x3f85cd, "peru"),
    (0xcbc0ff, "pink"),
    (0xdda0dd, "plum"),
    (0xe6e0b0, "powderblue"),
    (0x800080, "purple"),
    (0x0000ff, "red"),
    (0x8f8fbc, "rosybrown"),
    (0xe16941, "royalblue"),
    (0x13458b, "saddlebrown"),
    (0x7280fa, "salmon"),
    (0x60a4f4, "sandybrown"),
    (0x578b2e, "seagreen"),
    (0xeef5ff, "seashell"),
    (0x2d52a0, "sienna"),
    (0xc0c0c0, "silver"),
    (0xebce87, "skyblue"),
    (0xcd5a6a, "slateblue"),
    (0x908070, "slategray"),
    (0x908070, "slategrey"),
    (0xfafaff, "snow"),
    (0x7fff00, "springgreen"),
    (0xb48246, "steelblue"),
    (0x8cb4d2, "tan"),
    (0x808000, "teal"),
    (0xd8bfd8, "thistle"),
    (0x4763ff, "tomato"),
    (0xd0e040, "turquoise"),
    (0xee82ee, "violet"),
    (0xb3def5, "wheat"),
    (0xffffff, "white"),
    (0xf5f5f5, "whitesmoke"),
    (0x00ffff, "yellow"),
    (0x32cd9a, "yellowgreen"),
];

lazy_static!{
    static ref COLOR_OF_RGB: HashMap<u32, &'static str>  = COLOR_DICTIONARY.iter().map(|(a,b)| (*a,*b)).collect();
    static ref COLOR_OF_NAME: HashMap<&'static str, u32> = COLOR_DICTIONARY.iter().map(|(a,b)| (*b,*a)).collect();
}

pub fn rgb_of_name(s:&str) -> Option<&'static u32> {
    COLOR_OF_NAME.get(s)
}

pub fn name_of_rgb(rgb:u32) -> Option<&'static str> {
    match COLOR_OF_RGB.get(&rgb) {
        None => None,
        Some(c) => Some(*c)
    }
}

pub fn as_floats(rgb:u32, v:&mut Vec<f64>) -> () {
    let (r,g,b) = ( (  (rgb>> 0) & 0xff) as f64,
                      ((rgb>> 8) & 0xff) as f64,
                      ((rgb>>16) & 0xff) as f64,
    );
    v[0] = r; v[1] = g; v[2] = b;
}

pub fn as_u32(rgb:&Vec<f64>) -> u32 {
    match rgb.len() {
        0 => 0,
        1 => ((rgb[0]*255.) as u32) * 0x010101,
        2 => (((rgb[0]*255.) as u32) << 0) | (((rgb[1]*255.) as u32) << 8) | (((rgb[0]*255.) as u32) << 16),
        _ => (((rgb[0]*255.) as u32) << 0) | (((rgb[1]*255.) as u32) << 8) | (((rgb[2]*255.) as u32) << 16),
    }
}

pub fn as_string(rgb:u32) -> String {
    match name_of_rgb(rgb) {
        Some(s) => s.to_string(),
        None    => format!("#{:06x}",rgb),
    }
}

//fp of_string
pub fn of_string(s:&str) -> Option<u32> {
    match rgb_of_name(s) {
        Some(rgb) => Some(*rgb),
        None      => {
            if s.bytes().nth(0)!=Some(35) {
                None
            } else {
                match u32::from_str_radix(s,16) {
                    Ok(rgb) => Some(rgb & 0xffffff),
                    _ => None,
                }
            }
        },
    }
}

//t Tests
#[cfg(test)]
mod tests {
    use super::*;
    fn test_of_string() {
        assert_eq!( Some(0x000000), of_string("black") );
        assert_eq!( Some(0x000000), of_string("#0") );
        assert_eq!( Some(0x000000), of_string("#0000000") );
        assert_eq!( Some(0x000000), of_string("#000000000") );
        assert_eq!( Some(0xffffff), of_string("white") );
        assert_eq!( Some(0x012345), of_string("#012345") );
        assert_eq!( Some(0x234567), of_string("#01234567") );
        assert_eq!( Some(0x456789), of_string("#0123456789") );
        assert_eq!( Some(0x0000ff), of_string("red") );
        assert_eq!( Some(0x00ff00), of_string("green") );
        assert_eq!( Some(0xff0000), of_string("blue") );
    }
    fn test_as_string() {
        assert_eq!( "black", as_string(0x000000) );
        assert_eq!( "coral", as_string(0x507fff) );
        assert_eq!( "#507ffe", as_string(0x507ffe) );
    }
}
