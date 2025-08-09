pub struct Client {
    root: std::path::PathBuf
}

impl Client {
    pub fn new(root: std::path::PathBuf) -> Self {
        return Self { root }
    }
    
    pub fn execute(query: crate::parser::Statement) -> Result<QueryExecuteResult, QueryExecuteError> {
        return Ok(QueryExecuteResult::Null);
    }
}

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

pub enum QueryExecuteError {}
