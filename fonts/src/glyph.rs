use super::Value;

/// An outline font glyph
///
/// Glyphs are defined by bezier curves, with an even-odd winding rule
pub struct Glyph<'a, V:Value> {
    // name not used
    // name : String,
    // unicode point - probably not used if indexed by this?
    unichr  : char,
    metrics : &'a GlyphMetrics<V>,
    /// 2 points for straight lines, 3 points for quartic beziers, 4 points for cubic beziers
    bezier_type : usize,
    points  : Vec<Point2D<V>>,
    beziers : Vec<usize>,
}
