/*
 * Radarr
 *
 * Radarr API docs
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddMovieOptions {
    #[serde(rename = "ignoreEpisodesWithFiles", skip_serializing_if = "Option::is_none")]
    pub ignore_episodes_with_files: Option<bool>,
    #[serde(rename = "ignoreEpisodesWithoutFiles", skip_serializing_if = "Option::is_none")]
    pub ignore_episodes_without_files: Option<bool>,
    #[serde(rename = "monitor", skip_serializing_if = "Option::is_none")]
    pub monitor: Option<crate::models::MonitorTypes>,
    #[serde(rename = "searchForMovie", skip_serializing_if = "Option::is_none")]
    pub search_for_movie: Option<bool>,
    #[serde(rename = "addMethod", skip_serializing_if = "Option::is_none")]
    pub add_method: Option<crate::models::AddMovieMethod>,
}

impl AddMovieOptions {
    pub fn new() -> AddMovieOptions {
        AddMovieOptions {
            ignore_episodes_with_files: None,
            ignore_episodes_without_files: None,
            monitor: None,
            search_for_movie: None,
            add_method: None,
        }
    }
}


