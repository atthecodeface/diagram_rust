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
use hml;
use std::collections::HashMap;
use crate::constants::{attributes, elements};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    map : HashMap<hml::NSNameId, KnownName>,
}

impl NameIds {
    pub fn create(namespace_stack:&mut hml::NamespaceStack) -> Self {
        let mut map = HashMap::new();
        map.insert( namespace_stack.add_name("library"), KnownName::Library );
        map.insert( namespace_stack.add_name("rule"), KnownName::Rule    );
        map.insert( namespace_stack.add_name("style"), KnownName::Style   );
        map.insert( namespace_stack.add_name("defs"), KnownName::Defs    );
        map.insert( namespace_stack.add_name("id"), KnownName::Id      );
        map.insert( namespace_stack.add_name("class"), KnownName::Class   );
        map.insert( namespace_stack.add_name("depth"), KnownName::Depth   );

        map.insert( namespace_stack.add_name(elements::MARKER),  KnownName::Marker  );
        map.insert( namespace_stack.add_name(elements::USE),     KnownName::Use  );
        map.insert( namespace_stack.add_name(elements::DIAGRAM), KnownName::Diagram );
        map.insert( namespace_stack.add_name(elements::GROUP),   KnownName::Group );
        map.insert( namespace_stack.add_name(elements::LAYOUT),  KnownName::Layout);
        map.insert( namespace_stack.add_name(elements::RECT),    KnownName::Rect);
        map.insert( namespace_stack.add_name(elements::CIRCLE),  KnownName::Circle);
        map.insert( namespace_stack.add_name(elements::POLYGON), KnownName::Polygon);
        map.insert( namespace_stack.add_name(elements::TEXT),    KnownName::Text);
        map.insert( namespace_stack.add_name(elements::PATH),    KnownName::Path);


        map.insert( namespace_stack.add_name(attributes::DEBUG),       KnownName::Debug );
        map.insert( namespace_stack.add_name(attributes::BBOX),        KnownName::Bbox );

        map.insert( namespace_stack.add_name(attributes::GRID),        KnownName::Grid );
        map.insert( namespace_stack.add_name(attributes::GRIDX),       KnownName::GridX );
        map.insert( namespace_stack.add_name(attributes::GRIDY),       KnownName::GridY );
        map.insert( namespace_stack.add_name(attributes::MINX),        KnownName::MinX );
        map.insert( namespace_stack.add_name(attributes::MINY),        KnownName::MinY );
        map.insert( namespace_stack.add_name(attributes::GROWX),       KnownName::GrowX );
        map.insert( namespace_stack.add_name(attributes::GROWY),       KnownName::GrowY );

        map.insert( namespace_stack.add_name(attributes::PLACE),       KnownName::Place );

        map.insert( namespace_stack.add_name(attributes::ANCHOR),      KnownName::Anchor );
        map.insert( namespace_stack.add_name(attributes::EXPAND),      KnownName::Expand );

        map.insert( namespace_stack.add_name(attributes::PAD),         KnownName::Pad );
        map.insert( namespace_stack.add_name(attributes::MARGIN),      KnownName::Margin );
        map.insert( namespace_stack.add_name(attributes::BG),          KnownName::Bg );
        map.insert( namespace_stack.add_name(attributes::BORDERWIDTH), KnownName::BorderWidth );
        map.insert( namespace_stack.add_name(attributes::BORDERROUND), KnownName::BorderRound );
        map.insert( namespace_stack.add_name(attributes::BORDERCOLOR), KnownName::BorderColor );
        map.insert( namespace_stack.add_name(attributes::SCALE),       KnownName::Scale );
        map.insert( namespace_stack.add_name(attributes::ROTATE),      KnownName::Rotate );
        map.insert( namespace_stack.add_name(attributes::TRANSLATE),   KnownName::Translate );
        map.insert( namespace_stack.add_name(attributes::FILL),        KnownName::Fill );
        map.insert( namespace_stack.add_name(attributes::STROKE),      KnownName::Stroke );
        map.insert( namespace_stack.add_name(attributes::STROKEWIDTH), KnownName::StrokeWidth );
        map.insert( namespace_stack.add_name(attributes::WIDTH),       KnownName::Width );
        map.insert( namespace_stack.add_name(attributes::HEIGHT),      KnownName::Height );
        map.insert( namespace_stack.add_name(attributes::COORDS),      KnownName::Coords );
        map.insert( namespace_stack.add_name(attributes::ROUND),       KnownName::Round );
        map.insert( namespace_stack.add_name(attributes::STELLATE),    KnownName::Stellate );
        map.insert( namespace_stack.add_name(attributes::MARKERS),     KnownName::Markers );
        map.insert( namespace_stack.add_name(attributes::FONT),        KnownName::Font );
        map.insert( namespace_stack.add_name(attributes::FONTSIZE),    KnownName::FontSize );
        map.insert( namespace_stack.add_name(attributes::FONTWEIGHT),  KnownName::FontWeight );
        map.insert( namespace_stack.add_name(attributes::FONTSTYLE),   KnownName::FontStyle );
        map.insert( namespace_stack.add_name(attributes::VERTICES),    KnownName::Vertices );
        map.insert( namespace_stack.add_name(attributes::FLAGS),       KnownName::Flags );
        map.insert( namespace_stack.add_name(attributes::REF),         KnownName::Ref );
        map.insert( namespace_stack.add_name(attributes::POINT),       KnownName::Point );
        map.insert( namespace_stack.add_name(attributes::RELIEF),      KnownName::Relief );

        Self {
            map
        }
    }
    pub fn known_id(&self, name:&hml::Name) -> Option<KnownName> {
        self.map.get(&name.name).map(|x| *x)
    }
}

