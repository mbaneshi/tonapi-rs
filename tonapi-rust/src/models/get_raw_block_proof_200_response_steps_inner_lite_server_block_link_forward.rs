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
pub struct GetRawBlockProof200ResponseStepsInnerLiteServerBlockLinkForward {
    #[serde(rename = "to_key_block")]
    pub to_key_block: bool,
    #[serde(rename = "from")]
    pub from: Box<crate::models::BlockRaw>,
    #[serde(rename = "to")]
    pub to: Box<crate::models::BlockRaw>,
    #[serde(rename = "dest_proof")]
    pub dest_proof: String,
    #[serde(rename = "config_proof")]
    pub config_proof: String,
    #[serde(rename = "signatures")]
    pub signatures: Box<crate::models::GetRawBlockProof200ResponseStepsInnerLiteServerBlockLinkForwardSignatures>,
}

impl GetRawBlockProof200ResponseStepsInnerLiteServerBlockLinkForward {
    pub fn new(to_key_block: bool, from: crate::models::BlockRaw, to: crate::models::BlockRaw, dest_proof: String, config_proof: String, signatures: crate::models::GetRawBlockProof200ResponseStepsInnerLiteServerBlockLinkForwardSignatures) -> GetRawBlockProof200ResponseStepsInnerLiteServerBlockLinkForward {
        GetRawBlockProof200ResponseStepsInnerLiteServerBlockLinkForward {
            to_key_block,
            from: Box::new(from),
            to: Box::new(to),
            dest_proof,
            config_proof,
            signatures: Box::new(signatures),
        }
    }
}


