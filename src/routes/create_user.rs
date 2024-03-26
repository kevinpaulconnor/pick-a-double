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

#[tracing::instrument(name = "Adding user to the users table", skip(form, pool))]
pub async fn insert_user(form: &UserFormData, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, first_name, last_name, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        Uuid::new_v4(),
        &form.email,
        &form.first_name,
        &form.last_name,
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
    match insert_user(&_form, &_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
