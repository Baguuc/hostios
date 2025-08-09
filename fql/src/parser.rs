use crate::path::Path;

#[derive(Debug)]
pub enum Statement {
    CreateDir(Path),
    ReadFile(Path),
    ReadDir(Path),
    MoveFile { path: Path, new_path: Path },
    MoveDir { path: Path, new_path: Path },
    DeleteDir(Path),
    DeleteFile(Path)
}

impl Statement {
    pub fn parse(string: impl ToString) -> Result<Self, StatementParseError> {
        let string = string.to_string();
        let mut state = StatementParseState::Verb;
        
        let mut current = String::new();

        let mut raw_verb = String::new();
        let mut raw_entity = String::new();
        let mut raw_data = String::new();

        for char in string.chars() {
            match (&state, char) {
                (StatementParseState::Verb, ' ') => {
                    state = StatementParseState::Entity;
                    
                    raw_verb = current;
                    current = String::new();
                },

                (StatementParseState::Entity, ' ') => {
                    state = StatementParseState::Data;

                    raw_entity = current;
                    current = String::new();
                },
                
                (StatementParseState::Data, ';') => {
                    raw_data = current;

                    break;
                },

                (_, c) => {
                    let lowercase = c.to_lowercase().to_string();
                    current.push_str(&lowercase);
                }
            }
        }

        let statement = match (raw_verb.as_str(), raw_entity.as_str()) {
            ("create", "dir") => {
                let path = Path::parse(raw_data)
                    .map_err(|error| StatementParseError::InvalidPath(error))?;

                Self::CreateDir(path)
            },
            ("read", "file") => {
                let path = Path::parse(raw_data)
                    .map_err(|error| StatementParseError::InvalidPath(error))?;

                Self::ReadFile(path)
            },
            ("read", "dir") => {
                let path = Path::parse(raw_data)
                    .map_err(|error| StatementParseError::InvalidPath(error))?;

                Self::ReadDir(path)
            },
            ("move", "file") => {
                let paths = raw_data.split(", ").collect::<Vec<&str>>();
                
                if paths.len() != 2 {
                    return Err(StatementParseError::InvalidData(String::from("too much or too less paths, have to be 2 splitted by ',' character.")));
                }

                let path = Path::parse(paths.get(0).unwrap())
                    .map_err(|error| StatementParseError::InvalidPath(error))?;

                let new_path = Path::parse(paths.get(1).unwrap())
                    .map_err(|error| StatementParseError::InvalidPath(error))?;

                Self::MoveFile { path, new_path }
            },
            ("move", "dir") => {
                let paths = raw_data.split(", ").collect::<Vec<&str>>();
                
                if paths.len() != 2 {
                    return Err(StatementParseError::InvalidData(String::from("too much or too less paths, have to be 2 splitted by ',' character.")));
                }

                let path = Path::parse(paths.get(0).unwrap())
                    .map_err(|error| StatementParseError::InvalidPath(error))?;

                let new_path = Path::parse(paths.get(1).unwrap())
                    .map_err(|error| StatementParseError::InvalidPath(error))?;

                Self::MoveDir { path, new_path }
            },
            ("delete", "dir") => {
                let path = Path::parse(raw_data)
                    .map_err(|error| StatementParseError::InvalidPath(error))?;

                Self::DeleteDir(path)
            },
            ("delete", "file") => {
                let path = Path::parse(raw_data)
                    .map_err(|error| StatementParseError::InvalidPath(error))?;

                Self::DeleteFile(path)
            }
        };
        
        return Ok(statement);
    }
}

enum StatementParseState {
    Verb,
    Entity,
    Data
}

#[derive(thiserror::Error, Debug)]
pub enum StatementParseError {
    #[error("INVALID_PATH:{0}")]
    InvalidPath(#[from] crate::path::PathParseError),
    #[error("INVALID_TARGET:{0}")]
    InvalidData(String),
    #[error("INVALID_VERB_AND_ENTITY")]
    InvalidVerbAndEntity
}
