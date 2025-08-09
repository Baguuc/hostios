pub mod parser;
pub mod path;

fn main() {
    let statement = parser::Statement::parse("move dir path, new_path;").unwrap();
    println!("{:?}", statement);
}
