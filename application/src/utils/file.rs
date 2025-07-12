use crate::prelude::*;

pub fn get_stash_root(root: &String) -> String { format!("{}/stash", root.trim_end_matches('/')) }

pub fn get_stash_dir_path(root: &String, path: &String) -> String {
    let root = get_stash_root(root);
    let path = path
        .trim_start_matches(root.as_str())
        .to_string();
    let mut path = path
        .split('/')
        .collect::<Vec<&str>>();
    let _ = path.pop();
    let path = path.join("/");

    return path;
}

pub fn strip_stash_root(root: &String, path: &String) -> String {
    let root = get_stash_root(root);
    let path = path
        .trim_start_matches(root.as_str())
        .to_string();

    return path;
}

pub fn get_tags_root(root: &String) -> String { format!("{}/tags/", root.trim_end_matches('/')) }

pub fn get_tag_dir_path(root: &String, path: &String) -> String {
    let root = get_tags_root(root);
    let path = path
        .trim_start_matches(root.as_str())
        .to_string();
    let mut path = path
        .split('/')
        .collect::<Vec<&str>>();
    let _ = path.pop();
    let path = path.join("/");

    return path;
}

pub fn strip_tags_root(root: &String, path: &String) -> String {
    let root = get_stash_root(root);
    let path = path
        .trim_start_matches(root.as_str())
        .to_string();

    return path;
}
