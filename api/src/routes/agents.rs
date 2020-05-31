// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//use crate::database::{helpers as db, models::Agent};
//use crate::rest_api::{
//    error::RestApiResponseError, routes::DbExecutor, AcceptServiceIdParam, AppState, QueryServiceId,
//};
use crate::error::RestApiResponseError;
use crate::AppState;
use crate::submitter::{BatchStatusResponse, BatchStatuses, SubmitBatches, DEFAULT_TIME_OUT};
use crate::submitter::BatchSubmitter;

use actix::{Handler, Message, SyncContext};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

//use crate::error::CliError;
//use crate::http::submit_batches;
use crate::transaction::{pike_batch_builder, PIKE_NAMESPACE};
use grid_sdk::protocol::pike::state::{
    KeyValueEntry, KeyValueEntryBuilder,
    //Agent,
};
use grid_sdk::protocol::pike::payload::{
    Action, PikePayloadBuilder, 
    CreateAgentAction, CreateAgentActionBuilder, 
    UpdateAgentAction, UpdateAgentActionBuilder, 
};
use grid_sdk::protos::IntoProto;
use std::collections::HashMap;

use validator::Validate;

const GRID_DAEMON_KEY: &str = "GRID_DAEMON_KEY";
const GRID_DAEMON_ENDPOINT: &str = "GRID_DAEMON_ENDPOINT";
const GRID_SERVICE_ID: &str = "GRID_SERVICE_ID";
//const DEFAULT_TIME_OUT: u32 = 300; // Max timeout 300 seconds == 5 minutes

/*
let url = matches
    .value_of("url")
    .map(String::from)
    .or_else(|| env::var(GRID_DAEMON_ENDPOINT).ok())
    .unwrap_or_else(|| String::from("http://localhost:8000"));

let key = matches
    .value_of("key")
    .map(String::from)
    .or_else(|| env::var(GRID_DAEMON_KEY).ok());

let wait = value_t!(matches, "wait", u64).unwrap_or(0);

let service_id = matches
    .value_of("service_id")
    .map(String::from)
    .or_else(|| env::var(GRID_SERVICE_ID).ok());
*/
#[derive(Deserialize)]
pub struct NewAgent {
    agent: NewAgentData,
}

//#[derive(Deserialize, Validate)]
#[derive(Deserialize)]
struct NewAgentData {
    //private_key: Option<String>,
    org_id: Option<String>, 
    roles: Option<String>, 
    metadata: Option<String>
/*
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
*/
}
/*
#[derive(Debug, Serialize, Deserialize)]
pub struct AgentSlice {
    pub public_key: String,
    pub org_id: String,
    pub active: bool,
    pub roles: Vec<String>,
    pub metadata: JsonValue,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

impl AgentSlice {
    pub fn from_agent(agent: &Agent) -> Self {
        Self {
            public_key: agent.public_key.clone(),
            org_id: agent.org_id.clone(),
            active: agent.active,
            roles: agent.roles.clone(),
            metadata: agent.metadata.clone(),
            service_id: agent.service_id.clone(),
        }
    }
}

struct ListAgents {
    service_id: Option<String>,
}

impl Message for ListAgents {
    type Result = Result<Vec<AgentSlice>, RestApiResponseError>;
}
*/
/*
impl Handler<ListAgents> for DbExecutor {
    type Result = Result<Vec<AgentSlice>, RestApiResponseError>;

    fn handle(&mut self, msg: ListAgents, _: &mut SyncContext<Self>) -> Self::Result {
        let fetched_agents =
            db::get_agents(&*self.connection_pool.get()?, msg.service_id.as_deref())?
                .iter()
                .map(|agent| AgentSlice::from_agent(agent))
                .collect::<Vec<AgentSlice>>();

        Ok(fetched_agents)
    }
}
*/
pub async fn list_agents(
    //state: web::Data<AppState>,
    //query: web::Query<QueryServiceId>,
    //_: AcceptServiceIdParam,
) -> Result<HttpResponse, RestApiResponseError> {
/*
    state
        .database_connection
        .send(ListAgents {
            service_id: query.into_inner().service_id,
        })
        .await?
        .map(|agents| HttpResponse::Ok().json(agents))
*/        
    Ok(HttpResponse::Ok().body("Hello world! list_agents"))

}
/*
struct FetchAgent {
    public_key: String,
    service_id: Option<String>,
}

impl Message for FetchAgent {
    type Result = Result<AgentSlice, RestApiResponseError>;
}
*/
/*
impl Handler<FetchAgent> for DbExecutor {
    type Result = Result<AgentSlice, RestApiResponseError>;

    fn handle(&mut self, msg: FetchAgent, _: &mut SyncContext<Self>) -> Self::Result {
        let fetched_agent = match db::get_agent(
            &*self.connection_pool.get()?,
            &msg.public_key,
            msg.service_id.as_deref(),
        )? {
            Some(agent) => AgentSlice::from_agent(&agent),
            None => {
                return Err(RestApiResponseError::NotFoundError(format!(
                    "Could not find agent with public key: {}",
                    msg.public_key
                )));
            }
        };

        Ok(fetched_agent)
    }
}
*/
pub async fn fetch_agent(
    //state: web::Data<AppState>,
    public_key: web::Path<String>,
//    query: web::Query<QueryServiceId>,
//    _: AcceptServiceIdParam,
) -> Result<HttpResponse, RestApiResponseError> {
/*    
    state
        .database_connection
        .send(FetchAgent {
            public_key: public_key.into_inner(),
            service_id: query.into_inner().service_id,
        })
        .await?
        .map(|agent| HttpResponse::Ok().json(agent))
*/        
    Ok(HttpResponse::Ok().body("Hello world! fetch_agent"))

}

