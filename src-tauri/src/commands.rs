use tauri::State;
use crate::models::{TimeEntry, DailySummary, Report};
use crate::repository::TimeEntryRepository;
use crate::sqlite_repository::SqliteTimeEntryRepository;
use rust_xlsxwriter::Workbook;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Manager;

pub type AppRepo = Arc<SqliteTimeEntryRepository>;

#[tauri::command]
pub async fn start_timer(repo: State<'_, AppRepo>) -> Result<TimeEntry, String> {
    repo.start_session().await
}

#[tauri::command]
pub async fn stop_timer(repo: State<'_, AppRepo>, id: i64) -> Result<TimeEntry, String> {
    repo.stop_session(id).await
}

#[tauri::command]
pub async fn get_active_session(repo: State<'_, AppRepo>) -> Result<Option<TimeEntry>, String> {
    repo.get_active_session().await
}

#[tauri::command]
pub async fn get_today_summary(repo: State<'_, AppRepo>) -> Result<DailySummary, String> {
    repo.get_today_summary().await
}

#[tauri::command]
pub async fn get_reports(repo: State<'_, AppRepo>) -> Result<Vec<Report>, String> {
    repo.get_all_reports().await
}

#[tauri::command]
pub async fn export_data(app: AppHandle, repo: State<'_, AppRepo>) -> Result<String, String> {
    let entries = repo.get_all_entries().await?;
    
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "ID").map_err(|e| e.to_string())?;
    worksheet.write(0, 1, "Start Time").map_err(|e| e.to_string())?;
    worksheet.write(0, 2, "End Time").map_err(|e| e.to_string())?;
    worksheet.write(0, 3, "Duration (seconds)").map_err(|e| e.to_string())?;

    for (i, entry) in entries.iter().enumerate() {
        let row = (i + 1) as u32;
        worksheet.write(row, 0, entry.id.unwrap_or(0)).map_err(|e| e.to_string())?;
        worksheet.write(row, 1, entry.start_time.to_rfc3339()).map_err(|e| e.to_string())?;
        if let Some(end) = entry.end_time {
            worksheet.write(row, 2, end.to_rfc3339()).map_err(|e| e.to_string())?;
            let duration = end.signed_duration_since(entry.start_time).num_seconds();
            worksheet.write(row, 3, duration).map_err(|e| e.to_string())?;
        }
    }

    let docs_dir = app.path().document_dir().map_err(|e| e.to_string())?;
    let file_path = docs_dir.join("ponto_export.xlsx");
    let file_path_str = file_path.to_string_lossy().to_string();

    workbook.save(&file_path).map_err(|e| e.to_string())?;

    Ok(file_path_str)
}
