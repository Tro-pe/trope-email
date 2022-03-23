use actix_web::{HttpResponse, web};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
	email: String,
	name: String
}

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
	HttpResponse::Ok().finish()
}