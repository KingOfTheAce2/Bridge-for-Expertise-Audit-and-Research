## Phase 3: Advanced Local AI Infrastructure (Priority: HIGH - Post-MVP)

**⚠️ NOTE: This phase is POST-MVP. MVP (Phase 1-2) already includes basic LLM via Ollama.**

**Objective**: Replace Ollama dependency with native Rust inference engine (Candle). Adds GPU optimization, quantization control, and eliminates external dependencies. This phase is about performance and polish, not core functionality.

**Why After MVP**:
- Phase 1 already delivers working AI (Ollama integration)
- Candle adds 8-12 weeks of development time
- MVP users can start using the product while this is built
- Advanced features benefit from real user feedback

---

### Step 20: Advanced Model Support with Candle and Hugging Face
**Priority**: High | **Effort**: Very High | **Legal Risk**: Low

**What**: Replace Ollama with native Rust inference using Candle. Gain full control over model loading, quantization, GPU acceleration. Remove Ollama as a dependency.

**Implementation**:
- Integrate Candle inference engine (Rust-native)
- Support Hugging Face model formats
- Implement manual model download interface
- Support multiple model sizes:
  - Small: 1-3B parameters (fast, lower quality)
  - Medium: 7-13B parameters (balanced)
  - Large: 30-70B parameters (slow, higher quality)
- Model quantization support (4-bit, 8-bit)
- GPU acceleration (CUDA, Metal, ROCm)
- Fallback to CPU inference

**Supported Models (Initial)**:
- Mistral 7B Instruct
- Llama 2 7B/13B
- Phi-2 (2.7B)
- TinyLlama 1.1B (fast inference)
- Legal-specific models (if available)

**Technical Details**:
- Add Candle dependencies to Cargo.toml:
```toml
candle-core = "0.3"
candle-nn = "0.3"
candle-transformers = "0.3"
hf-hub = "0.3"
tokenizers = "0.15"
```

- Implement inference engine:
  - Model loading from disk
  - Tokenization
  - Inference pipeline
  - Response streaming
  - Context management

- GPU acceleration:
  - CUDA support (NVIDIA)
  - Metal support (Apple Silicon)
  - ROCm support (AMD)
  - Automatic fallback to CPU

- Quantization support:
  - GGUF format (4-bit, 8-bit)
  - Reduced memory footprint
  - Faster inference on CPU

**Model Manager Features**:
- Browse available models
- Filter by size, language, specialization
- Download with progress tracking
- Verify checksums
- Delete unused models
- Configure active model per project

**Performance Targets**:
- 7B model: ~2-5 tokens/second on CPU
- 7B model: ~20-50 tokens/second on GPU
- Memory usage: <8GB for quantized 7B model
- Cold start: <30 seconds
- Warm start: <5 seconds

**Success Criteria**:
- Fully offline AI inference
- Support for at least 3 model sizes
- GPU acceleration functional on NVIDIA/Apple hardware
- Quantization reduces memory by 50%+
- User-friendly model management interface
- Response quality suitable for legal drafting assistance

**Rust Files (Phase 3 - Local AI with Candle)**:
```
src-tauri/src/
├── ai/
│   ├── mod.rs                           # AI module exports
│   ├── candle_engine.rs                 # Candle inference engine
│   ├── model_loader.rs                  # Model loading and caching
│   ├── tokenizer.rs                     # Tokenization
│   ├── inference.rs                     # Inference pipeline
│   ├── streaming.rs                     # Response streaming
│   ├── context.rs                       # Context management
│   └── gpu.rs                           # GPU acceleration (CUDA/Metal/ROCm)
├── models/
│   ├── mod.rs                           # Model management
│   ├── downloader.rs                    # Model download manager
│   ├── registry.rs                      # Model registry
│   ├── quantization.rs                  # GGUF quantization support
│   └── validator.rs                     # Checksum verification
├── services/llm/                        # Already exists in codebase
│   ├── mod.rs
│   ├── client.rs                        # LLM client interface
│   ├── chat.rs                          # Chat management
│   ├── models.rs                        # Model definitions
│   ├── types.rs                         # Type definitions
│   ├── utils.rs                         # Utility functions
│   └── providers/
│       ├── mod.rs
│       ├── types.rs
│       ├── candle.rs                    # Candle provider (NEW)
│       └── ollama/                      # Existing Ollama support
│           ├── mod.rs
│           ├── config.rs
│           ├── chat.rs
│           └── models.rs
└── commands/
    ├── ai.rs                            # AI commands
    ├── model.rs                         # Model management commands
    └── inference.rs                     # Inference commands

migration/src/
├── m20250109_000009_add_models.rs       # Model registry table
└── m20250110_000010_add_ai_settings.rs  # AI configuration

entity/src/
├── models.rs                            # Model entity
└── ai_sessions.rs                       # AI session tracking
```

---

