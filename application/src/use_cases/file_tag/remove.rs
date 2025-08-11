impl crate::FileTagsUseCase {
    /// # FileTagsUseCase::remove
    ///
    /// check if user has permission to use this function ("hostios:tags:add"), then try to
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
        params: &FileTagRemoveParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::Sdk>, 
        fql_client: &std::sync::Arc<fql::Client>, 
        client: A
    ) -> Result<(), FileTagRemoveError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = FileTagRemoveError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:tags:add")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };
                
        if crate::TagsRepository::retrieve(&params.tag_name, &mut *client).await.is_err() {
            return Err(Error::TagNotExist);
        }
        
        let path = params.file_path.to_string();
        
        let statement = fql::Statement::parse(format!("EXISTS {};", path))
            .map_err(|_| Error::InvalidPath)?;
        let exists = fql_client.execute(statement).await
            .map_err(|_| Error::InvalidPath)?
            .unwrap_bool();
        
        if !exists {
            return Err(Error::PathNotExist);
        }

        let tags = crate::FileTagsRepository::list_tags(&params.file_path, &mut *client)
            .await
            .unwrap();

        if tags.iter().find(|tag| tag == &&params.tag_name).is_none() {
            return Err(Error::NotAddedYet);
        }

        let _ = crate::FileTagsRepository::delete(
            &params.file_path,
            &params.tag_name,
            &mut *client
        ).await;

        return Ok(());
    }
}

pub struct FileTagRemoveParams {
    pub tag_name: String,
    pub file_path: hostios_domain::Path,
    pub user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum FileTagRemoveError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("TAG_NOT_EXIST")]
    TagNotExist,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("PATH_NOT_EXIST")]
    PathNotExist,
    #[error("ALREADY_ADDED")]
    NotAddedYet
}
