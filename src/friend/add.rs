use super::*;

pub async fn add(db: Data<Database>, Json(add_friend): Json<AddFriend>) -> Result<HttpResponse> {
    // ユーザーが存在するか確認
    let friend = sqlx::query_as!(
        QueryUser,
        "SELECT * FROM users WHERE id = ?",
        add_friend.friend().id()
    )
    .fetch_one(&db.pool)
    .await?;
    let user = sqlx::query_as!(
        QueryUser,
        "SELECT * FROM users WHERE id = ? AND password = ?",
        add_friend.user().id(),
        add_friend.user().password()
    )
    .fetch_one(&db.pool)
    .await?;

    let mut friends: UserList = user.friends.into();
    friends.list.push(friend.id);
    let result = sqlx::query!(
        "UPDATE users SET friends = JSON_SET(friends, '$.list', ?) WHERE id = ?",
        serde_json::to_value(friends.list).unwrap(),
        user.id
    )
    .execute(&db.pool)
    .await?;

    Ok(Response::ok(result.last_insert_id().to_string()))
}
