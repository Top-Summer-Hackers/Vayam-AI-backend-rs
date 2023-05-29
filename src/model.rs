use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProviderModel {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub user_name: String,
  pub description: Option<String>,
  pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskModel {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub start_time: String,
  pub deadline: String,
  pub description: String,
}
