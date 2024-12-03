use super::*;

use actix_web::get;

/// ユーザーの情報
///
/// `AccountInfo`
#[get("/{id}")]
pub async fn info(path: Path<UserId>, db: Data<Database>) -> Result<HttpResponse> {
    let id = path.into_inner();

    let user = sqlx::query_as!(QueryUser, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&db.pool)
        .await?;

    let mut friends = Vec::new();
    let user_friends: UserList = user.friends.into();
    for id in user_friends.list {
        friends.push(get_user_info(&id, &db).await?);
    }

    let account = AccountInfo::new(UserInfo::new(user.name, user.id), friends);
    Ok(Response::ok(account))
}
