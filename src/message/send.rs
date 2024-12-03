use super::*;

pub async fn send_message(
    db: Data<Database>,
    Json(message): Json<MessageInfo>,
) -> Result<HttpResponse> {
    // 部屋が存在するか
    let _ = sqlx::query!("SELECT * FROM room WHERE id = ?", message.room_id())
        .fetch_one(&db.pool)
        .await?;
    // メッセージを保存する
    let _ = sqlx::query!(
        "INSERT INTO message (user_id, room_id, text) VALUES (?, ?, ?)",
        message.user_id(),
        message.room_id(),
        message.text()
    )
    .execute(&db.pool)
    .await?;
    Ok(Response::ok("send message"))
}
