use std::fs;
use actix_web::{http, web, HttpResponse, Result, middleware};

async fn index() -> Result<HttpResponse> {
    let data = fs::read_to_string("./src/info.json").expect("Unable to read file");
    let response_json: serde_json::Value = serde_json::from_str(&data).expect("JSON was not well-formatted");
    Ok(HttpResponse::Ok().json(response_json))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| {
        App::new()
        .wrap(middleware::DefaultHeaders::new()
            .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET,POST,OPTIONS")
            .header(http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type")
            .header(http::header::CONTENT_TYPE, "application/json")
        )
        .service(web::resource("/")
                 .route(web::method(http::Method::OPTIONS).to(|| HttpResponse::Ok()))
                 .route(web::get().to(|| HttpResponse::Ok()))
                 .route(web::post().to(index)))
             })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}