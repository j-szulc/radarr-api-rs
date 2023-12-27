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
pub enum ImportListType {
    #[serde(rename = "program")]
    Program,
    #[serde(rename = "tmdb")]
    Tmdb,
    #[serde(rename = "trakt")]
    Trakt,
    #[serde(rename = "plex")]
    Plex,
    #[serde(rename = "simkl")]
    Simkl,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "advanced")]
    Advanced,

}

impl ToString for ImportListType {
    fn to_string(&self) -> String {
        match self {
            Self::Program => String::from("program"),
            Self::Tmdb => String::from("tmdb"),
            Self::Trakt => String::from("trakt"),
            Self::Plex => String::from("plex"),
            Self::Simkl => String::from("simkl"),
            Self::Other => String::from("other"),
            Self::Advanced => String::from("advanced"),
        }
    }
}

impl Default for ImportListType {
    fn default() -> ImportListType {
        Self::Program
    }
}




