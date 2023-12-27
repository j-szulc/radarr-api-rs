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
pub enum Modifier {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "regional")]
    Regional,
    #[serde(rename = "screener")]
    Screener,
    #[serde(rename = "rawhd")]
    Rawhd,
    #[serde(rename = "brdisk")]
    Brdisk,
    #[serde(rename = "remux")]
    Remux,

}

impl ToString for Modifier {
    fn to_string(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::Regional => String::from("regional"),
            Self::Screener => String::from("screener"),
            Self::Rawhd => String::from("rawhd"),
            Self::Brdisk => String::from("brdisk"),
            Self::Remux => String::from("remux"),
        }
    }
}

impl Default for Modifier {
    fn default() -> Modifier {
        Self::None
    }
}




