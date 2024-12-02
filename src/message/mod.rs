mod get;
mod send;

pub use get::get_message;
pub use send::send_message;

use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use tracing::info;

use crate::{
    types::{GetMessages, Message, MessageInfo, Response},
    DataList, MessageList, RoomList, UserList,
};
