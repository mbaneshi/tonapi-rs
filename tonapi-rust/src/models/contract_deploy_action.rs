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
pub struct ContractDeployAction {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "interfaces")]
    pub interfaces: Vec<String>,
}

impl ContractDeployAction {
    pub fn new(address: String, interfaces: Vec<String>) -> ContractDeployAction {
        ContractDeployAction {
            address,
            interfaces,
        }
    }
}


