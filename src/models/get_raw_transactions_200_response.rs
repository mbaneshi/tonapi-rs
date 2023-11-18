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
pub struct GetRawTransactions200Response {
    #[serde(rename = "ids")]
    pub ids: Vec<crate::models::BlockRaw>,
    #[serde(rename = "transactions")]
    pub transactions: String,
}

impl GetRawTransactions200Response {
    pub fn new(ids: Vec<crate::models::BlockRaw>, transactions: String) -> GetRawTransactions200Response {
        GetRawTransactions200Response {
            ids,
            transactions,
        }
    }
}


