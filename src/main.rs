mod error;
mod prelude;
mod utils;

use crate::prelude::*;

fn main() -> Result<()> {
    let file_tree = FileTree::read(
        &String::from("/home/baguuc/hostios-example/baguuc"),
    )?;
    let as_string = serde_json::to_string_pretty(&file_tree).unwrap();

    println!("{}", as_string);
    
    return Ok(());    
}

#[derive(Debug, serde::Serialize)]
struct Directory {
    name: String,
    files: Vec<File>,
    dirs: Vec<Self>
}

#[derive(Debug, serde::Serialize)]
struct File {
    name: String,
    tags: Vec<String>
}

#[derive(Debug, serde::Serialize)]
struct FileTree {
    root: String,
    main_directory: Directory
}

impl FileTree {
    fn read(root: &String) -> Result<Self> {
        let stash_root = format!(
            "{}/stash",
            root.trim_end_matches('/'),
        );
        let tags_root = format!(
            "{}/tags/",
            root.trim_end_matches('/'),
        );
        
        let mut main_directory = Directory { name: String::new(), dirs: vec![], files: vec![] };
        let _ = main_directory.read_inner(
            &tags_root,
            &tags_root,
            &stash_root
        )?;
        let main_directory = main_directory;
        let root = root.clone();

        let file_tree = FileTree {
            root,
            main_directory
        };

        return Ok(file_tree);
    }
}

impl Directory {
    fn read_inner(&mut self, path_to_read: &String, tags_root: &String, stash_root: &String) -> Result<()> {
        use std::fs::{read_dir, read_link};
        use std::path::PathBuf;

        let entries = read_dir(path_to_read)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let inner_path: String = W(path).try_into()?;
                self.read_inner(
                    &inner_path,
                    tags_root,
                    stash_root
                )?;
            } else if path.is_symlink() {
                let tag_path = path
                    .parent()
                    .ok_or(Error::Generic("cannot retrieve parent".to_string()))?;
                let tag: String = W(PathBuf::from(tag_path)).try_into()?;
                let tag = tag.trim_start_matches(tags_root.as_str())
                    .to_string();

                let spath = PathBuf::from(read_link(path)?);
                let sname = spath
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                let sparent = spath
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .trim_start_matches(stash_root.as_str())
                    .to_string();
                
                if let Some(file) = self.get_file_from_representation_mut(sparent.clone(), sname.clone()) {
                    file.tags.push(tag);
                } else {
                    let file = File {
                        name: sname,
                        tags: vec![tag]
                    };
                    self.add_file_to_representation(sparent, file);
                }
            } else {
                // error
                return Err(Error::Generic(String::from("wrong file type: has to be dir or symlink in tags/")));
            }
        }

        return Ok(());
    }
    
    fn add_file_to_representation(&mut self, path: String, file: File) {
        let path = path.trim_matches('/');

        if path == String::new() {
            self.files.push(file);
            return;
        }

        let split = path
            .split("/")
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        
        let mut current_dir = self;
        for part in split {
            let mut found_idx: Option<usize> = None;

            for (idx,inner_dir) in current_dir.dirs.iter().enumerate() {
                if inner_dir.name == part {
                    found_idx = Some(idx);
                    break;
                }
            }
            
            if let Some(idx) = found_idx {
                current_dir = current_dir.dirs
                    .get_mut(idx)
                    .unwrap();
            } else {
                let new_dir = Directory {
                    name: part,
                    files: vec![],
                    dirs: vec![]
                };
                current_dir.dirs.push(new_dir);
                current_dir = current_dir.dirs.last_mut().unwrap();
            }
        }

        current_dir.files.push(file);
    }
    
    fn get_file_from_representation_mut(&mut self, path: String, file_name: String) -> Option<&mut File> {
        let path = path.trim_matches('/');

        if path == String::new() {
            let mut found_file = None;

            for file in self.files.iter_mut() {
                if file.name == file_name {
                    found_file = Some(file);
                    break;
                }
            }

            return found_file;
        }

        let split = path
            .split("/")
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        
        let mut current_dir = self;
        for part in split {
            let mut found_idx: Option<usize> = None;

            for (idx,inner_dir) in current_dir.dirs.iter().enumerate() {
                if inner_dir.name == part {
                    found_idx = Some(idx);
                    break;
                }
            }
            
            if let Some(idx) = found_idx {
                current_dir = current_dir.dirs
                    .get_mut(idx)
                    .unwrap();
            } else {
                let new_dir = Directory {
                    name: part,
                    files: vec![],
                    dirs: vec![]
                };
                current_dir.dirs.push(new_dir);
                current_dir = current_dir.dirs.last_mut().unwrap();
            }
        }
        
        let mut found_file = None;

        for file in current_dir.files.iter_mut() {
            if file.name == file_name {
                found_file = Some(file);
                break;
            }
        }

        return found_file;
    }
}
