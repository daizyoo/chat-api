mod add;
mod delete;
mod get;

use add::add;
use delete::delete;
use get::get;

use std::sync::Mutex;

use crate::types::Result;
use actix_web::{
    web::{post, Data, Json, ServiceConfig},
    HttpResponse,
};

use crate::{
    types::{AddFriend, Response, UserInfo},
    DataList, FriendList, UserList,
};
use crate::{Database, Friends, QueryUser};

pub fn friend_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/add", post().to(add))
        .route("/delete", post().to(delete))
        .route("/get", post().to(get));
}
