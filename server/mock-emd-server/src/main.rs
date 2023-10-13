use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/usb-data")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() {
    let mut http_server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(closure_settings.clone()))
            .wrap(logger_middleware())
            .wrap(cors_policy(&closure_settings))
            .wrap(compress_middleware())
            // needed for static files service
            .app_data(Data::new(closure_settings.clone()))
            .configure(attach_graphql_schema(graphql_schema.clone()))
            .configure(config_static_files)
            .configure(config_server_frontend)
    })
    .disable_signals();
}
