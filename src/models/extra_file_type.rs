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
pub enum ExtraFileType {
    #[serde(rename = "subtitle")]
    Subtitle,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "other")]
    Other,

}

impl ToString for ExtraFileType {
    fn to_string(&self) -> String {
        match self {
            Self::Subtitle => String::from("subtitle"),
            Self::Metadata => String::from("metadata"),
            Self::Other => String::from("other"),
        }
    }
}

impl Default for ExtraFileType {
    fn default() -> ExtraFileType {
        Self::Subtitle
    }
}




