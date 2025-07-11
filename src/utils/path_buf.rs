use crate::prelude::*;

impl TryInto<String> for W<std::path::PathBuf> {
    type Error = Error;

    fn try_into(self) -> Result<String> {
        let as_string = self.0
            .to_str()
            .ok_or(Error::Generic(String::from("file name do not exist")))?
            .to_string();

        return Ok(as_string);
    }
}
