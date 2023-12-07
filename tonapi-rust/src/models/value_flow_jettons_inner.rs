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
pub struct ValueFlowJettonsInner {
    #[serde(rename = "account")]
    pub account: Box<crate::models::AccountAddress>,
    #[serde(rename = "jetton")]
    pub jetton: Box<crate::models::JettonPreview>,
    #[serde(rename = "quantity")]
    pub quantity: i64,
}

impl ValueFlowJettonsInner {
    pub fn new(account: crate::models::AccountAddress, jetton: crate::models::JettonPreview, quantity: i64) -> ValueFlowJettonsInner {
        ValueFlowJettonsInner {
            account: Box::new(account),
            jetton: Box::new(jetton),
            quantity,
        }
    }
}


