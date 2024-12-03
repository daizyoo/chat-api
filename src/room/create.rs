use serde_json::json;

use super::*;

pub async fn create(db: Data<Database>, Json(new_room): Json<CreateRoom>) -> Result<HttpResponse> {
    let friends = sqlx::query_as!(
        QueryUser,
        "SELECT * FROM users WHERE id = ?",
        new_room.user().id()
    )
    .fetch_one(&db.pool)
    .await?;
    let friends: UserList = friends.friends.into();
    if !new_room
        .members()
        .iter()
        .filter(|&id| id != new_room.user().id()) // 自分を除外
        .all(|id| friends.list.contains(id))
    {
        tracing::error!("not friends");
        return Err(Error::NotFriends.into());
    }

    let members = json!({"list": serde_json::to_string(&new_room.members()).unwrap()});
    let execute = sqlx::query!("INSERT INTO room (members) VALUES (?)", members)
        .execute(&db.pool)
        .await?;

    Ok(Response::ok("create room"))
}