pub async fn create_agent(
    //url: &str,
    //secret_key: Option<String>,
    //wait: u64,
    //create_agent: web::Json<CreateAgentAction>,
    //new_agent: web::Json<NewAgent>,
    query: web::Query<HashMap<String, String>>,
    state: web::Data<AppState>,
    //batch_submitter: Box<dyn BatchSubmitter + 'static>,
    //service_id: Option<String>,
//) -> Result<(), CliError> {
) -> Result<HttpResponse, RestApiResponseError> {

    //let new_agent = new_agent.into_inner().agent;
/*
    let mut extractor = FieldValidator::validate(&new_agent);
    let org_id = extractor.extract("org_id", new_agent.org_id);
    let roles_as_strings = extractor.extract("roles", new_agent.roles);
    let metadata_as_strings = extractor.extract("metadata", new_agent.metadata);
    //let private_key_hex_string = extractor.extract("private_key", new_agent.private_key);
    extractor.check()?;
*/

    //let org_id: String = new_agent.org_id.unwrap();
    let org_id = match query.get("org_id") {
        Some(org_id) => org_id.to_string(),
        None => "".to_string(),
    };

    let roles_as_string = match query.get("roles_as_string") {
        Some(roles_as_string) => roles_as_string.to_string(),
        None => "".to_string(),
    };

    let metadata_as_string = match query.get("metadata_as_string") {
        Some(metadata_as_string) => metadata_as_string.to_string(),
        None => "".to_string(),
    };

    let mut roles = Vec::<String>::new();
    //for role in new_agent.roles {
    for role in roles_as_string.chars() {
        let entry: String = role.to_string().split(",").collect();
        roles.push(entry.clone());
    }

    let mut metadata = Vec::<KeyValueEntry>::new();
    //for meta in new_agent.metadata {
    for meta in metadata_as_string.chars() {
        let meta_as_string = meta.to_string();
        let key_val: Vec<&str> = meta_as_string.split(",").collect();
        if key_val.len() != 2 {
            "Metadata is formated incorrectly".to_string();            
        }
        let key = match key_val.get(0) {
            Some(key) => key.to_string(),
            None => "Metadata is formated incorrectly".to_string()
        };
        let value = match key_val.get(1) {
            Some(value) => value.to_string(),
            None => "Metadata is formated incorrectly".to_string()
        };

        let key_value = KeyValueEntryBuilder::new()
            .with_key(key.to_string())
            .with_value(value.to_string())
            .build()
            .unwrap();

        metadata.push(key_value.clone());
    }

    let action = CreateAgentActionBuilder::new()
        .with_org_id(org_id)
        .with_public_key("public_key".to_string())
        .with_active(true)
        .with_roles(roles)
        .with_metadata(metadata)
        .build()
        .unwrap();

    let payload = PikePayloadBuilder::new()
        .with_action(Action::CreateAgent)
        //.with_create_agent(create_agent)
        .with_create_agent(action)
        .build()
        //.map_err(|err| CliError::UserError(format!("{}", err)))?;
        .map_err(|err| RestApiResponseError::UserError(format!("{}", err)))?;

    let private_key = match query.get("private_key") {
        Some(private_key) => Some(private_key.as_str().to_string()),
        None => Some("".to_string()),
    };
    
    //let batch_list = pike_batch_builder(secret_key)
    let batch_list = pike_batch_builder(private_key)
        .add_transaction(
            &payload.into_proto()?,
            &[PIKE_NAMESPACE.to_string()],
            &[PIKE_NAMESPACE.to_string()],
        )?
        .create_batch_list();

    let url = match query.get("url") {
        Some(url) => url,
        None => GRID_DAEMON_ENDPOINT,
    };

    let service_id = match query.get("service_id") {
        Some(service_id) => Some(service_id.as_str()),
        None => Some(GRID_SERVICE_ID),
    };

    // Max wait time allowed is 95% of network's configured timeout
    let max_wait_time = (DEFAULT_TIME_OUT * 95) / 100;

    let wait = match query.get("wait") {
        Some(wait_time) => {
            if wait_time == "false" {
                None
            } else {
                match wait_time.parse::<u32>() {
                    Ok(wait_time) => {
                        if wait_time > max_wait_time {
                            Some(max_wait_time)
                            //max_wait_time
                        } else {
                            Some(wait_time)
                            //wait_time
                        }
                    }
                    Err(_) => {
                        return Err(RestApiResponseError::BadRequest(format!(
                            "Query wait has invalid value {}. \
                             It should set to false or a time in seconds to wait for the commit",
                            wait_time
                        )));
                    }
                }
            }
        }

        None => Some(max_wait_time),
        //None => max_wait_time,
    };

    //submit_batches(url, wait, &batch_list, service_id.as_deref());
//    submit_batches(url, wait.unwrap().into(), &batch_list, service_id);
/*
    let bytes = batch_list.write_to_bytes()?;

    let batch_list: BatchList = match protobuf::parse_from_bytes(&*body) {
        Ok(batch_list) => batch_list,
        Err(err) => {
            return Err(RestApiResponseError::BadRequest(format!(
                "Protobuf message was badly formatted. {}",
                err.to_string()
            )));
        }
    };

    let response_url = req.url_for_static("batch_statuses")?;
*/
    state
        .batch_submitter
    //batch_submitter
        .submit_batches(SubmitBatches {
            batch_list,
            //response_url,
            //service_id: query_service_id.into_inner().service_id,
        })
        .await
        .map(|link| HttpResponse::Ok().json(link))


    //Ok(HttpResponse::Ok().body("Hello world! create_agent"))
}

pub async fn update_agent(
    //url: &str,
    //key: Option<String>,
    //wait: u64,
    //update_agent: web::Json<UpdateAgentAction>,
    //service_id: Option<String>,
//) -> Result<(), CliError> {
) -> Result<HttpResponse, RestApiResponseError> {
    Ok(HttpResponse::Ok().body("Hello world! update_agent"))
/*
    let payload = PikePayloadBuilder::new()
        .with_action(Action::UpdateAgent)
        .with_update_agent(update_agent)
        .build()
        .map_err(|err| CliError::UserError(format!("{}", err)))?;

    let batch_list = pike_batch_builder(key)
        .add_transaction(
            &payload.into_proto()?,
            &[PIKE_NAMESPACE.to_string()],
            &[PIKE_NAMESPACE.to_string()],
        )?
        .create_batch_list();

    submit_batches(url, wait, &batch_list, service_id.as_deref())
*/    
}
