use super::Value;

pub struct GlyphMetrics<V:Value> {
    pub(crate) width : V,
    pub(crate) height : V,
    pub(crate) depth : V,
    pub(crate) italic : V,
    pub(crate) options : usize
}
impl <V:Value> GlyphMetrics<V> {
    pub fn zero() -> Self {
        Self { width  : V::zero(),
               height : V::zero(),
               depth  : V::zero(),
               italic : V::zero(),
               options : 0,
        }
    }
    pub fn add(&self, other:&Self) -> Self {
        Self { width  : self.width + other.width,
               height : if self.height > other.height {self.height} else {other.height},
               depth  : if self.depth > other.depth {self.depth} else {other.depth},
               italic : other.italic,
               options : other.options }
    }
}
