use crate::db::Result;
use crate::error::MyError::MongoSerializeBsonError;
use crate::model::{DealModel, MilestoneModel, ProposalModel, ReviewModel, TaskModel};
use crate::response::{
  DealResponse, MilestoneResponse, PartialDealResponse, ProposalResponse, ReviewResponse,
  TaskResponse,
};
use crate::schema::{
  CreateMilestoneSchema, CreateProposalSchema, CreateReviewSchema, CreateTaskSchema,
};
use crate::{model::UserModel, response::UserResponse, schema::CreateUserSchema};
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

pub fn doc_to_user_response(user: &UserModel) -> Result<UserResponse> {
  let tasks_id = user.tasks_id.to_owned().unwrap_or_else(Vec::new);

  let user_response = UserResponse {
    role: user.role.to_owned(),
    id: user.id.to_owned(),
    user_name: user.user_name.to_owned(),
    description: user.description.to_owned().unwrap(),
    password: user.password.to_owned(),
    tasks_id,
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
  let serialized_data = bson::to_bson(&body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();
  let mut doc_with_extras = doc! {"_id": _id, "accepted": false, "proposal_price": 0};
  doc_with_extras.extend(document.clone());

  Ok(doc_with_extras)
}

pub fn build_deal_document(_id: String, partial_deal: &PartialDealResponse) -> Result<Document> {
  let serialized_data = bson::to_bson(&partial_deal).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();

  let mut doc_with_id = doc! {"_id": _id};
  doc_with_id.extend(document.clone());
  Ok(doc_with_id)
}

// pub fn split_proposal_body(
//   body: &CreateProposalSchema,
// ) -> (CreateBasicProposalSchema, Vec<CreateMilestoneSchema>) {
//   let proposal_body = CreateBasicProposalSchema {
//     client_id: body.client_id.to_owned(),
//     task_id: body.task_id.to_owned(),
//     freelancer_id: body.freelancer_id.to_owned(),
//   };
//   let milestones = body.milestones.to_owned();
//   (proposal_body, milestones)
// }

pub fn build_milestone_document(body: &CreateMilestoneSchema, id: u64) -> Result<bson::Document> {
  let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();
  let status = "Initialized";
  let link = "";
  let mut doc_with_status = doc! {"_id": id.to_string(), "status": status, "link": link};
  doc_with_status.extend(document.clone());
  Ok(doc_with_status)
}

pub fn build_milestones_document(
  body: &Vec<CreateMilestoneSchema>,
  mut id: u64,
) -> Result<Vec<bson::Document>> {
  let docs = body
    .iter()
    .map(|milestone| {
      let serialized_data = bson::to_bson(milestone).map_err(MongoSerializeBsonError)?;
      let document = serialized_data.as_document().unwrap();
      let status = "Initialized";
      let link = "";
      id += 1;
      let mut doc_with_status = doc! {"_id": id.to_string(), "status": status, "link": link};
      doc_with_status.extend(document.clone());
      Ok(doc_with_status)
    })
    .collect::<Result<Vec<bson::Document>>>()?;
  Ok(docs)
}

pub fn doc_to_task_response(task: &TaskModel) -> Result<TaskResponse> {
  let proposals_id = task.proposals_id.to_owned().unwrap_or_else(Vec::new);

  let task_response = TaskResponse {
    id: task.id.to_owned(),
    client_id: task.client_id.to_owned(),
    title: task.title.to_owned(),
    start_time: task.start_time.to_owned(),
    deadline: task.deadline.to_owned(),
    description: task.description.to_owned(),
    skills: task.skills.to_owned(),
    bounty: task.bounty,
    proposals_id,
  };
  Ok(task_response)
}

pub fn doc_to_proposal_response(proposal: &ProposalModel) -> Result<ProposalResponse> {
  //let (milestones, price) = milestone_model_to_response(&proposal.milestones);
  let milestones_id = proposal.milestones_id.to_owned().unwrap_or_else(Vec::new);
  let proposal_response = ProposalResponse {
    id: proposal.id.to_owned(),
    task_id: proposal.task_id.to_owned(),
    client_id: proposal.client_id.to_owned(),
    freelancer_id: proposal.freelancer_id.to_owned(),
    milestones_id,
    proposal_price: proposal.proposal_price,
    accepted: proposal.accepted,
  };
  Ok(proposal_response)
}

pub fn doc_to_deal_response(deal: &DealModel) -> Result<DealResponse> {
  let deal_response = DealResponse {
    id: deal.id.to_owned(),
    task_id: deal.task_id.to_owned(),
    proposal_id: deal.proposal_id.to_owned(),
    freelancer_id: deal.freelancer_id.to_owned(),
    client_id: deal.client_id.to_owned(),
    price: deal.price,
    status: deal.status.to_owned(),
    address: deal.address.to_owned(),
  };
  Ok(deal_response)
}

pub fn doc_to_proposal_and_deal_response(
  proposal: &ProposalModel,
) -> Result<(ProposalResponse, PartialDealResponse)> {
  //let (milestones_id, price) = milestone_model_to_response(&proposal.milestones);
  let proposal_response = ProposalResponse {
    id: proposal.id.to_owned(),
    client_id: proposal.client_id.to_owned(),
    task_id: proposal.task_id.to_owned(),
    freelancer_id: proposal.freelancer_id.to_owned(),
    milestones_id: proposal.milestones_id.to_owned().unwrap_or_else(Vec::new),
    proposal_price: proposal.proposal_price,
    accepted: proposal.accepted,
  };

  let partial_deal_response = PartialDealResponse {
    task_id: proposal.task_id.to_owned(),
    proposal_id: proposal.id.to_owned(),
    freelancer_id: proposal.freelancer_id.to_owned(),
    client_id: proposal.client_id.to_owned(),
    price: 1000,
    status: "Initialized".to_string(),
    address: "0x0".to_string(),
  };
  Ok((proposal_response, partial_deal_response))
}

pub fn docs_to_deal_response(
  deal: &DealModel,
  partial_deal: &PartialDealResponse,
) -> Result<DealResponse> {
  let deal_response = DealResponse {
    id: deal.id.to_owned(),
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

// pub fn milestone_model_to_response(
//   vec_milestone: &Vec<MilestoneModel>,
// ) -> (Vec<MilestoneResponse>, usize) {
//   let price = vec_milestone
//     .iter()
//     .fold(0, |acc, milestone| acc + milestone.price);
//   let vec = vec_milestone
//     .iter()
//     .map(|milestone| doc_to_milestone_response(milestone))
//     .collect();
//   (vec, price)
// }

pub fn doc_to_milestone_response(milestone: &MilestoneModel) -> Result<MilestoneResponse> {
  let milestone_response = MilestoneResponse {
    id: milestone.id.to_owned(),
    proposal_id: milestone.proposal_id.to_owned(),
    description: milestone.description.to_owned(),
    deadline: milestone.deadline.to_owned(),
    price: milestone.price,
    link: milestone.link.to_owned(),
    status: milestone.status.to_owned(),
  };
  Ok(milestone_response)
}

pub fn build_review_document(body: &CreateReviewSchema, _id: String) -> Result<bson::Document> {
  let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();
  let mut doc_with_id = doc! {"_id": _id};
  doc_with_id.extend(document.clone());
  Ok(doc_with_id)
}

pub fn doc_to_review_response(review: &ReviewModel) -> Result<ReviewResponse> {
  let review_response = ReviewResponse {
    id: review.id.to_owned(),
    freelancer_id: review.freelancer_id.to_owned(),
    client_id: review.client_id.to_owned(),
    deal_id: review.deal_id.to_owned(),
    stars: review.stars.to_owned(),
    review: review.review.to_owned(),
  };
  Ok(review_response)
}
