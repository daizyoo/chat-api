mod get;
mod send;

use get::get_message;
use send::send_message;

use std::sync::Mutex;

use actix_web::{
    web::{post, Data, Json, ServiceConfig},
    HttpResponse,
};
use tracing::info;

use crate::{
    types::{GetMessages, Message, MessageInfo, Response},
    DataList, MessageList, RoomList, UserList,
};

pub fn message_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/send", post().to(send_message))
        .route("/get", post().to(get_message));
}
