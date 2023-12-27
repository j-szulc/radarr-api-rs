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
pub enum DatabaseType {
    #[serde(rename = "sqLite")]
    SqLite,
    #[serde(rename = "postgreSQL")]
    PostgreSql,

}

impl ToString for DatabaseType {
    fn to_string(&self) -> String {
        match self {
            Self::SqLite => String::from("sqLite"),
            Self::PostgreSql => String::from("postgreSQL"),
        }
    }
}

impl Default for DatabaseType {
    fn default() -> DatabaseType {
        Self::SqLite
    }
}



