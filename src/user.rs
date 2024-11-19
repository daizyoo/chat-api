use std::sync::Mutex;

use actix_web::{
    get,
    web::{Data, Json, Path},
    Responder,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    types::{Response, User, UserInfo},
    DataList, FriendList, UserId, UserList,
};

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

impl AccountInfo {
    pub fn new(friends: &Vec<UserInfo>, user: &UserInfo) -> AccountInfo {
        AccountInfo {
            friends: friends.clone(),
            name: user.name().clone(),
            id: user.id().clone(),
        }
    }
}

impl LoginInfo {
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn password(&self) -> &String {
        &self.password
    }
}

pub async fn login(
    user_list: Data<Mutex<UserList>>,
    Json(login): Json<LoginInfo>,
) -> impl Responder {
    let user_list = user_list.lock().unwrap();
    if let Some(user) = user_list.find(login.id()) {
        if *user.password() == login.password {
            info!("login: {:?}", user);
            Response::ok(LoginInfo {
                id: user.id().to_string(),
                password: user.password().to_string(),
            })
        } else {
            Response::error("not match password")
        }
    } else {
        Response::error("not found user")
    }
}

pub async fn create(
    user_list: Data<Mutex<UserList>>,
    friend_list: Data<Mutex<FriendList>>,
    Json(user): Json<User>,
) -> impl Responder {
    let mut users = user_list.lock().unwrap();

    if !users.exist(user.id()) {
        let res = User::new(
            user.name().to_string(),
            user.id().to_string(),
            user.password().to_string(),
        );
        users.0.push(user);

        let mut frineds = friend_list.lock().unwrap();
        frineds.0.insert(res.id().clone(), Vec::new());

        info!("{:#?}", users.0);

        Response::ok(res)
    } else {
        Response::error("this user already exists")
    }
}

pub async fn search(
    user_list: Data<Mutex<UserList>>,
    Json(user): Json<UserInfo>,
) -> impl Responder {
    let users = user_list.lock().unwrap();
    let users: Vec<UserInfo> = users
        .iter()
        .filter(|&u| u.id().contains(user.id()) || u.name().contains(user.name()))
        .map(|user| UserInfo::from(user))
        .collect();

    Response::ok(users)
}

#[get("/{id}")]
pub async fn info(
    path: Path<UserId>,
    user_list: Data<Mutex<UserList>>,
    friend_list: Data<Mutex<FriendList>>,
) -> impl Responder {
    let id = path.into_inner();
    let users = user_list.lock().unwrap();
    if let Some(user) = users.find(&id) {
        let user_info = &UserInfo::from(user);
        info!("user info: {:?}", user);
        let friends = friend_list.lock().unwrap();
        if let Some(friends) = friends.0.get(&id) {
            Response::ok(AccountInfo::new(friends, user_info))
        } else {
            Response::ok("not found friend_list")
        }
    } else {
        info!("not found user: {}", id);
        Response::error("not found user")
    }
}
