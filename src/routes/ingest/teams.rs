use crate::domain::{NewUser, UserEmail, UserName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

#[tracing::instrument(name = "writing teams to the teams table")]
pub async fn write_teams() -> Result<(), sqlx::Error> {
    // sqlx::query!(
    //     r#"
    //     INSERT INTO users (id, email, first_name, last_name, created_at)
    //     VALUES ($1, $2, $3, $4, $5)
    //     "#,
    //     Uuid::new_v4(),
    //     &user.email.as_ref(),
    //     &user.name.first.as_ref(),
    //     &user.name.last.as_ref(),
    //     Utc::now()
    // )
    // .execute(pool)
    // .await
    // .map_err(|e| {
    //     tracing::error!("Failed to execute query. {:?}", e);
    //     e
    // })?;
    Ok(())
}

#[tracing::instrument(name = "ingesting teams")]
pub async fn ingest_teams() -> HttpResponse {
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
