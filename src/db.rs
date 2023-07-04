use crate::error::MyError;
use crate::model::{DealModel, MilestoneModel, ProposalModel, ReviewModel, TaskModel};
use crate::response::{
  DealData, DealListResponse, DealResponse, MilestoneData, MilestoneListResponse,
  MilestoneResponse, PartialDealResponse, ProposalData, ProposalDealData, ProposalDetailedData,
  ProposalListResponse, ProposalResponse, ReviewData, SingleDealResponse, SingleMilestoneResponse,
  SingleProposalDealResponse, SingleProposalDetailedResponse, SingleProposalResponse,
  SingleReviewResponse, SingleTaskResponse, SingleUserResponse, TaskData, TaskListResponse,
  TaskResponse, UserData, UserResponse, UsersListResponse,
};
use crate::schema::{
  CreateMilestoneSchema, CreateProposalSchema, CreateReviewSchema, CreateTaskSchema,
  LoginUserSchema,
};
use crate::utils::{
  build_deal_document, build_milestone_document, build_milestones_document,
  build_proposal_document, build_review_document, build_task_document, build_user_document,
  doc_to_deal_response, doc_to_detailed_proposal_response, doc_to_milestone_response,
  doc_to_proposal_and_deal_response, doc_to_proposal_response, doc_to_review_response,
  doc_to_task_response, doc_to_user_response, docs_to_deal_response,
};
use crate::web;
use crate::{error::MyError::*, model::UserModel, schema::CreateUserSchema};

use futures::StreamExt;
use mongodb::bson::{doc, Bson, Document};
use mongodb::options::{FindOneAndUpdateOptions, IndexOptions, ReturnDocument};
use mongodb::{options::ClientOptions, Client, Collection, IndexModel};

use tower_cookies::{Cookie, Cookies};
#[derive(Clone, Debug)]
pub struct DB {
  pub client_collection_model: Collection<UserModel>,
  pub client_collection: Collection<Document>,
  pub tasks_collection_model: Collection<TaskModel>,
  pub tasks_collection: Collection<Document>,
  pub freelancer_collection_model: Collection<UserModel>,
  pub freelancer_collection: Collection<Document>,
  pub review_collection_model: Collection<ReviewModel>,
  pub review_collection: Collection<Document>,
  pub proposals_collection_model: Collection<ProposalModel>,
  pub proposals_collection: Collection<Document>,
  pub deals_collection_model: Collection<DealModel>,
  pub deals_collection: Collection<Document>,
  pub milestones_collection_model: Collection<MilestoneModel>,
  pub milestones_collection: Collection<Document>,
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
    let review_collection_name =
      std::env::var("MONGODB_REVIEW_COLLECTION").expect("MONGODB_REVIEW_COLLECTION must be set.");
    let proposals_collection_name = std::env::var("MONGODB_PROPOSALS_COLLECTION")
      .expect("MONGODB_PROPOSALS_COLLECTION must be set.");
    let deals_collection_name =
      std::env::var("MONGODB_DEALS_COLLECTION").expect("MONGODB_DEALS_COLLECTION must be set.");
    let milestones_collection_name = std::env::var("MONGODB_MILESTONES_COLLECTION")
      .expect("MONGODB_MILESTONES_COLLECTION must be set.");

    let mut client_options = ClientOptions::parse(mongodb_uri).await?;
    client_options.app_name = Some(database_name.clone());

    let client_side = Client::with_options(client_options)?;
    let database = client_side.database(database_name.as_str());

    let client_collection_model = database.collection(clients_collection_name.as_str());
    let client_collection = database.collection::<Document>(clients_collection_name.as_str());
    let tasks_collection_model = database.collection(tasks_collection_name.as_str());
    let tasks_collection = database.collection::<Document>(tasks_collection_name.as_str());
    let freelancer_collection_model = database.collection(freelancers_collection_name.as_str());
    let freelancer_collection =
      database.collection::<Document>(freelancers_collection_name.as_str());
    let review_collection_model = database.collection(review_collection_name.as_str());
    let review_collection = database.collection::<Document>(review_collection_name.as_str());
    let proposals_collection_model = database.collection(proposals_collection_name.as_str());
    let proposals_collection = database.collection::<Document>(proposals_collection_name.as_str());
    let deals_collection_model = database.collection(deals_collection_name.as_str());
    let deals_collection = database.collection::<Document>(deals_collection_name.as_str());
    let milestones_collection_model = database.collection(milestones_collection_name.as_str());
    let milestones_collection =
      database.collection::<Document>(milestones_collection_name.as_str());

    println!("✅ Database connected successfully");

