use async_graphql::{Context, Object, Result, Schema, EmptySubscription};
use crate::resolvers::users_resolver;
use crate::models::users::User;

pub type SchemaQL = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        let users = users_resolver::get_users();
        Ok(users)
    }
    async fn user(&self, _ctx: &Context<'_>, id: i32) -> Result<User> {
        let user = users_resolver::get_user_by_id(id);
        Ok(user)
    }
    async fn active_users(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        let users = users_resolver::get_active_users();
        Ok(users)
    }
    async fn inactive_users(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        let users = users_resolver::get_inactive_users();
        Ok(users)
    }
    async fn superusers(&self, _ctx: &Context<'_>) -> Result<Vec<User>> {
        let users = users_resolver::get_superusers();
        Ok(users)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn new_user(&self, _ctx: &Context<'_>, first_name: String, last_name: String, email: String, password: String, phone: String, active: bool, superuser: bool) -> Result<User> {
        let user = users_resolver::create_user(first_name, last_name, email, password, phone, active, superuser);
        Ok(user)
    }
    async fn update_user(&self, _ctx: &Context<'_>, id: i32, first_name: String, last_name: String, email: String, password: String, phone: String, active: bool, superuser: bool) -> Result<User> {
        let user = users_resolver::update_user(id, first_name, last_name, email, password, phone, active, superuser);
        Ok(user)
    }
    async fn delete_user(&self, _ctx: &Context<'_>, id: i32) -> Result<User> {
        let user = users_resolver::delete_user(id);
        Ok(user)
    }
    async fn deactivate_user(&self, _ctx: &Context<'_>, id: i32) -> Result<User> {
        let user = users_resolver::deactivate_user(id);
        Ok(user)
    }
    async fn activate_user(&self, _ctx: &Context<'_>, id: i32) -> Result<User> {
        let user = users_resolver::activate_user(id);
        Ok(user)
    }
}