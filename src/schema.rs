use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
  pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CredentialUserSchema {
  pub user_name: String,
  pub password: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginUserSchema {
  pub role: String,
  #[serde(flatten)]
  pub credentials: CredentialUserSchema,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
  #[serde(rename(serialize = "_id"))]
  pub id: String,
  #[serde(flatten)]
  pub credential: CredentialUserSchema,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateClientSchema {
  #[serde(flatten)]
  pub user: CreateUserSchema,
  pub task_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateFreelancerSchema {
  #[serde(flatten)]
  pub user: CreateUserSchema,
  pub skills: Option<Vec<String>>,
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
  pub price: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateReviewSchema {
  pub freelancer_id: String,
  pub client_id: String,
  pub deal_id: String,
  pub review: String,
  pub stars: u16,
}
