use super::*;

pub async fn login(db: Data<Database>, Json(login): Json<LoginInfo>) -> Result<HttpResponse> {
    let user = sqlx::query_as!(DBUser, "SELECT * FROM users WHERE id = ?", login.id())
        .fetch_one(&db.pool)
        .await?;

    if user.password == *login.password() {
        Ok(Response::ok(LoginInfo::new(user.id, user.password)))
    } else {
        Err(Error::NotMatchPassword.into())
    }
}
