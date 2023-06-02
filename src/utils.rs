use crate::db::Result;
use crate::error::MyError::MongoSerializeBsonError;
use crate::model::{DealModel, MilestoneModel, ProposalModel, TaskModel};
use crate::response::{DealResponse, MilestoneResponse, ProposalResponse, TaskResponse};
use crate::schema::{
  CreateBasicProposalSchema, CreateMilestoneSchema, CreateProposalSchema, CreateTaskSchema,
};
use crate::{model::UserModel, response::UserResponse, schema::CreateUserSchema};
use axum::http::status;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc, Document};

pub fn build_user_document(
  body: &CreateUserSchema,
  description: String,
  role: String,
) -> Result<bson::Document> {
  let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();
  let mut doc_with_description = doc! {"role": role, "description": description};
  doc_with_description.extend(document.clone());

  Ok(doc_with_description)
}

pub fn map_object_id_to_string(vec: Option<Vec<ObjectId>>) -> Vec<String> {
  match vec {
    Some(vec) => vec.iter().map(|id| id.to_hex()).collect(),
    None => return Vec::new(),
  }
}

pub fn doc_to_user_response(user: &UserModel) -> Result<UserResponse> {
  let user_response = UserResponse {
    role: user.role.to_owned(),
    id: user.id.to_owned(),
    user_name: user.user_name.to_owned(),
    description: user.description.to_owned().unwrap(),
    password: user.password.to_owned(),
    tasks_id: user.tasks_id.to_owned().unwrap(),
  };

  Ok(user_response)
}

pub fn build_task_document(body: &CreateTaskSchema, _id: String) -> Result<bson::Document> {
  let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();
  let mut doc_with_id = doc! {"_id": _id};
  doc_with_id.extend(document.clone());
  Ok(doc_with_id)
}

pub fn build_proposal_document(body: &CreateProposalSchema, _id: String) -> Result<Document> {
  let (basic_proposal_body, milestones) = split_proposal_body(body);
  let milestone_document = build_milestones_document(&milestones)?;
  //let price = price.to_string();
  let serialized_data = bson::to_bson(&basic_proposal_body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();
  let mut doc_with_milestones =
    doc! {"_id": _id, "milestones": milestone_document, "accepted": false};
  doc_with_milestones.extend(document.clone());

  Ok(doc_with_milestones)
}

pub fn build_deal_document(_id: String, partial_deal: &DealResponse) -> Result<Document> {
  let task_id = partial_deal.task_id.to_owned();
  let proposal_id = partial_deal.id.to_owned();
  let freelancer_id = partial_deal.freelancer_id.to_owned();
  let client_id = partial_deal.client_id.to_owned();
  let price = partial_deal.price.to_string();
  let status = partial_deal.status.to_owned();
  let address = partial_deal.address.to_owned();

  let doc = doc! {"_id": _id, "task_id": task_id, "proposal_id": proposal_id, "freelancer_id": freelancer_id, "client": client_id, "price": price, "status": status, "address": address};
  Ok(doc)
}

pub fn split_proposal_body(
  body: &CreateProposalSchema,
) -> (CreateBasicProposalSchema, Vec<CreateMilestoneSchema>) {
  let proposal_body = CreateBasicProposalSchema {
    task_id: body.task_id.to_owned(),
    freelancer_id: body.freelancer_id.to_owned(),
  };
  let milestones = body.milestones.to_owned();
  (proposal_body, milestones)
}

pub fn build_milestones_document(body: &Vec<CreateMilestoneSchema>) -> Result<Vec<bson::Document>> {
  let docs = body
    .iter()
    .map(|milestone| {
      let serialized_data = bson::to_bson(milestone).map_err(MongoSerializeBsonError)?;
      let document = serialized_data.as_document().unwrap();
      let status = "Initialized";
      let mut doc_with_status = doc! {"status": status};
      doc_with_status.extend(document.clone());
      Ok(doc_with_status)
    })
    .collect::<Result<Vec<bson::Document>>>()?;
  Ok(docs)
}

pub fn doc_to_task_response(task: &TaskModel) -> Result<TaskResponse> {
  let task_response = TaskResponse {
    id: task.id.to_owned(),
    title: task.title.to_owned(),
    start_time: task.start_time.to_owned(),
    deadline: task.deadline.to_owned(),
    description: task.description.to_owned(),
    skills: task.skills.to_owned(),
    bounty: task.bounty,
    proposals: task.proposals.to_owned().unwrap(),
  };
  Ok(task_response)
}

pub fn doc_to_proposal_response(proposal: &ProposalModel) -> Result<ProposalResponse> {
  let (milestones, price) = milestone_model_to_response(&proposal.milestones);
  let proposal_response = ProposalResponse {
    id: proposal.id.to_owned(),
    task_id: proposal.task_id.to_owned(),
    freelancer_id: proposal.freelancer_id.to_owned(),
    milestones,
    price,
    accepted: proposal.accepted,
  };
  Ok(proposal_response)
}

pub fn doc_to_proposal_and_deal_response(
  proposal: &ProposalModel,
) -> Result<(ProposalResponse, DealResponse)> {
  let (milestones, price) = milestone_model_to_response(&proposal.milestones);
  let proposal_response = ProposalResponse {
    id: proposal.id.to_owned(),
    task_id: proposal.task_id.to_owned(),
    freelancer_id: proposal.freelancer_id.to_owned(),
    milestones,
    price,
    accepted: proposal.accepted,
  };

  let partial_deal_response = DealResponse {
    id: "".to_owned(),
    task_id: proposal.task_id.to_owned(),
    proposal_id: proposal.id.to_owned(),
    freelancer_id: proposal.freelancer_id.to_owned(),
    client_id: "".to_owned(),
    price,
    status: "Initialized".to_string(),
    address: "0x0".to_string(),
  };
  Ok((proposal_response, partial_deal_response))
}

pub fn docs_to_deal_response(
  deal: &DealModel,
  partial_deal: &DealResponse,
) -> Result<DealResponse> {
  let deal_response = DealResponse {
    id: deal._id.to_owned(),
    task_id: partial_deal.task_id.to_owned(),
    proposal_id: partial_deal.proposal_id.to_owned(),
    freelancer_id: partial_deal.freelancer_id.to_owned(),
    client_id: partial_deal.client_id.to_owned(),
    price: partial_deal.price,
    status: partial_deal.status.to_owned(),
    address: partial_deal.address.to_owned(),
  };
  Ok(deal_response)
}

pub fn milestone_model_to_response(
  vec_milestone: &Vec<MilestoneModel>,
) -> (Vec<MilestoneResponse>, usize) {
  let price = vec_milestone
    .iter()
    .fold(0, |acc, milestone| acc + milestone.price);
  let vec = vec_milestone
    .iter()
    .map(|milestone| doc_to_milestone_response(milestone))
    .collect();
  (vec, price)
}

pub fn doc_to_milestone_response(milestone: &MilestoneModel) -> MilestoneResponse {
  let milestone_response = MilestoneResponse {
    description: milestone.description.to_owned(),
    deadline: milestone.deadline.to_owned(),
    price: milestone.price,
    status: milestone.status.to_owned(),
  };
  milestone_response
}
