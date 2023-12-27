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
pub enum MovieStatusType {
    #[serde(rename = "tba")]
    Tba,
    #[serde(rename = "announced")]
    Announced,
    #[serde(rename = "inCinemas")]
    InCinemas,
    #[serde(rename = "released")]
    Released,
    #[serde(rename = "deleted")]
    Deleted,

}

impl ToString for MovieStatusType {
    fn to_string(&self) -> String {
        match self {
            Self::Tba => String::from("tba"),
            Self::Announced => String::from("announced"),
            Self::InCinemas => String::from("inCinemas"),
            Self::Released => String::from("released"),
            Self::Deleted => String::from("deleted"),
        }
    }
}

impl Default for MovieStatusType {
    fn default() -> MovieStatusType {
        Self::Tba
    }
}



