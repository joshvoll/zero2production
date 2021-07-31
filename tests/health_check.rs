use once_cell::sync::Lazy;
use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::{
    get_configuration,
    DatabaseSettings
};
use sqlx::{ 
    Connection,
    Executor,
    PgConnection,
    PgPool
};
use uuid::Uuid;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    pub address:String,
    pub db_pool: PgPool,
}


// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
         init_subscriber(subscriber);
    };
    
});


async fn spawn_app() -> TestApp {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);
    let mut configuration = get_configuration().expect("Failed to read the configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;
    let server = run(listener, connection_pool.clone())
        .expect("Failed to bind addreess");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }

}

// configuration_database is return a random configuration for a database
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create database 
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failet to create database");
    // migrate the database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Faile to connect to postgres when migrating");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool 
}

#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}


#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute");
    // Assert
    assert_eq!(200, response.status().as_u16());
    
    // save the data to the database
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscrition");
    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    // Act
    // loop thro the test cases
    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to request");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

