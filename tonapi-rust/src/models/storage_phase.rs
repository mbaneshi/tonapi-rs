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
pub struct StoragePhase {
    #[serde(rename = "fees_collected")]
    pub fees_collected: i64,
    #[serde(rename = "fees_due", skip_serializing_if = "Option::is_none")]
    pub fees_due: Option<i64>,
    #[serde(rename = "status_change")]
    pub status_change: crate::models::AccStatusChange,
}

impl StoragePhase {
    pub fn new(fees_collected: i64, status_change: crate::models::AccStatusChange) -> StoragePhase {
        StoragePhase {
            fees_collected,
            fees_due: None,
            status_change,
        }
    }
}


