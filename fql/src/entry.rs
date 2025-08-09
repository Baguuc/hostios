#[derive(Debug)]
pub enum Entry {
    Directory(crate::path::Path),
    File(crate::path::Path),
}
