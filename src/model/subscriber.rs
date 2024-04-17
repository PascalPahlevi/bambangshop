use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde:json::to_string;
use rocket::tokio;
use bambangshop::REQUEST_CLIENT;
use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serder")]
pub struct Subscriber {
    pub url: String,
    pub namae: String,
}