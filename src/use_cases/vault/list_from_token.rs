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
        
        let mut vault_permissions: HashMap<String, Vec<String>> = HashMap::new();
        
        // TODO: extract this whole logic into authios with some method like "search_permissions"
        // and pattern suppling like this:
        // ```
        // // returns a HashMap with named values
        // sdk.search_permissions(
        //   Params { token, pattern: String::from("hostios:vaults:{vault_id}:{vault_permission}") }
        // )
        // ```
        for permission_name in permissions {
            if !permission_name.starts_with("hostios:vaults:") { continue; }
            
            // get vault id
            let vault_id = permission_name.chars()
                // skip "hostios:vaults:" perfix
                .skip(15)
                // take while another part of the permission doesn't come up 
                .take_while(|c| *c != ':')
                .collect::<String>();
            let vault_id_len = vault_id.len();
            
            // hostios:vaults:<id>:---read|write
            // |             ^|  ^|   |        ^
            // 15.............x...1---4/5.......
            //
            // the permission name is shorter than the bare minimum of all possibilities
            // so it's invalid so skip
            //
            if permission_name.len() <= 15+vault_id_len+1+4 { continue; }
            
            // get the permission
            let vault_permission = permission_name.chars()
                // skip "hostios:vaults:" perfix
                .skip(15+vault_id_len+1)
                // take while another part of the permission doesn't come up
                .collect::<String>();
            
            // invalid permission name
            if vault_permission != "read" && vault_permission != "write" { continue; }
            
            if let Some(permissions) = vault_permissions.get_mut(&vault_id) {
                permissions.push(vault_permission);
            } else {
                vault_permissions.insert(vault_id, vec![vault_permission]);
            }
        }

        let mut vaults = vec![];

        let read_permission = String::from("read");
        let write_permission = String::from("write");
        
        // merge the permissions array into one consisten structure
        for (key,value) in vault_permissions.iter() { 
            if value.contains(&read_permission) && value.contains(&write_permission) {
                vaults.push(UserVault { id: key.to_string(), permissions: UserVaultPermission::ReadWrite });
                continue; 
            } 
            
            if value.contains(&read_permission) {
                vaults.push(UserVault { id: key.to_string(), permissions: UserVaultPermission::Read });
                continue;
            }
            
            if value.contains(&write_permission) {
                vaults.push(UserVault { id: key.to_string(), permissions: UserVaultPermission::Write });
            }
        }

        return Ok(vaults);
    }
}
