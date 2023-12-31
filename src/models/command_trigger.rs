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
pub enum CommandTrigger {
    #[serde(rename = "unspecified")]
    Unspecified,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "scheduled")]
    Scheduled,

}

impl ToString for CommandTrigger {
    fn to_string(&self) -> String {
        match self {
            Self::Unspecified => String::from("unspecified"),
            Self::Manual => String::from("manual"),
            Self::Scheduled => String::from("scheduled"),
        }
    }
}

impl Default for CommandTrigger {
    fn default() -> CommandTrigger {
        Self::Unspecified
    }
}




