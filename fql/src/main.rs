pub mod parser;
pub mod client;
pub mod path;
pub mod entry;
pub mod tag;

#[tokio::main]
async fn main() {
    let root = std::path::PathBuf::from("/www/hostios");
    let client = client::Client::new(root);
    
    let statement = match parser::Statement::parse("DELETE FILE hello2.txt;") {
        Ok(statement) => statement,
        Err(error) => { eprintln!("{:?}", error); return; }
    };
    
    let result = match client.execute(statement).await {
        Ok(result) => result,
        Err(error) => { eprintln!("{:?}", error); return; }
    };
    
    println!("{:?}", result);
}
