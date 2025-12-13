## Phase 1: GDPR Compliance + Auto-Updater + Basic LLM (Priority: CRITICAL)
**Legal Foundation + Infrastructure + AI Integration**

**Objective**: Build GDPR-compliant infrastructure while integrating basic LLM capabilities (Ollama). This phase delivers a working AI assistant that can download and use models while respecting privacy laws.

**Key Deliverables**:
- ✅ GDPR Articles 5, 12-17, 25, 30, 32 compliance
- ✅ Privacy-respecting auto-updater system (moved from Step 0.6a)
- ✅ Basic LLM integration (Ollama API)
- ✅ Model download and management
- ✅ Functional chat interface with AI responses

**Timeline**: 8-10 weeks

**Note**: Auto-updater implementation details are in Step 0.6a (lines ~1070-1356). Basic LLM integration added as new Step 1.1 below.

---

### Step 1.0: Auto-Updater System - MOVED TO PHASE 1
**Priority**: High | **Effort**: Low | **Risk**: Low

**Implementation**: See **Step 0.6a** above (lines ~1070-1356) for complete implementation details including:
- Tauri updater configuration
- Rust update check/install commands
- React UI notification component
- Settings integration
- CI/CD GitHub Actions workflow
- Cryptographic signing setup
- Privacy guarantees and documentation

**Why in Phase 1**: Auto-updater is infrastructure needed before MVP launch. Allows pushing security patches and bug fixes while maintaining privacy-first principles.

**Success Criteria**: See Step 0.6a above.

---

### Step 1.1: Basic LLM Integration - Ollama API
**Priority**: Critical | **Effort**: Medium (1-2 weeks) | **Risk**: Medium

**What**: Integrate with Ollama for local LLM inference. Users can download models via Ollama and chat with them through BEAR LLM. This is the "Basic AI" that makes the MVP actually functional.

**Why Ollama for MVP**:
| Factor | Ollama (Phase 1) | Candle (Phase 3) |
|--------|------------------|------------------|
| **Development Time** | 1-2 weeks | 8-12 weeks |
| **Complexity** | Low (REST API) | Very High (Rust ML) |
| **Model Management** | Ollama handles it | Build ourselves |
| **GPU Support** | Ollama manages | Manual CUDA/Metal |
| **User Setup** | Install Ollama | Built-in |
| **MVP Ready?** | ✅ YES | ❌ Overkill for MVP |

**Implementation Summary**:
1. **Ollama Client Service** (`src-tauri/src/services/ollama.rs`):
   - Check if Ollama is running (HTTP GET to localhost:11434)
   - List available models
   - Send chat messages
   - Handle streaming responses

2. **Tauri Commands** (`src-tauri/src/commands/llm.rs`):
   - `check_ollama_status() -> bool`
   - `get_available_models() -> Vec<String>`
   - `send_chat_message(model, messages) -> String`

3. **Frontend Service** (`src/services/llm.ts`):
   - TypeScript wrapper for Tauri commands
   - Type-safe message interface

4. **Chat UI** (`src/components/ChatInterface.tsx`):
   - Model selector dropdown
   - Message history display
   - Input field with send button
   - "Ollama not installed" warning state
   - Loading indicators

**Dependencies**:
```toml
# src-tauri/Cargo.toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**User Workflow**:
1. Install BEAR LLM → App detects no Ollama
2. Install Ollama → `curl https://ollama.ai/install.sh | sh`
3. Download model → `ollama pull llama2`
4. Return to BEAR LLM → Model appears in dropdown
5. Start chatting → Fully local AI responses

**Success Criteria**:
- ✅ Detects Ollama running/not running
- ✅ Lists all models from Ollama
- ✅ Sends messages and displays responses
- ✅ Conversation history persists in database
- ✅ Model switching works mid-conversation
- ✅ Graceful error handling
- ✅ 100% local processing (verify with network monitor)

**What This Delivers**: A working AI assistant! Users can chat with local LLMs through a clean interface. This is the core functionality needed for MVP.

**Migration to Candle (Phase 3)**:
- Phase 1: Ollama required, simple integration
- Phase 3: Add Candle, make Ollama optional
- Phase 4+: Candle default, Ollama fallback for compatibility

---

### Step 1.2: Data Minimization (Art. 5(1)(c))
**Priority**: Critical | **Effort**: Medium | **Legal Risk**: High

**What**: Store only files or chats the user explicitly creates or imports.

