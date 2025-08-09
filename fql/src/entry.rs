pub enum Entry {
    Directory { path: crate::path::Path },
    File { path: crate::path::Path, tags: Vec<crate::tag::Tag> },
}
