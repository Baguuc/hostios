pub fn validate_path_scope(path: &String) -> bool {
    if path == &String::from("/") {
        return true;
    } else {
        return !(
            path.starts_with("/")
            || path.starts_with("..")
            || path.starts_with("~")
        );
    }
}

pub fn join_paths(p1: String, p2: String) -> String {
    return format!(
        "{}/{}",
        p1.trim_end_matches("/"),
        p2.trim_start_matches("/")
    );
}
