use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
  pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateUserSchema {
  #[serde(rename = "_id")]
  pub _id: String, //Todo: serde no converte "id" into "_id"
  pub user_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub password: String,
  // #[serde(skip_serializing_if = "Option::is_none")]
  // pub tasks: Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub freelancer: Option<Vec<Freelancer>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Freelancer {
  pub skills: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTaskSchema {
  pub title: String,
  pub start_time: String,
  pub deadline: String,
  pub description: String,
  pub skills: Vec<String>,
  pub bounty: u16,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub proposals: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProposalSchema {
  pub task_id: String,
  pub freelancer_id: String,
  pub milestones: Vec<CreateMilestoneSchema>,
  //pub price: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateBasicProposalSchema {
  pub task_id: String,
  pub freelancer_id: String,
  //pub price: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMilestoneSchema {
  pub description: String,
  pub deadline: String,
  pub price: usize,
}
