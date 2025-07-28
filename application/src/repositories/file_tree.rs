use crate::prelude::*;

pub struct FileTreeRepository;

impl FileTreeRepository {
    pub async fn read_dir(root: String, path: String) -> Result<hostios_domain::Directory> { crate::DirectoryRepository::read_dir(&root, path).await }

    pub async fn stash_file(root: &String, file_path: String) -> Result<()> {
        use tokio::fs::rename;
        
        let file_path_parts = file_path
            .split('/')
            .collect::<Vec<&str>>();
        let file_name = file_path_parts 
            .last()
            .ok_or(Error::Generic(String::from("Invalid file name")))?;

        let target = format!("{}/stash/{}", root, file_name);
        rename(file_path, target).await?;

        return Ok(());
    }
    
    pub async fn unstash_file(root: &String, stash_path: String) -> Result<()> {
        use tokio::fs::remove_file as remove;
        
        let target = format!("{}/stash/{}", root, stash_path.trim_matches('/'));
        remove(target).await?;

        return Ok(());
    }

    pub async fn tag_file(root: &String, stash_path: String, tag: String) -> Result<()> {
        use std::os::unix::fs::symlink;
        
        let tag_path_filename = stash_path.replace("/", "-");
        let to = format!("{}/stash/{}", root, stash_path);
        let at = format!("{}/tags/{}/{}", root, tag, tag_path_filename);
        
        symlink(to, at)?;
        return Ok(());   
    }
    
    pub async fn untag_file(root: &String, stash_path: String, tag: String) -> Result<()> {
        use tokio::fs::remove_file as remove;
        
        let tag_path_filename = stash_path.replace("/", "-");
        let at = format!("{}/tags/{}/{}", root, tag, tag_path_filename);
        
        remove(at).await?;
        return Ok(());   
    }
}
