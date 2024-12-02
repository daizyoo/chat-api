use serde_json::Value;

use crate::QueryUser;

use super::*;

struct QueryRoom {
    id: i32,
    members: Value,
}

pub async fn get_rooms(
    db: Data<Database>,
    Json(get_user): Json<LoginInfo>,
) -> Result<HttpResponse> {
    let user = sqlx::query_as!(QueryUser, "SELECT * FROM users WHERE id = ?", get_user.id())
        .fetch_one(&db.pool)
        .await?;

    let rooms = sqlx::query_as!(
        QueryRoom,
        "SELECT * FROM room WHERE members LIKE ?",
        user.id
    )
    .fetch_all(&db.pool)
    .await?;

    let rooms: Vec<Room> = rooms
        .iter()
        .map(|r| {
            let members: UserList = r.members.clone().into();
            Room::new(r.id, members.list)
        })
        .collect();

    Ok(Response::ok(rooms))
}
