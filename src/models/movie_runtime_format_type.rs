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
pub enum MovieRuntimeFormatType {
    #[serde(rename = "hoursMinutes")]
    HoursMinutes,
    #[serde(rename = "minutes")]
    Minutes,

}

impl ToString for MovieRuntimeFormatType {
    fn to_string(&self) -> String {
        match self {
            Self::HoursMinutes => String::from("hoursMinutes"),
            Self::Minutes => String::from("minutes"),
        }
    }
}

impl Default for MovieRuntimeFormatType {
    fn default() -> MovieRuntimeFormatType {
        Self::HoursMinutes
    }
}




