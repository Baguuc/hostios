impl crate::DirectoriesUseCase {
    /// # DirectoriesUseCase::read
    ///
    /// check if user has permission to use this function ("hostios:directories:read), then try to
    /// read a directory, checking for errors
    ///
    /// Errors:
    /// + when user is not authorized to use this function;
    ///
    pub async fn read<'a, A: sqlx::Acquire<'a, Database = sqlx::Postgres>>(
        params: &DirectoryReadParams, 
        _authios_sdk: authios_sdk::Sdk,
        client: A
    ) -> Result<hostios_domain::Directory, DirectoryReadError> {
        pub use authios_sdk::user::authorize::AuthorizeParams;
        
        type Error = DirectoryReadError;

        let mut client = client.acquire()
            .await
            .map_err(|_| Error::DatabaseConnection)?;

        let authorize_params = AuthorizeParams {
            token: params.user_token.clone(),
            permission: String::from("hostios:directories:read")
        };

        match _authios_sdk.authorize(authorize_params).await {
            Ok(true) => (),
            Err(_) | Ok(false) => return Err(Error::Unauthorized)
        };

        // won't error as the path checks if the file exists
        let data = crate::DirectoriesRepository::read(&params.file_path, &params.data_dir)
            .await
            .unwrap();

        let mut entries = vec![];

        for path in data {
            let pathbuf = std::path::PathBuf::from(path.to_string());

            if pathbuf.is_file() {
                let file = crate::FilesRepository::retrieve(&path, &mut *client)
                    .await
                    // it won't error
                    .unwrap();

                entries.push(hostios_domain::Entry::File(file));
            } else {
                let directory = hostios_domain::Entry::Directory { full_path: path.to_string() };

                entries.push(directory);
            }
        }

        let joined_path = params.data_dir.join(&params.file_path);        
        let entries = entries;

        let data = hostios_domain::Directory {
            full_path: joined_path.to_string_lossy().to_string(),
            entries
        };

        return Ok(data);
    }
}

pub struct DirectoryReadParams {
    file_path: crate::utils::Path,
    data_dir: crate::utils::DataDirPath,
    user_token: String
}

#[derive(thiserror::Error, Debug)]
pub enum DirectoryReadError {
    #[error("UNAUTHORIZED")]
    Unauthorized,
    #[error("DATABASE_CONNECTION")]
    DatabaseConnection
}
