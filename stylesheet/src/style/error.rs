//a Imports
use thiserror::Error;

//a Value error
//tp ValueError
#[derive(Error, Debug)]
pub enum ValueError {
    #[error("Bad value {reason}")]
    BadValue { reason: String },
}

//ti ValueError
impl ValueError {
    pub fn bad_value<I: Into<String>>(s: I) -> Self {
        let reason = s.into();
        Self::BadValue { reason }
    }
}
