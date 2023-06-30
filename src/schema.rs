use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
  pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUserSchema {
  pub user_name: String,
  pub password: String,
  pub role: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserSchema {
  #[serde(rename = "_id")]
  pub _id: String, //Todo: serde no convert "id" into "_id"
  pub user_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub password: String,
  // #[serde(skip_serializing_if = "Option::is_none")]
  // pub tasks_id: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTaskSchema {
  pub client_id: String,
  pub title: String,
  pub start_time: String,
  pub deadline: String,
  pub description: String,
  pub skills: Vec<String>,
  pub bounty: u16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub proposals_id: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProposalSchema {
  pub client_id: String,
  pub task_id: String,
  pub freelancer_id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub milestones_id: Option<Vec<String>>,
  //pub price: u16,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct CreateBasicProposalSchema {
//   pub client_id: String,
//   pub task_id: String,
//   pub freelancer_id: String,
//   //pub price: u16,
//   #[serde(skip_serializing_if = "Option::is_none")]
//   pub milestones_id: Option<Vec<String>>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMilestoneSchema {
  pub proposal_id: String,
  pub description: String,
  pub deadline: String,
  pub price: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateReviewSchema {
  pub freelancer_id: String,
  pub client_id: String,
  pub deal_id: String,
  pub review: String,
  pub stars: u16,
}
