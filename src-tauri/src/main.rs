use tauri::Manager;
mod database;

#[tokio::main]
async fn main() {
    env_logger::init();

    let db_manager = database::DatabaseManager::new();

    tauri::Builder::default()
        .setup(|app| {
            // Determine app data directory
            let app_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("Failed to get app data directory");

            std::fs::create_dir_all(&app_dir)?;

            let db_path = app_dir.join("bear_llm.db");

            // Initialize database and run migrations
            tauri::async_runtime::block_on(async {
                db_manager
                    .initialize(db_path.to_str().unwrap())
                    .await
                    .expect("Failed to initialize database");
            });

            app.manage(db_manager);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
