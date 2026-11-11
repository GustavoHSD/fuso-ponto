use crate::models::{TimeEntry, DailySummary, Report};
use crate::repository::{TimeEntryRepository, Result};
use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use sqlx::{Row, FromRow};
use chrono::Utc;

pub struct SqliteTimeEntryRepository {
    pool: SqlitePool,
}

impl SqliteTimeEntryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

// Manually implement FromRow for TimeEntry because of Option and chrono
impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for TimeEntry {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(TimeEntry {
            id: row.try_get("id")?,
            start_time: row.try_get("start_time")?,
            end_time: row.try_get("end_time")?,
        })
    }
}

#[async_trait]
impl TimeEntryRepository for SqliteTimeEntryRepository {
    async fn start_session(&self) -> Result<TimeEntry> {
        let now = Utc::now();
        let id = sqlx::query("INSERT INTO time_entries (start_time) VALUES (?)")
            .bind(now)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?
            .last_insert_rowid();

        Ok(TimeEntry {
            id: Some(id),
            start_time: now,
            end_time: None,
        })
    }

    async fn stop_session(&self, id: i64) -> Result<TimeEntry> {
        let now = Utc::now();
        sqlx::query("UPDATE time_entries SET end_time = ? WHERE id = ?")
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let entry = sqlx::query_as::<_, TimeEntry>("SELECT id, start_time, end_time FROM time_entries WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(entry)
    }

    async fn get_active_session(&self) -> Result<Option<TimeEntry>> {
        let entry = sqlx::query_as::<_, TimeEntry>("SELECT id, start_time, end_time FROM time_entries WHERE end_time IS NULL ORDER BY start_time DESC LIMIT 1")
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(entry)
    }

    async fn get_today_summary(&self) -> Result<DailySummary> {
        let today = Utc::now().date_naive().to_string();
        let row = sqlx::query("SELECT SUM(strftime('%s', end_time) - strftime('%s', start_time)) as total_seconds FROM time_entries WHERE date(start_time) = date('now') AND end_time IS NOT NULL")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let total_seconds: i64 = row.try_get("total_seconds").unwrap_or(0);

        Ok(DailySummary {
            date: today,
            total_seconds,
        })
    }

    async fn get_all_reports(&self) -> Result<Vec<Report>> {
        let rows = sqlx::query("SELECT date(start_time) as date, SUM(strftime('%s', end_time) - strftime('%s', start_time)) as total_seconds FROM time_entries WHERE end_time IS NOT NULL GROUP BY date(start_time) ORDER BY date DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let reports = rows.into_iter().map(|row| {
            Report {
                date: row.get("date"),
                total_seconds: row.get("total_seconds"),
            }
        }).collect();

        Ok(reports)
    }

    async fn get_all_entries(&self) -> Result<Vec<TimeEntry>> {
        let entries = sqlx::query_as::<_, TimeEntry>("SELECT id, start_time, end_time FROM time_entries ORDER BY start_time DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(entries)
    }

    async fn get_today_entries(&self) -> Result<Vec<TimeEntry>> {
        let entries = sqlx::query_as::<_, TimeEntry>("SELECT id, start_time, end_time FROM time_entries WHERE date(start_time) = date('now') ORDER BY start_time DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(entries)
    }

    async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let row = sqlx::query("SELECT value FROM settings WHERE key = ?")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        match row {
            Some(row) => Ok(Some(row.try_get("value").map_err(|e| e.to_string())?)),
            None => Ok(None),
        }
    }

    async fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query("INSERT INTO settings (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value")
            .bind(key)
            .bind(value)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
