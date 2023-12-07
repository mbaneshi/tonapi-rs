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
pub struct NftCollections {
    #[serde(rename = "nft_collections")]
    pub nft_collections: Vec<crate::models::NftCollection>,
}

impl NftCollections {
    pub fn new(nft_collections: Vec<crate::models::NftCollection>) -> NftCollections {
        NftCollections {
            nft_collections,
        }
    }
}


