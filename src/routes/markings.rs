use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct MarkingData {
    name: String,
    definition_type: String,
    definition: String,
}

pub async fn create_marking(form: web::Json<MarkingData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding a new marking.",
        %request_id,
        marking_name = form.name,
        marking_type = form.definition_type,
        marking_definition = form.definition
    );
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new marking in the database.");

    tracing::info!(
        "request_id {} - Creating new marking in database.",
        request_id
    );
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
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
