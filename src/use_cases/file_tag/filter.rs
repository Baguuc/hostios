impl crate::use_cases::FileTagsUseCase {
    /// # FileTagsUseCase::filter
    ///
    /// check if user has permission to use this function ("hostios:tags:filter"), then try to
    /// filter files with provided tag, returning their paths and checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the tag do not exist;
    /// + when the database connection cannot be acquired;
    ///
    pub async fn filter<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &FileTagFilterParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::AuthiosSdk>, 
        client: A
    ) -> Result<Vec<String>, FileTagFilterError> {
        pub use authios_sdk::params::UserSdkAuthorizeParams;
        
        type Error = FileTagFilterError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = UserSdkAuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:tags:filter")
        };
        let user_sdk = _authios_sdk.user();

        match user_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
        
        if crate::repositories::TagsRepository::retrieve(&params.tag_name, &mut *client).await.is_err() {
            return Err(Error::TagNotExist);
        }

        let data = crate::repositories::FileTagsRepository::list_paths(
            &params.tag_name,
            &mut *client
        )
            .await
            // won't error
            .unwrap();

        return Ok(data);
    }
}

pub struct FileTagFilterParams {
    pub tag_name: String,
    pub user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum FileTagFilterError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("TAG_NOT_EXIST")]
    TagNotExist,
}
