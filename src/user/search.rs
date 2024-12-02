use super::*;

pub async fn search(
    search: Query<UserInfo>,
    db: Data<Database>,
    Json(user_id): Json<SearchUserId>,
) -> HttpResponse {
    let search_name = format!("%{}%", search.0.id());
    let user = sqlx::query_as!(QueryUser, "SELECT * FROM users WHERE id = ?", user_id.id)
        .fetch_one(&db.pool)
        .await;

    let Ok(user) = user else {
        return Response::error("not found your id");
    };
    let user_friends: Friends = user.friends.into();

    let result = sqlx::query_as!(
        QueryUser,
        "SELECT * FROM users WHERE id != ? AND name like ?",
        user_id.id,
        search_name
    )
    .fetch_all(&db.pool)
    .await;

    match result {
        Ok(users) => {
            let search_users = users
                .iter()
                .map(|u| UserInfo::new(u.name.clone(), u.id.clone()))
                .map(|u| SearchUserInfo::new(&u, user_friends.list.contains(&u.id())))
                .collect::<Vec<SearchUserInfo>>();

            return Response::ok(search_users);
        }
        Err(e) => Response::error(e.to_string()),
    }
}
