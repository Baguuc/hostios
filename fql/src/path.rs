#[derive(Debug)]
pub struct Path(String);

impl Path {
    pub fn parse(string: impl ToString) -> Result<Self, PathParseError> {
        let string = string.to_string();

        if string.starts_with("/") || string.starts_with(".") || string.starts_with("~") || string.contains(" ") || string.contains(",") {
            return Err(PathParseError::EscapesScope);
        }

        return Ok(Self(string));
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PathParseError {
    #[error("ESCAPES_SCOPE")]
    EscapesScope
}

impl ToString for Path {
    fn to_string(self: &Self) -> String { self.0.clone() }
}

impl From<Path> for std::path::PathBuf {
    fn from(path: Path) -> std::path::PathBuf {
        return Self::from(path.0);
    }
}
