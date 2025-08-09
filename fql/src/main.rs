pub mod parser;
pub mod client;
pub mod path;
pub mod entry;
pub mod tag;

fn main() {
    let root = std::path::PathBuf::from("/www/hostios");
    let client = client::Client::new(root);
    
    let result = client.execute(parser::Statement::parse("CREATE DIR tst-directory/test;").unwrap());
    
    if result.is_err() {
        eprintln!("{:?}", result.unwrap_err());
    }
}
