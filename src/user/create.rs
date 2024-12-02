use super::*;

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
