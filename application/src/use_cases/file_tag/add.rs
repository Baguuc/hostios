impl crate::FileTagsUseCase {
    /// # FileTagsUseCase::add
    ///
    /// check if user has permission to use this function ("hostios:tags:add"), then try to
    /// add a tag for file path checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    /// + when the tag with provided name do not exist;
    /// + when the path do not exist;
    /// + when the path is invalid;
    /// + when the tag with provided name is already added to this file path;
    /// + when the database connection cannot be acquired;
    ///
    pub async fn add<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &FileTagAddParams, 
        _authios_sdk: &std::sync::Arc<authios_sdk::Sdk>, 
        fql_client: &std::sync::Arc<fql::Client>, 
        client: A
    ) -> Result<(), FileTagAddError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = FileTagAddError;

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

        let path = params.file_path.to_string();
        
        let statement = fql::Statement::parse(format!("EXISTS {};", path))
            .map_err(|_| Error::InvalidPath)?;
        let exists = fql_client.execute(statement).await
            .map_err(|_| Error::InvalidPath)?
            .unwrap_bool();
        
        if !exists {
            return Err(Error::PathNotExist);
        }
        
        if crate::TagsRepository::retrieve(&params.tag_name, &mut *client).await.is_err() {
            return Err(Error::TagNotExist);
        }

        crate::FileTagsRepository::insert(
            &params.file_path,
            &params.tag_name,
            &mut *client
        )
            .await
            .map_err(|_| Error::AlreadyAdded)?;

        return Ok(());
    }
}

pub struct FileTagAddParams {
    pub tag_name: String,
    pub file_path: hostios_domain::Path,
    pub user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum FileTagAddError {
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection,
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("INVALID_PATH")]
    InvalidPath,
    #[error("TAG_NOT_EXIST")]
    TagNotExist,
    #[error("PATH_NOT_EXIST")]
    PathNotExist,
    #[error("ALREADY_ADDED")]
    AlreadyAdded,
}
