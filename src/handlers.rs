use axum::{
    response::{Html, Json, IntoResponse},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::model_manager;

// --- DTOs (Data Transfer Objects) ---
#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub reply: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

use askama::Template;

// --- Templates ---
#[derive(Template)]
#[template(path = "info.html")]
pub struct InfoTemplate {
    title: String,
}

#[derive(Template)]
#[template(path = "chat.html")]
pub struct ChatTemplate {
    title: String,
}

// --- Handlers ---

/// Renders the Info Page (Using Askama)
pub async fn info_page_handler() -> impl IntoResponse {
    let template = InfoTemplate {
        title: "Info".to_string(),
    };
    Html(template.render().unwrap())
}

/// Renders the Chat Page (Using Askama)
pub async fn chat_page_handler() -> impl IntoResponse {
    let template = ChatTemplate {
        title: "Chat".to_string(),
    };
    Html(template.render().unwrap())
}

/// Handles the API request for chat.
/// This is where the AI integration happens.
pub async fn chat_api_handler(
    Json(payload): Json<ChatRequest>,
) -> impl IntoResponse {
    // 1. Instantiate the correct model based on .env
    // We now handle configuration errors gracefully instead of panicking
    let model = match model_manager::try_create_model() {
        Ok(m) => m,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR, 
                Json(ErrorResponse { error: e })
            ).into_response();
        }
    };

    // 2. Send prompt to model
    match model.chat(&payload.message).await {
        Ok(reply) => (StatusCode::OK, Json(ChatResponse { reply })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: e })).into_response(),
    }
}
