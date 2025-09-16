pub mod upload;
pub mod read;
pub mod move_;
pub mod delete;
pub mod add_tag;
pub mod remove_tag;
pub mod search_by_tag;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/files")
        .service(upload::controller)
        .service(read::controller)
        .service(move_::controller)
        .service(delete::controller)
        .service(add_tag::controller)
        .service(remove_tag::controller)
        .service(search_by_tag::controller)
}