    Ok(Self {
      client_collection_model,
      client_collection,
      tasks_collection_model,
      tasks_collection,
      freelancer_collection_model,
      freelancer_collection,
      review_collection_model,
      review_collection,
      proposals_collection_model,
      proposals_collection,
      deals_collection_model,
      deals_collection,
      milestones_collection_model,
      milestones_collection,
    })
  }

  pub async fn api_login(
    &self,
    cookies: Cookies,
    body: &LoginUserSchema,
  ) -> Result<SingleUserResponse> {
    let role = body.role.to_owned();
    if role == "client" {
      let user_model = match self
        .client_collection_model
        .find_one(
          doc! {"user_name": body.credential.user_name.to_owned()},
          None,
        )
        .await
      {
        Ok(user) => user,
        Err(e) => return Err(MongoQueryError(e)),
      };
      match user_model {
        Some(user) => {
          let user = doc_to_user_response(&user)?;
          if user.password == body.credential.password {
            // FIXME: Implement real auth-token generation/signature.
            cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
            return Ok(SingleUserResponse {
              status: "Success",
              data: UserData { user },
            });
          }
          Err(InvalidPasswordError)
        }
        None => Err(NotFoundError(body.credential.user_name.to_owned())),
      }
    } else if role == "freelancer" {
      let user = match self
        .freelancer_collection_model
        .find_one(
          doc! {"user_name": body.credential.user_name.to_owned()},
          None,
        )
        .await
      {
        Ok(user) => user,
        Err(e) => return Err(MongoQueryError(e)),
      };
      match user {
        Some(user) => {
          let user = doc_to_user_response(&user)?;
          if user.password == body.credential.password {
            cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));
            return Ok(SingleUserResponse {
              status: "Success",
              data: UserData { user },
            });
          }
          return Err(InvalidPasswordError);
        }
        None => return Err(NotFoundError(body.credential.user_name.to_owned())),
      };
    } else {
      return Err(InvalidRoleError);
    }
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
    let role = "client".to_string();
    let document = build_user_document(body, description, role)?;

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

    Ok(TaskListResponse {
      status: "Success",
      results: json_result.len(),
      tasks: json_result,
    })
  }

  pub async fn get_proposal(&self, proposal_id: &str) -> Result<SingleProposalDetailedResponse> {
    let proposal = self
      .proposals_collection_model
      .find_one(doc! {"_id": proposal_id}, None)
      .await
      .map_err(MongoQueryError)?;

    let mut cursor = self
      .milestones_collection_model
      .find(doc! {"proposal_id": proposal_id}, None)
      .await
      .map_err(MongoQueryError)?;

    let mut milestones = Vec::new();
    while let Some(doc) = cursor.next().await {
      milestones.push(doc_to_milestone_response(&doc.unwrap())?);
    }

    match proposal {
      Some(doc) => {
        let detailed_proposal = doc_to_detailed_proposal_response(&doc, milestones)?;
        Ok(SingleProposalDetailedResponse {
          status: "Success",
          data: ProposalDetailedData { detailed_proposal },
        })
      }
      None => Err(NotFoundError(proposal_id.to_string())),
    }
  }
  pub async fn create_task(&self, body: &CreateTaskSchema) -> Result<SingleTaskResponse> {
    let _id = self
      .tasks_collection
      .count_documents(None, None)
      .await
      .map_err(MongoQueryError)?
      + 1;
    let document: Document = build_task_document(body, _id.to_string())?;

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

  pub async fn add_review(&self, body: &CreateReviewSchema) -> Result<SingleReviewResponse> {
    let _id = self
      .review_collection
      .count_documents(None, None)
      .await
      .map_err(MongoQueryError)?
      + 1;
    let document: Document = build_review_document(body, _id.to_string())?;
    let options = IndexOptions::builder().unique(true).build();
    let index = IndexModel::builder()
      .keys(doc! {"review": 1})
      .options(options)
      .build();

    match self.review_collection_model.create_index(index, None).await {
      Ok(_) => {}
      Err(e) => return Err(MongoQueryError(e)),
    };

    let insert_result = match self.review_collection.insert_one(&document, None).await {
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

    let review_model = match self
      .review_collection_model
      .find_one(doc! {"_id": new_id}, None)
      .await
    {
      Ok(Some(doc)) => doc,
      Ok(None) => return Err(NotFoundError(new_id.to_string())),
      Err(e) => return Err(MongoQueryError(e)),
    };

    let review = doc_to_review_response(&review_model)?;

    Ok(SingleReviewResponse {
      status: "Success",
      data: ReviewData { review },
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

  pub async fn fetch_milestones(&self) -> Result<MilestoneListResponse> {
    let mut cursor = self
      .milestones_collection_model
      .find(None, None)
      .await
      .map_err(MongoQueryError)?;

    let mut json_result = Vec::new();
    while let Some(doc) = cursor.next().await {
      json_result.push(doc_to_milestone_response(&doc.unwrap())?);
    }

    Ok(MilestoneListResponse {
      status: "Success",
      results: json_result.len(),
      milestones: json_result,
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

  pub async fn add_milestones(
    &self,
    body: &Vec<CreateMilestoneSchema>,
  ) -> Result<SingleProposalResponse> {
    let mil_id = self
      .milestones_collection
      .count_documents(None, None)
      .await
      .map_err(MongoQueryError)?;
    let proposal_price = body.iter().fold(0.0, |acc, x| acc + x.price);
    let document = build_milestones_document(body, mil_id)?;

    let insert_result = match self
      .milestones_collection
      .insert_many(&document, None)
      .await
    {
      Ok(res) => res,
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

    let new_ids = insert_result
      .inserted_ids
      .iter()
      .map(|id| id.1.as_str().expect("issue with new _id"))
      .collect::<Vec<&str>>();

    let proposal_id = body[0].proposal_id.clone();
    let filter = doc! {"_id": &proposal_id};
    let update = doc! {"$set": {"milestones_id": new_ids, "proposal_price": proposal_price}};
    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::After)
      .build();

    if let Some(doc) = self
      .proposals_collection_model
      .find_one_and_update(filter, update, options)
      .await
      .map_err(MongoQueryError)?
    {
      let proposal = doc_to_proposal_response(&doc)?;
      let proposal_response = SingleProposalResponse {
        status: "Success",
        data: ProposalData { proposal },
      };
      Ok(proposal_response)
    } else {
      Err(NotFoundError(proposal_id.to_string()))
    }
  }
  pub async fn aprove_proposal(&self, proposal_id: &String) -> Result<SingleProposalDealResponse> {
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
      let deal = self.add_deal(&partial_deal).await?;
      let proposal_response = SingleProposalDealResponse {
        status: "Success",
        data: ProposalDealData { proposal, deal },
      };
      Ok(proposal_response)
    } else {
      Err(NotFoundError(proposal_id.to_string()))
    }
  }

  pub async fn add_deal(&self, partial_deal: &PartialDealResponse) -> Result<DealResponse> {
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

  pub async fn fetch_deals(&self) -> Result<DealListResponse> {
    let mut cursor = self
      .deals_collection_model
      .find(None, None)
      .await
      .map_err(MongoQueryError)?;

    let mut json_result: Vec<DealResponse> = Vec::new();
    while let Some(doc) = cursor.next().await {
      json_result.push(doc_to_deal_response(&doc.unwrap())?);
    }

    Ok(DealListResponse {
      status: "Success",
      results: json_result.len(),
      deals: json_result,
    })
  }

  pub async fn update_deal(
    &self,
    deal_id: &String,
    transaccion_id: &String,
  ) -> Result<SingleDealResponse> {
    let filter = doc! {"_id": deal_id};
    let update = doc! {"$set": {"address": transaccion_id}};

    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::After)
      .build();

    if let Some(doc) = self
      .deals_collection_model
      .find_one_and_update(filter, update, options)
      .await
      .map_err(MongoQueryError)?
    {
      let deal = doc_to_deal_response(&doc)?;
      let proposal_response = SingleDealResponse {
        status: "Success",
        data: DealData { deal },
      };
      Ok(proposal_response)
    } else {
      Err(NotFoundError(deal_id.to_string()))
    }
  }
  pub async fn submit_milestone(
    &self,
    proposal_id: &String,
    milestone_id: &String,
    link: &String,
  ) -> Result<SingleMilestoneResponse> {
    let filter = doc! {"proposal_id": proposal_id, "_id": milestone_id};
    let update = doc! {"$set": {"link": link}};

    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::After)
      .build();

    if let Some(doc) = self
      .milestones_collection_model
      .find_one_and_update(filter, update, options)
      .await
      .map_err(MongoQueryError)?
    {
      let milestone = doc_to_milestone_response(&doc)?;
      let proposal_response = SingleMilestoneResponse {
        status: "Success",
        data: MilestoneData { milestone },
      };
      Ok(proposal_response)
    } else {
      Err(NotFoundError(proposal_id.to_string()))
    }
  }

  pub async fn submit_milestones(
    &self,
    deal_id: &String,
    milestone_id: &String,
    link: &String,
  ) -> Result<SingleDealResponse> {
    let filter = doc! {"_id": deal_id};
    //let query = format!("id: "{}"", milestone_id);
    let update = doc! {"$set": {"milestones": milestone_id, "link": link}};

    let options = FindOneAndUpdateOptions::builder()
      .return_document(ReturnDocument::After)
      .build();

    if let Some(doc) = self
      .deals_collection_model
      .find_one_and_update(filter, update, options)
      .await
      .map_err(MongoQueryError)?
    {
      let deal = doc_to_deal_response(&doc)?;
      let proposal_response = SingleDealResponse {
        status: "Success",
        data: DealData { deal },
      };
      Ok(proposal_response)
    } else {
      Err(NotFoundError(deal_id.to_string()))
    }
  }
}
