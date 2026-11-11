use tauri::State;
use crate::models::{TimeEntry, DailySummary, Report};
use crate::repository::TimeEntryRepository;
use crate::sqlite_repository::SqliteTimeEntryRepository;
use paid_core::PaidFile;

pub const PAID_PUBLIC_KEY: &str = include_str!("../paid_public_key.txt");
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
pub async fn get_today_entries(repo: State<'_, AppRepo>) -> Result<Vec<TimeEntry>, String> {
    repo.get_today_entries().await
}

async fn ensure_device_id(repo: &AppRepo) -> Result<String, String> {
    if let Some(id) = repo.get_setting("device_id").await? {
        return Ok(id);
    }
    let id = uuid::Uuid::new_v4().to_string();
    repo.set_setting("device_id", &id).await?;
    Ok(id)
}

#[tauri::command]
pub async fn get_device_id(repo: State<'_, AppRepo>) -> Result<String, String> {
    ensure_device_id(&repo).await
}

#[tauri::command]
pub async fn get_paid_up_to(repo: State<'_, AppRepo>) -> Result<Option<String>, String> {
    repo.get_setting("paid_up_to").await
}

#[tauri::command]
pub async fn import_paid_file(repo: State<'_, AppRepo>, path: String) -> Result<String, String> {
    let content = std::fs::read_to_string(&path)
        .map_err(|_| "Não foi possível ler o arquivo".to_string())?;
    let file: PaidFile =
        serde_json::from_str(&content).map_err(|_| "Arquivo inválido".to_string())?;

    paid_core::verify(PAID_PUBLIC_KEY.trim(), &file)?;

    let device_id = ensure_device_id(&repo).await?;
    if file.device != device_id {
        return Err("Este comprovante é para outro dispositivo".to_string());
    }

    // An older file can never roll the status back
    let current = repo.get_setting("paid_up_to").await?.unwrap_or_default();
    let new_value = if file.paid_up_to > current {
        file.paid_up_to.clone()
    } else {
        current
    };
    repo.set_setting("paid_up_to", &new_value).await?;

    Ok(new_value)
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
