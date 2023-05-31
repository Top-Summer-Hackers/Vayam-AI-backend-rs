use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserResponse {
  pub id: String,
  pub role: String,
  pub user_name: String,
  pub description: String,
  pub password: String,
  pub tasks: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct TaskResponse {
  pub id: String,
  pub title: String,
  pub start_time: String,
  pub deadline: String,
  pub description: String,
  pub skills: Vec<String>,
  pub bounty: u16,
  pub proposals: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ProposalResponse {
  pub id: String,
  pub task_id: String,
  pub freelancer_id: String,
  pub milestones: Vec<String>,
  pub price: u16,
  pub accepted: bool,
}

#[derive(Serialize, Debug)]
pub struct MilestoneResponse {
  pub id: String,
  pub description: String,
  pub deadline: String,
  pub price: u16,
  pub status: String,
}

#[derive(Serialize, Debug)]
pub struct UserData {
  pub user: UserResponse,
}

#[derive(Serialize, Debug)]
pub struct TaskData {
  pub task: TaskResponse,
}

#[derive(Serialize, Debug)]
pub struct ProposalData {
  pub task: ProposalResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleUserResponse {
  pub status: &'static str,
  pub data: UserData,
}

#[derive(Serialize, Debug)]
pub struct UsersListResponse {
  pub status: &'static str,
  pub results: usize,
  pub users: Vec<UserResponse>,
}

#[derive(Serialize, Debug)]
pub struct SingleTaskResponse {
  pub status: &'static str,
  pub data: TaskData,
}

#[derive(Serialize, Debug)]
pub struct SingleProposalResponse {
  pub status: &'static str,
  pub data: ProposalData,
}

#[derive(Serialize, Debug)]
pub struct TaskListResponse {
  pub status: &'static str,
  pub results: usize,
  pub tasks: Vec<TaskResponse>,
}

#[derive(Serialize, Debug)]
pub struct ProposalListResponse {
  pub status: &'static str,
  pub results: usize,
  pub tasks: Vec<ProposalResponse>,
}
