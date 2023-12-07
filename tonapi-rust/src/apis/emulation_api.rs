/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`decode_message`]
#[derive(Clone, Debug)]
pub struct DecodeMessageParams {
    /// bag-of-cells serialized to base64
    pub decode_message_request: crate::models::DecodeMessageRequest
}

/// struct for passing parameters to the method [`emulate_message_to_account_event`]
#[derive(Clone, Debug)]
pub struct EmulateMessageToAccountEventParams {
    /// account ID
    pub account_id: String,
    /// bag-of-cells serialized to base64
    pub decode_message_request: crate::models::DecodeMessageRequest,
    pub accept_language: Option<String>
}

/// struct for passing parameters to the method [`emulate_message_to_event`]
#[derive(Clone, Debug)]
pub struct EmulateMessageToEventParams {
    /// bag-of-cells serialized to base64
    pub decode_message_request: crate::models::DecodeMessageRequest,
    pub accept_language: Option<String>,
    pub ignore_signature_check: Option<bool>
}

/// struct for passing parameters to the method [`emulate_message_to_trace`]
#[derive(Clone, Debug)]
pub struct EmulateMessageToTraceParams {
    /// bag-of-cells serialized to base64
    pub decode_message_request: crate::models::DecodeMessageRequest,
    pub ignore_signature_check: Option<bool>
}

/// struct for passing parameters to the method [`emulate_message_to_wallet`]
#[derive(Clone, Debug)]
pub struct EmulateMessageToWalletParams {
    /// bag-of-cells serialized to base64 and additional parameters to configure emulation
    pub emulate_message_to_wallet_request: crate::models::EmulateMessageToWalletRequest,
    pub accept_language: Option<String>
}


/// struct for typed errors of method [`decode_message`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DecodeMessageError {
    DefaultResponse(crate::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`emulate_message_to_account_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulateMessageToAccountEventError {
    DefaultResponse(crate::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`emulate_message_to_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulateMessageToEventError {
    DefaultResponse(crate::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`emulate_message_to_trace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulateMessageToTraceError {
    DefaultResponse(crate::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`emulate_message_to_wallet`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulateMessageToWalletError {
    DefaultResponse(crate::models::GetBlockchainBlockDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// Decode a given message. Only external incoming messages can be decoded currently.
pub async fn decode_message(configuration: &configuration::Configuration, params: DecodeMessageParams) -> Result<crate::models::DecodedMessage, Error<DecodeMessageError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let decode_message_request = params.decode_message_request;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/message/decode", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&decode_message_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DecodeMessageError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Emulate sending message to blockchain
pub async fn emulate_message_to_account_event(configuration: &configuration::Configuration, params: EmulateMessageToAccountEventParams) -> Result<crate::models::AccountEvent, Error<EmulateMessageToAccountEventError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let account_id = params.account_id;
    let decode_message_request = params.decode_message_request;
    let accept_language = params.accept_language;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/accounts/{account_id}/events/emulate", local_var_configuration.base_path, account_id=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept_language {
        local_var_req_builder = local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&decode_message_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EmulateMessageToAccountEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Emulate sending message to blockchain
pub async fn emulate_message_to_event(configuration: &configuration::Configuration, params: EmulateMessageToEventParams) -> Result<crate::models::Event, Error<EmulateMessageToEventError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let decode_message_request = params.decode_message_request;
    let accept_language = params.accept_language;
    let ignore_signature_check = params.ignore_signature_check;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/events/emulate", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ignore_signature_check {
        local_var_req_builder = local_var_req_builder.query(&[("ignore_signature_check", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept_language {
        local_var_req_builder = local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&decode_message_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EmulateMessageToEventError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Emulate sending message to blockchain
pub async fn emulate_message_to_trace(configuration: &configuration::Configuration, params: EmulateMessageToTraceParams) -> Result<crate::models::Trace, Error<EmulateMessageToTraceError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let decode_message_request = params.decode_message_request;
    let ignore_signature_check = params.ignore_signature_check;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/traces/emulate", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ignore_signature_check {
        local_var_req_builder = local_var_req_builder.query(&[("ignore_signature_check", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&decode_message_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EmulateMessageToTraceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Emulate sending message to blockchain
pub async fn emulate_message_to_wallet(configuration: &configuration::Configuration, params: EmulateMessageToWalletParams) -> Result<crate::models::MessageConsequences, Error<EmulateMessageToWalletError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let emulate_message_to_wallet_request = params.emulate_message_to_wallet_request;
    let accept_language = params.accept_language;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/wallet/emulate", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept_language {
        local_var_req_builder = local_var_req_builder.header("Accept-Language", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.json(&emulate_message_to_wallet_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EmulateMessageToWalletError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

