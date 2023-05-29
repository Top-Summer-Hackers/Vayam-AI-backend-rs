use crate::db::Result;
use crate::error::MyError::MongoSerializeBsonError;
use crate::model::TaskModel;
use crate::response::TaskResponse;
use crate::schema::CreateTaskSchema;
use crate::{model::ProviderModel, response::ProviderResponse, schema::CreateProviderSchema};
use mongodb::bson::{self, doc};

pub fn build_provider_document(
  body: &CreateProviderSchema,
  description: String,
) -> Result<bson::Document> {
  let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();
  let mut doc_with_description = doc! {"description": description};
  doc_with_description.extend(document.clone());

  Ok(doc_with_description)
}

pub fn doc_to_provider_response(provider: &ProviderModel) -> Result<ProviderResponse> {
  let provider_response = ProviderResponse {
    id: provider.id.to_hex(),
    user_name: provider.user_name.to_owned(),
    description: provider.description.to_owned().unwrap(),
    password: provider.password.to_owned(),
  };

  Ok(provider_response)
}

pub fn build_task_document(body: &CreateTaskSchema) -> Result<bson::Document> {
  let serialized_data = bson::to_bson(body).map_err(MongoSerializeBsonError)?;
  let document = serialized_data.as_document().unwrap();

  Ok(document.clone())
}

pub fn doc_to_task_response(task: &TaskModel) -> Result<TaskResponse> {
  let task_response = TaskResponse {
    id: task.id.to_hex(),
    title: task.title.to_owned(),
    start_time: task.start_time.to_owned(),
    deadline: task.deadline.to_owned(),
    description: task.description.to_owned(),
  };
  Ok(task_response)
}
