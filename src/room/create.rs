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
        .all(|id| friends.list.contains(id))
    {
        return Err(Error::NotFriends.into());
    }

    let members = json!({"list": new_room.members()});
    let execute = sqlx::query!("INSERT INTO room (members) VALUES (?)", members)
        .execute(&db.pool)
        .await?;
    info!("{:#?}", execute);

    Ok(Response::ok("create room"))
}
