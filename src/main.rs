use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize};
use dotenv::dotenv;

mod api;
mod models;
mod repository;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let server_url = std::env::var("SERVER_URL").expect("SERVER_URL must be set");
    let port = std::env::var("PORT").expect("PORT must be set");

    let todo_db = repository::database::Database::new();
    let app_data = web::Data::new(todo_db);

    println!("Server started at http://{}:{} 🚀", server_url, port);
    
    // cargo watch -x test -x 'run -q'
    // cargo watch -w src/ -x test -x 'run -q'

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    )
        .bind((
            server_url.as_str(),
            port.parse::<u16>().expect("PORT must be a number"),
        ))?
        .run()
        .await
}
