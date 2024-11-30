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
    // ログインするユーザーが存在するか
    if let Some(user) = user_list.find(login.id()) {
        // パスワードの確認
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

    // ユーザーがすでに存在しているか
    if !users.exist(user.id()) {
        // ユーザーリストに保存
        users.0.push(user.clone());

        // フレンドリストを作成
        let mut frineds = friend_list.lock().unwrap();
        frineds.0.insert(user.id().clone(), Vec::new());

        info!("{:#?}", users.0);

        Response::ok(user)
    } else {
        Response::error("this user already exists")
    }
}

#[derive(Debug, Serialize)]
pub struct SearchUserInfo {
    name: String,
    id: UserId,
    // 検索したユーザーのフレンドかどうか
    status: bool,
}

#[derive(Deserialize)]
pub struct SearchUserId {
    id: UserId,
}

impl SearchUserInfo {
    pub fn new(user: &UserInfo, status: bool) -> SearchUserInfo {
        SearchUserInfo {
            name: user.name().clone(),
            id: user.id().clone(),
            status,
        }
    }
}

pub async fn search(
    search: Query<UserInfo>,
    user_list: Data<Mutex<UserList>>,
    friend_list: Data<Mutex<FriendList>>,
    Json(user_id): Json<SearchUserId>,
) -> HttpResponse {
    let user_id = &user_id.id; // ログインしているユーザーのid,このユーザーは探さない
    let users = user_list.lock().unwrap();
    // nameまたはidに検索した文字列を含むユーザーを探す
    let users: Vec<UserInfo> = users
        .serach(search.0)
        .filter(|&u| u.id() != user_id)
        .map(|user| UserInfo::from(user))
        .collect();

    let friends = friend_list.lock().unwrap();
    info!("{:?} {:?} {:?}", users, user_id, friends);
    let friends = friends.0.get(user_id).unwrap();
    let search_users = users
        .iter()
        // 検索したユーザーのフレンドかどうか
        .map(|u| SearchUserInfo::new(u, friends.iter().any(|f| f.id() == u.id())))
        .collect::<Vec<SearchUserInfo>>();

    Response::ok(search_users)
}

/// ユーザーの情報
///
/// `AccountInfo`
#[get("/{id}")]
pub async fn info(
    path: Path<UserId>,
    user_list: Data<Mutex<UserList>>,
    friend_list: Data<Mutex<FriendList>>,
) -> HttpResponse {
    let id = path.into_inner();

    // ユーザーを探す
    if let Some(user) = user_list.lock().unwrap().find(&id) {
        let friends = friend_list.lock().unwrap();

        if let Some(friends) = friends.0.get(&id) {
            let account = AccountInfo::new(friends, &UserInfo::from(user));
            Response::ok(account)
        } else {
            Response::error("not found friend_list")
        }
    } else {
        Response::error("not found user")
    }
}
