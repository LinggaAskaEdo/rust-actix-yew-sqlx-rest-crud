use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FeedbackModelResponse {
    pub id: Uuid,
    pub rating: i32,
    pub text: String,
    pub createdAt: Option<chrono::DateTime<chrono::Utc>>,
    pub updatedAt: Option<chrono::DateTime<chrono::Utc>>,
}
