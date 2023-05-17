use diesel::prelude::*;
use crate::models::users::{User, NewUser};
use crate::data::postgresql::establish_connection;
use crate::schema::users::first_name;

pub fn get_users() -> Vec<User> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let results = users
        .load::<User>(connection)
        .expect("Error loading users");
    results
}

pub fn get_user_by_id(param_id: i32) -> User {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let results = users
        .find(param_id)
        .get_result::<User>(connection)
        .expect("Error loading users");
    results
}

pub fn get_active_users() -> Vec<User> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let results = users
        .filter(active.eq(true))
        .load::<User>(connection)
        .expect("Error loading users");
    results
}

pub fn get_inactive_users() -> Vec<User> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let results = users
        .filter(active.eq(false))
        .load::<User>(connection)
        .expect("Error loading users");
    results
}

pub fn get_superusers() -> Vec<User> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let results = users
        .filter(superuser.eq(true))
        .load::<User>(connection)
        .expect("Error loading users");
    results
}

pub fn create_user(
    param_first_name: String,
    param_last_name: String,
    param_email: String,
    param_password: String,
    param_phone: String,
    param_active: bool,
    param_superuser: bool,
) -> User {
    use crate::schema::users;
    let connection = &mut establish_connection();
    let new_user = NewUser {
        first_name: param_first_name,
        last_name: param_last_name,
        email: param_email,
        password: param_password,
        phone: param_phone,
        active: param_active,
        superuser: param_superuser,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(connection)
        .expect("Error saving new user")
}

pub fn update_user(
    param_id: i32,
    param_first_name: String,
    param_last_name: String,
    param_email: String,
    param_password: String,
    param_phone: String,
    param_active: bool,
    param_superuser: bool,
) -> User {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let updated_user: NewUser = NewUser {
        first_name: param_first_name,
        last_name: param_last_name,
        email: param_email,
        password: param_password,
        phone: param_phone,
        active: param_active,
        superuser: param_superuser,
    };
    diesel::update(users.find(param_id))
        .set(&updated_user)
        .get_result(connection)
        .expect("Error saving new user")
}

pub fn deactivate_user(param_id: i32) -> User {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    diesel::update(users.find(param_id))
        .set(active.eq(false))
        .get_result(connection)
        .expect("Error saving new user")
}