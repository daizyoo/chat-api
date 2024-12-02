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
    get,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    types::{AccountInfo, LoginInfo, Response, User, UserInfo},
    DataList, FriendList, UserId, UserList,
};

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
