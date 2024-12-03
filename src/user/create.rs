use serde_json::json;

use super::*;

pub async fn create(db: Data<Database>, Json(user): Json<User>) -> Result<HttpResponse> {
    let result = sqlx::query!(
        "INSERT INTO users (name, id, password ,friends) VALUES (?, ?, ?, ?)",
        user.name(),
        user.id(),
        user.password(),
        json!({ "list": serde_json::to_string(&Vec::<String>::new()).unwrap() })
    )
    .execute(&db.pool)
    .await?;
    if result.last_insert_id() == 0 {
        return Err(Error::UserAlreadyExists.into());
    }
    Ok(Response::ok(LoginInfo::from(&user)))
}
