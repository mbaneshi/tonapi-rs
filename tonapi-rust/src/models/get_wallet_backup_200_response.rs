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
pub struct GetWalletBackup200Response {
    #[serde(rename = "dump")]
    pub dump: String,
}

impl GetWalletBackup200Response {
    pub fn new(dump: String) -> GetWalletBackup200Response {
        GetWalletBackup200Response {
            dump,
        }
    }
}


