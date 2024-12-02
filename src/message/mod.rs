mod get;
mod send;

use get::get_message;
use send::send_message;

use actix_web::{
    web::{post, Data, Json, ServiceConfig},
    HttpResponse,
};

use crate::types::GetMessages;
use crate::types::{Response, Result};
use crate::Database;

pub fn message_service_config(cfg: &mut ServiceConfig) {
    cfg.route("/send", post().to(send_message))
        .route("/get", post().to(get_message));
}
