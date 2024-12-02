mod create;
mod get;

use create::create;
use get::get_rooms;

use actix_web::{
    web::{post, Data, Json, ServiceConfig},
    HttpResponse,
};
use tracing::info;

use crate::types::Result;

use crate::{
    types::{CreateRoom, LoginInfo, Response},
    Room,
};
use crate::{Database, QueryUser, UserList};

pub fn room_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/create", post().to(create))
        .route("/get", post().to(get_rooms));
}
