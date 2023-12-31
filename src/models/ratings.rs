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
pub struct Ratings {
    #[serde(rename = "imdb", skip_serializing_if = "Option::is_none")]
    pub imdb: Option<Box<crate::models::RatingChild>>,
    #[serde(rename = "tmdb", skip_serializing_if = "Option::is_none")]
    pub tmdb: Option<Box<crate::models::RatingChild>>,
    #[serde(rename = "metacritic", skip_serializing_if = "Option::is_none")]
    pub metacritic: Option<Box<crate::models::RatingChild>>,
    #[serde(rename = "rottenTomatoes", skip_serializing_if = "Option::is_none")]
    pub rotten_tomatoes: Option<Box<crate::models::RatingChild>>,
}

impl Ratings {
    pub fn new() -> Ratings {
        Ratings {
            imdb: None,
            tmdb: None,
            metacritic: None,
            rotten_tomatoes: None,
        }
    }
}


