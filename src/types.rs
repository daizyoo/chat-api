use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an unspecified internal error occurred: {0}")]
    InternalError(#[from] anyhow::Error),
    #[error("an unhandled database error occurred")]
    DatabaseError(#[from] sqlx::Error),
    #[error("user already exists")]
    UserAlreadyExists,
    #[error("not friends")]
    NotFriends,
    #[error("not match password")]
    NotMatchPassword,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match &self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFriends => StatusCode::BAD_REQUEST,
            Self::UserAlreadyExists => StatusCode::BAD_REQUEST,
            Self::NotMatchPassword => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

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
        info!("response: {:#?}", data);
        HttpResponse::Ok().json(Response::new(Some(data), true))
    }
}

pub type RoomId = i32;

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
    name: String,
    id: UserId,
    friends: Vec<UserInfo>,
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
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Room {
    id: RoomId,
    members: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRoom {
    user: LoginInfo,
    members: Vec<UserId>,
}

impl User {
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
    pub fn new(user: UserInfo, friends: Vec<UserInfo>) -> AccountInfo {
        AccountInfo {
            friends,
            name: user.name.clone(),
            id: user.id.clone(),
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
    pub const fn room_id(&self) -> RoomId {
        self.room_id
    }
    pub const fn user_name(&self) -> &String {
        &self.user.name()
    }
}

impl MessageInfo {
    pub const fn room_id(&self) -> RoomId {
        self.room
    }
    pub const fn user_id(&self) -> &UserId {
        &self.user.id()
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
    pub const fn user(&self) -> &LoginInfo {
        &self.user
    }
}

impl Room {
    pub const fn new(id: RoomId, members: Vec<String>) -> Room {
        Room { id, members }
    }
}

impl From<&User> for LoginInfo {
    fn from(user: &User) -> Self {
        LoginInfo::new(user.id.clone(), user.password.clone())
    }
}
