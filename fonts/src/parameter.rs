use super::Value;

//tp Parameter
/// Font parameters - generally metrics
///
/// These are font-wide parameters
#[derive(Debug, Copy, Clone)]
pub enum Parameter<V:Value> {
    /// Size of a space in the font (standard gap between words)
    Space(V),
    /// Size of an 'em' in the font (length of an em-dash, not necessarily the width of 'M')
    Em(V),
    /// Space after a period at the end of a sentence
    PunctSpace(V),
    // x height
    // cap height
    Ascent(V),
    Descent(V),
    LineSpacing(V),
    LineGap(V),
}
