use serde::Serialize;
use serde_json::Value;

use crate::types::UserInfo;

use super::*;

struct QueryRoom {
    id: i64,
    members: Value,
}

struct QueryMessage {
    user_id: String,
    room_id: i32,
    text: String,
}

#[derive(Debug, Serialize)]
struct Message {
    text: String,
    user: UserInfo,
}

pub async fn get_message(db: Data<Database>, Json(get): Json<GetMessages>) -> Result<HttpResponse> {
    let _ = sqlx::query_as!(QueryRoom, "SELECT * FROM room WHERE id = ?", get.room_id())
        .fetch_one(&db.pool)
        .await?;
    let message = sqlx::query_as!(
        QueryMessage,
        "SELECT * FROM message WHERE room_id = ?",
        get.room_id()
    )
    .fetch_all(&db.pool)
    .await?;

    let message: Vec<Message> = message
        .iter()
        .map(|m| Message {
            text: m.text.clone(),
            user: UserInfo::new(get.user_name().clone(), m.user_id.clone()),
        })
        .collect();
    Ok(Response::ok(message))
}
