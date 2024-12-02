mod add;
mod delete;
mod get;

pub use add::add;
pub use delete::delete;
pub use get::get;

use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    types::{AddFriend, Response, UserInfo},
    DataList, FriendList, UserList,
};
