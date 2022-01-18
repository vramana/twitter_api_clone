pub mod users {

    use chrono::{DateTime, Utc};
    use diesel::prelude::*;
    use diesel::{AsChangeset, Insertable, Queryable};
    use serde::{Deserialize, Serialize};
    use tide::{Body, Request, Response};

    use crate::db::*;
    use crate::schema::users;
    use crate::Application;

    #[derive(Clone, Debug, PartialEq)]
    pub struct Password(pub String);

    impl Password {
        pub fn from_plain_text(s: &str) -> anyhow::Result<Password> {
            let hash = bcrypt::hash(s, 10)?;
            Ok(Password(hash))
        }

        fn from_hash(s: String) -> Password {
            Password(s)
        }

        pub fn verify(&self, plain_password: &str) -> anyhow::Result<bool> {
            Ok(bcrypt::verify(plain_password, &self.0)?)
        }
    }

    #[derive(Queryable, Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct User {
        pub id: i64,
        pub name: String,
        pub username: String,
        pub encrypted_password: String,
        pub follower_count: i32,
        pub following_count: i32,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }

    #[derive(Serialize, Deserialize, Debug, AsChangeset, Default, Clone)]
    #[table_name = "users"]
    pub struct UpdateUser<'a> {
        pub name: Option<&'a str>,
        pub username: Option<&'a str>,
        pub encrypted_password: Option<String>,
        pub following_count: Option<i32>,
        pub follower_count: Option<i32>,
    }

    #[derive(Insertable, Deserialize, Debug, Clone)]
    #[table_name = "users"]
    pub struct NewUser<'a> {
        pub username: &'a str,
        pub name: &'a str,
        pub encrypted_password: &'a str,
        pub follower_count: i32,
        pub following_count: i32,
    }

    #[derive(Debug, Deserialize)]
    struct NewUserRequest {
        pub name: String,
        pub username: String,
        pub password: String,
    }

    fn insert_user(db: &PgDB, user: NewUser) -> anyhow::Result<User> {
        let new_user: User = diesel::insert_into(users::table)
            .values(user)
            .get_result(&db.conn())?;

        Ok(new_user)
    }

    pub async fn create_user(mut req: Request<Application>) -> tide::Result {
        let new_user_request: NewUserRequest = req.body_json().await?;
        let encrypted_password: Password = Password::from_plain_text(&new_user_request.password)
            .map_err(|err| tide::Error::new(tide::StatusCode::InternalServerError, err))?;

        let new_user = NewUser {
            name: &new_user_request.name,
            username: &new_user_request.username,
            encrypted_password: &encrypted_password.0,
            follower_count: 0,
            following_count: 0,
        };

        let db = &req.state().db;

        insert_user(db, new_user)
            .map_err(|err| tide::Error::new(tide::StatusCode::InternalServerError, err))?;

        Ok(format!("Created a new user {}", &new_user_request.name).into())
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct UsersResponse {
        pub users: Vec<UserResponse>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct UserResponse {
        pub id: i64,
        pub name: String,
        pub username: String,
        pub follower_count: i32,
        pub following_count: i32,
    }

    impl From<User> for UserResponse {
        fn from(c: User) -> Self {
            Self {
                id: c.id,
                name: c.name,
                username: c.username,
                following_count: c.following_count,
                follower_count: c.follower_count,
            }
        }
    }

    impl<T: Into<UserResponse>> From<Vec<T>> for UsersResponse {
        fn from(v: Vec<T>) -> Self {
            let users: Vec<UserResponse> = v.into_iter().map(|u| u.into()).collect();
            Self { users }
        }
    }

    pub async fn get_users(mut req: Request<Application>) -> tide::Result {
        use crate::schema::users::dsl::*;
        let db = &req.state().db;

        let results: UsersResponse = users
            .load::<User>(&db.conn())
            .map_err(|err| tide::Error::new(tide::StatusCode::InternalServerError, err))?
            .into();

        Ok(Response::builder(200)
            .body(Body::from_json(&results)?)
            .build())
    }
}
