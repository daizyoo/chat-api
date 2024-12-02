mod create;
mod get;

pub use create::create;
pub use get::get_rooms;

use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use tracing::info;

use crate::{
    types::{CreateRoom, LoginInfo, Response, UserInfo},
    DataList, FriendList, MessageList, Room, RoomList, UserList,
};
