mod create;
mod info;
mod login;
mod search;

pub use create::create;
pub use info::info as user_info;
pub use login::login;
pub use search::search;

use std::sync::Mutex;

use actix_web::{
    web::{post, Data, Json, Path, Query, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    types::{AccountInfo, LoginInfo, Response, User, UserInfo},
    DataList, FriendList, UserId, UserList,
};
use crate::{DBUser, Database};

pub fn user_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/create", post().to(create))
        .route("/login", post().to(login))
        .route("/search", post().to(search))
        .service(user_info);
}

#[derive(Debug, Serialize)]
pub struct SearchUserInfo {
    name: String,
    id: UserId,
    // 検索したユーザーのフレンドかどうか
    status: bool,
}

#[derive(Deserialize)]
pub struct SearchUserId {
    id: UserId,
}

impl SearchUserInfo {
    pub fn new(user: &UserInfo, status: bool) -> SearchUserInfo {
        SearchUserInfo {
            name: user.name().clone(),
            id: user.id().clone(),
            status,
        }
    }
}

async fn get_user_info(id: &UserId, db: &Database) -> anyhow::Result<UserInfo> {
    let result = sqlx::query_as!(DBUser, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(&db.pool)
        .await;
    match result {
        Ok(user) => Ok(UserInfo::new(user.name, user.id)),
        Err(e) => anyhow::bail!(e),
    }
}
