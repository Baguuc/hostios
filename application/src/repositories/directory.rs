use crate::prelude::*;

pub struct DirectoryRepository;

impl DirectoryRepository {
    pub async fn read_dir(root: &String, path: String) -> Result<hostios_domain::Directory> {
        use tokio::fs::{read_dir,read_link};
        use std::collections::HashMap;
        use crate::utils::file::get_tags_root;
        
        let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();
        
        let tags_path = get_tags_root(root);
        let mut dir_content = read_dir(&tags_path).await?;

        while let Some(entry) = dir_content.next_entry().await? {
            let path: String = W(entry.path())
                .try_into()?;

            match Self::read_tag(root, path, &mut tag_map).await {
               Err(err) => return Err(err), 
                _ => ()
            };
        }
        
        let tag_map = tag_map;

        return Self::read_stash(root, path, tag_map).await;
    }
    
    #[async_recursion::async_recursion]
    async fn read_tag(root: &String, path: String, tag_map: &mut std::collections::HashMap<String, Vec<String>>) -> Result<()> {
        use tokio::fs::{read_dir,read_link};
        use crate::utils::file::{strip_stash_root,get_tag_dir_path};
        
        let mut dir_content = read_dir(path).await?;
        
        while let Some(entry) = dir_content.next_entry().await? {
            let path = entry.path();

            if path.is_symlink() {
                let path = W(path.clone()).try_into()?;
                let tag = get_tag_dir_path(root, &path);
                let link_content = read_link(path).await?;

                let path = W(link_content).try_into()?;                
                let stash_path = strip_stash_root(root, &path);
                
                if let Some(entry) = tag_map.get_mut(&stash_path) {
                    entry.push(tag);
                } else {
                    tag_map.insert(stash_path, vec![tag]);
                }
            }
            else if path.is_dir() {
                let dir_path: String = W(path).try_into()?;

                Box::pin(Self::read_tag(root, dir_path, tag_map).await?);
            }
        }

        return Ok(());
    }

    async fn read_stash(root: &String, path: String, tag_map: std::collections::HashMap<String, Vec<String>>) -> Result<hostios_domain::Directory> {
        use tokio::fs::read_dir;
        use hostios_domain::{File,FileType,Directory};
        use crate::utils::file::{get_stash_root,strip_stash_root};
        
        let stash_root = get_stash_root(root);
        let full_path = format!("{}/{}", stash_root, path.trim_matches('/'));
        
        let mut files = vec![];

        let mut dir_content = read_dir(&full_path).await?;
        
        while let Some(entry) = dir_content.next_entry().await? {
            let path = entry.path();
            let path_string: String = W(entry.path()).try_into()?;
            let stash_path = strip_stash_root(root, &path_string);
            let name = stash_path
                .clone()
                .split("/")
                .collect::<Vec<&str>>()
                .pop()
                .unwrap()
                .to_string();
            let tags = tag_map.get(&stash_path)
                .unwrap_or(&vec![])
                .clone();
            let file_type = if path.is_file() { FileType::File } else { FileType::Directory };
            
            let file = File {
                stash_path,
                name,
                tags,
                file_type
            };
            files.push(file);
        }

        let full_path = strip_stash_root(root, &path);
        let name = full_path
            .clone()
            .split("/")
            .collect::<Vec<&str>>()
            .pop()
            .unwrap()
            .to_string();
        let files = files;

        let directory = Directory {
            full_path,
            name,
            files
        };

        return Ok(directory);
    }
}
