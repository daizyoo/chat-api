use super::*;

pub async fn get(db: Data<Database>, Json(user): Json<UserInfo>) -> Result<HttpResponse> {
    let user = sqlx::query_as!(QueryUser, "SELECT * FROM users WHERE id = ?", user.id())
        .fetch_one(&db.pool)
        .await?;
    let friends: UserList = user.friends.into();

    Ok(Response::ok(friends.list))
}