**Implementation**:
- Audit all data collection points in the application
- Remove any automatic logging or telemetry
- Implement explicit user consent for all data storage
- Automatically delete temporary files after use
- Delete cache files on application close
- Avoid storing unnecessary intermediate processing data

**Technical Details**:
- Review `src-tauri/src/` for any automatic data collection
- Implement temporary file cleanup in Rust backend
- Add configuration for cache management
- Create data lifecycle management system

**Success Criteria**:
- Zero automatic data collection without user action
- Temporary files cleaned up within 5 minutes or on app close
- All stored data directly linked to user-initiated actions

---

### Step 2: Purpose Limitation (Art. 5(1)(b))
**Priority**: Critical | **Effort**: Medium | **Legal Risk**: High

**What**: Keep user data organized by project or case. Prevent automatic mixing of unrelated files or contexts.

**Implementation**:
- Design project-based data organization structure
- Implement strict data isolation between projects/cases
- Prevent cross-contamination of conversation contexts
- Add project/case metadata to all stored files
- Implement context boundaries in AI processing

**Technical Details**:
- Create project management system in database schema
- Add project_id foreign key to all relevant tables
- Implement project switching with complete context isolation
- Add UI for project organization and management

**Success Criteria**:
- Each project maintains isolated data storage
- No automatic merging of contexts across projects
- Clear UI indication of current project context
- Ability to export/import projects independently

---

### Step 3: Transparency & Notice (Arts. 12, 13)
**Priority**: Critical | **Effort**: Low | **Legal Risk**: High

**What**: Include a short in-app privacy notice explaining local-only processing.

**Implementation**:
- Create clear privacy notice in simple language
- Display on first run (onboarding flow)
- Make accessible from settings at all times
- Translate to Dutch and German
- Include key information:
  - "All data stays on your device"
  - "No data is sent or shared externally"
  - "You control all data retention and deletion"

**Technical Details**:
- Add onboarding modal component in React
- Create settings page section for privacy information
- Add to i18n translation files (en, nl, de)
- Implement "show on first run" flag in user preferences

**Success Criteria**:
- Privacy notice shown on first application launch
- Accessible within 2 clicks from any screen
- Available in all supported languages
- Clear, non-technical language (8th grade reading level)

---

### Step 4: Encryption at Rest (Art. 32 - Security)
**Priority**: Critical | **Effort**: High | **Legal Risk**: High

**What**: Encrypt all chat logs, configs, and project data locally.

**Implementation**:
- Implement full database encryption
- Encrypt all file storage
- Use Rust crypto libraries (`ring`, `aes-gcm`, or OS-native APIs)
- Optional password protection for additional security
- Encrypt configuration files containing user preferences
- Secure key storage using OS keychain/credential manager

**Technical Details**:
- Choose encryption standard: AES-256-GCM recommended
- Implement key derivation from OS user credentials or optional password
- Use `ring` crate for cryptographic operations
- Encrypt SQLite database using SQLCipher or similar
- Implement encrypted file I/O wrappers
- Add key rotation mechanism
- Handle encryption/decryption performance optimization

**Dependencies**:
```toml
ring = "0.17"
aes-gcm = "0.10"
argon2 = "0.5" # for key derivation
```

**Success Criteria**:
- All data encrypted at rest using AES-256-GCM
- Keys stored securely in OS keychain
- Performance impact < 5% on typical operations
- Transparent to user (auto-unlock with OS credentials)
- Optional password lock for high-security environments

---

### Step 5: PII Layer 1 - Regex-Based Detection & Redaction (Art. 5, 25)
**Priority**: Critical | **Effort**: Medium | **Legal Risk**: High

**What**: Detect and redact personally identifiable information using regex patterns.

**Implementation**:
- Build comprehensive regex library for PII detection:
  - Names (common patterns)
  - Email addresses (RFC 5322 compliant)
  - Phone numbers (international formats)
  - National ID numbers (NL BSN, DE Steuer-ID, etc.)
  - IP addresses
  - Postal addresses
  - Credit card numbers (Luhn algorithm validation)
  - Bank account numbers (IBAN)
  - Dates of birth
  - Social security numbers

**Technical Details**:
- Create `pii-detector` module in Rust
- Implement configurable redaction strategies:
  - Full redaction: `[REDACTED-EMAIL]`
  - Partial redaction: `j***@example.com`
  - Tokenization: `<PII-TOKEN-12345>`
- Add whitelist functionality for expected PII
- Performance optimization using lazy regex compilation
- Multi-language support (Dutch, German, English patterns)

