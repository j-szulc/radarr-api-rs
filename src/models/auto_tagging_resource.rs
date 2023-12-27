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
pub struct AutoTaggingResource {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "removeTagsAutomatically", skip_serializing_if = "Option::is_none")]
    pub remove_tags_automatically: Option<bool>,
    #[serde(rename = "tags", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Option<Vec<i32>>>,
    #[serde(rename = "specifications", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub specifications: Option<Option<Vec<crate::models::AutoTaggingSpecificationSchema>>>,
}

impl AutoTaggingResource {
    pub fn new() -> AutoTaggingResource {
        AutoTaggingResource {
            id: None,
            name: None,
            remove_tags_automatically: None,
            tags: None,
            specifications: None,
        }
    }
}


