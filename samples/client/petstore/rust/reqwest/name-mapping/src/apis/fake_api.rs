/*
 * rust test
 *
 * rust name mapping test
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`get_parameter_name_mapping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetParameterNameMappingError {
    UnknownValue(serde_json::Value),
}


pub fn get_parameter_name_mapping(configuration: &configuration::Configuration, underscore_type: i64, r#type: &str, type_with_underscore: &str, dash_type: &str, http_debug_option: &str) -> Result<(), Error<GetParameterNameMappingError>> {
    let uri_str = format!("{}/fake/parameter-name-mapping", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("type", &r#type.to_string())]);
    req_builder = req_builder.query(&[("http_debug_option", &http_debug_option.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("_type", underscore_type.to_string());
    req_builder = req_builder.header("type_", type_with_underscore.to_string());
    req_builder = req_builder.header("-type", dash_type.to_string());

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<GetParameterNameMappingError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

