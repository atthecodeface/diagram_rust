//a Imports
use serde::{Serialize, Serializer};
use std::rc::Rc;

use crate::colors::{ColorDatabase, Rgba};

//a Colors
//tp Color
/// This is a color with a name;
#[derive(Debug, Clone)]
pub struct Color {
    /// String representation (if transparency is 0)
    text: Option<Rc<String>>,
    /// RGBA
    rgba: Rgba,
}

//tp ColorToSerialize
#[derive(Serialize)]
struct ColorToSerialize<'a> {
    text: Option<&'a str>,
    rgba: &'a Rgba,
}
//ip From<&Color> for ColorToSerialize
impl<'a> From<&'a Color> for ColorToSerialize<'a> {
    fn from(c: &'a Color) -> ColorToSerialize<'a> {
        Self {
            text: c.name_as_str(),
            rgba: &c.rgba,
        }
    }
}

//ip Color
impl Color {
    //cp new
    /// Create a new Color from a string and an Rgba representation
    #[inline]
    #[must_use]
    pub fn new<I: Into<Option<String>>, J: Into<Rgba>>(text: I, rgba: J) -> Self {
        let text = text.into().map(|s| s.into());
        let rgba = rgba.into();
        Self { text, rgba }
    }

    //bp set_alpha
    /// Set the alpha of the provided color
    #[inline]
    #[must_use]
    pub fn set_alpha(mut self, alpha: u8) -> Self {
        self.rgba = self.rgba.set_alpha(alpha);
        self
    }

    //cp of_rgb
    /// Create a [Color] from sommething that describes an Rgba (such as a 4-tuple, 3-tuple, etc)
    #[inline]
    #[must_use]
    pub fn of_rgb<I: Into<Rgba>>(rgba: I) -> Self {
        let rgba = rgba.into();
        let text = None;
        Self { text, rgba }
    }

    //cp color_if_name_is_none
    /// If the color indicates 'none' in somme form then return Some(Color::transparent)
    pub fn color_if_name_is_none(name: &str) -> Option<Self> {
        match name {
            "None" | "none" | "NONE" => Some(Self::new(None, (0, 0, 0, 255))),
            _ => None,
        }
    }

    //ap rgba
    /// Get a reference to the Rgba of the color
    pub fn rgba(&self) -> &Rgba {
        &self.rgba
    }

    //fp name_as_str
    /// Return a &str of the name if it has one; this ignores the alpha
    ///
    /// The color must have been created with a name for this to return Some()
    pub fn name_as_str(&self) -> Option<&str> {
        match &self.text {
            None => None,
            Some(x) => Some(x.as_ref().as_str()),
        }
    }

    //fp as_rc_string
    /// Return an Rc<String> for the color, creating one if required
    ///
    /// This maps the Rgba into a hex string if required
    pub fn as_rc_string(&self) -> Option<Rc<String>> {
        if self.rgba.alpha() == 255 {
            self.text.clone()
        } else {
            Some(Rc::new(self.rgba.into()))
        }
    }
}

//ip From<(&str, &'a ColorDatabase<'a>)> for Color
impl<'a> From<(&str, &'a ColorDatabase<'a>)> for Color {
    #[inline]
    fn from((s, db): (&str, &'a ColorDatabase<'a>)) -> Self {
        db.find_color(s)
            .unwrap_or_else(|| panic!("Color must be found in the database, but '{}' was not", s))
    }
}

//ip From<(&Color, &'a ColorDatabase<'a>)> for Color
impl<'a> From<(&Color, &'a ColorDatabase<'a>)> for Color {
    #[inline]
    fn from((c, _db): (&Color, &'a ColorDatabase<'a>)) -> Self {
        c.clone()
    }
}

//ip From<(Into<Rgba>, &'a ColorDatabase<'a>)> for Color
impl<'a, I: Into<Rgba>> From<(I, &'a ColorDatabase<'a>)> for Color {
    #[inline]
    fn from((rgb, _db): (I, &'a ColorDatabase<'a>)) -> Self {
        Color::of_rgb(rgb.into())
    }
}
//ip Serialize for Color for debug/reference; Rc<String> does not serialize cleanly
impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: ColorToSerialize = self.into();
        s.serialize(serializer)
    }
}
