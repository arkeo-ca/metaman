use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct MarkingData {
    name: String,
    definition_type: String,
    definition: String,
}

pub async fn create_marking(form: web::Json<MarkingData>, pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
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
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
