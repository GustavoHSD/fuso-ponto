mod models;
mod repository;
mod sqlite_repository;
mod commands;

use tauri::Manager;
use sqlx::sqlite::SqlitePoolOptions;
use std::sync::Arc;
use crate::sqlite_repository::SqliteTimeEntryRepository;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Resolve the database path
            let app_data_dir = app_handle.path().app_data_dir().expect("failed to get app data dir");
            println!("App data dir: {:?}", app_data_dir);
            
            if !app_data_dir.exists() {
                println!("Creating app data dir...");
                std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
            }
            
            let db_path = app_data_dir.join("ponto.db");
            // Use sqlite:// protocol, ensure absolute path, and add mode=rwc
            let db_url = format!("sqlite:{}?mode=rwc", db_path.to_string_lossy());
            println!("Connecting to database at: {}", db_url);

            // Initialize the pool
            tauri::async_runtime::block_on(async move {
                let pool = SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect(&db_url)
                    .await
                    .expect("failed to connect to database");

                // Basic migration since we removed the plugin's migration
                sqlx::query("CREATE TABLE IF NOT EXISTS time_entries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    start_time DATETIME NOT NULL,
                    end_time DATETIME
                );")
                .execute(&pool)
                .await
                .expect("failed to run initial migration");

                let repo = Arc::new(SqliteTimeEntryRepository::new(pool));
                app_handle.manage(repo);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_timer,
            commands::stop_timer,
            commands::get_active_session,
            commands::get_today_summary,
            commands::get_reports,
            commands::export_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
