mod friend;
mod message;
mod room;
mod types;
mod user;

use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    web::{post, scope, Data},
    App, HttpServer,
};

use serde::{Deserialize, Serialize};
use tracing_subscriber::EnvFilter;

use message::Message;
use types::{User, UserInfo};
use user::LoginInfo;

type RoomId = u32;

type UserId = String;

#[derive(Debug, Serialize, Deserialize)]
struct Room {
    id: RoomId,
    members: Vec<UserInfo>,
}

#[derive(Deserialize)]
struct CreateRoom {
    user: LoginInfo,
    members: Vec<UserId>,
}

impl CreateRoom {
    fn members(&self) -> &Vec<UserId> {
        &self.members
    }
}

impl Room {
    fn new(id: RoomId, members: Vec<UserInfo>) -> Room {
        Room { id, members }
    }
    fn id(&self) -> &RoomId {
        &self.id
    }
    fn members(&self) -> &Vec<UserInfo> {
        &self.members
    }
}

struct RoomList(Vec<Room>);

struct MessageList(HashMap<RoomId, Vec<Message>>);

struct FriendList(HashMap<UserId, Vec<UserInfo>>);

struct UserList(Vec<User>);

trait DataList {
    type Data;
    type ID;
    // idの要素が存在するか
    fn exist(&self, id: &Self::ID) -> bool;
    // idの要素を探す
    fn find(&self, id: &Self::ID) -> Option<&Self::Data>;
    // iterにする
    fn iter(&self) -> std::slice::Iter<'_, Self::Data>;
}

impl DataList for UserList {
    type Data = User;
    type ID = UserId;

    fn exist(&self, id: &Self::ID) -> bool {
        self.0.iter().any(|u| u.id() == id)
    }
    fn find(&self, id: &Self::ID) -> Option<&Self::Data> {
        self.iter().find(|user| user.id() == id)
    }
    fn iter(&self) -> std::slice::Iter<'_, Self::Data> {
        self.0.iter()
    }
}

impl DataList for RoomList {
    type Data = Room;
    type ID = RoomId;

    fn exist(&self, id: &Self::ID) -> bool {
        self.0.iter().any(|room| room.id() == id)
    }
    fn find(&self, id: &Self::ID) -> Option<&Self::Data> {
        self.iter().find(|room| room.id() == id)
    }
    fn iter(&self) -> std::slice::Iter<'_, Self::Data> {
        self.0.iter()
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=trace");
    dotenvy::dotenv()?;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let user = vec![
        User::new(
            "daiki".to_string(),
            "@daiki".to_string(),
            "daiki".to_string(),
        ),
        User::new(
            "kouta".to_string(),
            "@kouta".to_string(),
            "kouta".to_string(),
        ),
    ];

    let rooms = Data::new(Mutex::new(RoomList(vec![Room::new(
        1,
        vec![UserInfo::from(&user[0]), UserInfo::from(&user[1])],
    )])));
    let messages = Data::new(Mutex::new(MessageList(HashMap::from([(1, Vec::new())]))));
    let friends = Data::new(Mutex::new(FriendList(HashMap::from([
        (String::from(user[0].id()), vec![UserInfo::from(&user[1])]),
        (String::from(user[1].id()), vec![UserInfo::from(&user[0])]),
    ]))));
    let users = Data::new(Mutex::new(UserList(user)));

    HttpServer::new(move || {
        App::new()
            .service(
                scope("/message")
                    .route("/send", post().to(message::send_message))
                    .route("/get", post().to(message::get_message)),
            )
            .service(
                scope("/room")
                    .route("/create", post().to(room::create))
                    .route("/get", post().to(room::get_rooms)),
            )
            .service(
                scope("/user")
                    .route("/create", post().to(user::create))
                    .route("/login", post().to(user::login))
                    .route("/search", post().to(user::search))
                    .service(user::info),
            )
            .service(
                scope("/friend")
                    .route("/add", post().to(friend::add))
                    .route("/search", post().to(friend::get))
                    .route("/delete", post().to(friend::delete)),
            )
            .app_data(rooms.clone())
            .app_data(messages.clone())
            .app_data(friends.clone())
            .app_data(users.clone())
    })
    .bind(("127.0.0.1", 3478))?
    .run()
    .await?;

    Ok(())
}
