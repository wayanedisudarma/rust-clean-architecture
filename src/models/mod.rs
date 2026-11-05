use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
}