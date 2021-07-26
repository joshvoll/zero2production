use actix_web::HttpResponse;

// health_check is going to check the 200 response code
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
