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

## 🚀 Current Status: Phase 0 Complete

Phase 0 establishes the foundation - a complete wireframe with database, UI, and compliance patterns.

### ✅ What's Implemented

- **Frontend**: React 18 + TypeScript + Vite + Tailwind CSS
- **Backend**: Rust + Tauri 2.0 + WebView2
- **Database**: SQLite + Sea-ORM with automated migrations
- **UI**: Navigation, theme toggle (dark/light), settings
- **i18n**: Full internationalization framework with 7 languages
- **Compliance Patterns**:
  - Case/matter organization
  - Audit log structure
  - AI transparency badges (placeholders)
  - Review workflow UI patterns
- **CI/CD**: Windows release workflow with GitHub Actions

### 🚧 Not Yet Implemented

- AI/LLM integration (Phase 3)
- Full GDPR compliance tools (Phase 1)
- Encryption (Phase 1)
- PII detection (Phase 4)
- Legal research/RAG (Phase 7)

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
│   │   ├── AIBadge.tsx         # AI transparency badges
│   │   ├── ReviewModal.tsx     # Human review workflow
│   │   ├── CaseList.tsx
│   │   └── ThemeToggle.tsx
│   ├── pages/                   # Route pages
│   │   ├── Home.tsx
│   │   ├── Cases.tsx
│   │   ├── Settings.tsx
│   │   └── About.tsx
│   ├── contexts/                # React contexts
│   │   └── ThemeContext.tsx
│   ├── services/                # Frontend services
│   │   └── settings.ts
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
│   └── styles/                  # Global styles
│
├── src-tauri/                   # Rust backend
│   ├── src/
│   │   ├── main.rs             # Application entry point
│   │   ├── commands/           # Tauri commands (API endpoints)
│   │   │   ├── mod.rs
│   │   │   └── settings.rs
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
│   │       ├── messages.rs
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
├── LOGBOOK.md                   # Development log
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

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Tauri**: For the amazing Rust + Web framework
- **Sea-ORM**: For elegant database handling
- **React**: For the UI framework
- UI inspiration from **Kaas**, **LM Studio**, and **Jan AI**

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research/issues)
- **Discussions**: [GitHub Discussions](https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research/discussions)

## 🗺️ Roadmap

See [DETAILED_ROADMAP.md](./DETAILED_ROADMAP.md) for the complete development plan.

**Next Milestones**:
- **Phase 1**: GDPR compliance, encryption, secure storage
- **Phase 2**: UI polish, case management
- **Phase 3**: LLM integration (Ollama, local models)
- **Phase 4**: PII detection and redaction
- **Phase 5-8**: Advanced features (RAG, multi-client, voice, etc.)

---

**Built with ❤️ for legal professionals who value privacy and precision.**
