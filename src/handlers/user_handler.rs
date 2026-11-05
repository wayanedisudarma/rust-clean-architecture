use axum::Json;
use crate::models::UserResponse;

pub async fn get_users() -> Json<Vec<UserResponse>> {
    Json(vec![
        UserResponse {
            id: 1,
            name: "Wayan".to_string(),
        },
    ])
}