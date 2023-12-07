/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

/// BlockchainConfig12 : Workchains in the TON Blockchain



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig12 {
    #[serde(rename = "workchains")]
    pub workchains: Vec<crate::models::WorkchainDescr>,
}

impl BlockchainConfig12 {
    /// Workchains in the TON Blockchain
    pub fn new(workchains: Vec<crate::models::WorkchainDescr>) -> BlockchainConfig12 {
        BlockchainConfig12 {
            workchains,
        }
    }
}


