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
pub enum DownloadProtocol {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "usenet")]
    Usenet,
    #[serde(rename = "torrent")]
    Torrent,

}

impl ToString for DownloadProtocol {
    fn to_string(&self) -> String {
        match self {
            Self::Unknown => String::from("unknown"),
            Self::Usenet => String::from("usenet"),
            Self::Torrent => String::from("torrent"),
        }
    }
}

impl Default for DownloadProtocol {
    fn default() -> DownloadProtocol {
        Self::Unknown
    }
}




