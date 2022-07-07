use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct JsonData {
    name: String,
    definition_type: String,
    definition: String,
}

#[tracing::instrument(
    name = "Adding a new marking",
    skip(form, pool),
    fields(
        marking_name = %form.name,
        marking_type = %form.definition_type,
        marking_definition = %form.definition
    )
)]
pub async fn create_marking(form: web::Json<JsonData>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_marking(&pool, &form).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new marking in the database", skip(form, pool))]
pub async fn insert_marking(pool: &PgPool, form: &JsonData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO markings (id, name, definition_type, definition, created_at, created_by)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        form.name,
        form.definition_type,
        form.definition,
        Utc::now(),
        Uuid::new_v4()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
