use crate::error::MyError;
use crate::model::TaskModel;
use crate::response::{
  SingleTaskResponse, SingleUserResponse, TaskData, TaskListResponse, TaskResponse, UserData,
  UserResponse, UsersListResponse,
};
use crate::schema::CreateTaskSchema;
use crate::utils::{
  build_task_document, build_user_document, doc_to_task_response, doc_to_user_response,
};
use crate::{error::MyError::*, model::UserModel, schema::CreateUserSchema};

use futures::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::IndexOptions;
use mongodb::{options::ClientOptions, Client, Collection, IndexModel};
#[derive(Clone, Debug)]
pub struct DB {
  pub provider_collection_model: Collection<UserModel>,
  pub provider_collection: Collection<Document>,
  pub tasks_collection_model: Collection<TaskModel>,
  pub tasks_collection: Collection<Document>,
  pub freelancer_collection_model: Collection<UserModel>,
  pub freelancer_collection: Collection<Document>,
}

pub type Result<T> = std::result::Result<T, MyError>;

impl DB {
  pub async fn init() -> Result<Self> {
    let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_name =
      std::env::var("MONGO_INIT_DATABASE").expect("MONGO_INIT_DATABASE must be set.");
    let providers_collection_name = std::env::var("MONGODB_PROVIDERS_COLLECTION")
      .expect("MONGODB_PROVIDERS_COLLECTION must be set.");
    let tasks_collection_name =
      std::env::var("MONGODB_TASKS_COLLECTION").expect("MONGODB_TASKS_COLLECTION must be set.");
    let freelancers_collection_name = std::env::var("MONGODB_FREELANCERS_COLLECTION")
      .expect("MONGODB_FREELANCERS_COLLECTION must be set.");

    let mut client_options = ClientOptions::parse(mongodb_uri).await?;
    client_options.app_name = Some(database_name.to_string());

    let client = Client::with_options(client_options)?;
    let database = client.database(database_name.as_str());

    let provider_collection_model = database.collection(providers_collection_name.as_str());
    let provider_collection = database.collection::<Document>(providers_collection_name.as_str());
    let tasks_collection_model = database.collection(tasks_collection_name.as_str());
    let tasks_collection = database.collection::<Document>(tasks_collection_name.as_str());
    let freelancer_collection_model = database.collection(freelancers_collection_name.as_str());
    let freelancer_collection =
      database.collection::<Document>(freelancers_collection_name.as_str());

    println!("✅ Database connected successfully");

    Ok(Self {
      provider_collection_model,
      provider_collection,
      tasks_collection_model,
      tasks_collection,
      freelancer_collection_model,
      freelancer_collection,
    })
  }

  pub async fn fetch_providers(&self) -> Result<UsersListResponse> {
    let mut cursor = self
      .provider_collection_model
      .find(None, None)
      .await
      .map_err(MongoQueryError)?;

    let mut json_result: Vec<UserResponse> = Vec::new();
    while let Some(doc) = cursor.next().await {
      json_result.push(doc_to_user_response(&doc.unwrap())?);
    }

    Ok(UsersListResponse {
      status: "Success",
      results: json_result.len(),
      providers: json_result,
    })
  }

  pub async fn add_provider(&self, body: &CreateUserSchema) -> Result<SingleUserResponse> {
    let description = body.description.to_owned().unwrap_or_default();
    let role = "provider";
    let document = build_user_document(body, description, role.to_owned())?;

    let options = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
      .keys(doc! {"user_name": 1})
      .options(options)
      .build();

    match self
      .provider_collection_model
      .create_index(index, None)
      .await
    {
      Ok(_) => {}
      Err(e) => return Err(MongoQueryError(e)),
    };

    let insert_result = match self.provider_collection.insert_one(&document, None).await {
      Ok(result) => result,
      Err(e) => {
        if e
          .to_string()
          .contains("E11000 duplicate key error collection")
        {
          return Err(MongoDuplicateError(e));
        }
        return Err(MongoQueryError(e));
      }
    };

    let new_id = insert_result
      .inserted_id
      .as_object_id()
      .expect("issue with new _id");

    let provider_model = match self
      .provider_collection_model
      .find_one(doc! {"_id": new_id}, None)
      .await
    {
      Ok(Some(doc)) => doc,
      Ok(None) => return Err(NotFoundError(new_id.to_string())),
      Err(e) => return Err(MongoQueryError(e)),
    };

    let provider = doc_to_user_response(&provider_model)?;

    Ok(SingleUserResponse {
      status: "Success",
      data: UserData { user: provider },
    })
  }

