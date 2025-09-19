/// # UserVault
///
/// Represents data about vault and user's permission to it
///
pub struct UserVault {
    pub id: String,
    pub permissions: UserVaultPermission
}

pub enum UserVaultPermission {
    Read,
    Write,
    ReadWrite,
    None
}
