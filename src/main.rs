mod error;
mod prelude;
mod utils;

use crate::prelude::*;

struct File {
    name: String,
    tags: Vec<String>
}

fn main() -> Result<()> {
    let files = read(
        &String::from("/home/baguuc/hostios-example/baguuc"),
    )?;

    println!("{:?}", files);
    
    return Ok(());    
}

fn read(root: &String) -> Result<std::collections::HashMap<String, Vec<String>>> {
    use std::fs::{read_dir,read_link};
    use std::path::PathBuf;
    use std::collections::HashMap;
    
    let stash_root = format!(
        "{}/stash/",
        root.trim_end_matches('/'),
    );
    let tags_root = format!(
        "{}/tags/",
        root.trim_end_matches('/'),
    );

    let mut files = HashMap::new();
    
    let _ = read_inner(
        &tags_root,
        &mut files,
        &tags_root,
        &stash_root
    )?;

    let files = files;

    return Ok(files);
}

fn read_inner(path_to_read: &String, files: &mut std::collections::HashMap<String, Vec<String>>, tags_root: &String, stash_root: &String) -> Result<()> {
    use std::fs::{read_dir, read_link};
    use std::path::PathBuf;

    let entries = read_dir(path_to_read)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let inner_path: String = W(path).try_into()?;
            read_inner(
                &inner_path,
                files,
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
            let spath: String = W(spath).try_into()?;
            let spath = spath.trim_start_matches(stash_root.as_str())
                .to_string();
            
            if let Some(file) = files.get_mut(&spath) {
                file.push(tag);
            } else {
                files.insert(spath, vec![tag]);
            }
        } else {
            // error
            return Err(Error::Generic(String::from("wrong file type: has to be dir or symlink in tags/")));
        }
    }

    return Ok(());
}
