use actix_web::dev::Server;
use actix_web::{
    web,
    App,
    HttpServer,
    HttpResponse
};
use std::net::TcpListener;

// FormData definition 
#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}

// health_check response if the server is up and running
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// subscribe is going to add the subscription method for the users
async fn subscribe(_web: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// run is the method that initialize the server
// method needs to be public
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {  
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // return the server and error
    Ok(server)
}

pub mod configuration;
pub mod routes;
pub mod startup;
