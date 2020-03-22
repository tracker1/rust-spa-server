// use std::{env, io};

use actix_files as fs;
// use actix_session::{CookieSession, Session};
// use actix_utils::mpsc;
use actix_web::http::{
    //header, Method, 
    StatusCode
};
use actix_web::{
    //error, 
    guard, middleware, web, App, 
    //Error, HttpRequest, 
    HttpResponse, HttpServer,
    // Guard,
    Result,
};
// use bytes::Bytes;

/// 404 handler
async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?.set_status_code(StatusCode::FOUND))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(
                // static files
                fs::Files::new("/", "./static/")
                    .redirect_to_slash_directory()
                    .use_etag(true)
                    .use_last_modified(true)
                    .index_file("index.html"),
            )
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}