import { invoke } from "@tauri-apps/api/core";

export interface TimeEntry {
  id: number | null;
  start_time: string;
  end_time: string | null;
}

export interface DailySummary {
  date: string;
  total_seconds: number;
}

export interface Report {
  date: string;
  total_seconds: number;
}

export const api = {
  startTimer: () => invoke<TimeEntry>("start_timer"),
  stopTimer: (id: number) => invoke<TimeEntry>("stop_timer", { id }),
  getActiveSession: () => invoke<TimeEntry | null>("get_active_session"),
  getTodaySummary: () => invoke<DailySummary>("get_today_summary"),
  getReports: () => invoke<Report[]>("get_reports"),
  exportData: () => invoke<string>("export_data"),
};
