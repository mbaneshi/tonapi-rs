use serde::{Deserialize, Serialize};
use tonlib::address::TonAddress;

/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoolInfo {
    #[serde(rename = "address")]
    pub address: TonAddress,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "total_amount")]
    pub total_amount: i64,
    #[serde(rename = "implementation")]
    pub implementation: crate::rest_api::codegen::models::PoolImplementationType,
    /// APY in percent
    #[serde(rename = "apy")]
    pub apy: f32,
    #[serde(rename = "min_stake")]
    pub min_stake: i64,
    /// current nomination cycle beginning timestamp
    #[serde(rename = "cycle_start")]
    pub cycle_start: i64,
    /// current nomination cycle ending timestamp
    #[serde(rename = "cycle_end")]
    pub cycle_end: i64,
    /// this pool has verified source code or managed by trusted company
    #[serde(rename = "verified")]
    pub verified: bool,
    /// current number of nominators
    #[serde(rename = "current_nominators")]
    pub current_nominators: i32,
    /// maximum number of nominators
    #[serde(rename = "max_nominators")]
    pub max_nominators: i32,
    /// for liquid staking master account of jetton
    #[serde(
        rename = "liquid_jetton_master",
        skip_serializing_if = "Option::is_none"
    )]
    pub liquid_jetton_master: Option<String>,
    /// total stake of all nominators
    #[serde(rename = "nominators_stake")]
    pub nominators_stake: i64,
    /// stake of validator
    #[serde(rename = "validator_stake")]
    pub validator_stake: i64,
    #[serde(rename = "cycle_length", skip_serializing_if = "Option::is_none")]
    pub cycle_length: Option<i64>,
}

impl PoolInfo {
    pub fn new(
        address: TonAddress,
        name: String,
        total_amount: i64,
        implementation: crate::rest_api::codegen::models::PoolImplementationType,
        apy: f32,
        min_stake: i64,
        cycle_start: i64,
        cycle_end: i64,
        verified: bool,
        current_nominators: i32,
        max_nominators: i32,
        nominators_stake: i64,
        validator_stake: i64,
    ) -> PoolInfo {
        PoolInfo {
            address,
            name,
            total_amount,
            implementation,
            apy,
            min_stake,
            cycle_start,
            cycle_end,
            verified,
            current_nominators,
            max_nominators,
            liquid_jetton_master: None,
            nominators_stake,
            validator_stake,
            cycle_length: None,
        }
    }
}
