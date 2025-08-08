use crate::prelude::*;

#[derive(Clone)]
/// utils::Path (struct)
///
/// represents a path inside the application context.
/// this path is parsed from PathBuf and always has this traits:
/// + doesn't start with '..', '/' or '~' (exit the application's data directory scope);
/// + exists;
///
pub struct Path(std::path::PathBuf);

impl Path {
    /// # Path::parse
    ///
    /// parse a applcation data path, checking if it follows the constraints
    ///
    pub fn parse(path: std::path::PathBuf, data_dir: &DataDirPath) -> Result<Self> {
        if !Self::validate(&path) {
            return Err(Error::Generic(String::from("provided path tries to escape the scope of the application")));
        }

        let joined = data_dir.join_pathbuf(&path);

        if !joined.exists() {
            return Err(Error::Generic(String::from("path do not exists")));
        }

        return Ok(Self(path));
    }
    
    /// # Path::validate
    ///
    /// check if a PathBuf follows constraints of application's context path.
    ///
    pub fn validate(path: &std::path::PathBuf) -> bool {
        use std::path::PathBuf;
        
        return !(
            path.starts_with(PathBuf::from(".")) || 
            path.starts_with(PathBuf::from("/")) || 
            path.starts_with(PathBuf::from("~"))
        );
    }
} 

impl ToString for Path {
    fn to_string(self: &Self) -> String {
        return self.0
            .to_string_lossy()
            .to_string();
    }
}

#[derive(Clone)]
pub struct DataDirPath(String);

/// # utils::DataDirPath (struct)
///
/// path representing data dir path.
/// this path is parsed from PathBuf and always has this traits:
/// + is absolute;
/// + exists;
/// + is a directory;
///
impl DataDirPath {
    /// # DataDirPath::parse
    ///
    /// parse a DataDirPath, checking if it follows constraints
    ///
    pub fn parse(path: std::path::PathBuf) -> Result<Self> {
        if !path.is_absolute() {
            return Err(Error::Generic("Path has to be absolute".to_string()));
        }
        
        if !path.exists() {
            return Err(Error::Generic("Path do not exist".to_string()));
        }

        if !path.is_dir() {
            return Err(Error::Generic("Path is not a directory".to_string()));
        }

        let as_string = path
            .to_string_lossy()
            .to_string();

        return Ok(Self(as_string));
    }
    
    /// DataDirPath::join
    ///
    /// join a data dir path with application's context path (utils::Path), returning
    /// std::path::PathBuf.
    ///
    pub fn join(self: &Self, inner_path: &Path) -> std::path::PathBuf {
        let inner_path = inner_path.to_string();
        let joined = format!("{}/{}", self.0, inner_path);
        let joined_buf = std::path::PathBuf::from(joined);

        return joined_buf;
    }
    
    /// DataDirPath::join
    ///
    /// join a data dir path with application's context path (utils::Path), returning
    /// std::path::PathBuf.
    ///
    pub fn join_pathbuf(self: &Self, inner_path: &std::path::PathBuf) -> std::path::PathBuf {
        let inner_path = inner_path.to_string_lossy().to_string();
        let joined = format!("{}/{}", self.0, inner_path);
        let joined_buf = std::path::PathBuf::from(joined);

        return joined_buf;
    }
}

impl ToString for DataDirPath {
    fn to_string(self: &Self) -> String {
        return self.0.clone();
    }
}
