use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    model::FeedbackModel,
    request::{CreateFeedbackSchema, FilterOptions},
    response::FeedbackModelResponse,
    AppState,
};

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build API with Rust, SQLX, Postgres,and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/feedbacks")]
pub async fn feedback_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        FeedbackModel,
        "SELECT * FROM feedbacks ORDER by id LIMIT ? OFFSET ?",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let message = "Something bad happened while fetching all feedback items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let feedbacks = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": feedbacks.len(),
        "feedbacks": feedbacks
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/feedbacks/")]
async fn create_feedback_handler(
    body: web::Json<CreateFeedbackSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let id = uuid::Uuid::new_v4().to_string();
    let query_result =
        sqlx::query(r#"INSERT INTO feedbacks (id, rating, comment) VALUES (?, ?, ?, ?)"#)
            .bind(id.clone())
            .bind(body.rating)
            .bind(body.comment.to_string())
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            return HttpResponse::BadRequest().json(
            serde_json::json!({"status": "fail","message": "Feedback with that comment already exists"}),
        );
        }

        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","message": format!("{:?}", err)}));
    }

    let query_result =
        sqlx::query_as!(FeedbackModel, r#"SELECT * FROM feedbacks WHERE id = ?"#, id)
            .fetch_one(&data.db)
            .await;

    match query_result {
        Ok(feedback) => {
            let feedback_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "feedback": filter_db_record(&feedback)
            })});

            return HttpResponse::Ok().json(feedback_response);
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return HttpResponse::BadRequest()
                .json(serde_json::json!({"status": "fail","message": "Feedback with that title already exists"}));
            }

            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

fn filter_db_record(feedback: &FeedbackModel) -> FeedbackModelResponse {
    FeedbackModelResponse {
        id: feedback.id.to_owned(),
        rating: feedback.rating.to_owned(),
        comment: feedback.comment.to_owned(),
        createdAt: feedback.created_at,
        updatedAt: feedback.updated_at,
    }
}
