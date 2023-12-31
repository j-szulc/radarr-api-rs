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
pub struct ManualImportReprocessResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "path", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub path: Option<Option<String>>,
    #[serde(rename = "movieId", skip_serializing_if = "Option::is_none")]
    pub movie_id: Option<i32>,
    #[serde(rename = "movie", skip_serializing_if = "Option::is_none")]
    pub movie: Option<Box<crate::models::MovieResource>>,
    #[serde(rename = "quality", skip_serializing_if = "Option::is_none")]
    pub quality: Option<Box<crate::models::QualityModel>>,
    #[serde(rename = "languages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub languages: Option<Option<Vec<crate::models::Language>>>,
    #[serde(rename = "releaseGroup", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub release_group: Option<Option<String>>,
    #[serde(rename = "downloadId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub download_id: Option<Option<String>>,
    #[serde(rename = "customFormats", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub custom_formats: Option<Option<Vec<crate::models::CustomFormatResource>>>,
    #[serde(rename = "customFormatScore", skip_serializing_if = "Option::is_none")]
    pub custom_format_score: Option<i32>,
    #[serde(rename = "rejections", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rejections: Option<Option<Vec<crate::models::Rejection>>>,
}

impl ManualImportReprocessResource {
    pub fn new() -> ManualImportReprocessResource {
        ManualImportReprocessResource {
            id: None,
            path: None,
            movie_id: None,
            movie: None,
            quality: None,
            languages: None,
            release_group: None,
            download_id: None,
            custom_formats: None,
            custom_format_score: None,
            rejections: None,
        }
    }
}


