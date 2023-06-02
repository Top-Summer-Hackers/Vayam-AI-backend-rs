use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserResponse {
  pub id: String,
  pub role: String,
  pub user_name: String,
  pub description: String,
  pub password: String,
  pub tasks_id: Vec<String>,
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
  pub proposals_id: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct MilestoneResponse {
  pub description: String,
  pub deadline: String,
  pub price: usize,
  pub status: String,
}
#[derive(Serialize, Debug)]
pub struct ProposalResponse {
  pub id: String,
  pub task_id: String,
  pub freelancer_id: String,
  pub milestones: Vec<MilestoneResponse>,
  pub price: usize,
  pub accepted: bool,
}

#[derive(Serialize, Debug)]
pub struct DealResponse {
  pub id: String,
  pub task_id: String,
  pub proposal_id: String,
  pub freelancer_id: String,
  pub client_id: String,
  pub price: usize,
  pub status: String,
  pub address: String,
}

#[derive(Serialize, Debug)]
pub struct PartialDealResponse {
  pub task_id: String,
  pub proposal_id: String,
  pub freelancer_id: String,
  pub client_id: String,
  pub price: usize,
  pub status: String,
  pub address: String,
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
  pub proposal: ProposalResponse,
}
#[derive(Serialize, Debug)]
pub struct MilestoneData {
  pub milestone: MilestoneResponse,
}

#[derive(Serialize, Debug)]
pub struct DealData {
  pub deal: DealResponse,
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
pub struct SingleMilestoneResponse {
  pub status: &'static str,
  pub data: MilestoneData,
}

// pub struct SingleDealResponse {
//   pub status: &'static str,
//   pub data: DealData,
// }

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
  pub proposals: Vec<ProposalResponse>,
}
#[derive(Serialize, Debug)]
pub struct MilestoneListResponse {
  pub status: &'static str,
  pub results: usize,
  pub milestones: Vec<MilestoneResponse>,
}
