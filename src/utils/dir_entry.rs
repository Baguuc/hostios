use crate::prelude::*;

impl TryInto<String> for W<std::fs::DirEntry> {
    type Error = Error;

    fn try_into(self) -> Result<String> {
        return W(self.0.path()).try_into();
    }
}
