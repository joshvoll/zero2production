use actix_web::dev::Server;
use actix_web::{
    web,
    App,
    HttpServer,
    HttpResponse
};
use std::net::TcpListener;

// FormData represent the data from the post request
#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}



// health_check is going to return a status code HTTP 200
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// subscribe access the subscription for users
async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

// run is the main method to run our server
// the method needs to be public method
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server) 
}

