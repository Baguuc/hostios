use notify::{Event, EventKind, event::{CreateKind, ModifyKind, RemoveKind}, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};

fn handle_event(root: &String, event: Event) {
    use std::path::PathBuf;
    use futures::executor::block_on;
    use hostios_application::{FileTreeRepository,utils::file::{get_stash_root,get_tags_root}};

    match event.kind {
        EventKind::Modify(ModifyKind::Name(_)) | EventKind::Create(CreateKind::File) => {
            let default = std::path::PathBuf::new();
            let root_path = PathBuf::from(root);
            let path = event.paths.get(0).unwrap_or(&default);
            let path_string = path.to_str().unwrap().to_string();
            let tags_root = PathBuf::from(get_tags_root(root));
            let stash_root = PathBuf::from(get_stash_root(root));
            let parent_path = path.parent().unwrap();
            let parent_path_string = parent_path.to_str().unwrap().to_string();
            let stash_root_string = stash_root.to_str().unwrap().to_string();
            let tags_root_string = tags_root.to_str().unwrap().to_string();

            if !path.is_file() || path.is_symlink() {
                return;
            }
            
            if parent_path == root_path || parent_path == tags_root {
                // CREATED | MOVED, tags/
                block_on(FileTreeRepository::stash_file(root, path_string));

                return;
            }
            
            if parent_path.starts_with(&tags_root) {
                // CREATED | MOVED, tags/<tag_name>
                match block_on(FileTreeRepository::stash_file(root, path_string.clone())) {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", err)
                };

                let stash_path = path.file_name().unwrap().to_str().unwrap().to_string();
                let tag = parent_path_string.replace("./tags", "tags").trim_start_matches(tags_root_string.as_str()).to_string();

                match block_on(FileTreeRepository::tag_file(root, stash_path, tag)) {
                    Ok(_) => (),
                    Err(err) => eprintln!("{}", err)
                };
                
                return;
            }
        },
        EventKind::Remove(RemoveKind::File) => {
            let default = std::path::PathBuf::new();
            let root_path = PathBuf::from(root);
            let stash_root = PathBuf::from(get_stash_root(root));

            // delete all related tags
        },
        EventKind::Remove(RemoveKind::Folder) => {
            // delete all related tags
        },
        _ => ()
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
    
    let root = String::from("/home/baguuc/code/hostios/daemon");

    for res in rx {
        match res {
            Ok(event) => handle_event(&root, event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
