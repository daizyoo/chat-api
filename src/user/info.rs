use super::*;

use actix_web::get;

/// ユーザーの情報
///
/// `AccountInfo`
#[get("/{id}")]
pub async fn info(path: Path<UserId>, db: Data<Database>) -> HttpResponse {
    let id = path.into_inner();

    let result = sqlx::query_as!(DBUser, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&db.pool)
        .await;

    match result {
        Ok(user) => {
            let mut friends = Vec::new();
            for id in user.friends.list {
                friends.push(get_user_info(&id, &db).await.unwrap());
            }

            let account = AccountInfo::new(UserInfo::new(user.name, user.id), friends);
            Response::ok(account)
        }
        Err(e) => Response::error(e.to_string()),
    }
}
