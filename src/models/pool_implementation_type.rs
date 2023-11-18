/*
 * REST api to TON blockchain explorer
 *
 * Provide access to indexed TON blockchain
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: support@tonkeeper.com
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PoolImplementationType {
    #[serde(rename = "whales")]
    Whales,
    #[serde(rename = "tf")]
    Tf,
    #[serde(rename = "liquidTF")]
    LiquidTf,

}

impl ToString for PoolImplementationType {
    fn to_string(&self) -> String {
        match self {
            Self::Whales => String::from("whales"),
            Self::Tf => String::from("tf"),
            Self::LiquidTf => String::from("liquidTF"),
        }
    }
}

impl Default for PoolImplementationType {
    fn default() -> PoolImplementationType {
        Self::Whales
    }
}




