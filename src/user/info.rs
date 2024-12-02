use super::*;

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
