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
pub struct TonConnectProofRequest {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "proof")]
    pub proof: Box<crate::models::TonConnectProofRequestProof>,
}

impl TonConnectProofRequest {
    pub fn new(address: String, proof: crate::models::TonConnectProofRequestProof) -> TonConnectProofRequest {
        TonConnectProofRequest {
            address,
            proof: Box::new(proof),
        }
    }
}


