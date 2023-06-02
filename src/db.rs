use crate::error::MyError;
use crate::model::{DealModel, ProposalModel, TaskModel};
use crate::response::{
  DealResponse, ProposalData, ProposalListResponse, ProposalResponse, SingleProposalResponse,
  SingleTaskResponse, SingleUserResponse, TaskData, TaskListResponse, TaskResponse, UserData,
  UserResponse, UsersListResponse,
};
use crate::schema::{CreateProposalSchema, CreateTaskSchema};
use crate::utils::{
  build_deal_document, build_proposal_document, build_task_document, build_user_document,
  doc_to_proposal_and_deal_response, doc_to_proposal_response, doc_to_task_response,
  doc_to_user_response, docs_to_deal_response,
};
use crate::{error::MyError::*, model::UserModel, schema::CreateUserSchema};

use futures::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOneAndUpdateOptions, IndexOptions, ReturnDocument};
use mongodb::{options::ClientOptions, Client, Collection, IndexModel};
#[derive(Clone, Debug)]
pub struct DB {
  pub client_collection_model: Collection<UserModel>,
  pub client_collection: Collection<Document>,
  pub tasks_collection_model: Collection<TaskModel>,
  pub tasks_collection: Collection<Document>,
  pub freelancer_collection_model: Collection<UserModel>,
  pub freelancer_collection: Collection<Document>,
  pub proposals_collection_model: Collection<ProposalModel>,
  pub proposals_collection: Collection<Document>,
  pub deals_collection_model: Collection<DealModel>,
  pub deals_collection: Collection<Document>,
}

pub type Result<T> = std::result::Result<T, MyError>;

impl DB {
  pub async fn init() -> Result<Self> {
    let mongodb_uri = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_name =
      std::env::var("MONGO_INIT_DATABASE").expect("MONGO_INIT_DATABASE must be set.");
    let clients_collection_name =
      std::env::var("MONGODB_CLIENTS_COLLECTION").expect("MONGODB_CLIENTS_COLLECTION must be set.");
    let tasks_collection_name =
      std::env::var("MONGODB_TASKS_COLLECTION").expect("MONGODB_TASKS_COLLECTION must be set.");
    let freelancers_collection_name = std::env::var("MONGODB_FREELANCERS_COLLECTION")
      .expect("MONGODB_FREELANCERS_COLLECTION must be set.");
    let proposals_collection_name = std::env::var("MONGODB_PROPOSALS_COLLECTION")
      .expect("MONGODB_PROPOSALS_COLLECTION must be set.");
    let deals_collection_name =
      std::env::var("MONGODB_DEALS_COLLECTION").expect("MONGODB_DEALS_COLLECTION must be set.");

    let mut client_options = ClientOptions::parse(mongodb_uri).await?;
    client_options.app_name = Some(database_name.to_string());

    let client_side = Client::with_options(client_options)?;
    let database = client_side.database(database_name.as_str());

    let client_collection_model = database.collection(clients_collection_name.as_str());
    let client_collection = database.collection::<Document>(clients_collection_name.as_str());
    let tasks_collection_model = database.collection(tasks_collection_name.as_str());
    let tasks_collection = database.collection::<Document>(tasks_collection_name.as_str());
    let freelancer_collection_model = database.collection(freelancers_collection_name.as_str());
    let freelancer_collection =
      database.collection::<Document>(freelancers_collection_name.as_str());
    let proposals_collection_model = database.collection(proposals_collection_name.as_str());
    let proposals_collection = database.collection::<Document>(proposals_collection_name.as_str());
    let deals_collection_model = database.collection(deals_collection_name.as_str());
    let deals_collection = database.collection::<Document>(deals_collection_name.as_str());

    println!("✅ Database connected successfully");

    Ok(Self {
      client_collection_model,
      client_collection,
      tasks_collection_model,
      tasks_collection,
      freelancer_collection_model,
      freelancer_collection,
      proposals_collection_model,
      proposals_collection,
      deals_collection_model,
      deals_collection,
    })
  }

