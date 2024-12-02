use super::*;

pub async fn login(db: Data<Database>, Json(login): Json<LoginInfo>) -> HttpResponse {
    let result = sqlx::query_as!(DBUser, "SELECT * FROM users WHERE id = ?", login.id())
        .fetch_one(&db.pool)
        .await;
    match result {
        Ok(user) => {
            if user.password == *login.password() {
                Response::ok(LoginInfo::new(user.id, user.password))
            } else {
                Response::error("not match password")
            }
        }
        Err(e) => Response::error(e.to_string()),
    }
}
