use crate::ai::{
    ChatMessage, GenerateRequest, GenerationConfig, GenerationResult, InferenceEngine,
    ModelConfig, ModelStatus, TokenResponse,
};
use crate::database::DatabaseManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

/// Request to load AI model for inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadModelRequest {
    pub model_id: String,
}

/// AI generation request from frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateTextRequest {
    pub conversation_id: Option<i32>,
    pub messages: Vec<ChatMessage>,
    pub system_prompt: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<usize>,
}

/// Load AI model for inference
#[tauri::command]
pub async fn load_ai_model(
    request: LoadModelRequest,
    inference_engine: State<'_, Arc<Mutex<InferenceEngine>>>,
) -> Result<String, String> {
    let engine = inference_engine.lock().await;

    // Get model path from data directory
    let app_dir = dirs::data_dir()
        .ok_or("Failed to get data directory")?
        .join("bear-llm-ai")
        .join("models")
        .join(request.model_id.replace('/', "_"));

    if !app_dir.exists() {
        return Err(format!("Model not found: {}", request.model_id));
    }

    // Create model config (simplified - would load from config.json)
    let config = ModelConfig::default();

    engine
        .load_model(app_dir, config)
        .await
        .map_err(|e| format!("Failed to load model: {}", e))?;

    Ok(format!("Model loaded: {}", request.model_id))
}

/// Unload current AI model
#[tauri::command]
pub async fn unload_ai_model(
    inference_engine: State<'_, Arc<Mutex<InferenceEngine>>>,
) -> Result<String, String> {
    let engine = inference_engine.lock().await;
    engine.unload_model().await;
    Ok("Model unloaded".to_string())
}

/// Get AI model status
#[tauri::command]
pub async fn get_ai_model_status(
    inference_engine: State<'_, Arc<Mutex<InferenceEngine>>>,
) -> Result<String, String> {
    let engine = inference_engine.lock().await;
    let status = engine.get_status().await;

    let status_str = match status {
        ModelStatus::NotLoaded => "not_loaded",
        ModelStatus::Loading => "loading",
        ModelStatus::Loaded => "loaded",
        ModelStatus::Error(_) => "error",
    };

    Ok(status_str.to_string())
}

/// Generate AI response
#[tauri::command]
pub async fn generate_ai_response(
    request: GenerateTextRequest,
    inference_engine: State<'_, Arc<Mutex<InferenceEngine>>>,
    db: State<'_, DatabaseManager>,
) -> Result<GenerationResult, String> {
    let engine = inference_engine.lock().await;

    // Check if model is loaded
    if !engine.is_loaded().await {
        return Err("No AI model loaded. Please load a model first.".to_string());
    }

    // Build generation config
    let mut config = GenerationConfig::default();
    if let Some(temp) = request.temperature {
        config.temperature = temp;
    }
    if let Some(max) = request.max_tokens {
        config.max_new_tokens = max;
    }

    // Create generation request
    let gen_request = GenerateRequest {
        messages: request.messages.clone(),
        config,
        system_prompt: request.system_prompt.clone(),
    };

    // Generate response
    let result = engine
        .generate(gen_request)
        .await
        .map_err(|e| format!("Generation failed: {}", e))?;

    // TODO: Store conversation in database if conversation_id is provided

    Ok(result)
}

/// Generate AI response with streaming
#[tauri::command]
pub async fn generate_ai_response_stream(
    request: GenerateTextRequest,
    inference_engine: State<'_, Arc<Mutex<InferenceEngine>>>,
    window: tauri::Window,
) -> Result<String, String> {
    let engine = inference_engine.lock().await;

    // Check if model is loaded
    if !engine.is_loaded().await {
        return Err("No AI model loaded. Please load a model first.".to_string());
    }

    // Build generation config
    let mut config = GenerationConfig::default();
    if let Some(temp) = request.temperature {
        config.temperature = temp;
    }
    if let Some(max) = request.max_tokens {
        config.max_new_tokens = max;
    }

    // Create generation request
    let gen_request = GenerateRequest {
        messages: request.messages.clone(),
        config,
        system_prompt: request.system_prompt.clone(),
    };

    // Generate with streaming
    let conversation_id = request.conversation_id;
    let result = engine
        .generate_stream(gen_request, move |token_response| {
            // Emit token to frontend
            let _ = window.emit(
                "ai-token",
                serde_json::json!({
                    "conversation_id": conversation_id,
                    "token": token_response.token,
                    "is_final": token_response.is_final,
                    "total_tokens": token_response.total_tokens,
                }),
            );
        })
        .await
        .map_err(|e| format!("Generation failed: {}", e))?;

    Ok(result.text)
}

/// Get available system prompts
#[tauri::command]
pub async fn get_system_prompts() -> Result<Vec<SystemPrompt>, String> {
    Ok(vec![
        SystemPrompt {
            id: "assistant".to_string(),
            name: "General Assistant".to_string(),
            prompt: "You are a helpful, respectful, and honest assistant. Always answer as helpfully as possible, while being safe. If you don't know the answer to a question, please don't share false information.".to_string(),
        },
        SystemPrompt {
            id: "legal".to_string(),
            name: "Legal Assistant".to_string(),
            prompt: "You are a legal assistant helping with document drafting and analysis. Provide accurate legal information but always remind users that this is not legal advice and they should consult with a qualified attorney.".to_string(),
        },
        SystemPrompt {
            id: "formal".to_string(),
            name: "Formal Writer".to_string(),
            prompt: "You are a professional writing assistant specializing in formal and business communication. Use clear, concise, and professional language.".to_string(),
        },
        SystemPrompt {
            id: "summarizer".to_string(),
            name: "Document Summarizer".to_string(),
            prompt: "You are a document summarization assistant. Extract key points, main arguments, and important details from documents. Present information in a clear, structured format.".to_string(),
        },
    ])
}

/// Get conversation history
#[tauri::command]
pub async fn get_conversation_history(
    conversation_id: i32,
    db: State<'_, DatabaseManager>,
) -> Result<Vec<ChatMessage>, String> {
    // TODO: Implement database query to get conversation messages
    // For now, return empty
    Ok(vec![])
}

/// Create new conversation
#[tauri::command]
pub async fn create_conversation(
    title: Option<String>,
    db: State<'_, DatabaseManager>,
) -> Result<i32, String> {
    // TODO: Implement database insert for new conversation
    // For now, return placeholder ID
    Ok(1)
}

/// Delete conversation
#[tauri::command]
pub async fn delete_conversation(
    conversation_id: i32,
    db: State<'_, DatabaseManager>,
) -> Result<String, String> {
    // TODO: Implement database delete
    Ok(format!("Conversation {} deleted", conversation_id))
}

/// System prompt template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPrompt {
    pub id: String,
    pub name: String,
    pub prompt: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompts() {
        let prompts = tokio_test::block_on(get_system_prompts()).unwrap();
        assert!(prompts.len() >= 4);
        assert!(prompts.iter().any(|p| p.id == "assistant"));
        assert!(prompts.iter().any(|p| p.id == "legal"));
    }
}
