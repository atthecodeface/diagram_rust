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
 */

use std::collections::HashMap;
const COLOR_DICTIONARY : [(u32, &str);8] = [
    (0x000000, "black"),
    (0x0000ff, "red"),
    (0x00ff00, "green"),
    (0xff0000, "blue"),
    (0xffff00, "cyan"),
    (0x00ffff, "yellow"),
    (0xff00ff, "magenta"),
    (0xffffff, "white"),
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
