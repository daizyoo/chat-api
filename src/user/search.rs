use super::*;

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
