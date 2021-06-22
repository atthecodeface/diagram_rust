use super::Value;
use super::{Parameter, GlyphMetrics};

//a CharIndices
//ti CharIndices
/// This structure provides simple metrics for a font or a region of
/// characters in a font. It is based on the TeX Font Metrics.
#[derive(Debug, Clone, Copy)]
pub struct CharIndices(u32);
impl std::fmt::Display for CharIndices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0 )
    }
}
impl CharIndices {
    fn of_indices(width:usize, height:usize, depth:usize, italic:usize, options:usize) -> Self {
        assert!(width  < 256);
        assert!(height < 16);
        assert!(depth  < 16);
        assert!(italic  < 64);
        assert!(options < 1024);
        let v = (options << 22) | (italic << 16) | (depth << 12) | (height << 8) | (width << 0);
        Self ( v as u32 )
    }
    pub fn width_index(&self)  -> usize {((self.0 >>  0) &  0xff ) as usize }
    pub fn height_index(&self) -> usize {((self.0 >>  8) &   0xf ) as usize }
    pub fn depth_index(&self)  -> usize {((self.0 >> 12) &   0xf ) as usize }
    pub fn italic_index(&self) -> usize {((self.0 >> 16) &  0x3f ) as usize }
    pub fn options(&self)      -> usize {((self.0 >> 22) & 0x3ff ) as usize }
    pub fn width<V:Value> (&self, metrics:&Metrics<V>) -> V {
        metrics.get_width(self.width_index())
    }
    pub fn height<V:Value> (&self, metrics:&Metrics<V>) -> V {
        metrics.get_height(self.height_index())
    }
    pub fn depth<V:Value> (&self, metrics:&Metrics<V>) -> V {
        metrics.get_depth(self.depth_index())
    }
    pub fn italic<V:Value> (&self, metrics:&Metrics<V>) -> V {
        metrics.get_italic(self.italic_index())
    }
}

//a Metrics
//tp Metrics
/// This structure provides simple metrics for a font or a region of
/// characters in a font. It is based on the TeX Font Metrics.
///
/// It is designed to be built into a hierarchy, such that an
/// arbitrary font can be described, but that the details for a single
/// font may be kept small, and accessing a character's details are
/// lightweight
#[derive(Debug)]
pub struct Metrics<V:Value> {
    /// First Unicode Scalar Value represented by these metrics (inclusive)
    first_char : char,
    /// Last Unicode Scalar Value represented by these metrics (inclusive)
    last_char : char,
    /// Widths - at most 256 long, with zeroth element of 0
    widths : Vec<V>,
    /// Heights - at most 16 long, with zeroth element of 0
    heights : Vec<V>,
    /// Depths - at most 16 long, with zeroth element of 0
    depths : Vec<V>,
    /// Italic - at most 64 long, with zeroth element of 0
    italics : Vec<V>,
    /// Character metrics - as indices in to the above vectors, for characters from first_char to last_char
    char_metrics : Vec<CharIndices>,
    /// parameters, sorted by the parameter order for faster indexing
    parameters : Vec<Parameter<V>>,
    /// Exceptions to the metrics provided here - allowing for more than 16 heights, 256 widths, etc.
    exceptions : Vec<Metrics<V>>,
}

impl <V:Value> Metrics<V> {
    pub fn new_monospace(width : V, height : V, depth : V, italic : V) -> Self {
        let first_char = '\0';
        let last_char  = '\0';
        let widths  = vec![width];
        let heights = vec![height];
        let depths  = vec![depth];
        let italics = vec![italic];
        let char_metrics = vec![CharIndices::of_indices(0,0,0,0,0)];
        let parameters = Vec::new();
        let exceptions = Vec::new();
        Self { first_char, last_char,
               widths, heights, depths, italics,
               char_metrics,
               parameters, exceptions }
    }
    pub fn get_width(&self, index:usize) -> V {
        assert!(index < self.widths.len());
        self.widths[index]
    }
    pub fn get_height(&self, index:usize) -> V {
        assert!(index < self.heights.len());
        self.heights[index]
    }
    pub fn get_depth(&self, index:usize) -> V {
        assert!(index < self.depths.len());
        self.depths[index]
    }
    pub fn get_italic(&self, index:usize) -> V {
        assert!(index < self.italics.len());
        self.italics[index]
    }
    pub fn get_glyph_metrics(&self, index:usize) -> GlyphMetrics<V> {
        let ci      = self.char_metrics[index];
        let width   = ci.width(self);
        let height  = ci.height(self);
        let depth   = ci.depth(self);
        let italic  = ci.italic(self);
        let options = ci.options();
        GlyphMetrics { width, height, depth, italic, options }
    }
    pub fn borrow_metrics_of_char(&self, c:char) -> Option<(&Metrics<V>, usize)> {
        if c < self.first_char {
            None
        } else if c > self.last_char {
            None
        } else {
            for e in &self.exceptions {
                if let Some(m) = e.borrow_metrics_of_char(c) {
                    return Some(m);
                }
            }
            Some((&self, ((c as u32) - (self.first_char as u32)) as usize))
        }
    }
    pub fn glyph_metrics(&self, c:char) -> GlyphMetrics<V> {
        if let Some((m,i)) = self.borrow_metrics_of_char(c) {
            m.get_glyph_metrics(i)
        } else {
            self.get_glyph_metrics(0)
        }
    }
}

