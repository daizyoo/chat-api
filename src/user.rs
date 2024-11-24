use std::sync::Mutex;

use actix_web::{
    get,
    web::{Data, Json, Path, Query},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    types::{AccountInfo, LoginInfo, Response, User, UserInfo},
    DataList, FriendList, UserId, UserList,
};

pub async fn login(user_list: Data<Mutex<UserList>>, Json(login): Json<LoginInfo>) -> HttpResponse {
    let user_list = user_list.lock().unwrap();
    if let Some(user) = user_list.find(login.id()) {
        if user.password() == login.password() {
            info!("login: {:?}", user);
            Response::ok(LoginInfo::new(
                user.id().to_string(),
                user.password().to_string(),
            ))
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
) -> HttpResponse {
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

#[derive(Debug, Serialize)]
pub struct SearchUserInfo {
    name: String,
    id: UserId,
    status: bool,
}

#[derive(Deserialize)]
pub struct SearchUserId {
    id: UserId,
}

pub async fn search(
    search: Query<UserInfo>,
    user_list: Data<Mutex<UserList>>,
    friend_list: Data<Mutex<FriendList>>,
    Json(user_id): Json<SearchUserId>,
) -> HttpResponse {
    let user_id = &user_id.id;
    let users = user_list.lock().unwrap();
    let users: Vec<UserInfo> = users
        .iter()
        .filter(|&u| u.id().contains(search.id()) || u.name().contains(search.name()))
        .filter(|&u| u.id() != user_id)
        .map(|user| UserInfo::from(user))
        .collect();
    let friends = friend_list.lock().unwrap();
    let friends = friends.0.get(user_id).unwrap();
    let search_users = users.iter().map(|u| SearchUserInfo {
        name: u.name().to_string(),
        id: u.id().to_string(),
        status: friends.iter().any(|f| f.id() == u.id()),
    });

    let search_users = search_users.collect::<Vec<SearchUserInfo>>();
    Response::ok(search_users)
}

#[get("/{id}")]
pub async fn info(
    path: Path<UserId>,
    user_list: Data<Mutex<UserList>>,
    friend_list: Data<Mutex<FriendList>>,
) -> HttpResponse {
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
