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
use hml_rs::names::{NSNameId, Name, NamespaceStack};
// use hml_rs::names::{Name, NamespaceStack};
use std::collections::HashMap;
use crate::constants::{attributes, elements};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KnownName {
    Library,
    Rule   ,
    Style  ,
    Defs   ,
    Id     ,
    Class  ,
    Depth  ,

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

    Debug,
    Bbox,

    Grid,
    GridX,
    GridY,
    MinX,
    MinY,
    GrowX,
    GrowY,

    Place,

    Anchor,
    Expand,

    Pad,
    Margin,
    Bg,
    BorderWidth,
    BorderRound,
    BorderColor,
    Scale,
    Rotate,
    Translate,
    Fill,
    Stroke,
    StrokeWidth,
    Width,
    Height,
    Coords,
    Round,
    Stellate,
    Markers,
    Font,
    FontSize,
    FontWeight,
    FontStyle,
    Vertices,
    Flags,
    Ref,
    Point,
    Relief,

}

pub struct NameIds {
    map     : HashMap<NSNameId, KnownName>,
    str_map : HashMap<KnownName, &'static str>,
}

impl NameIds {
    pub fn create(namespace_stack:&mut NamespaceStack) -> Self {
        let map     = HashMap::new();
        let str_map = HashMap::new();
        let mut name_ids = Self { map, str_map };

        name_ids.add_name(namespace_stack, "library", KnownName::Library );
        name_ids.add_name(namespace_stack, "rule", KnownName::Rule    );
        name_ids.add_name(namespace_stack, "style", KnownName::Style   );
        name_ids.add_name(namespace_stack, "defs", KnownName::Defs    );
        name_ids.add_name(namespace_stack, "id", KnownName::Id      );
        name_ids.add_name(namespace_stack, "class", KnownName::Class   );
        name_ids.add_name(namespace_stack, "depth", KnownName::Depth   );

        name_ids.add_name(namespace_stack, elements::MARKER,  KnownName::Marker  );
        name_ids.add_name(namespace_stack, elements::USE,     KnownName::Use  );
        name_ids.add_name(namespace_stack, elements::DIAGRAM, KnownName::Diagram );
        name_ids.add_name(namespace_stack, elements::GROUP,   KnownName::Group );
        name_ids.add_name(namespace_stack, elements::LAYOUT,  KnownName::Layout);
        name_ids.add_name(namespace_stack, elements::RECT,    KnownName::Rect);
        name_ids.add_name(namespace_stack, elements::CIRCLE,  KnownName::Circle);
        name_ids.add_name(namespace_stack, elements::POLYGON, KnownName::Polygon);
        name_ids.add_name(namespace_stack, elements::TEXT,    KnownName::Text);
        name_ids.add_name(namespace_stack, elements::PATH,    KnownName::Path);


        name_ids.add_name(namespace_stack, attributes::DEBUG,       KnownName::Debug );
        name_ids.add_name(namespace_stack, attributes::BBOX,        KnownName::Bbox );

        name_ids.add_name(namespace_stack, attributes::GRID,        KnownName::Grid );
        name_ids.add_name(namespace_stack, attributes::GRIDX,       KnownName::GridX );
        name_ids.add_name(namespace_stack, attributes::GRIDY,       KnownName::GridY );
        name_ids.add_name(namespace_stack, attributes::MINX,        KnownName::MinX );
        name_ids.add_name(namespace_stack, attributes::MINY,        KnownName::MinY );
        name_ids.add_name(namespace_stack, attributes::GROWX,       KnownName::GrowX );
        name_ids.add_name(namespace_stack, attributes::GROWY,       KnownName::GrowY );

        name_ids.add_name(namespace_stack, attributes::PLACE,       KnownName::Place );

        name_ids.add_name(namespace_stack, attributes::ANCHOR,      KnownName::Anchor );
        name_ids.add_name(namespace_stack, attributes::EXPAND,      KnownName::Expand );

        name_ids.add_name(namespace_stack, attributes::PAD,         KnownName::Pad );
        name_ids.add_name(namespace_stack, attributes::MARGIN,      KnownName::Margin );
        name_ids.add_name(namespace_stack, attributes::BG,          KnownName::Bg );
        name_ids.add_name(namespace_stack, attributes::BORDERWIDTH, KnownName::BorderWidth );
        name_ids.add_name(namespace_stack, attributes::BORDERROUND, KnownName::BorderRound );
        name_ids.add_name(namespace_stack, attributes::BORDERCOLOR, KnownName::BorderColor );
        name_ids.add_name(namespace_stack, attributes::SCALE,       KnownName::Scale );
        name_ids.add_name(namespace_stack, attributes::ROTATE,      KnownName::Rotate );
        name_ids.add_name(namespace_stack, attributes::TRANSLATE,   KnownName::Translate );
        name_ids.add_name(namespace_stack, attributes::FILL,        KnownName::Fill );
        name_ids.add_name(namespace_stack, attributes::STROKE,      KnownName::Stroke );
        name_ids.add_name(namespace_stack, attributes::STROKEWIDTH, KnownName::StrokeWidth );
        name_ids.add_name(namespace_stack, attributes::WIDTH,       KnownName::Width );
        name_ids.add_name(namespace_stack, attributes::HEIGHT,      KnownName::Height );
        name_ids.add_name(namespace_stack, attributes::COORDS,      KnownName::Coords );
        name_ids.add_name(namespace_stack, attributes::ROUND,       KnownName::Round );
        name_ids.add_name(namespace_stack, attributes::STELLATE,    KnownName::Stellate );
        name_ids.add_name(namespace_stack, attributes::MARKERS,     KnownName::Markers );
        name_ids.add_name(namespace_stack, attributes::FONT,        KnownName::Font );
        name_ids.add_name(namespace_stack, attributes::FONTSIZE,    KnownName::FontSize );
        name_ids.add_name(namespace_stack, attributes::FONTWEIGHT,  KnownName::FontWeight );
        name_ids.add_name(namespace_stack, attributes::FONTSTYLE,   KnownName::FontStyle );
        name_ids.add_name(namespace_stack, attributes::VERTICES,    KnownName::Vertices );
        name_ids.add_name(namespace_stack, attributes::FLAGS,       KnownName::Flags );
        name_ids.add_name(namespace_stack, attributes::REF,         KnownName::Ref );
        name_ids.add_name(namespace_stack, attributes::POINT,       KnownName::Point );
        name_ids.add_name(namespace_stack, attributes::RELIEF,      KnownName::Relief );

        name_ids
    }
    fn add_name(&mut self, namespace_stack:&mut NamespaceStack, s:&'static str, kn:KnownName) {
        self.map.insert( namespace_stack.add_name(s), kn);
        self.str_map.insert( kn, s );
    }
    pub fn known_id(&self, name:&Name) -> Option<KnownName> {
        self.map.get(&name.name).map(|x| *x)
    }
    pub fn str_of_name(&self, name:&KnownName) -> &'static str {
        self.str_map.get(&name).map(|x| *x).unwrap()
    }
}

