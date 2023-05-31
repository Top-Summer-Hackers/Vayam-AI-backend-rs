use crate::db::Result;
use crate::error::MyError::MongoSerializeBsonError;
use crate::model::{MilestoneModel, ProposalModel, TaskModel};
use crate::response::{MilestoneResponse, ProposalResponse, TaskResponse};
use crate::schema::{CreateMilestoneSchema, CreateProposalSchema, CreateTaskSchema};
use crate::{model::UserModel, response::UserResponse, schema::CreateUserSchema};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, doc};

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
    id: user.id.to_hex(),
    user_name: user.user_name.to_owned(),
    description: user.description.to_owned().unwrap(),
    password: user.password.to_owned(),
    tasks: map_object_id_to_string(user.tasks.to_owned()),
  };

  Ok(user_response)
}

pub fn build_task_document(body: &CreateTaskSchema) -> Result<bson::Document> {
  let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();

  Ok(document.clone())
}

pub fn build_proposal_document(body: &CreateProposalSchema) -> Result<bson::Document> {
  let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();
  let mut doc_with_accepted = doc! {"price": 12000, "accepted": false};
  doc_with_accepted.extend(document.clone());

  Ok(doc_with_accepted)
}

// pub fn build_milestones_document(body: &Vec<CreateMilestoneSchema>) -> Result<bson::Document> {
//   let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
//   let document = serialized_data.as_document().unwrap();
//   let mut doc_with_status = doc! {"status": "Initialized"};
//   doc_with_status.extend(document.clone());
//   Ok(document.clone())
// }

pub fn doc_to_task_response(task: &TaskModel) -> Result<TaskResponse> {
  let task_response = TaskResponse {
    id: task.id.to_hex(),
    title: task.title.to_owned(),
    start_time: task.start_time.to_owned(),
    deadline: task.deadline.to_owned(),
    description: task.description.to_owned(),
    skills: task.skills.to_owned(),
    bounty: task.bounty,
    proposals: map_object_id_to_string(task.proposals.to_owned()),
  };
  Ok(task_response)
}

pub fn doc_to_proposal_response(proposal: &ProposalModel) -> Result<ProposalResponse> {
  let proposal_response = ProposalResponse {
    id: proposal.id.to_hex(),
    task_id: proposal.task_id.to_hex(),
    freelancer_id: proposal.freelancer_id.to_hex(),
    milestones: map_object_id_to_string(proposal.milestones.to_owned()),
    price: proposal.price,
    accepted: proposal.accepted,
  };
  Ok(proposal_response)
}

// pub fn doc_to_milestones_response(milestones: Vec<MilestoneModel>) -> Vec<MilestoneResponse> {
//   milestones
//     .iter()
//     .map(|milestone| {
//       let milestone_response = MilestoneResponse {
//         description: milestone.description.to_owned(),
//         deadline: milestone.deadline.to_owned(),
//         price: milestone.price,
//         status: milestone.status.to_owned(),
//       };
//       milestone_response
//     })
//     .collect()
// }
