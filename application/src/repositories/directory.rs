use crate::prelude::*;

pub struct DirectoryRepository;

impl DirectoryRepository {
    pub fn read(path_to_read: &String, tags_root: &String, stash_root: &String) -> Result<domain::Directory> {
        let mut directory = domain::Directory { name: String::new(), dirs: vec![], files: vec![] };
        Self::read_inner(&mut directory, path_to_read, tags_root, stash_root)?;

        return Ok(directory);
    }
    
    pub fn add_file_to_representation(representation: &mut domain::Directory, path: String, file: domain::File) {
        let path = path.trim_matches('/');

        if path == String::new() {
            representation.files.push(file);
            return;
        }

        let split = path
            .split("/")
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        
        let mut current_dir = representation;
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
                let new_dir = domain::Directory {
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
    
    pub fn get_file_from_representation_mut(representation: &mut domain::Directory, path: String, file_name: String) -> Option<&mut domain::File> {
        let path = path.trim_matches('/');

        if path == String::new() {
            let mut found_file = None;

            for file in representation.files.iter_mut() {
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
        
        let mut current_dir = representation;
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
                let new_dir = domain::Directory {
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
    
    fn read_inner(representation: &mut domain::Directory, path_to_read: &String, tags_root: &String, stash_root: &String) -> Result<()> {
        use std::fs::{read_dir, read_link};
        use std::path::PathBuf;

        let entries = read_dir(path_to_read)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let inner_path: String = W(path).try_into()?;
                Self::read_inner(
                    representation,
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
                
                if let Some(file) = Self::get_file_from_representation_mut(representation, sparent.clone(), sname.clone()) {
                    file.tags.push(tag);
                } else {
                    let file = domain::File {
                        name: sname,
                        tags: vec![tag]
                    };
                    Self::add_file_to_representation(representation, sparent, file);
                }
            } else {
                // error
                return Err(Error::Generic(String::from("wrong file type: has to be dir or symlink in tags/")));
            }
        }

        return Ok(());
    }
}
