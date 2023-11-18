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
pub struct ElectionsRecoverStakeAction {
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "staker")]
    pub staker: Box<crate::models::AccountAddress>,
}

impl ElectionsRecoverStakeAction {
    pub fn new(amount: i64, staker: crate::models::AccountAddress) -> ElectionsRecoverStakeAction {
        ElectionsRecoverStakeAction {
            amount,
            staker: Box::new(staker),
        }
    }
}


