// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{DateTime, Utc};
#[tauri::command]
fn countdown() -> String {
    let now: DateTime<Utc> = Utc::now();
    let target_date = "2024-07-05T13:00:00Z".parse::<DateTime<Utc>>().unwrap();
    if now < target_date {
        let duration = target_date - now;
        let days = duration.num_days();
        let hours = duration.num_hours() % 24;
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;
        format!(
            "{} days {} hours {} minutes {} seconds remaining...",
            days, hours, minutes, seconds
        )
    } else {
        "The event has started!".to_string()
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![countdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
