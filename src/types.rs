use actix_web::{cookie::CookieBuilder, HttpResponse};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Serialize)]
pub struct Response<T: Serialize + std::fmt::Debug = bool> {
    data: Option<T>,
    status: bool,
}

impl<T: Serialize + std::fmt::Debug> Response<T> {
    pub const fn new(data: Option<T>, status: bool) -> Response<T> {
        Response { data, status }
    }

    pub fn ok(data: T) -> HttpResponse {
        info!("{:#?}", data);
        HttpResponse::Ok().json(Response::new(Some(data), true))
    }

    pub fn error(data: T) -> HttpResponse {
        error!("{:#?}", data);
        HttpResponse::Ok().json(Response::new(Some(data), false))
    }
    pub fn _set_cookie_ok(data: T, cookie: CookieBuilder) -> HttpResponse {
        let res = HttpResponse::Ok()
            .cookie(cookie.finish())
            .json(Response::new(Some(data), true));
        info!("{:#?}", res);
        res
    }
}

pub type RoomId = u32;

pub type UserId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
    id: UserId,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    name: String,
    id: UserId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
    id: UserId,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct AccountInfo {
    friends: Vec<UserInfo>,
    name: String,
    id: UserId,
}

#[derive(Debug, Deserialize)]
pub struct AddFriend {
    user: LoginInfo,
    friend: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct Message {
    text: String,
    user: LoginInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageInfo {
    room: RoomId,
    text: String,
    user: LoginInfo,
}

#[derive(Debug, Deserialize)]
pub struct GetMessages {
    room_id: RoomId,
    user: LoginInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    id: RoomId,
    members: Vec<UserInfo>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoom {
    user: LoginInfo,
    members: Vec<UserId>,
}

impl User {
    pub const fn new(name: String, id: String, password: String) -> User {
        User { name, id, password }
    }
    pub const fn name(&self) -> &String {
        &self.name
    }
    pub const fn id(&self) -> &UserId {
        &self.id
    }
    pub const fn password(&self) -> &String {
        &self.password
    }
}

impl UserInfo {
    pub const fn new(name: String, id: String) -> UserInfo {
        UserInfo { name, id }
    }
    pub const fn id(&self) -> &UserId {
        &self.id
    }
    pub const fn name(&self) -> &String {
        &self.name
    }
}

impl LoginInfo {
    pub const fn new(id: String, password: String) -> LoginInfo {
        LoginInfo { id, password }
    }
    pub const fn id(&self) -> &String {
        &self.id
    }
    pub const fn password(&self) -> &String {
        &self.password
    }
}

impl AccountInfo {
    pub fn new(friends: &Vec<UserInfo>, user: &UserInfo) -> AccountInfo {
        AccountInfo {
            friends: friends.clone(),
            name: user.name().clone(),
            id: user.id().clone(),
        }
    }
}

impl AddFriend {
    pub fn user(&self) -> &LoginInfo {
        &self.user
    }
    pub fn friend(&self) -> &UserInfo {
        &self.friend
    }
}

impl GetMessages {
    pub const fn new(room_id: RoomId, user: LoginInfo) -> GetMessages {
        GetMessages { room_id, user }
    }
    pub const fn room_id(&self) -> RoomId {
        self.room_id
    }
    pub const fn user_id(&self) -> &UserId {
        self.user.id()
    }
    pub const fn user_password(&self) -> &String {
        self.user.password()
    }
}

impl MessageInfo {
    pub const fn new(room: RoomId, text: String, user: LoginInfo) -> MessageInfo {
        MessageInfo { room, text, user }
    }
    pub const fn id(&self) -> RoomId {
        self.room
    }
    pub const fn text(&self) -> &String {
        &self.text
    }
}

impl From<MessageInfo> for Message {
    fn from(value: MessageInfo) -> Self {
        Message {
            text: value.text().clone(),
            user: LoginInfo {
                id: value.user.id().to_string(),
                password: value.user.password().to_string(),
            },
        }
    }
}

impl CreateRoom {
    pub const fn members(&self) -> &Vec<UserId> {
        &self.members
    }
    pub const fn user_id(&self) -> &UserId {
        &self.user.id()
    }
    pub const fn user_password(&self) -> &String {
        &self.user.password()
    }
}

impl Room {
    pub const fn new(id: RoomId, members: Vec<UserInfo>) -> Room {
        Room { id, members }
    }
    pub const fn id(&self) -> &RoomId {
        &self.id
    }
    pub const fn members(&self) -> &Vec<UserInfo> {
        &self.members
    }
}

impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        UserInfo::new(user.name.to_string(), user.id.to_string())
    }
}

impl From<(&RoomId, Vec<UserInfo>)> for Room {
    fn from(value: (&RoomId, Vec<UserInfo>)) -> Self {
        Room {
            id: *value.0,
            members: value.1,
        }
    }
}
