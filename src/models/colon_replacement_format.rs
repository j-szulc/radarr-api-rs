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
pub enum ColonReplacementFormat {
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "dash")]
    Dash,
    #[serde(rename = "spaceDash")]
    SpaceDash,
    #[serde(rename = "spaceDashSpace")]
    SpaceDashSpace,

}

impl ToString for ColonReplacementFormat {
    fn to_string(&self) -> String {
        match self {
            Self::Delete => String::from("delete"),
            Self::Dash => String::from("dash"),
            Self::SpaceDash => String::from("spaceDash"),
            Self::SpaceDashSpace => String::from("spaceDashSpace"),
        }
    }
}

impl Default for ColonReplacementFormat {
    fn default() -> ColonReplacementFormat {
        Self::Delete
    }
}




