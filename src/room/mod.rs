mod create;
mod get;

use create::create;
use get::get_rooms;

use actix_web::{
    web::{post, Data, Json, ServiceConfig},
    HttpResponse,
};

use crate::types::{CreateRoom, Error, LoginInfo, QueryUser, Response, Result, Room, UserList};
use crate::Database;

pub fn room_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/create", post().to(create))
        .route("/get", post().to(get_rooms));
}
