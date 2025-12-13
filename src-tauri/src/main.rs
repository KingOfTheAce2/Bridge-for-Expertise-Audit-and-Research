mod commands;
mod database;
mod models;
mod pii;
mod ner;
mod ai;
mod services;
mod prompts;
mod templates;

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

    // AI inference state (Phase 3)
    let inference_engine: Arc<Mutex<ai::InferenceEngine>> = Arc::new(Mutex::new(ai::InferenceEngine::new()));

    // Presidio state (Phase 5 - Layer 3 PII)
    let presidio_manager: Arc<Mutex<pii::PresidioManager>> = Arc::new(Mutex::new(pii::PresidioManager::new()));

    // Prompt library state (Phase 5)
    let base_dir = dirs::data_dir()
        .expect("Failed to get data directory")
        .join("bear-llm-ai");

    let prompt_library = Arc::new(Mutex::new(
        prompts::PromptLibrary::new(base_dir.clone())
            .expect("Failed to initialize prompt library")
    ));

    let template_library = Arc::new(Mutex::new(
        templates::TemplateLibrary::new(base_dir.clone())
            .expect("Failed to initialize template library")
    ));

    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            std::fs::create_dir_all(&app_dir)?;
            let db_path = app_dir.join("bear_llm.db");

            // Clone for async block
            let db_manager_clone = db_manager.clone();
            let prompt_library_clone = prompt_library.clone();
            let db_path_str = db_path.to_str().unwrap().to_string();

            // Use spawn to avoid blocking the runtime
            tauri::async_runtime::spawn(async move {
                db_manager_clone.initialize(&db_path_str)
                    .await
                    .expect("Failed to initialize database");

                // Initialize prompt library with built-in prompts
                let lib = prompt_library_clone.lock().await;
                lib.initialize()
                    .expect("Failed to initialize prompt library with built-in prompts");
            });

            app.manage(db_manager);
            app.manage(download_state);
            app.manage(anonymizer);
            app.manage(ner_manager);
            app.manage(hybrid_detector);
            app.manage(inference_engine);
            app.manage(presidio_manager);
            app.manage(prompt_library);
            app.manage(template_library);
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
            commands::ner::get_ner_recommendations_for_language,
            commands::ner::get_ner_models_by_use_case,
            commands::ner::cancel_ner_download,
            commands::ner::get_ner_status,
            // AI conversation and inference commands (Phase 3)
            commands::conversation::load_ai_model,
            commands::conversation::unload_ai_model,
            commands::conversation::get_ai_model_status,
            commands::conversation::get_device_info,
            commands::conversation::generate_ai_response,
            commands::conversation::generate_ai_response_stream,
            commands::conversation::get_system_prompts,
            commands::conversation::get_conversation_history,
            commands::conversation::create_conversation,
            commands::conversation::delete_conversation,
            // Prompt library commands (Phase 5)
            commands::prompts::get_all_prompts,
            commands::prompts::get_prompt_by_id,
            commands::prompts::search_prompts,
            commands::prompts::get_prompts_by_category,
            commands::prompts::get_prompts_by_tag,
            commands::prompts::get_prompts_by_tier,
            commands::prompts::get_prompt_categories,
            commands::prompts::get_prompt_tags,
            commands::prompts::save_prompt,
            commands::prompts::delete_prompt,
            commands::prompts::import_prompt_file,
            commands::prompts::apply_prompt_variables,
            // Template library commands (Phase 5)
            commands::templates::get_all_templates,
            commands::templates::get_template_by_id,
            commands::templates::get_templates_by_category,
            commands::templates::save_template,
            commands::templates::delete_template,
            commands::templates::import_template_file,
            commands::templates::render_template,
            commands::templates::validate_template_syntax,
            // Presidio commands (Phase 5 - Layer 3 PII)
            commands::presidio::get_presidio_status,
            commands::presidio::is_docker_available,
            commands::presidio::install_presidio,
            commands::presidio::start_presidio,
            commands::presidio::stop_presidio,
            commands::presidio::enable_presidio,
            commands::presidio::disable_presidio,
            commands::presidio::presidio_analyze,
            commands::presidio::presidio_anonymize,
            commands::presidio::get_presidio_entity_types,
            commands::presidio::get_presidio_languages,
            commands::presidio::get_presidio_config,
            commands::presidio::is_presidio_enabled,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
