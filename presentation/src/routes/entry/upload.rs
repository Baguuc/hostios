#[derive(Debug, actix_multipart::form::MultipartForm)]
struct UploadForm {
    #[multipart(limit = "2GB")]
    file: actix_multipart::form::tempfile::TempFile,
}

#[derive(serde::Deserialize)]
pub struct PathData {
   name: String
}

#[actix_web::post("/entries/{name}")]
pub async fn controller(
    actix_multipart::form::MultipartForm(form): actix_multipart::form::MultipartForm<UploadForm>,
    path: actix_web::web::Path<PathData>,
    req: actix_web::HttpRequest,
    config: actix_web::web::Data<crate::config::Config>,
    _authios_sdk: actix_web::web::Data<authios_sdk::Sdk>
) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;
    use std::path::PathBuf;
    use hostios_application::utils::Path;
    use crate::utils::authios::authorize;
    
    if !authorize(_authios_sdk, &req, config.service_permission.clone()).await {
        return HttpResponse::Unauthorized().into();
    }

    let file_path = {
        let file_path = path.name.clone();
        
        if !Path::validate(&PathBuf::from(&file_path)) { 
            return HttpResponse::BadRequest().into();
        }

        format!(
            "{}/{}",
            config.data_dir,
            file_path
        )
    };

    if std::path::Path::new(&file_path).exists() {
        return HttpResponse::Conflict().into();
    }

    return match form.file.file.persist(file_path) {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => HttpResponse::Conflict().into()
    };
}
