/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */

/// BlockchainConfig25 : The cost of sending messages in the basechains of the TON blockchain.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BlockchainConfig25 {
    #[serde(rename = "msg_forward_prices")]
    pub msg_forward_prices: Box<crate::models::MsgForwardPrices>,
}

impl BlockchainConfig25 {
    /// The cost of sending messages in the basechains of the TON blockchain.
    pub fn new(msg_forward_prices: crate::models::MsgForwardPrices) -> BlockchainConfig25 {
        BlockchainConfig25 {
            msg_forward_prices: Box::new(msg_forward_prices),
        }
    }
}


