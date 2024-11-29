/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`tests_file_response_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestsFileResponseGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`tests_type_testing_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestsTypeTestingGetError {
    UnknownValue(serde_json::Value),
}


pub fn tests_file_response_get(configuration: &configuration::Configuration, ) -> Result<reqwest::blocking::Response, Error<TestsFileResponseGetError>> {
    let uri_str = format!("{}/tests/fileResponse", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text()?;
        let entity: Option<TestsFileResponseGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub fn tests_type_testing_get(configuration: &configuration::Configuration, ) -> Result<models::TypeTesting, Error<TestsTypeTestingGetError>> {
    let uri_str = format!("{}/tests/typeTesting", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<TestsTypeTestingGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

