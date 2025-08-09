pub mod parser;
pub mod client;
pub mod path;
pub mod entry;
pub mod tag;

fn main() {
    let content = std::fs::read_to_string(".tags/dir1f%1dir2f%1test.txt").unwrap();
    println!("{}", content);
}
