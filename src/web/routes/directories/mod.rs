pub mod create;
pub mod read;
pub mod move_;
pub mod delete;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/directories")
        .service(create::controller)
        .service(delete::controller)
        .service(move_::controller)
        .service(read::controller)
}
