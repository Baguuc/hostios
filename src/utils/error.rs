pub fn error_if_necessary<T, E: std::fmt::Display>(r: std::result::Result<T, E>) -> T {
    match r {
        Ok(ok) => return ok,
        Err(err) => {
            clin::components::error("something went wrong", err);
            std::process::exit(1);
        }
    }
}
