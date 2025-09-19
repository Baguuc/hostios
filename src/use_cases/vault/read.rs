impl crate::use_cases::VaultUseCase {
    /// # VaultUseCase::read
    ///
    /// Read directory, checking for errors
    ///
    pub async fn read_directory(
        params: crate::params::use_case::VaultReadParams,
        user_client: authios_sdk::UserSdk
    ) -> Result<Vec<crate::models::Entry>, crate::errors::use_case::VaultReadError> {
        use crate::{
            errors::{
                use_case::VaultReadError as Error,
                repository::DirectoryReadError as RepoError
            },
            params::repository::DirectoryReadParams as ReadParams,
            repositories::DirectoriesRepository,
            utils::path::join_paths
        };
        use authios_sdk::{
            errors::UserSdkAuthorizeError as AuthorizeError,
            params::UserSdkAuthorizeParams as AuthorizeParams
        };

        match user_client.authorize(AuthorizeParams { 
            token: params.token.clone(), 
            permission: format!("hostios:vaults:{}:read", params.vault_id.clone())
        }).await {
            Ok(true) => (),
            Ok(false) => return Err(Error::Unauthorized),
            Err(err) => match err {
                AuthorizeError::InvalidToken => return Err(Error::InvalidToken),
                // necessary permissions are created on vault creating so if not found, the vault
                // is not created yet. 
                AuthorizeError::PermissionNotFound => return Err(Error::VaultNotFound),
                _ => return Err(Error::AuthUnavaible),
            },
        };

        let base_path = join_paths(params.base_system_path, params.vault_id);
        let result = DirectoriesRepository::read(ReadParams { system_base_path: base_path, internal_path: params.internal_path }).await;

        match result {
            Ok(read) => return Ok(read),
            Err(err) => match err {
                RepoError::NotExist => return Err(Error::PathNotFound),
                RepoError::InvalidPath => return Err(Error::InvalidPathScope)
            }
        };
    }
}
