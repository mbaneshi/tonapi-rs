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
pub struct InitStateRaw {
    #[serde(rename = "workchain")]
    pub workchain: i32,
    #[serde(rename = "root_hash")]
    pub root_hash: String,
    #[serde(rename = "file_hash")]
    pub file_hash: String,
}

impl InitStateRaw {
    pub fn new(workchain: i32, root_hash: String, file_hash: String) -> InitStateRaw {
        InitStateRaw {
            workchain,
            root_hash,
            file_hash,
        }
    }
}


