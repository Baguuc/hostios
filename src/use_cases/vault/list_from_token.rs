impl crate::use_cases::VaultUseCase {
    /// # VaultUseCase::list_from_token
    ///
    /// List users permissions from session token and return all vaults the user is permitted to
    /// use.
    ///
    pub async fn list_from_token(
        params: crate::params::use_case::VaultListFromTokenParams,
        user_client: authios_sdk::UserSdk
    ) -> Result<Vec<crate::models::UserVault>, crate::errors::use_case::VaultListFromTokenError> {
        use crate::{
            errors::use_case::VaultListFromTokenError as Error,
            models::{
                UserVault,
                UserVaultPermission
            }
        };
        use authios_sdk::{
            errors::UserSdkListPermissionsError as ListPermissionsError,
            params::UserSdkListPermissionsParams as ListPermissionsParams
        };
        use std::collections::HashMap;

        let permissions = user_client
            .list_permissions(ListPermissionsParams { token: params.token })
            .await
            .map_err(|err| match err {
                ListPermissionsError::InvalidToken => Error::InvalidToken,
                _ => Error::CannotFetch
            })?;
        
        let mut vault_permissions: HashMap<String, Vec<UserVaultPermission>> = HashMap::new();
        
        for permission_name in permissions {
            if !permission_name.starts_with("hostios:vaults:") { continue; }
            
            let splitted = permission_name
                .split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            // 1......|2.....|3...|4...........
            // hostios:vaults:<id>:<permission>
            if splitted.len() != 4 { continue; }

            let vault_id = splitted
                .get(2)
                .unwrap()
                .to_string();
            let permission_name = splitted
                .get(3)
                .unwrap();
            
            if let Some(vault) = vault_permissions.get_mut(&vault_id) {
                let permission = match permission_name.as_str() {
                    "read" => UserVaultPermission::Read,
                    "write" => UserVaultPermission::Write,
                    "manage" => UserVaultPermission::Manage,
                    _ => continue
                };
                vault.push(permission); 
            } else {
                let permission = match permission_name.as_str() {
                    "read" => UserVaultPermission::Read,
                    "write" => UserVaultPermission::Write,
                    "manage" => UserVaultPermission::Manage,
                    _ => continue
                };
                
                vault_permissions.insert(vault_id, vec![permission]);                
            }
        }

        let mut vaults = vec![];

        for (key,value) in vault_permissions.iter() {
            let vault = UserVault {
                id: key.clone(),
                permissions: value.to_vec()
            };
            vaults.push(vault);
        }

        return Ok(vaults);
    }
}
