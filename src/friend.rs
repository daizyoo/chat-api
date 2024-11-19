use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    Responder,
};
use serde::Deserialize;
use tracing::error;

use crate::{
    types::{Response, UserInfo},
    user::LoginInfo,
    DataList, FriendList, UserList,
};

#[derive(Debug, Deserialize)]
pub struct AddFriend {
    user: LoginInfo,
    friend: UserInfo,
}

/// add friend
pub async fn add(
    friends: Data<Mutex<FriendList>>,
    users: Data<Mutex<UserList>>,
    Json(AddFriend { user, friend }): Json<AddFriend>,
) -> impl Responder {
    let users = users.lock().unwrap();

    let id = user.id();

    // are there any user?
    let mut iter = users.0.iter();
    if iter.find(|u| u.id() == id).is_none() {
        return Response::error("not found user");
    }
    if iter.find(|u| u.password() == user.password()).is_none() {
        return Response::error("not match password");
    }

    let mut friends = friends.lock().unwrap();

    // add to FriendList
    if let Some(friends) = friends.0.get_mut(id) {
        friends.push(friend);
        return Response::ok("ok add friend");
    } else {
        if let Some(user) = friends.0.insert(id.to_string(), vec![friend]) {
            let message = format!("FriendList insert error found: {:#?}", user);
            error!(message);
            return Response::error(message);
        }
        Response::error("not found user and friend list")
    }
}

pub async fn get(
    friends: Data<Mutex<FriendList>>,
    users: Data<Mutex<UserList>>,
    Json(user): Json<UserInfo>,
) -> impl Responder {
    {
        let users = users.lock().unwrap();
        if !users.exist(user.id()) {
            return Response::error("not found user");
        }
    }
    let friends = friends.lock().unwrap();
    if let Some(vec) = friends.0.get(user.id()) {
        Response::ok(vec)
    } else {
        Response::error("not found friend list")
    }
}

/// delete friend
pub async fn delete() -> impl Responder {
    Response::ok("non implement")
}
