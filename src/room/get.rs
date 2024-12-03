use serde_json::Value;

use super::*;

#[derive(Debug)]
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

    let rooms = sqlx::query_as!(QueryRoom, "SELECT * FROM room",)
        .fetch_all(&db.pool)
        .await?;
    let rooms: Vec<Room> = rooms
        .iter()
        .filter(|r| {
            let users: UserList = r.members.clone().into();
            users.list.contains(&user.id)
        })
        .map(|r| {
            let users: UserList = r.members.clone().into();
            let members: Vec<String> = users.list.into_iter().filter(|id| id != &user.id).collect();
            Room::new(r.id, members)
        })
        .collect();

    Ok(Response::ok(rooms))
}
