# BEAR LLM AI

**Bridge for Expertise, Audit & Research**

> A privacy-first, locally-run legal AI assistant built for professional excellence. BEAR LLM (or "Legistus Mastrum" - Master of Law) unites legal reasoning, audit precision, and research insight while keeping all data secure and under your control.

[![Windows Release](https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research/actions/workflows/windows-release.yml/badge.svg)](https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research/actions/workflows/windows-release.yml)

## 🎯 Project Vision

BEAR LLM is designed specifically for legal professionals who need AI assistance without compromising client confidentiality. Everything runs locally on your machine - no data leaves your computer.

### Key Principles

- **🔒 Privacy First**: All data stays on your machine
- **⚖️ Compliance Built-In**: Audit trails, human review workflows, GDPR-ready
- **🏢 Case-Centric**: Organized by legal matters/cases
- **🌍 Multilingual**: Support for EN, DE, FR, NL, ZH, RU
- **🎨 Professional UI**: Clean, modern interface inspired by leading AI tools

## 🚀 Current Status: Phases 2, 3 & 4 Complete ✅

BEAR LLM now features a complete AI inference system, EU AI Act compliance, and advanced PII protection.

### ✅ What's Implemented

#### Phase 0: Foundation
- **Frontend**: React 18 + TypeScript + Vite + Tailwind CSS
- **Backend**: Rust + Tauri 2.0 + WebView2
- **Database**: SQLite + Sea-ORM with automated migrations
- **UI**: Navigation, theme toggle (dark/light), settings
- **i18n**: Full internationalization framework with 7 languages
- **CI/CD**: Windows release workflow with GitHub Actions

#### Phase 2: EU AI Act Compliance ✅
- **AI Transparency Labels**: Automatic badging of AI-generated content (Article 52.1)
- **About AI Page**: Comprehensive explanation of AI usage (Article 52.2)
- **Output Provenance**: Complete tracking of model name, version, timestamps (Article 52.3)
- **Print Persistence**: AI labels persist in exports and prints
- **100% GDPR Compliant**: All processing happens locally on device

#### Phase 3: AI Inference Engine ✅
- **Candle Framework**: Rust-native ML inference for local LLM processing
- **Streaming Generation**: Real-time token-by-token text generation
- **Chat Interface**: Modern chat UI with message bubbles and animations
- **System Prompts**: 4 built-in prompts (General Assistant, Legal Assistant, Formal Writer, Document Summarizer)
- **Model Management**: Download, load, and manage LLM models
- **Device Auto-Detection**: Automatic CPU/CUDA/Metal selection
- **Conversation Persistence**: Database-backed conversation history

#### Phase 4: Advanced PII Protection ✅
- **NER Models**: 5 state-of-the-art Named Entity Recognition models
- **Hybrid Detection**: Combines pattern-based (regex) and ML-based (NER) detection
- **Entity Linking**: Smart detection of name variations ("Mr. John Doe" = "John Doe")
- **14 Entity Types**: EMAIL, PHONE, SSN, CREDIT_CARD, IP_ADDRESS, URL, PERSON, ORG, LOCATION, DATE, TIME, MONEY, PERCENTAGE, MEDICAL
- **Batch Processing**: Anonymize multiple documents simultaneously
- **Detection Modes**: Pattern-only, NER-only, or Hybrid (recommended)

### 📊 Implementation Stats
- **16,900+ lines of code** across 62 files
- **27 backend modules** (Rust)
- **25 frontend components** (React/TypeScript)
- **15 Tauri commands** with full frontend integration
- **3 event streams** for real-time updates

### 🚧 Not Yet Implemented

- Full encryption at rest (Phase 1)
- Advanced case management (Phase 2 extension)
- Legal research/RAG (Phase 7)
- Multi-client workflows (Phase 8)

See [DETAILED_ROADMAP.md](./DETAILED_ROADMAP.md) for the full development plan.

## 📋 Prerequisites

### Development Environment

