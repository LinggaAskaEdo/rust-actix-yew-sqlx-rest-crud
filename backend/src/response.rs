use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FeedbackModelResponse {
    pub id: String,
    pub rating: i32,
    pub comment: String,
    pub createdAt: Option<chrono::DateTime<chrono::Utc>>,
    pub updatedAt: Option<chrono::DateTime<chrono::Utc>>,
}
