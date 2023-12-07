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
pub struct DecodedMessageExtInMsgDecoded {
    #[serde(rename = "wallet_v3", skip_serializing_if = "Option::is_none")]
    pub wallet_v3: Option<Box<crate::models::DecodedMessageExtInMsgDecodedWalletV3>>,
    #[serde(rename = "wallet_v4", skip_serializing_if = "Option::is_none")]
    pub wallet_v4: Option<Box<crate::models::DecodedMessageExtInMsgDecodedWalletV4>>,
    #[serde(rename = "wallet_highload_v2", skip_serializing_if = "Option::is_none")]
    pub wallet_highload_v2: Option<Box<crate::models::DecodedMessageExtInMsgDecodedWalletHighloadV2>>,
}

impl DecodedMessageExtInMsgDecoded {
    pub fn new() -> DecodedMessageExtInMsgDecoded {
        DecodedMessageExtInMsgDecoded {
            wallet_v3: None,
            wallet_v4: None,
            wallet_highload_v2: None,
        }
    }
}


