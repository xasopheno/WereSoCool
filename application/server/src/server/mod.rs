pub mod types;
use crate::server::types::Language;
use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use weresocool::{
    interpretable::InputType,
    manager::{prepare_render_outside, RenderManager},
};

pub async fn single_page_app(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path = PathBuf::from("./src/server/build/index.html");
    Ok(NamedFile::open(path)?)
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Success {
    RenderSuccess(String),
}

pub async fn render(
    render_manager: web::Data<Arc<Mutex<RenderManager>>>,
    req: web::Json<Language>,
) -> HttpResponse {
    // TODO: Pull out prepare_render so it's not inside the lock.
    match prepare_render_outside(InputType::Language(&req.language)) {
        Ok(render) => {
            render_manager.lock().unwrap().push_render(render);
            println!("Success.");
            HttpResponse::Ok().json(Success::RenderSuccess("Success".to_string()))
        }
        Err(parse_error) => {
            let inner = *parse_error.inner;
            HttpResponse::Ok().json(inner.into_serializeable())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::header, test, web, App};

    #[actix_rt::test]
    async fn test_index() {
        let language = Language {
            language: "{f: 100, l: 1, g: 1, p: 0}\nmain={Tm 1}\n".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/api/render")
            .header(header::CONTENT_TYPE, "application/json")
            .set_json(&language)
            .to_request();

        let render_manager = Arc::new(Mutex::new(RenderManager::init_silent()));
        let rm = web::Data::new(Arc::clone(&render_manager));

        let mut app = test::init_service(
            App::new()
                .app_data(rm.clone())
                .service(web::resource("/api/render").route(web::post().to(render))),
        )
        .await;

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}
