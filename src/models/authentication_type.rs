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
pub enum AuthenticationType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "forms")]
    Forms,
    #[serde(rename = "external")]
    External,

}

impl ToString for AuthenticationType {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::Basic => String::from("basic"),
            Self::Forms => String::from("forms"),
            Self::External => String::from("external"),
        }
    }
}

impl Default for AuthenticationType {
    fn default() -> AuthenticationType {
        Self::None
    }
}



