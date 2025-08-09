pub struct Client {
    root: std::path::PathBuf
}

impl Client {
    pub fn new(root: std::path::PathBuf) -> Self {
        return Self { root }
    }
    
    pub async fn execute(self: &Self, query: crate::parser::Statement) -> Result<QueryExecuteResult, QueryExecuteError> {
        use crate::parser::Statement;

        match query {
            Statement::CreateDir(path) => {
                let path: std::path::PathBuf = path.into();
                let full_path = self.root.join(path);

                tokio::fs::create_dir(full_path)
                    .await
                    .map_err(|_| QueryExecuteError::Fs(String::from("cannot create dir")))?;

                return Ok(QueryExecuteResult::Null);
            },
            Statement::ReadFile(path) => {
                let path: std::path::PathBuf = path.into();
                let full_path = self.root.join(path);

                let content = tokio::fs::read_to_string(full_path)
                    .await
                    .map_err(|_| QueryExecuteError::Fs(String::from("file not exist")))?;

                return Ok(QueryExecuteResult::String(content));
            },
            Statement::ReadDir(path) => {
                let path: std::path::PathBuf = path.into();
                let full_path = self.root.join(path);

                let mut entries = tokio::fs::read_dir(full_path)
                    .await
                    .map_err(|_| QueryExecuteError::Fs(String::from("dir not exist")))?;
                
                let mut entries_parsed = vec![];

                while let Some(entry) = entries.next_entry().await.map_err(|_| QueryExecuteError::Fs(String::from("cannot read entry")))? {
                    let path = entry.path();

                    if path.is_file() {
                        // TODO: add tags reading
                        let root_str = self.root
                            .to_string_lossy()
                            .to_string();
                        let root_str = format!("{}/", root_str.trim_end_matches('/'));
                        let root_str = root_str.as_str();
                        
                        let path = path
                            .to_string_lossy()
                            .to_string()
                            .trim_start_matches(root_str)
                            .to_string();

                        let path = crate::path::Path::parse(path).unwrap();
                        
                        entries_parsed.push(crate::entry::Entry::File { path, tags: vec![] });
                    }
                    else if path.is_dir() {
                        let root_str = self.root
                            .to_string_lossy()
                            .to_string();
                        let root_str = format!("{}/", root_str.trim_end_matches('/'));
                        let root_str = root_str.as_str();
                        
                        let path = path
                            .to_string_lossy()
                            .to_string()
                            .trim_start_matches(root_str)
                            .to_string();

                        let path = crate::path::Path::parse(path).unwrap();
                        entries_parsed.push(crate::entry::Entry::Directory { path });
                    }
                }
                
                let entries = entries_parsed;

                return Ok(QueryExecuteResult::EntryList(entries));
            },
            Statement::MoveFile { path, new_path } | Statement::MoveDir { path, new_path } => {
                let path: std::path::PathBuf = path.into();
                let new_path: std::path::PathBuf = new_path.into();

                let full_path = self.root.join(path);
                let full_new_path = self.root.join(new_path);

                tokio::fs::rename(full_path, full_new_path)
                    .await
                    .map_err(|_| QueryExecuteError::Fs(String::from("cannot move")))?;

                return Ok(QueryExecuteResult::Null);
            },
            Statement::DeleteDir(path) => {
                let path: std::path::PathBuf = path.into();
                let full_path = self.root.join(path);

                tokio::fs::remove_dir(full_path)
                    .await
                    .map_err(|_| QueryExecuteError::Fs(String::from("directory not empty")))?;

                return Ok(QueryExecuteResult::Null);
            },
            Statement::DeleteFile(path) => {
                let path: std::path::PathBuf = path.into();
                let full_path = self.root.join(path);

                tokio::fs::remove_file(full_path)
                    .await
                    .map_err(|_| QueryExecuteError::Fs(String::from("file not foundd")))?;

                return Ok(QueryExecuteResult::Null);
            },
            _ => ()
        };

        return Ok(QueryExecuteResult::Null);
    }
}

#[derive(Debug)]
pub enum QueryExecuteResult {
    Null,
    PathList(Vec<crate::path::Path>),
    EntryList(Vec<crate::entry::Entry>),
    String(String)
}

impl QueryExecuteResult {
    pub fn unwrap_null(self) -> () {
        return match self {
            Self::Null => (),
            _ => panic!("Cannot unwrap Null value!")
        };
    }
    
    pub fn unwrap_path_list(self) -> Vec<crate::path::Path> {
        return match self {
            Self::PathList(list) => list,
            _ => panic!("Cannot unwrap PathList value!")
        };
    }
    
    pub fn unwrap_entry_list(self) -> Vec<crate::entry::Entry> {
        return match self {
            Self::EntryList(list) => list,
            _ => panic!("Cannot unwrap EntryList value!")
        };
    }
    
    pub fn unwrap_string(self) -> String {
        return match self {
            Self::String(string) => string,
            _ => panic!("Cannot unwrap String value!")
        };
    }
}

#[derive(thiserror::Error, Debug)]
pub enum QueryExecuteError {
    #[error("FS:{0}")] 
    Fs(String)
}
