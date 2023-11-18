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
pub struct Auction {
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "price")]
    pub price: i64,
    #[serde(rename = "bids")]
    pub bids: i64,
    #[serde(rename = "date")]
    pub date: i64,
}

impl Auction {
    pub fn new(domain: String, owner: String, price: i64, bids: i64, date: i64) -> Auction {
        Auction {
            domain,
            owner,
            price,
            bids,
            date,
        }
    }
}


