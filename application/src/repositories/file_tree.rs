use crate::prelude::*;

pub struct FileTreeRepository;

impl FileTreeRepository {
    pub fn read(root: &String) -> Result<hostios_domain::FileTree> {
        let stash_root = format!(
            "{}/stash",
            root.trim_end_matches('/'),
        );
        let tags_root = format!(
            "{}/tags/",
            root.trim_end_matches('/'),
        );
        
        let main_directory = crate::DirectoryRepository::read(
            &tags_root,
            &tags_root,
            &stash_root
        )?;
        let root = root.clone();

        let file_tree = hostios_domain::FileTree {
            root,
            main_directory
        };

        return Ok(file_tree);
    }

    pub fn stash_file(root: &String, file_path: String) -> Result<()> {
        use std::fs::rename;
        
        let file_path_parts = file_path
            .split('/')
            .collect::<Vec<&str>>();
        let file_name = file_path_parts 
            .last()
            .ok_or(Error::Generic(String::from("Invalid file name")))?;

        let target = format!("{}/stash/{}", root, file_name);
        rename(file_path, target)?;

        return Ok(());
    }
}
