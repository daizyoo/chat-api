mod friend;
mod message;
mod room;
mod types;
mod user;

use std::{collections::HashMap, env, sync::Mutex};

use actix_cors::Cors;
use actix_web::{
    http::header,
    web::{post, scope, Data},
    App, HttpServer,
};

use sqlx::MySqlPool;
use tracing_subscriber::EnvFilter;

use types::{Message, Room, RoomId, User, UserId, UserInfo};

#[derive(Debug)]
struct RoomList(Vec<Room>);

#[derive(Debug)]
struct MessageList(HashMap<RoomId, Vec<Message>>);

#[derive(Debug)]
struct FriendList(HashMap<UserId, Vec<UserInfo>>);

#[derive(Debug)]
struct UserList(Vec<User>);

trait DataList {
    type Data;
    type ID;
    /// idの要素が存在するか
    fn exist(&self, id: &Self::ID) -> bool;
    /// idの要素を探す
    fn find(&self, id: &Self::ID) -> Option<&Self::Data>;
    /// iterにする
    fn iter(&self) -> std::slice::Iter<'_, Self::Data>;
}

impl UserList {
    fn serach(&self, user: UserInfo) -> impl Iterator<Item = &User> {
        self.0
            .iter()
            .filter(move |&u| u.id().contains(user.id()) || u.name().contains(user.name()))
    }
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

impl RoomList {
    fn new_id(&self) -> u32 {
        let mut ids = self.iter().map(|r| *r.id()).collect::<Vec<RoomId>>();
        ids.sort();
        ids.last().unwrap_or(&0) + 1
    }
}

struct Database {
    pool: MySqlPool,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");

    let pool = MySqlPool::connect(&env::var("DATABASE_URL")?).await?;
    let pool = Data::new(Mutex::new(Database { pool }));

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600),
            )
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
            .app_data(pool.clone())
    })
    .bind(("127.0.0.1", 3478))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::env;

    use anyhow::Result;

    use serde_json::Value;
    use sqlx::MySqlPool;

    pub async fn connect() -> Result<MySqlPool> {
        dotenvy::dotenv()?;
        Ok(MySqlPool::connect(&env::var("DATABASE_URL")?).await?)
    }

    #[tokio::test]
    async fn mysql_connect() -> anyhow::Result<()> {
        let pool = connect().await?;
        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn select_user() -> Result<()> {
        let pool = connect().await?;
        let user = sqlx::query_as!(User, "SELECT * FROM users")
            .fetch_all(&pool)
            .await?;
        println!("{:#?}", user);
        Ok(())
    }

    #[tokio::test]
    async fn insert_user() -> Result<()> {
        let pool = connect().await?;
        let user = QueryUser {
            id: "id".to_string(),
            name: "test".to_string(),
            password: "password".to_string(),
            friends: Friends { list: vec![] }.into(),
        };
        sqlx::query!(
            "INSERT INTO users (id, name, password, friends) VALUES (?, ?, ?, ?)",
            user.id,
            user.name,
            user.password,
            user.friends
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    #[derive(Debug)]
    struct User {
        id: String,
        name: String,
        password: String,
        friends: Friends,
    }

    struct QueryUser {
        id: String,
        name: String,
        password: String,
        friends: Value,
    }

    #[derive(Debug)]
    struct Friends {
        list: Vec<String>,
    }

    impl From<Friends> for Value {
        fn from(friends: Friends) -> Self {
            serde_json::json!({ "list": friends.list })
        }
    }

    /// mysqlに保存されたJson形式の {"list": [...]}をFriendsに変換する
    impl From<Value> for Friends {
        fn from(value: Value) -> Self {
            Self {
                list: value
                    .as_object()
                    .unwrap()
                    .get("list")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect::<Vec<String>>(),
            }
        }
    }
}
