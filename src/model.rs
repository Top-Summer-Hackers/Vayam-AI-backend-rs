use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserModel {
  #[serde(rename = "_id")]
  pub id: String,
  pub role: String, // Provider/Client or Freelancer
  pub user_name: String,
  pub description: Option<String>,
  pub password: String,
  pub tasks_id: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskModel {
  #[serde(rename = "_id")]
  pub id: String,
  pub client_id: String,
  pub title: String,
  pub start_time: String,
  pub deadline: String,
  pub description: String,
  //#[validate(length(max = 5))]
  pub skills: Vec<String>,
  pub bounty: u16,
  pub proposals_id: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProposalModel {
  #[serde(rename = "_id")]
  pub id: String,
  pub client_id: String,
  pub task_id: String,
  pub freelancer_id: String,
  pub milestones: Vec<MilestoneModel>,
  //pub price: usize,
  pub accepted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MilestoneModel {
  pub description: String,
  pub deadline: String,
  pub price: usize,
  pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DealModel {
  #[serde(rename = "_id")]
  pub id: String,
  pub task_id: String,
  pub proposal_id: String,
  pub freelancer_id: String,
  pub client_id: String,
  pub price: usize,
  pub status: String,
  pub address: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReviewModel {
  #[serde(rename = "_id")]
  pub id: String,
  pub freelancer_id: String,
  pub client_id: String,
  pub deal_id: String,
  pub review: String,
  pub stars: u16,
}
