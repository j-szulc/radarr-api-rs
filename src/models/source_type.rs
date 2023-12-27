/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SourceType {
    #[serde(rename = "tmdb")]
    Tmdb,
    #[serde(rename = "mappings")]
    Mappings,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "indexer")]
    Indexer,

}

impl ToString for SourceType {
    fn to_string(&self) -> String {
        match self {
            Self::Tmdb => String::from("tmdb"),
            Self::Mappings => String::from("mappings"),
            Self::User => String::from("user"),
            Self::Indexer => String::from("indexer"),
        }
    }
}

impl Default for SourceType {
    fn default() -> SourceType {
        Self::Tmdb
    }
}



