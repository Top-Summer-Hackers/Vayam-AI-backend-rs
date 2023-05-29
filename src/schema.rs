use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
  pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateProviderSchema {
  pub user_name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<String>,
  pub password: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTaskSchema {
  pub title: String,
  pub start_time: String,
  pub deadline: String,
  pub description: String,
}