  pub async fn fetch_clients(&self) -> Result<UsersListResponse> {
    let mut cursor = self
      .client_collection_model
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
      users: json_result,
    })
  }

  pub async fn add_client(&self, body: &CreateUserSchema) -> Result<SingleUserResponse> {
    let description = body.description.to_owned().unwrap_or_default();
    let role = "client";
    let document = build_user_document(body, description, role.to_owned())?;

    let options = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
      .keys(doc! {"user_name": 1})
      .options(options)
      .build();

    match self.client_collection_model.create_index(index, None).await {
      Ok(_) => {}
      Err(e) => return Err(MongoQueryError(e)),
    };

    let insert_result = match self.client_collection.insert_one(&document, None).await {
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
      .as_str()
      .expect("issue with new _id");

    let client_model = match self
      .client_collection_model
      .find_one(doc! {"_id": new_id}, None)
      .await
    {
      Ok(Some(doc)) => doc,
      Ok(None) => return Err(NotFoundError(new_id.to_string())),
      Err(e) => return Err(MongoQueryError(e)),
    };

    let client = doc_to_user_response(&client_model)?;

    Ok(SingleUserResponse {
      status: "Success",
      data: UserData { user: client },
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
      tasks: json_result,
    })
  }

  pub async fn get_task(&self, skill: &str) -> Result<TaskListResponse> {
    let mut task = self
      .tasks_collection_model
      .find(doc! {"skills": skill}, None)
      .await
      .map_err(MongoQueryError)?;

    let mut json_result: Vec<TaskResponse> = Vec::new();
    while let Some(doc) = task.next().await {
      json_result.push(doc_to_task_response(&doc.unwrap())?);
    }
    println!("{:?}", json_result);

    Ok(TaskListResponse {
      status: "Success",
      results: json_result.len(),
      tasks: json_result,
    })
  }

  pub async fn create_task(&self, body: &CreateTaskSchema) -> Result<SingleTaskResponse> {
    let _id = self
      .tasks_collection
      .count_documents(None, None)
      .await
      .map_err(MongoQueryError)?
      + 1;
    let document = build_task_document(body, _id.to_string())?;

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
      .as_str()
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
      users: json_result,
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
      .as_str()
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

  pub async fn fetch_proposals(&self) -> Result<ProposalListResponse> {
    let mut cursor = self
      .proposals_collection_model
      .find(None, None)
      .await
      .map_err(MongoQueryError)?;

    let mut json_result: Vec<ProposalResponse> = Vec::new();
    while let Some(doc) = cursor.next().await {
      json_result.push(doc_to_proposal_response(&doc.unwrap())?);
    }

    Ok(ProposalListResponse {
      status: "Success",
      results: json_result.len(),
      proposals: json_result,
    })
  }

  pub async fn submit_proposal(
    &self,
    body: &CreateProposalSchema,
  ) -> Result<SingleProposalResponse> {
    let _id = self
      .proposals_collection
      .count_documents(None, None)
      .await
      .map_err(MongoQueryError)?
      + 1;
    let document = build_proposal_document(body, _id.to_string())?;

    let insert_result = match self.proposals_collection.insert_one(&document, None).await {
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
      .as_str()
      .expect("issue with new _id");
    let proposal_model = match self
      .proposals_collection_model
      .find_one(doc! {"_id": new_id}, None)
      .await
    {
      Ok(Some(doc)) => doc,
      Ok(None) => return Err(NotFoundError(new_id.to_string())),
      Err(e) => return Err(MongoQueryError(e)),
    };

    let proposal = doc_to_proposal_response(&proposal_model)?;

    Ok(SingleProposalResponse {
      status: "Success",
      data: ProposalData { proposal },
    })
  }

  pub async fn aprove_proposal(&self, proposal_id: &String) -> Result<SingleProposalResponse> {
    let filter = doc! {"_id": proposal_id};
    let update = doc! {"$set": {"accepted": true}};

    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::After)
      .build();

    if let Some(doc) = self
      .proposals_collection_model
      .find_one_and_update(filter, update, options)
      .await
      .map_err(MongoQueryError)?
    {
      let (proposal, partial_deal) = doc_to_proposal_and_deal_response(&doc)?;
      println!(
        "proposal_id: {}, accepted: {}",
        proposal_id, proposal.accepted
      );
      let deal = self.add_deal(&partial_deal).await?;
      let proposal_response = SingleProposalResponse {
        status: "Success",
        data: ProposalData { proposal },
      };
      Ok(proposal_response)
    } else {
      Err(NotFoundError(proposal_id.to_string()))
    }
  }

  pub async fn add_deal(&self, partial_deal: &DealResponse) -> Result<DealResponse> {
    let _id = self
      .deals_collection
      .count_documents(None, None)
      .await
      .map_err(MongoQueryError)?
      + 1;
    let _id = _id.to_string();

    let document = build_deal_document(_id.to_string(), partial_deal)?;

    let insert_result = match self.deals_collection.insert_one(&document, None).await {
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
      .as_str()
      .expect("issue with new _id");

    let deal_model = match self
      .deals_collection_model
      .find_one(doc! {"_id": new_id}, None)
      .await
    {
      Ok(Some(doc)) => doc,
      Ok(None) => return Err(NotFoundError(new_id.to_string())),
      Err(e) => return Err(MongoQueryError(e)),
    };
    docs_to_deal_response(&deal_model, partial_deal)
  }
}
