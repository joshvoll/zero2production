use std::net::TcpListener;
use zero2prod::run;
use zero2prod::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let read the configuration and panic if we can't read it 
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await 
}
