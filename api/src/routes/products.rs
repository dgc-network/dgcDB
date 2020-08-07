// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use actix_web::*;
//use sawtooth_sdk::signing::CryptoFactory;
use sawtooth_sdk::signing::create_context;
use sawtooth_sdk::signing::secp256k1::Secp256k1PrivateKey;
use sawtooth_sdk::signing::PrivateKey;
use sawtooth_sdk::processor::handler::ApplyError;
use serde::Deserialize;
use protobuf::Message;
use reqwest;

use crate::transaction::BatchBuilder;
use crate::error::RestApiResponseError;
use crate::{List, Fetch};

use dgc_config::protos::*;
use dgc_config::addressing::*;
use dgc_config::protocol::product::state::*;
use dgc_config::protocol::product::payload::*;
use dgc_config::protos::schema_state::PropertyValue;
use dgc_config::protos::pike_state::KeyValueEntry;

#[derive(Deserialize)]
pub struct ProductData {
    private_key: String,
    product_id: String,
    product_type: ProductType,
    owner: String,
    properties: Vec<PropertyValue>,
    //org_id: String,
    //roles: String,
    //metadata: String,
}

pub async fn list_products(
    //req: HttpRequest,
) -> Result<HttpResponse, RestApiResponseError> {

    let url = format!("http://rest-api:8008/state?address={}{}", &hash(&PRODUCT_FAMILY_NAME, 6), PRODUCT_GS1_NAMESPACE);
    let list = reqwest::get(&url).await?.json::<List>().await?;
    println!("============ list_product_data ============");
    for sub in list.data {
        let msg = base64::decode(&sub.data).unwrap();
        let product: product_state::Product = match protobuf::parse_from_bytes(&msg){
            Ok(product) => product,
            Err(err) => {
                return Err(RestApiResponseError::ApplyError(ApplyError::InternalError(format!(
                    "Cannot deserialize organization: {:?}",
                    err,
                ))))
            }
        };
        println!("!dgc-network! serialized: {:?}", product);
        //println!("!dgc-network! public_key: {:?}", agent.public_key);
    }

    println!("============ list_product_link ============");
    println!("!dgc-network! link = {:?}", list.link);
    Ok(HttpResponse::Ok().body(list.link))
    
    //Ok(HttpResponse::Ok().json(pike_state::Agent {
    //    org_id: agent.org_id.to_string(),
    //}))
    
    //Ok(HttpResponse::Ok().body("Hello world! list_agent"))

}

pub async fn fetch_product(
    product_id: web::Path<String>,
) -> Result<HttpResponse, RestApiResponseError> {

    //println!("!dgc-network! public_key = {:?}", public_key);
    let address = make_agent_address(&product_id);
    let url = format!("http://rest-api:8008/state/{}", address);
    let res = reqwest::get(&url).await?.json::<Fetch>().await?;
    println!("============ fetch_product_data ============");
    let msg = base64::decode(&res.data).unwrap();
    let product: product_state::Product = match protobuf::parse_from_bytes(&msg){
        Ok(product) => product,
        Err(err) => {
            return Err(RestApiResponseError::ApplyError(ApplyError::InternalError(format!(
                "Cannot deserialize organization: {:?}",
                err,
            ))))
        }
    };
    println!("!dgc-network! serialized: {:?}", product);

    println!("============ fetch_product_link ============");
    println!("!dgc-network! link = {:?}", res.link);
    Ok(HttpResponse::Ok().body(res.link))
    //Ok(HttpResponse::Ok().body(res))

    //Ok(HttpResponse::Ok().body("Hello world! fetch_agent"))

}

pub async fn create_product(
    input_data: web::Json<ProductData>,
) -> Result<HttpResponse, RestApiResponseError> {

    // Create batch_list_bytes //
    let batch_list_bytes = match do_batches(input_data, Action::CreateProduct){
        Ok(product) => product,
        Err(err) => {
            return Err(RestApiResponseError::UserError(format!(
                "Cannot deserialize organization: {:?}",
                err,
            )))
        }
    };

    // Submitting Batches to the Validator //
    let res = reqwest::Client::new()
        .post("http://rest-api:8008/batches")
        .header("Content-Type", "application/octet-stream")
        .body(batch_list_bytes)
        .send().await?
        .text().await?;

    println!("============ create_product_link ============");
    println!("!dgc-network! submit_status = {:?}", res);

    Ok(HttpResponse::Ok().body(res))

    //Ok(HttpResponse::Ok().body("Hello world! create_agent"))
}

