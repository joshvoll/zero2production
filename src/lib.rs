use actix_web::dev::Server;
use actix_web::{
    web,
    App,
    HttpServer,
    HttpResponse
};
use std::net::TcpListener;

// health_check is going to return a status code HTTP 200
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// run is the main method to run our server
// the method needs to be public method
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

