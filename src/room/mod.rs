mod create;
mod get;

use create::create;
use get::get_rooms;

use std::sync::Mutex;

use actix_web::{
    web::{post, Data, Json, ServiceConfig},
    HttpResponse,
};
use tracing::info;

use crate::{
    types::{CreateRoom, LoginInfo, Response, UserInfo},
    DataList, FriendList, MessageList, Room, RoomList, UserList,
};

pub fn room_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/create", post().to(create))
        .route("/get", post().to(get_rooms));
}
