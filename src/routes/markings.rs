use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct MarkingData {
    name: String,
    definition_type: String,
    definition: String,
}

pub async fn create_marking(_form: web::Json<MarkingData>) -> HttpResponse {
    HttpResponse::Created().finish()
}
