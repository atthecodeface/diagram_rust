//a Imports
use crate::colors::Color;

//tp ColorDatabase
/// A database of named colors, each being an &str and a u32 color
/// encoding RRGGBB
pub struct ColorDatabase<'a> {
    /// The colors in the database
    pub colors: &'a [(&'a str, u32)],
}

//ip ColorDatabase
impl<'a> ColorDatabase<'a> {
    //mi find_color_exact_index
    /// Find a color from a name using an exact match on strings
    fn find_color_exact_index(&self, name: &str) -> Option<usize> {
        for (i, (n, _)) in self.colors.iter().enumerate() {
            if *n == name {
                return Some(i);
            }
        }
        None
    }

    //mi canonicalize_name
    /// Convert a color name into a canonical name; this is a
    /// lowercase version of the string without underscores
    ///
    /// This allows LightGrey and light_grey to retrieve lightgrey for example
    fn canonicalize_name(name: &str) -> Option<String> {
        let mut r = String::new();
        for mut c in name.chars() {
            if c == '_' {
                continue;
            }
            c.make_ascii_lowercase();
            r.push(c);
        }
        Some(r)
    }

    //mi find_color_index
    /// Find the index of a user color name, after canonicalizing it to a database-compatible name
    fn find_color_index(&self, name: &str) -> Option<usize> {
        Self::canonicalize_name(name).and_then(|s| self.find_color_exact_index(&s))
    }
    //mp find_color_name
    /// Find the database color name of a user color name
    pub fn find_color_name(&self, name: &str) -> Option<&str> {
        self.find_color_index(name).map(|i| self.colors[i].0)
    }

    //mi find_color_rgb
    /// Find the 32-bit RRGGBB value of a user color name within the
    /// database
    pub fn find_color_rgb(&self, name: &str) -> Option<u32> {
        self.find_color_index(name).map(|i| self.colors[i].1)
    }

    //mp find_color
    /// Find a color in the database from a user color name, allowing
    /// for 'none' (or variants therefor) as a name
    ///
    /// If the color is not in the database and the color name is not
    /// 'None' (in some form), then None is returned
    pub fn find_color(&self, name: &str) -> Option<Color> {
        if let Some(color_none) = Color::color_if_name_is_none(name) {
            Some(color_none)
        } else {
            self.find_color_index(name)
                .map(|i| Color::new(Some(self.colors[i].0.into()), self.colors[i].1))
        }
    }

    //zz All done
}
