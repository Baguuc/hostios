pub use crate::error::*;
pub type Result<T> = std::result::Result<T, Error>;

pub struct W<T>(pub T);
