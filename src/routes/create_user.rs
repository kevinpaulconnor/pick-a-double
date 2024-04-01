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

impl TryFrom<UserFormData> for NewUser {
    type Error = String;

    fn try_from(form: UserFormData) -> Result<Self, Self::Error> {
        let name = UserName::parse(form.first_name, form.last_name)?;

        let email = UserEmail::parse(form.email)?;

        Ok(NewUser { email, name })
    }
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
    let new_user = match _form.0.try_into() {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::BadRequest().finish();
        }
    };
    match insert_user(&new_user, &_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
