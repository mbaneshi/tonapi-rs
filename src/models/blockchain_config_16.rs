/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

/// BlockchainConfig16 : The limits on the number of validators in the TON blockchain.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig16 {
    #[serde(rename = "max_validators")]
    pub max_validators: i32,
    #[serde(rename = "max_main_validators")]
    pub max_main_validators: i32,
    #[serde(rename = "min_validators")]
    pub min_validators: i32,
}

impl BlockchainConfig16 {
    /// The limits on the number of validators in the TON blockchain.
    pub fn new(max_validators: i32, max_main_validators: i32, min_validators: i32) -> BlockchainConfig16 {
        BlockchainConfig16 {
            max_validators,
            max_main_validators,
            min_validators,
        }
    }
}


