mod commands;
mod database;
mod models;
mod pii;
mod ner;
mod services;

use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let db_manager = database::DatabaseManager::new();
    let download_state: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let anonymizer: Arc<Mutex<pii::Anonymizer>> = Arc::new(Mutex::new(pii::Anonymizer::new()));

    // NER state
    let ner_manager: Arc<Mutex<Option<ner::NerModelManager>>> = Arc::new(Mutex::new(None));
    let hybrid_detector: Arc<Mutex<Option<ner::HybridDetector>>> = Arc::new(Mutex::new(None));

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
            app.manage(download_state);
            app.manage(anonymizer);
            app.manage(ner_manager);
            app.manage(hybrid_detector);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Settings commands
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_app_version,
            // Model management commands
            commands::models::list_models,
            commands::models::download_model,
            commands::models::delete_model,
            commands::models::set_active_model,
            commands::models::get_active_model,
            commands::models::cancel_download,
            commands::models::add_custom_model,
            commands::models::check_disk_space,
            commands::models::import_model_file,
            // PII detection and anonymization commands (Phase 4)
            commands::pii::anonymize_text,
            commands::pii::anonymize_batch,
            commands::pii::clear_pii_replacements,
            commands::pii::get_pii_statistics,
            commands::pii::get_default_pii_settings,
            commands::pii::get_entity_types,
            commands::pii::detect_pii_entities,
            // NER model management and inference commands
            commands::ner::list_ner_models,
            commands::ner::download_ner_model,
            commands::ner::delete_ner_model,
            commands::ner::load_ner_model,
            commands::ner::run_ner_inference,
            commands::ner::get_ner_recommendations,
            commands::ner::cancel_ner_download,
            commands::ner::get_ner_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
