/// # UserVault
///
/// Represents data about vault and user's permission to it
///
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UserVault {
    pub id: String,
    pub permissions: Vec<UserVaultPermission>
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum UserVaultPermission {
    Read,
    Write,
    Manage
}
