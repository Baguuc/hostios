impl crate::TagsUseCase {
    /// # TagsUseCase::delete
    ///
    /// check if user has permission to use this function ("hostios:tags:delete"), then try to
    /// delete a tag checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the tag with provided name do not exist;
    /// + when the database connection cannot be acquired;
    ///
    pub async fn delete<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &TagCreateParams, 
        _authios_sdk: authios_sdk::Sdk, 
        client: A
    ) -> Result<(), TagCreateError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = TagCreateError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:tags:delete")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        if crate::TagsRepository::retrieve(&params.name, &mut *client).await.is_err() {
            return Err(Error::NotExist);
        }

        let _ = crate::TagsRepository::delete(
            &params.name,
            &mut *client
        ).await;

        return Ok(());
    }
}

pub struct TagCreateParams {
    name: String,
    description: String,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum TagCreateError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("NOT_EXIST")]
    NotExist
}
