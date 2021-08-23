use actix_web::{
    web,
    HttpResponse
};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use crate::domain::{
    NewSubscriber, 
    SubscriberName,
    SubscriberEmail
};
use std::convert::TryInto;

// FormData definition
#[derive(Debug, serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

impl TryInto<NewSubscriber> for FormData {
    type Error = String;
    fn try_into(self) -> Result<NewSubscriber, Self::Error> {
        let name = SubscriberName::parse(self.name)?;
        let email = SubscriberEmail::parse(self.email)?;
        Ok(NewSubscriber{email, name})
    }
    
}


#[tracing::instrument(
    name = "Adding new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        sbuscriber_name = %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>, 
    pool: web::Data<PgPool>
) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(email) => email,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool, 
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1,$2,$3,$4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
