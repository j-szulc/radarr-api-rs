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
pub struct TimeSpan {
    #[serde(rename = "ticks", skip_serializing_if = "Option::is_none")]
    pub ticks: Option<i64>,
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "hours", skip_serializing_if = "Option::is_none")]
    pub hours: Option<i32>,
    #[serde(rename = "milliseconds", skip_serializing_if = "Option::is_none")]
    pub milliseconds: Option<i32>,
    #[serde(rename = "minutes", skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i32>,
    #[serde(rename = "seconds", skip_serializing_if = "Option::is_none")]
    pub seconds: Option<i32>,
    #[serde(rename = "totalDays", skip_serializing_if = "Option::is_none")]
    pub total_days: Option<f64>,
    #[serde(rename = "totalHours", skip_serializing_if = "Option::is_none")]
    pub total_hours: Option<f64>,
    #[serde(rename = "totalMilliseconds", skip_serializing_if = "Option::is_none")]
    pub total_milliseconds: Option<f64>,
    #[serde(rename = "totalMinutes", skip_serializing_if = "Option::is_none")]
    pub total_minutes: Option<f64>,
    #[serde(rename = "totalSeconds", skip_serializing_if = "Option::is_none")]
    pub total_seconds: Option<f64>,
}

impl TimeSpan {
    pub fn new() -> TimeSpan {
        TimeSpan {
            ticks: None,
            days: None,
            hours: None,
            milliseconds: None,
            minutes: None,
            seconds: None,
            total_days: None,
            total_hours: None,
            total_milliseconds: None,
            total_minutes: None,
            total_seconds: None,
        }
    }
}

