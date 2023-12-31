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
pub enum MonitorTypes {
    #[serde(rename = "movieOnly")]
    MovieOnly,
    #[serde(rename = "movieAndCollection")]
    MovieAndCollection,
    #[serde(rename = "none")]
    None,

}

impl ToString for MonitorTypes {
    fn to_string(&self) -> String {
        match self {
            Self::MovieOnly => String::from("movieOnly"),
            Self::MovieAndCollection => String::from("movieAndCollection"),
            Self::None => String::from("none"),
        }
    }
}

impl Default for MonitorTypes {
    fn default() -> MonitorTypes {
        Self::MovieOnly
    }
}




