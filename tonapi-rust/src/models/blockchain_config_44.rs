/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

/// BlockchainConfig44 : suspended accounts



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig44 {
    #[serde(rename = "accounts")]
    pub accounts: Vec<String>,
    #[serde(rename = "suspended_until")]
    pub suspended_until: i32,
}

impl BlockchainConfig44 {
    /// suspended accounts
    pub fn new(accounts: Vec<String>, suspended_until: i32) -> BlockchainConfig44 {
        BlockchainConfig44 {
            accounts,
            suspended_until,
        }
    }
}


