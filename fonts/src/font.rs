use super::FontMetrics;
/// This structure provides simple metric storage for
#[derive(Debug)]
pub struct Font {
    metrics : FontMetrics<f64>,
}

impl Font {
    pub fn default() -> Self{
        Self {
            metrics : FontMetrics::new_monospace(0.5, 1.1, 0.3, 0.),
        }
    }
}

