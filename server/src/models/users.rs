use diesel::prelude::*;
use async_graphql::{Object, types::ID};
use crate::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub active: Option<bool>,
    pub superuser: Option<bool>,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub phone: String,
    pub active: bool,
    pub superuser: bool,
}

#[Object]
impl User {
    pub async fn id(&self) -> ID {
        ID::from(self.id)
    }
    pub async fn first_name(&self) -> &str {
        &self.first_name
    }
    pub async fn last_name(&self) -> &str {
        &self.last_name
    }
    pub async fn email(&self) -> &str {
        &self.email
    }
    pub async fn password(&self) -> &str {
        &self.password
    }
    pub async fn phone(&self) -> &str {
        &self.phone
    }
    pub async fn active(&self) -> Option<bool> {
        self.active
    }
    pub async fn superuser(&self) -> Option<bool> {
        self.superuser
    }
}