use crate::models::{TimeEntry, DailySummary, Report};
use async_trait::async_trait;

pub type Result<T> = std::result::Result<T, String>;

#[async_trait]
pub trait TimeEntryRepository: Send + Sync {
    async fn start_session(&self) -> Result<TimeEntry>;
    async fn stop_session(&self, id: i64) -> Result<TimeEntry>;
    async fn get_active_session(&self) -> Result<Option<TimeEntry>>;
    async fn get_today_summary(&self) -> Result<DailySummary>;
    async fn get_all_reports(&self) -> Result<Vec<Report>>;
    async fn get_all_entries(&self) -> Result<Vec<TimeEntry>>;
    async fn get_today_entries(&self) -> Result<Vec<TimeEntry>>;
    async fn get_setting(&self, key: &str) -> Result<Option<String>>;
    async fn set_setting(&self, key: &str, value: &str) -> Result<()>;
}
