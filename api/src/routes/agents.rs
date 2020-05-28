// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//use crate::database::{helpers as db, models::Agent};
//use crate::rest_api::{
//    error::RestApiResponseError, routes::DbExecutor, AcceptServiceIdParam, AppState, QueryServiceId,
//};
use crate::error::RestApiResponseError;
use crate::AppState;

use actix::{Handler, Message, SyncContext};
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

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
/*
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
*/
struct ListAgents {
    service_id: Option<String>,
}

impl Message for ListAgents {
    type Result = Result<Vec<AgentSlice>, RestApiResponseError>;
}
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
    state: web::Data<AppState>,
    //batch_submitter: Box<dyn BatchSubmitter + 'static>,
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

struct FetchAgent {
    public_key: String,
    service_id: Option<String>,
}

impl Message for FetchAgent {
    type Result = Result<AgentSlice, RestApiResponseError>;
}
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
    state: web::Data<AppState>,
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


//use crate::error::CliError;
//use crate::http::submit_batches;
//use crate::transaction::{pike_batch_builder, PIKE_NAMESPACE};
//use grid_sdk::{
//    protocol::pike::payload::{Action, CreateAgentAction, PikePayloadBuilder, UpdateAgentAction},
//    protos::IntoProto,
//};

pub async fn create_agent(
    //url: &str,
    //key: Option<String>,
    //wait: u64,
    //create_agent: CreateAgentAction,
    //service_id: Option<String>,
//) -> Result<(), CliError> {
) -> Result<HttpResponse, RestApiResponseError> {
    Ok(HttpResponse::Ok().body("Hello world! create_agent"))
/*
    let payload = PikePayloadBuilder::new()
        .with_action(Action::CreateAgent)
        .with_create_agent(create_agent)
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

pub async fn update_agent(
    //url: &str,
    //key: Option<String>,
    //wait: u64,
    //update_agent: UpdateAgentAction,
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