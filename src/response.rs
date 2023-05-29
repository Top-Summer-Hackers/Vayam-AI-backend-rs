use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct ProviderResponse {
  pub id: String,
  pub user_name: String,
  pub description: String,
  pub password: String,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct TaskResponse {
  pub id: String,
  pub title: String,
  pub start_time: String,
  pub deadline: String,
  pub description: String,
}
#[derive(Serialize, Debug)]
pub struct ProviderData {
  pub provider: ProviderResponse,
}

#[derive(Serialize, Debug)]
pub struct TaskData {
  pub task: TaskResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleProviderResponse {
  pub status: &'static str,
  pub data: ProviderData,
}

#[derive(Serialize, Debug)]
pub struct ProvidersListResponse {
  pub status: &'static str,
  pub results: usize,
  pub providers: Vec<ProviderResponse>,
}

#[derive(Serialize, Debug)]
pub struct SingleTaskResponse {
  pub status: &'static str,
  pub data: TaskData,
}

#[derive(Serialize, Debug)]
pub struct TaskListResponse {
  pub status: &'static str,
  pub results: usize,
  pub providers: Vec<TaskResponse>,
}
