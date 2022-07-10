use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{MarkingDefinition, MarkingDefinitionType, MarkingName, NewMarking};

#[derive(serde::Deserialize)]
pub struct JsonData {
    name: String,
    definition_type: String,
    definition: String,
}

impl TryFrom<JsonData> for NewMarking {
    type Error = String;

    fn try_from(value: JsonData) -> Result<Self, Self::Error> {
        let name = MarkingName::parse(value.name)?;
        let definition = MarkingDefinition::parse(value.definition)?;
        let definition_type = MarkingDefinitionType::parse(value.definition_type)?;

        Ok(Self {
            name,
            definition_type,
            definition,
        })
    }
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
    let new_marking = match form.0.try_into() {
        Ok(marking) => marking,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    match insert_marking(&pool, &new_marking).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new marking in the database", skip(new_marking, pool))]
pub async fn insert_marking(pool: &PgPool, new_marking: &NewMarking) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO markings (id, name, definition_type, definition, created_at, created_by)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        new_marking.name.as_ref(),
        new_marking.definition_type.as_ref(),
        new_marking.definition.as_ref(),
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
