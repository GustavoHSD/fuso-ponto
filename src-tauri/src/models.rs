use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeEntry {
    pub id: Option<i64>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailySummary {
    pub date: String,
    pub total_seconds: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub date: String,
    pub total_seconds: i64,
}
