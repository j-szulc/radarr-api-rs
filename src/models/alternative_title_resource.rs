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
pub struct AlternativeTitleResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "sourceType", skip_serializing_if = "Option::is_none")]
    pub source_type: Option<crate::models::SourceType>,
    #[serde(rename = "movieMetadataId", skip_serializing_if = "Option::is_none")]
    pub movie_metadata_id: Option<i32>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "cleanTitle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub clean_title: Option<Option<String>>,
}

impl AlternativeTitleResource {
    pub fn new() -> AlternativeTitleResource {
        AlternativeTitleResource {
            id: None,
            source_type: None,
            movie_metadata_id: None,
            title: None,
            clean_title: None,
        }
    }
}

