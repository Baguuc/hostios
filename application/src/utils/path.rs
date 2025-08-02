use crate::prelude::*;

pub struct Path(std::path::PathBuf);

impl Path {
    pub fn parse(path: std::path::PathBuf) -> Result<Self> {
        if !Self::validate(&path) {
            return Err(Error::Generic(String::from("provided path tries to escape the scope of the application")));
        }

        return Ok(Self(path));
    }
    
    pub fn validate(path: &std::path::PathBuf) -> bool {
        use std::path::PathBuf;
        
        return !(
            path.starts_with(PathBuf::from("..")) || 
            path.starts_with(PathBuf::from("/")) || 
            path.starts_with(PathBuf::from("~"))
        );
    }
}

impl TryInto<String> for Path {
    type Error = Error;

    fn try_into(self) -> Result<String> {
        let error = Error::Generic(String::from("cannot convert path buf to string"));
        
        let string = self.0
            .to_str()
            .ok_or(error)?
            .to_string();

        return Ok(string);
    }
}
