use crate::prelude::*;

pub struct FileTreeRepository;

impl FileTreeRepository {
    pub fn read(root: &String) -> Result<domain::FileTree> {
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

        let file_tree = domain::FileTree {
            root,
            main_directory
        };

        return Ok(file_tree);
    }
}
