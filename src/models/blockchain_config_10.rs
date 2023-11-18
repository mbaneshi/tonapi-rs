/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

/// BlockchainConfig10 : List of critical TON parameters, the change of which significantly affects the network, so more voting rounds are held.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig10 {
    #[serde(rename = "critical_params")]
    pub critical_params: Vec<i32>,
}

impl BlockchainConfig10 {
    /// List of critical TON parameters, the change of which significantly affects the network, so more voting rounds are held.
    pub fn new(critical_params: Vec<i32>) -> BlockchainConfig10 {
        BlockchainConfig10 {
            critical_params,
        }
    }
}


