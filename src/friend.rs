use std::sync::Mutex;

use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::{
    types::{AddFriend, Response, UserInfo},
    DataList, FriendList, UserList,
};

/// add friend
pub async fn add(
    friends: Data<Mutex<FriendList>>,
    users: Data<Mutex<UserList>>,
    Json(add_friend): Json<AddFriend>,
) -> HttpResponse {
    let user = add_friend.user();
    let friend = add_friend.friend().clone();

    let users = users.lock().unwrap();

    let id = user.id();

    if friend.id() == user.id() {
        return Response::error("login user == friend");
    }

    // are there any user?
    let mut iter = users.0.iter();
    if let Some(u) = iter.find(|u| u.id() == id) {
        if u.password() != user.password() {
            return Response::error("not match password");
        }
    } else {
        return Response::error("not found user");
    }

    let mut friends = friends.lock().unwrap();

    // ユーザーのフレンドリストを取得
    if let Some(friends) = friends.0.get_mut(id) {
        //
        if friends.iter().find(|f| f.id() == friend.id()).is_none() {
            friends.push(friend);
            Response::ok("ok add friend")
        } else {
            Response::error("the friend already exists")
        }
    } else {
        if let Some(user) = friends.0.insert(id.to_string(), vec![friend]) {
            Response::error(format!("FriendList insert error found: {:#?}", user));
        }
        Response::error("not found user and friend list")
    }
}

pub async fn get(
    friends: Data<Mutex<FriendList>>,
    users: Data<Mutex<UserList>>,
    Json(user): Json<UserInfo>,
) -> HttpResponse {
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
pub async fn delete() -> HttpResponse {
    Response::ok("non implement")
}