pub async fn update_product(
    input_data: web::Json<ProductData>,
) -> Result<HttpResponse, RestApiResponseError> {

    // create batch_list //
    let batch_list_bytes = match do_batches(input_data, Action::UpdateProduct){
        Ok(product) => product,
        Err(err) => {
            return Err(RestApiResponseError::UserError(format!(
                "Cannot deserialize organization: {:?}",
                err,
            )))
        }
    };

    // Submitting Batches to the Validator //
    let res = reqwest::Client::new()
        .post("http://rest-api:8008/batches")
        .header("Content-Type", "application/octet-stream")
        .body(batch_list_bytes)
        .send().await?
        .text().await?;

    println!("============ update_product_link ============");
    println!("!dgc-network! submit_status = {:?}", res);

    Ok(HttpResponse::Ok().body(res))
    
    //Ok(HttpResponse::Ok().body("Hello world! update_agent"))
}

fn do_batches(
    input_data: web::Json<ProductData>,
    action_plan: Action,
) -> Result<Vec<u8>, RestApiResponseError> {

    // Retrieving a Private Key from the input_data //
    let private_key_as_hex = &input_data.private_key;
    let private_key = Secp256k1PrivateKey::from_hex(&private_key_as_hex)
    .expect("Error generating a Private Key");
    let context = create_context("secp256k1")
    .expect("Error creating the right context");
    let public_key = context.get_public_key(&private_key)
    .expect("Error retrieving a Public Key");


    // Creating the Payload //
    let org_id = &input_data.org_id;
    let roles_as_string = &input_data.roles;
    let metadata_as_string = &input_data.metadata;

    let mut roles = Vec::<String>::new();
    for role in roles_as_string.chars() {
        let entry: String = role.to_string().split(",").collect();
        roles.push(entry.clone());
    }

    let mut metadata = Vec::<KeyValueEntry>::new();
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


    if action_plan == Action::CreateProduct {

        // Building the Action and Payload//
        let action = CreateProductActionBuilder::new()
        .with_org_id(org_id.to_string())
        .with_public_key(public_key.as_hex())
        .with_active(true)
        .with_roles(roles)
        .with_metadata(metadata)
        .build()
        .unwrap();

        let payload = ProductPayloadBuilder::new()
        .with_action(Action::CreateProduct)
        .with_create_product(action)
        .build()
        .map_err(|err| RestApiResponseError::UserError(format!("{}", err)))?;

        // Building the Transaction and Batch//
        let batch_list = BatchBuilder::new(
            PRODUCT_FAMILY_NAME, 
            PRODUCT_FAMILY_VERSION, 
            &private_key.as_hex(),
        )
        .add_transaction(
            &payload.into_proto()?,
            &[hash(&PRODUCT_FAMILY_NAME, 6)],
            &[hash(&PRODUCT_FAMILY_NAME, 6)],
            //&[PIKE_NAMESPACE.to_string()],
        )?
        .create_batch_list();

        let batch_list_bytes = batch_list
            .write_to_bytes()
            .expect("Error converting batch list to bytes");

        return Ok(batch_list_bytes);

    } else {

        // Building the Action and Payload//
        let action = UpdateProductActionBuilder::new()
        .with_org_id(org_id.to_string())
        .with_public_key(public_key.as_hex())
        .with_active(true)
        .with_roles(roles)
        .with_metadata(metadata)
        .build()
        .unwrap();

        let payload = ProductPayloadBuilder::new()
        .with_action(Action::UpdateProduct)
        .with_update_product(action)
        .build()
        .map_err(|err| RestApiResponseError::UserError(format!("{}", err)))?;

        // Building the Transaction and Batch//
        let batch_list = BatchBuilder::new(
            PRODUCT_FAMILY_NAME, 
            PRODUCT_FAMILY_VERSION, 
            &private_key.as_hex(),
        )
        .add_transaction(
            &payload.into_proto()?,
            &[hash(&PRODUCT_FAMILY_NAME, 6)],
            &[hash(&PRODUCT_FAMILY_NAME, 6)],
            //&[PRODUCT_NAMESPACE.to_string()],
            //&[PIKE_NAMESPACE.to_string()],
        )?
        .create_batch_list();

        let batch_list_bytes = batch_list
            .write_to_bytes()
            .expect("Error converting batch list to bytes");

        return Ok(batch_list_bytes);
    }
}