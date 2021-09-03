use std::collections::HashMap;

use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Result, http::ContentEncoding};

use sailfish::TemplateOnce;

use actix_files as fs;

#[derive(TemplateOnce)]  // automatically implement `TemplateOnce` trait
#[template(path = "index.html")]  // specify the path to template
struct IndexTemplate<'a, 'b, 'c, 'd> {
    title: &'a str,
    keywords: &'b str,
    description: &'c str,
    test: &'d str,
}

#[derive(TemplateOnce)]
#[template(path = "about/index.html")]
struct AboutTemplate<'a, 'b, 'c> {
    title: &'a str,
    keywords: &'b str,
    description: &'c str,
}

#[derive(TemplateOnce)]
#[template(path = "error.html")]
struct ErrorTemplate<'a, 'b> {
    error: &'a str,
    status_code: &'b str,
}

//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

async fn index(
    _query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let s = IndexTemplate {
        title: "Index Title",
        keywords: "Index Keywords",
        description: "Index Description",
        test: "Index Test",
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s.render_once().unwrap()))
}

async fn favicon() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./files/favicon.ico")?)
}

async fn styles() -> actix_web::Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./files/styles.css")?)
}

async fn about(
    _query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let s = AboutTemplate {
        title: "About Title",
        keywords: "About Keywords",
        description: "About Description",
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s.render_once().unwrap()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
/*
    // load ssl keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();
*/
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    println!("Listening on: 127.0.0.1:5000, open browser and visit have a try!");
    HttpServer::new(|| {

        App::new()
            .wrap(middleware::Logger::default()) // enable logger
            .wrap(middleware::Compress::new(ContentEncoding::Gzip))
            .service(web::resource("/favicon.ico").route(web::get().to(favicon)))
            .service(web::resource("/styles.css").route(web::get().to(styles)))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/index").route(web::get().to(index)))
            .service(web::resource("/about").route(web::get().to(about)))
            .service(fs::Files::new("/assets", "./files/assets").show_files_listing())
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind("0.0.0.0:5000")? //http
    //.bind_openssl("0.0.0.0:5000", builder)? //https
    .run()
    .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let status = res.status();
    let s = ErrorTemplate {
        error: error,
        status_code: status.as_str(),
    };

    Response::build(res.status())
        .content_type("text/html")
        .body(s.render_once().unwrap())
}
