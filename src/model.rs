use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub role: String, // Provider/Client or Freelancer
  pub user_name: String,
  pub description: Option<String>,
  pub password: String,
  pub tasks: Option<Vec<ObjectId>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskModel {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub title: String,
  pub start_time: String,
  pub deadline: String,
  pub description: String,
  //#[validate(length(max = 5))]
  pub skills: Vec<String>,
  pub bounty: u16,
  pub proposals: Option<Vec<ObjectId>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProposalModel {
  #[serde(rename = "_id")]
  pub id: ObjectId,
  pub task_id: ObjectId,
  pub freelancer_id: ObjectId,
  pub milestones: Vec<Milestone>,
  pub price: u16,
  pub accepted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Milestone {
  pub description: String,
  pub deadline: String,
  pub price: u16,
  pub status: String,
}
