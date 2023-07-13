use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct UserResponse {
  pub id: String,
  pub role: String,
  pub user_name: String,
  pub description: String,
  pub password: String,
}

#[derive(Serialize, Debug)]
pub struct FreelancerResponse {
  #[serde(flatten)]
  pub user: UserResponse,
  pub skills: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ClientResponse {
  #[serde(flatten)]
  pub user: UserResponse,
  pub tasks_ids: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct TaskResponse {
  pub id: String,
  pub client_id: String,
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
  pub id: String,
  pub proposal_id: String,
  pub description: String,
  pub deadline: String,
  pub price: f64,
  pub link: String,
  pub status: String,
}
#[derive(Serialize, Debug)]
pub struct ProposalResponse {
  pub id: String,
  pub client_id: String,
  pub task_id: String,
  pub freelancer_id: String,
  pub milestones_id: Vec<String>,
  pub proposal_price: f64,
  pub accepted: bool,
}

#[derive(Serialize, Debug)]
pub struct ProposalDetailedResponse {
  pub id: String,
  pub client_id: String,
  pub task_id: String,
  pub freelancer_id: String,
  pub milestones: Vec<MilestoneResponse>,
  pub proposal_price: f64,
  pub accepted: bool,
}

#[derive(Serialize, Debug)]
pub struct DealResponse {
  pub id: String,
  pub task_id: String,
  pub proposal_id: String,
  pub freelancer_id: String,
  pub client_id: String,
  pub price: f64,
  pub status: String,
  pub address: String,
}

#[derive(Serialize, Debug)]
pub struct PartialDealResponse {
  pub task_id: String,
  pub proposal_id: String,
  pub freelancer_id: String,
  pub client_id: String,
  pub price: f64,
  pub status: String,
  pub address: String,
}

#[derive(Serialize, Debug)]
pub struct UserData {
  pub user: UserResponse,
}

#[derive(Serialize, Debug)]
pub struct ClientData {
  pub client: ClientResponse,
}

#[derive(Serialize, Debug)]
pub struct FreelancerData {
  pub freelancer: FreelancerResponse,
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
pub struct ProposalDetailedData {
  pub detailed_proposal: ProposalDetailedResponse,
}
#[derive(Serialize, Debug)]
pub struct MilestoneData {
  pub milestone: MilestoneResponse,
}

#[derive(Serialize, Debug)]
pub struct ProposalDealData {
  pub proposal: ProposalResponse,
  pub deal: DealResponse,
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
pub struct SingleClientResponse {
  pub status: &'static str,
  pub data: ClientData,
}

#[derive(Serialize, Debug)]
pub struct SingleFreelancerResponse {
  pub status: &'static str,
  pub data: FreelancerData,
}

#[derive(Serialize, Debug)]
pub struct UsersListResponse {
  pub status: &'static str,
  pub results: usize,
  pub users: Vec<UserResponse>,
}

#[derive(Serialize, Debug)]
pub struct ClientListResponse {
  pub status: &'static str,
  pub results: usize,
  pub users: Vec<ClientResponse>,
}

#[derive(Serialize, Debug)]
pub struct FreelancerListResponse {
  pub status: &'static str,
  pub results: usize,
  pub users: Vec<FreelancerResponse>,
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
pub struct SingleProposalDetailedResponse {
  pub status: &'static str,
  pub data: ProposalDetailedData,
}
#[derive(Serialize, Debug)]
pub struct SingleMilestoneResponse {
  pub status: &'static str,
  pub data: MilestoneData,
}
#[derive(Serialize, Debug)]
pub struct SingleProposalDealResponse {
  pub status: &'static str,
  pub data: ProposalDealData,
}
#[derive(Serialize, Debug)]
pub struct SingleDealResponse {
  pub status: &'static str,
  pub data: DealData,
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
  pub proposals: Vec<ProposalResponse>,
}
#[derive(Serialize, Debug)]
pub struct MilestoneListResponse {
  pub status: &'static str,
  pub results: usize,
  pub milestones: Vec<MilestoneResponse>,
}
#[derive(Serialize, Debug)]
pub struct DealListResponse {
  pub status: &'static str,
  pub results: usize,
  pub deals: Vec<DealResponse>,
}

#[derive(Serialize, Debug)]
pub struct ReviewResponse {
  pub id: String,
  pub client_id: String,
  pub deal_id: String,
  pub freelancer_id: String,
  pub review: String,
  pub stars: u16,
}

#[derive(Serialize, Debug)]
pub struct ReviewData {
  pub review: ReviewResponse,
}

#[derive(Serialize, Debug)]
pub struct SingleReviewResponse {
  pub status: &'static str,
  pub data: ReviewData,
}