  pub async fn fetch_tasks(&self) -> Result<TaskListResponse> {
    let mut cursor = self
      .tasks_collection_model
      .find(None, None)
      .await
      .map_err(MongoQueryError)?;

    let mut json_result: Vec<TaskResponse> = Vec::new();
    while let Some(doc) = cursor.next().await {
      json_result.push(doc_to_task_response(&doc.unwrap())?);
    }

    Ok(TaskListResponse {
      status: "Success",
      results: json_result.len(),
      providers: json_result,
    })
  }

  pub async fn create_task(&self, body: &CreateTaskSchema) -> Result<SingleTaskResponse> {
    let document = build_task_document(body)?;

    let options = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
      .keys(doc! {"title": 1})
      .options(options)
      .build();

    match self.tasks_collection_model.create_index(index, None).await {
      Ok(_) => {}
      Err(e) => return Err(MongoQueryError(e)),
    };

    let insert_result = match self.tasks_collection.insert_one(&document, None).await {
      Ok(result) => result,
      Err(e) => {
        if e
          .to_string()
          .contains("E11000 duplicate key error collection")
        {
          return Err(MongoDuplicateError(e));
        }
        return Err(MongoQueryError(e));
      }
    };

    let new_id = insert_result
      .inserted_id
      .as_object_id()
      .expect("issue with new _id");

    let task_model = match self
      .tasks_collection_model
      .find_one(doc! {"_id": new_id}, None)
      .await
    {
      Ok(Some(doc)) => doc,
      Ok(None) => return Err(NotFoundError(new_id.to_string())),
      Err(e) => return Err(MongoQueryError(e)),
    };

    let task = doc_to_task_response(&task_model)?;

    Ok(SingleTaskResponse {
      status: "Success",
      data: TaskData { task },
    })
  }

  pub async fn fetch_freelancers(&self) -> Result<UsersListResponse> {
    let mut cursor = self
      .freelancer_collection_model
      .find(None, None)
      .await
      .map_err(MongoQueryError)?;

    let mut json_result: Vec<UserResponse> = Vec::new();
    while let Some(doc) = cursor.next().await {
      json_result.push(doc_to_user_response(&doc.unwrap())?);
    }

    Ok(UsersListResponse {
      status: "Success",
      results: json_result.len(),
      providers: json_result,
    })
  }

  pub async fn add_freelancer(&self, body: &CreateUserSchema) -> Result<SingleUserResponse> {
    let description = body.description.to_owned().unwrap_or_default();
    let role = "freelancer";
    let document = build_user_document(body, description, role.to_owned())?;

    let options = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
      .keys(doc! {"user_name": 1})
      .options(options)
      .build();

    match self
      .freelancer_collection_model
      .create_index(index, None)
      .await
    {
      Ok(_) => {}
      Err(e) => return Err(MongoQueryError(e)),
    };

    let insert_result = match self.freelancer_collection.insert_one(&document, None).await {
      Ok(result) => result,
      Err(e) => {
        if e
          .to_string()
          .contains("E11000 duplicate key error collection")
        {
          return Err(MongoDuplicateError(e));
        }
        return Err(MongoQueryError(e));
      }
    };

    let new_id = insert_result
      .inserted_id
      .as_object_id()
      .expect("issue with new _id");

    let user_model = match self
      .freelancer_collection_model
      .find_one(doc! {"_id": new_id}, None)
      .await
    {
      Ok(Some(doc)) => doc,
      Ok(None) => return Err(NotFoundError(new_id.to_string())),
      Err(e) => return Err(MongoQueryError(e)),
    };

    let user = doc_to_user_response(&user_model)?;

    Ok(SingleUserResponse {
      status: "Success",
      data: UserData { user },
    })
  }
}
