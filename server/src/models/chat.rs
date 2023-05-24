use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::message::Message;

#[derive(Serialize, Deserialize)]
pub struct Chat {
    id: ObjectId,
    name: String,
    description: String,
    users: Vec<i32>,
    messages: Vec<Message>,
    created_at: Option<chrono::NaiveDateTime>,
    updated_at: Option<chrono::NaiveDateTime>,
}