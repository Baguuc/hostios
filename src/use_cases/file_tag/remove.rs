impl crate::use_cases::FileTagsUseCase {
    /// # FileTagsUseCase::remove
    ///
    /// check if user has permission to use this function ("hostios:tags:remove"), then try to
    /// remove a tag from file path checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the tag with provided name do not exist;
    /// + when the path do not exist;
    /// + when the path is invalid;
    /// + when the tag with provided name is not added to this path;
    /// + when the database connection cannot be acquired;
    ///
    pub async fn remove<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &crate::params::use_case::FileTagRemoveParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>, 
        fql_client: &std::sync::Arc<crate::fql::Client>, 
        client: A
    ) -> Result<(), crate::errors::use_case::FileTagRemoveError> {
        use crate::params::repository::{
            FileTagDeleteParams,
            FileTagListTagsParams,
            TagRetrieveParams
        };
        use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = crate::errors::use_case::FileTagRemoveError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:tags:remove")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
                
        if crate::repositories::TagsRepository::retrieve(TagRetrieveParams { name: params.tag_name.clone() }, &mut *client).await.is_err() {
            return Err(Error::TagNotExist);
        }
        
        let path = params.file_path.to_string();
        
        let statement = crate::fql::Statement::parse(format!("EXISTS {};", path))
            .map_err(|_| Error::InvalidPath)?;
        let exists = fql_client.execute(statement).await
            .map_err(|_| Error::InvalidPath)?
            .unwrap_bool();
        
        if !exists {
            return Err(Error::PathNotExist);
        }

        let tags = crate::repositories::FileTagsRepository::list_tags(FileTagListTagsParams { path: params.file_path.to_string() }, &mut *client)
            .await
            .unwrap();

        if tags.iter().find(|tag| tag == &&params.tag_name).is_none() {
            return Err(Error::NotAddedYet);
        }

        let _ = crate::repositories::FileTagsRepository::delete(
            FileTagDeleteParams {
                path: params.file_path.clone().to_string(),
                tag_name: params.tag_name.clone()
            },
            &mut *client
        ).await;

        return Ok(());
    }
}