**Regex Patterns**:
```rust
// Email
r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}"

// Dutch BSN (9 digits with 11-proof)
r"\b\d{9}\b"

// IBAN
r"\b[A-Z]{2}\d{2}[A-Z0-9]{4}\d{7}([A-Z0-9]?){0,16}\b"

// Phone numbers (international)
r"(\+\d{1,3}[- ]?)?\(?\d{1,4}\)?[- ]?\d{1,4}[- ]?\d{1,9}"
```

**Success Criteria**:
- Detection rate > 95% for common PII types
- False positive rate < 5%
- Processing speed > 1MB/s of text
- User-configurable sensitivity levels
- Audit log of all redactions

---

### Step 6: Access Control (Art. 32 - Security)
**Priority**: Critical | **Effort**: Medium | **Legal Risk**: Medium

**What**: Integrate OS-level authentication or optional workspace password. Auto-lock on inactivity.

**Implementation**:
- Integrate with OS authentication (Windows Hello, macOS Touch ID, Linux PAM)
- Optional separate application password
- Auto-lock after configurable inactivity period (default: 5 minutes)
- Auto-lock when OS locks
- Require authentication to decrypt data
- Session management with secure token generation

**Technical Details**:
- Use `tauri-plugin-authenticator` or native OS APIs
- Implement session timeout mechanism
- Add system event listeners for lock screen detection
- Secure session storage (encrypted in memory)
- Implement biometric authentication where available

**Success Criteria**:
- Authentication required on app start
- Auto-lock after 5 minutes inactivity (configurable)
- Auto-lock when system locks
- Biometric authentication supported on compatible systems
- Session tokens never persisted to disk

---

### Step 7: Data Deletion - Right to Erasure (Art. 17)
**Priority**: Critical | **Effort**: Medium | **Legal Risk**: High

**What**: Add a "Delete All My Data" button that clears all local data comprehensively.

**Implementation**:
- Create "Delete All My Data" function in settings
- Delete all databases (SQLite files)
- Delete all chat history
- Delete all project files
- Delete all downloaded models (with confirmation)
- Delete all configuration files
- Delete all logs and cache
- Securely overwrite files (not just delete)
- Show confirmation dialog with clear warnings
- Create deletion receipt/confirmation

**Technical Details**:
- Implement secure file deletion (overwrite with random data)
- Use Rust `std::fs::remove_file` with secure overwrite
- Reset application to fresh install state
- Optionally export data before deletion
- Generate deletion timestamp and audit entry

**Success Criteria**:
- All user data removed within 30 seconds
- Files securely overwritten (not recoverable)
- Application returns to first-run state
- Confirmation dialog prevents accidental deletion
- Audit log entry created (stored separately or exported)

---

### Step 8: Data Correction (Art. 16)
**Priority**: High | **Effort**: Low | **Legal Risk**: Medium

**What**: Allow users to preview and edit all data before processing or saving.

**Implementation**:
- Add preview step before all AI processing
- Enable editing of inputs before submission
- Allow editing of AI outputs before saving
- Enable editing of anonymized text before export
- Add version history for edited content
- Implement undo/redo functionality

**Technical Details**:
- Create preview modal components in React
- Add edit mode to all processing workflows
- Implement content versioning system
- Add user confirmation checkpoints
- Store edit history in project metadata

**Success Criteria**:
- All AI processing has preview/edit step
- Users can modify inputs and outputs
- No automatic saving without user confirmation
- Edit history maintained for audit purposes

---

### Step 9: Storage Limitation (Art. 5(1)(e))
**Priority**: High | **Effort**: Medium | **Legal Risk**: Medium

**What**: Let users configure auto-deletion of old projects or logs after a set number of days.

**Implementation**:
- Add retention policy settings
- Default retention periods:
  - Temporary files: 0 days (delete immediately)
  - Logs: 30 days
  - Projects: Never (user controlled)
- Configurable auto-deletion per data type
- Warning before auto-deletion
- Optional export before deletion
- Audit trail of deletions

**Technical Details**:
- Implement background cleanup job
- Add timestamp to all stored data
- Create retention policy engine
- Add UI for retention configuration
- Implement safe deletion with user notification

**Success Criteria**:
- Users can configure retention for each data type
- Auto-deletion runs daily
- Warnings shown 7 days before auto-deletion
- Deletion audit trail maintained
- Minimal retention by default

---

### Step 10: Audit Logging (Arts. 5(2), 30)
**Priority**: High | **Effort**: Medium | **Legal Risk**: Medium

**What**: Maintain a local-only audit log of anonymization, deletions, and data actions.

