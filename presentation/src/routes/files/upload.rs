#[actix_web::post("/files/{name}")]
pub async fn controller(
    actix_multipart::form::MultipartForm(form): actix_multipart::form::MultipartForm<UploadForm>,
    path: actix_web::web::Path<PathData>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<crate::config::Config>,
    _authios_sdk: actix_web::web::Data<authios_sdk::Sdk>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use hostios_domain::Path;
    use crate::utils::authios::authorize;
    
    if !authorize(_authios_sdk, &req, String::from("hostios:files:upload")).await {
        return HttpResponse::Unauthorized().into();
    }

    let file_path = {
        if !Path::parse(&path.name).is_err() { 
            return HttpResponse::BadRequest().into();
        }

        let data_dir = std::path::PathBuf::from(&config.data_dir);
        let full_path = data_dir.join(&path.name);

        full_path
    };

    if file_path.exists() {
        return HttpResponse::Conflict().into();
    }

    return match form.file.file.persist(file_path) {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::Conflict().into()
    };
}

#[derive(Debug, actix_multipart::form::MultipartForm)]
struct UploadForm {
    #[multipart(limit = "2GB")]
    file: actix_multipart::form::tempfile::TempFile,
}

#[derive(serde::Deserialize)]
pub struct PathData {
   name: String
}
