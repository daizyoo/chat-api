use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use type_utilities_rs::omit;

use crate::UserId;

#[derive(Serialize)]
pub struct Response<T: Serialize = bool> {
    data: Option<T>,
    status: bool,
}

impl<T: Serialize> Response<T> {
    pub fn new(data: Option<T>, status: bool) -> Response<T> {
        Response { data, status }
    }

    pub fn ok(data: T) -> HttpResponse {
        HttpResponse::Ok().json(Response::new(Some(data), true))
    }
    pub fn error(data: T) -> HttpResponse {
        HttpResponse::Ok().json(Response::new(Some(data), false))
    }
}

#[omit(UserInfo, [password])]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
    id: UserId,
    password: String,
}

impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        UserInfo::new(user.name.to_string(), user.id.to_string())
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
