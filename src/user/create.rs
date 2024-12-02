use serde_json::json;

use super::*;
use crate::Database;

pub async fn create(db: Data<Database>, Json(user): Json<User>) -> HttpResponse {
    let pool = &db.pool;
    let result = sqlx::query!(
        "INSERT INTO users (name, id, password ,friends) VALUES (?, ?, ?, ?)",
        user.name(),
        user.id(),
        user.password(),
        json!({"list": []}),
    )
    .execute(pool)
    .await;

    match result {
        Ok(o) => {
            info!("{:#?}", o);
            Response::ok("successcreate user")
        }
        Err(e) => Response::error(e.to_string()),
    }
}
