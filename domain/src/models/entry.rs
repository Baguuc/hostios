#[derive(Debug, Clone)]
pub enum Entry {
    Directory(crate::Path),
    File(crate::Path)
}