**Implementation**:
- Create comprehensive audit logging system
- Log all data processing actions:
  - PII detection and redaction
  - Data deletions
  - Export operations
  - Encryption/decryption events
  - Access attempts (failed and successful)
  - Configuration changes
  - Model downloads and usage
- Store logs encrypted locally
- Never transmit logs externally
- Implement log rotation and retention
- Export functionality for compliance audits

**Technical Details**:
- Create structured logging format (JSON)
- Include timestamps, user, action type, affected data
- Implement log search and filter functionality
- Add log viewer in settings
- Encrypt audit logs separately
- Implement log integrity verification (checksums)

**Log Entry Example**:
```json
{
  "timestamp": "2025-10-24T18:30:00Z",
  "action": "pii_redaction",
  "details": {
    "items_detected": 12,
    "types": ["email", "phone", "name"],
    "document_id": "doc-123",
    "redaction_method": "full"
  },
  "user": "local",
  "integrity_hash": "sha256:abc123..."
}
```

**Success Criteria**:
- All data operations logged
- Logs encrypted and never transmitted
- Log viewer accessible in settings
- Export function for compliance audits
- Logs retained for 1 year minimum (configurable)

---

### Step 11: Security by Design & Default (Art. 25)
**Priority**: High | **Effort**: Low | **Legal Risk**: Medium

**What**: Default all settings to local-only mode with privacy-friendly defaults.

**Implementation**:
- Set all defaults to maximum privacy:
  - No telemetry (disabled, no option to enable)
  - No network requests except model downloads (user-initiated)
  - Encryption enabled by default
  - Auto-lock enabled (5 minutes)
  - Minimal data retention
  - PII detection enabled by default
- Remove any cloud or online features
- Hardcode local-only operations
- Add visual indicators of local-only status

**Technical Details**:
- Review all default configurations
- Remove any network-dependent features
- Add "Local Only" badge in UI
- Implement offline mode as only mode
- Code review for privacy by design

**Success Criteria**:
- Application functions 100% offline out of the box
- No user configuration needed for privacy
- Clear visual indication of local-only status
- No degraded functionality in offline mode

---

### Step 12: Privacy Notice Accessibility (Arts. 12-13)
**Priority**: Medium | **Effort**: Low | **Legal Risk**: Low

**What**: Keep a "Privacy Info" or "About Data" section in settings showing key rights.

**Implementation**:
- Create dedicated Privacy Info section
- Explain all GDPR rights in plain language:
  - Right to access (all data visible in app)
  - Right to rectification (edit functionality)
  - Right to erasure (delete all data button)
  - Right to restriction (pause AI processing)
  - Right to data portability (export functionality)
- Explain local-only processing
- Link to full privacy policy (local document)
- Available in all supported languages

**Technical Details**:
- Add Privacy section to settings
- Create privacy policy document
- Implement multi-language support
- Add FAQ about data handling
- Include contact information for questions

**Success Criteria**:
- Privacy information accessible within 2 clicks
- Written in plain language (8th grade level)
- Available in Dutch, German, French, Chinese, and English
- Covers all GDPR rights
- Updated with each significant feature addition

**Rust Files (Phase 1 - GDPR)**:
```
src-tauri/src/
├── gdpr/
│   ├── mod.rs                           # GDPR module exports
│   ├── data_minimization.rs             # Auto-cleanup, temp file management
│   ├── encryption.rs                    # AES-256-GCM encryption
│   ├── access_control.rs                # OS-level auth, session management
│   ├── data_deletion.rs                 # Right to erasure implementation
│   ├── audit.rs                         # Audit logging (Art. 30)
│   └── export.rs                        # Data portability (Art. 20)
├── pii/
│   ├── mod.rs                           # PII module exports
│   ├── regex_detector.rs                # Layer 1: Regex-based PII detection
│   ├── patterns.rs                      # PII regex patterns (email, phone, etc.)
│   └── redactor.rs                      # Redaction engine
├── encryption/
│   ├── mod.rs                           # Encryption module
│   ├── aes.rs                           # AES-256-GCM implementation
│   ├── key_manager.rs                   # Key derivation and storage
│   └── db_encryption.rs                 # Database encryption wrapper
└── commands/
    ├── gdpr.rs                          # GDPR-related commands
    ├── encryption.rs                    # Encryption commands
    └── privacy.rs                       # Privacy settings commands

migration/src/
├── m20250106_000006_add_encryption.rs   # Add encryption columns
├── m20250107_000007_add_pii_logs.rs     # PII detection logs
└── m20250108_000008_add_gdpr_exports.rs # GDPR export tracking
```

---