- **Node.js**: v20 or higher
- **Rust**: Latest stable (install via [rustup](https://rustup.rs/))
- **Operating System**:
  - Windows 10/11 (WebView2 required)
  - macOS 10.15+ (coming soon)
  - Linux (coming soon)

### Windows-Specific

WebView2 is required for Windows. It's included in Windows 11 and recent Windows 10 updates. If missing:

```powershell
# Download and install WebView2 Runtime
https://developer.microsoft.com/en-us/microsoft-edge/webview2/
```

## 🛠️ Setup Instructions

### 1. Clone the Repository

```bash
git clone https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research.git
cd Bridge-for-Expertise-Audit-and-Research
```

### 2. Install Dependencies

```bash
# Install frontend dependencies
npm install

# Rust dependencies will be handled automatically by Cargo
```

### 3. Run Development Server

```bash
# Start the development server with hot reload
npm run tauri:dev

# Or run frontend and backend separately:
npm run dev              # Frontend only (port 5173)
# Then in another terminal:
cd src-tauri && cargo run
```

The app will open automatically. Changes to frontend code will hot-reload. Rust changes require restarting the dev server.

### 4. Build for Production

```bash
# Build the complete application
npm run tauri:build

# Output will be in src-tauri/target/release/bundle/
# - Windows: .msi and .exe installers
# - macOS: .dmg and .app
# - Linux: .deb, .rpm, .AppImage
```

## 📁 Project Structure

```
Bridge-for-Expertise-Audit-and-Research/
├── src/                          # Frontend React application
│   ├── components/              # React components
│   │   ├── Sidebar.tsx
│   │   ├── AIBadge.tsx         # AI transparency badges (Phase 2)
│   │   ├── AIContentBadge.tsx  # EU AI Act compliance labels
│   │   ├── ReviewModal.tsx     # Human review workflow
│   │   ├── CaseList.tsx
│   │   ├── AddCustomModel.tsx  # Custom model management
│   │   └── ThemeToggle.tsx
│   ├── pages/                   # Route pages
│   │   ├── Home.tsx
│   │   ├── Chat.tsx            # AI chat interface (Phase 3)
│   │   ├── Models.tsx          # LLM model management
│   │   ├── NERModels.tsx       # NER model management (Phase 4)
│   │   ├── PIIProtection.tsx   # PII detection & anonymization
│   │   ├── AboutAI.tsx         # AI transparency page (Phase 2)
│   │   ├── Cases.tsx
│   │   ├── Settings.tsx
│   │   └── About.tsx
│   ├── contexts/                # React contexts
│   │   └── ThemeContext.tsx
│   ├── services/                # Frontend services
│   │   ├── settings.ts
│   │   └── modelService.ts     # Model download & management
│   ├── i18n/                    # Internationalization
│   │   ├── config.ts
│   │   └── locales/            # Translation files
│   │       ├── en-GB.json
│   │       ├── de-DE.json
│   │       ├── fr-FR.json
│   │       ├── nl-NL.json
│   │       ├── zh-Hans-CN.json
│   │       ├── zh-Hant-HK.json
│   │       └── ru-RU.json
│   └── styles/                  # Component styles
│       ├── Chat.css            # Chat interface styling
│       ├── Models.css          # Model management styling
│       └── AIBadge.css         # EU AI Act badge styling
│
├── src-tauri/                   # Rust backend
│   ├── src/
│   │   ├── main.rs             # Application entry point
│   │   ├── commands/           # Tauri commands (API endpoints)
│   │   │   ├── mod.rs
│   │   │   ├── settings.rs
│   │   │   ├── models.rs       # LLM model management
│   │   │   ├── conversation.rs # AI chat & inference (Phase 3)
│   │   │   ├── pii.rs          # PII detection & anonymization (Phase 4)
│   │   │   └── ner.rs          # NER model management (Phase 4)
│   │   ├── ai/                 # AI inference engine (Phase 3)
│   │   │   ├── mod.rs
│   │   │   ├── inference.rs    # Candle-based inference
│   │   │   └── types.rs        # AI type system
│   │   ├── models/             # Model management system
│   │   │   ├── mod.rs
│   │   │   ├── registry.rs     # Model catalog
│   │   │   ├── downloader.rs   # Download with progress
│   │   │   └── validator.rs    # Checksum validation
│   │   ├── ner/                # NER system (Phase 4)
│   │   │   ├── mod.rs
│   │   │   ├── types.rs        # NER type system & BIO tags
│   │   │   ├── inference.rs    # NER inference engine
│   │   │   ├── model_loader.rs # Load HuggingFace models
│   │   │   ├── tokenizer.rs    # Text tokenization
│   │   │   ├── hybrid_detector.rs # Hybrid detection
│   │   │   ├── registry.rs     # NER model catalog
│   │   │   └── downloader.rs   # NER model downloads
│   │   ├── pii/                # PII protection (Phase 4)
│   │   │   ├── mod.rs
│   │   │   ├── types.rs        # PII type system
│   │   │   ├── detector.rs     # Pattern-based detection
│   │   │   ├── anonymizer.rs   # Text anonymization
│   │   │   └── entity_linker.rs # Entity variation linking
│   │   ├── database/           # Database connection manager
│   │   │   └── mod.rs
│   │   └── services/           # Business logic
│   │       ├── mod.rs
│   │       └── audit.rs
│   ├── entity/                  # Database entities (ORM models)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── settings.rs
│   │       ├── cases.rs
│   │       ├── conversations.rs
│   │       ├── messages.rs     # With AI Act provenance fields
│   │       └── audit_logs.rs
│   ├── migration/               # Database migrations
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── m20250101_000001_create_settings.rs
│   │       ├── m20250101_000002_create_cases.rs
│   │       └── m20250101_000003_create_audit_log.rs
│   ├── Cargo.toml
│   └── tauri.conf.json         # Tauri configuration
│
├── .github/
│   └── workflows/
│       └── windows-release.yml  # CI/CD for Windows builds
│
├── DETAILED_ROADMAP.md          # Full development roadmap
├── LOGBOOK.md                   # Development log (520+ lines)
├── IMPLEMENTATION_STATUS.md     # Complete implementation status
└── README.md                    # This file
```

## 🧪 Testing

```bash
# Run Rust tests
cd src-tauri
cargo test

# Run frontend tests (when added)
npm test

# Check Rust code
cargo check
cargo clippy
```

## 🗄️ Database

BEAR LLM uses SQLite for local data storage. The database is automatically created and migrated on first run.

**Location**: `%APPDATA%/com.bear.llm.ai/bear_llm.db` (Windows)

**Tables**:
- `settings` - User preferences (theme, language)
- `cases` - Legal matters/cases
- `conversations` - Chat conversations per case
- `messages` - Individual chat messages
- `audit_log` - Compliance audit trail

Migrations are handled automatically by Sea-ORM.

## 🌍 Internationalization

BEAR LLM supports 7 languages out of the box:

- 🇬🇧 English (en-GB)
- 🇩🇪 German (de-DE)
- 🇫🇷 French (fr-FR)
- 🇳🇱 Dutch (nl-NL)
- 🇨🇳 Chinese Simplified (zh-Hans-CN)
- 🇭🇰 Chinese Traditional (zh-Hant-HK)
- 🇷🇺 Russian (ru-RU)

Add translations in `src/i18n/locales/*.json`. Language selection is in Settings.

## 🎨 Theming

- **Light Mode**: Professional light theme
- **Dark Mode**: Eye-friendly dark theme
- **System**: Follows OS preference

Toggle via Settings or the theme button in the sidebar.

## 🤝 Contributing

We welcome contributions! Please see [DEVELOPMENT.md](./DEVELOPMENT.md) for developer guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test` and `npm test`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📝 License

### OpenCode Philosophy: Building in Public for Trust and Collaboration

**BEAR LLM AI builds everything in public** - every line of code, every feature, and every decision is visible in this repository. We believe that transparency builds trust, and trust is essential when handling sensitive legal matters.

**However, this is proprietary software.** The codebase is publicly visible but not freely licensed, except where explicitly marked otherwise.

- **Default License**: All code is proprietary and copyright protected unless stated otherwise
- **MIT Licensed Components**: Certain portions of the code are licensed under the MIT License (clearly marked in source files with license headers)
- **Read the Code, Don't Copy**: You can read, learn from, and audit our code, but you cannot use it in your own projects without permission
- **Why Build in Public?**: Legal professionals need to trust their tools. By building openly, we invite scrutiny, collaboration, and transparency while protecting our intellectual property

See the [LICENSE](LICENSE) file for complete legal terms.

### Third-Party Components

This software incorporates certain third-party open-source components. See [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md) for attribution and license information for these components.

## 🙏 Acknowledgments

- **Tauri**: For the amazing Rust + Web framework
- **Sea-ORM**: For elegant database handling
- **React**: For the UI framework
- UI inspiration from **Kaas**, **LM Studio**, and **Jan AI**

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research/issues)
- **Discussions**: [GitHub Discussions](https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research/discussions)

## 🗺️ Roadmap

See [DETAILED_ROADMAP.md](./DETAILED_ROADMAP.md) and [IMPLEMENTATION_STATUS.md](./IMPLEMENTATION_STATUS.md) for complete details.

**Completed Phases**:
- ✅ **Phase 0**: Foundation (UI, database, i18n)
- ✅ **Phase 2**: EU AI Act Compliance (Article 52)
- ✅ **Phase 3**: AI Inference Engine (Candle, streaming generation)
- ✅ **Phase 4**: Advanced PII Protection (NER models, entity linking)

**Next Milestones**:
- **Phase 1**: Full encryption at rest, secure key management
- **Phase 5**: Conversation search and advanced history
- **Phase 6**: Document analysis and batch processing
- **Phase 7**: Legal research with RAG (Retrieval-Augmented Generation)
- **Phase 8**: Multi-client workflows and collaboration

---

**Built with ❤️ for legal professionals who value privacy and precision.**
