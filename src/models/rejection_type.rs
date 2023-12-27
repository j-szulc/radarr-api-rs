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
pub enum RejectionType {
    #[serde(rename = "permanent")]
    Permanent,
    #[serde(rename = "temporary")]
    Temporary,

}

impl ToString for RejectionType {
    fn to_string(&self) -> String {
        match self {
            Self::Permanent => String::from("permanent"),
            Self::Temporary => String::from("temporary"),
        }
    }
}

impl Default for RejectionType {
    fn default() -> RejectionType {
        Self::Permanent
    }
}




