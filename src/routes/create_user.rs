use crate::domain::{NewUser, UserEmail, UserName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct UserFormData {
    email: String,
    first_name: String,
    last_name: String,
}

#[tracing::instrument(name = "Adding user to the users table", skip(user, pool))]
pub async fn insert_user(user: &NewUser, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, first_name, last_name, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        &user.email.as_ref(),
        &user.name.first.as_ref(),
        &user.name.last.as_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query. {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Adding a new user",
    skip(_form, _pool),
    fields(
        email = %_form.email,
        first_name = %_form.first_name,
        last_name = %_form.last_name
    )
)]
pub async fn create_user(_form: web::Form<UserFormData>, _pool: web::Data<PgPool>) -> HttpResponse {
    let name = match UserName::parse(_form.0.first_name, _form.0.last_name) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let email = match UserEmail::parse(_form.0.email) {
        Ok(email) => email,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let _user = NewUser { email, name };
    match insert_user(&_user, &_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
