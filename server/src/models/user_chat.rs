use diesel::prelude::*;
use async_graphql::{Object, types::ID};
use crate::schema::user_chat;

#[derive(Queryable)]
#[diesel(belongs_to(User))]
#[disel(table_name = user_chat)]
pub struct UserChat {
    pub id: i32,
    pub user_id: i32,
    pub chat_id: String,
    pub date_joined: Option<chrono::NaiveDateTime>,
    pub is_admin: Option<bool>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = user_chat)]
pub struct NewUserChat {
    pub user_id: i32,
    pub chat_id: String,
    pub date_joined: Option<chrono::NaiveDateTime>,
    pub is_admin: Option<bool>,
}

#[Object]
impl UserChat {
    pub async fn id(&self) -> ID {
        ID::from(self.id)
    }
    pub async fn user_id(&self) -> i32 {
        self.user_id
    }
    pub async fn chat_id(&self) -> &str {
        &self.chat_id
    }
    pub async fn date_joined(&self) -> Option<chrono::NaiveDateTime> {
        self.date_joined
    }
    pub async fn is_admin(&self) -> Option<bool> {
        self.is_admin
    }
}