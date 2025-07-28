use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};

fn handle_event(event: Event) {
    match event {
        EventKind::Create(kind) => {},
        EventKind::Modify(kind) => {},
        EventKind::Remove(kind) => {},
        _ => ()
    };
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;
    
    for res in rx {
        match res {
            Ok(event) => handle_event(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
