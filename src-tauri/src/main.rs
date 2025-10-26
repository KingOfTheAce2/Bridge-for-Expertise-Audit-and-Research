mod database;
mod commands;

use tauri::Manager;

#[tokio::main]
async fn main() {
    let db_manager = database::DatabaseManager::new();

    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path_resolver()
                .app_data_dir()
                .expect("Failed to get app data directory");

            std::fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("bear_llm.db");

            tauri::async_runtime::block_on(async {
                db_manager.initialize(db_path.to_str().unwrap())
                    .await
                    .expect("Failed to initialize database");
            });

            app.manage(db_manager);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_app_version,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
