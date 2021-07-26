use crate::routes::health_check;
use crate::routes::subscribe;
use actix_web::dev::Server;
use actix_web::{
    web,
    App,
    HttpServer
};
use sqlx::PgPool;
use std::net::TcpListener;

// run is the method that start everything 
// Wrap the pool using web::Data, which boils down to an Arc smart pointer
// using .data would add another Arc pointer on top
// .app_data instead does not perform an additional layer of wrapping.
pub fn run(
    listener: TcpListener,
    db_pool: PgPool
) -> Result<Server, std::io::Error> {    
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}


