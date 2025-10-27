# BEAR LLM AI - Complete Development Roadmap

**From Zero to Production: Building a Privacy-First Legal AI Assistant**

This roadmap provides a complete development path from initial project setup through full production deployment. It prioritizes:
1. **Phase 0**: Initial application setup and wireframe (foundation)
2. **Phase 1-2**: Legal compliance (GDPR & AI Act) - mandatory for EU operations
3. **Phase 3-4**: Local AI infrastructure and PII protection
4. **Phase 5+**: Advanced features and strategic paths

---

## 🎯 MVP Definition: What's Required for Launch?

**Minimum Viable Product (MVP) = v0.3.0**

The MVP includes **Phase 0-2 only** (GDPR + AI Act compliance). This may seem counterintuitive - an AI assistant without full AI capabilities - but it's strategically correct:

### Why Legal Compliance is the MVP:

| Phase | Version | Timeline | Required for MVP? | Rationale |
|-------|---------|----------|-------------------|-----------|
| **Phase 0** | v0.1.0 | ✅ Complete | ✅ **YES** | Foundation - app must exist and run |
| **Phase 1** | v0.2.0 | 6-8 weeks | ✅ **YES** | GDPR compliance legally required in EU |
| **Phase 2** | v0.3.0 | 4-6 weeks | ✅ **YES** | AI Act compliance (Article 52) legally required |
| Phase 3 | v0.4.0 | 8-12 weeks | ❌ **NO** | Full AI can come after compliance framework |
| Phase 4 | v0.5.0 | 6-8 weeks | ❌ **NO** | Advanced PII is post-launch enhancement |

**Total Time to MVP**: ~4-5 months from Phase 0 start

### Why This Approach Works:

1. **Legal First**: You CANNOT launch in EU without GDPR/AI Act compliance - fix this first
2. **Basic AI Sufficient**: MVP can use simple LLM integration while compliance framework is rock-solid
3. **Iterate on AI**: Once compliant, you can safely add Candle, Ollama, RAG, agents, etc.
4. **Trust Building**: Lawyers trust compliance > features - show you take privacy seriously
5. **Avoid Rework**: Building compliance AFTER complex AI = massive refactoring

### What "MVP" Means:

- ✅ App runs stably on Windows/Mac/Linux
- ✅ Database, cases, conversations work
- ✅ GDPR compliance: encryption, audit logs, right to erasure
- ✅ AI Act compliance: transparency, human review, output labeling
- ✅ **Privacy-respecting auto-updater** (Step 0.6a) - push bug fixes without compromising privacy
- ✅ Basic chat interface (can use OpenAI API, Ollama, or simple responses)
- ❌ Advanced local models (Candle, quantization) - Phase 3
- ❌ Advanced PII detection (NER, Presidio) - Phase 4
- ❌ RAG/legal research - Phase 7
- ❌ Multi-modal, agents, Word add-in - Phase 6+

### Post-MVP Roadmap:

- **v0.4.0** (Phase 3): Candle integration, local 7B models, GPU acceleration
- **v0.5.0** (Phase 4): NER-based PII detection, >98% detection rate
- **v1.0.0** (Phase 7-8): RAG, legal research, fortress security - **PRODUCTION READY**

---

## 🔄 Auto-Updater Strategy: Privacy-First Updates

**Question**: "Does an updater conflict with local-first, no-telemetry philosophy?"

**Answer**: **NO! Modern updaters respect privacy perfectly.**

### How Privacy-Respecting Updates Work:

1. **What's Sent**: Only version number to GitHub API (e.g., "is v0.0.20 latest?")
2. **What's NOT Sent**: No user ID, no system info, no usage data, no analytics
3. **Anonymous Download**: Update files from GitHub Releases (no tracking)
4. **User Control**: Can disable auto-check or require manual approval
5. **Secure**: Updates cryptographically signed to prevent tampering

### Examples of Local-First Apps with Updaters:
- **VS Code**: Privacy-focused IDE, has auto-updater
- **Obsidian**: Local-first notes, has updater
- **Signal Desktop**: Privacy messenger, has updater
- **Brave Browser**: Privacy browser, has updater

### Implementation: See **Step 0.6a** below
- Tauri built-in updater (GitHub-based)
- No telemetry, no analytics, no tracking
- Update checks can be disabled in Settings
- All operations local after download

**Conclusion**: Updaters are ESSENTIAL for security patches and bug fixes. The implementation respects privacy completely.

---

## Completed Milestones ✅

| Step | Feature | Status |
|------|---------|--------|


**Achievement**: All operations are 100% local with no network callbacks or telemetry. Full localization for Dutch, German, French, and Chinese markets.

---

## Phase 0: Foundation & Wireframe Application (Priority: CRITICAL)
**Building the Skeleton - Verify Everything Works**

**Objective**: Create a minimal but functional application skeleton to verify the technology stack works correctly before adding compliance and AI features. This phase ensures the foundation is solid.

**UI/UX Design Reference**:
This application draws inspiration from leading local LLM UI clients:
- **Kaas** (https://github.com/0xfrankz/Kaas) - Cross-platform desktop LLM client with privacy focus
  - Clean sidebar navigation with chat history
  - Main chat interface with message bubbles
  - Settings panel with provider configuration
  - Model selection dropdown
  - Light/dark theme toggle
- **LM Studio** - Professional desktop interface for local models
- **Ollama Desktop** - Minimalist chat interface
- **Jan AI** - Modern, user-friendly design

**Core UI Components**:
1. **Left Sidebar** (240-280px):
   - Logo and app version
   - New chat/case button
   - Chat/case history list
   - Settings button (bottom)
   - Theme toggle (bottom)

2. **Main Chat Area**:
   - Message history with role indicators (User/AI)
   - Input field with send button
   - Attachment/file upload button
   - Case/matter context indicator

3. **Right Panel** (collapsible):
   - Model settings
   - Temperature/parameters
   - Case metadata
   - Compliance indicators

4. **Top Bar**:
   - Current case/matter name
   - Language selector
   - User menu/profile

### Step 0.1: Project Initialization & Setup
**Priority**: Critical | **Effort**: Low | **Risk**: Low

**What**: Set up the basic Tauri + React + TypeScript project structure.

**Implementation**:
1. **Initialize Tauri Project**:
   ```bash
   # Create new Tauri project (if starting from scratch)
   npm create tauri-app@latest bear-llm-ai

   # Select options:
   # - Package manager: npm
   # - UI template: React + TypeScript
   # - UI framework: React with Vite
   ```

2. **Configure Project Structure**:
   ```
   bear-llm-ai/
   ├── src/                    # React frontend
   │   ├── components/         # React components
   │   ├── hooks/              # Custom React hooks
   │   ├── pages/              # Page components
   │   ├── services/           # API/service layers
   │   ├── styles/             # CSS/styling
   │   ├── types/              # TypeScript types
   │   ├── App.tsx             # Main app component
   │   └── main.tsx            # Entry point
   ├── src-tauri/              # Rust backend
   │   ├── src/
   │   │   ├── commands/       # Tauri commands
   │   │   ├── models/         # Data models
   │   │   ├── services/       # Business logic
   │   │   ├── utils/          # Utilities
   │   │   └── main.rs         # Entry point
   │   ├── Cargo.toml          # Rust dependencies
   │   └── tauri.conf.json     # Tauri configuration
   ├── public/                 # Static assets
   ├── package.json            # Node dependencies
   └── tsconfig.json           # TypeScript config
   ```

3. **Install Core Dependencies**:
   ```bash
   # Frontend dependencies
   npm install react-router-dom
   npm install @radix-ui/react-dialog @radix-ui/react-select
   npm install tailwindcss @tailwindcss/typography
   npm install i18next react-i18next
   npm install zustand  # State management

   # Dev dependencies
   npm install -D @types/react @types/react-dom
   npm install -D typescript
   npm install -D eslint @typescript-eslint/parser
   npm install -D prettier
   ```

4. **Configure Tauri**:
   ```json
   // src-tauri/tauri.conf.json
   {
     "build": {
       "beforeDevCommand": "npm run dev",
       "beforeBuildCommand": "npm run build",
       "devPath": "http://localhost:5173",
       "distDir": "../dist"
     },
     "package": {
       "productName": "BEAR LLM AI",
       "version": "0.0.20"
     },
     "tauri": {
       "allowlist": {
         "all": false,
         "fs": {
           "all": false,
           "readFile": true,
           "writeFile": true,
           "createDir": true,
           "scope": ["$APPDATA/*", "$APPDATA/**"]
         },
         "dialog": {
           "all": true
         }
       },
       "windows": [{
         "title": "BEAR LLM AI",
         "width": 1200,
         "height": 800,
         "minWidth": 800,
         "minHeight": 600,
         "resizable": true,
         "fullscreen": false
       }]
     }
   }
   ```

**Success Criteria**:
- Project structure created and organized
- All dependencies installed without errors
- TypeScript compilation successful
- Development server starts without errors

---

### Step 0.2: Basic UI Wireframe
**Priority**: Critical | **Effort**: Low | **Risk**: Low

**What**: Create a minimal UI wireframe to verify rendering and navigation.

**Implementation**:

1. **Main Application Shell**:
   ```typescript
   // src/App.tsx
   import React from 'react';
   import { BrowserRouter, Routes, Route } from 'react-router-dom';
   import Sidebar from './components/Sidebar';
   import HomePage from './pages/Home';
   import SettingsPage from './pages/Settings';
   import AboutPage from './pages/About';

   function App() {
     return (
       <BrowserRouter>
         <div className="flex h-screen bg-gray-50 dark:bg-gray-900">
           <Sidebar />
           <main className="flex-1 overflow-auto">
             <Routes>
               <Route path="/" element={<HomePage />} />
               <Route path="/settings" element={<SettingsPage />} />
               <Route path="/about" element={<AboutPage />} />
             </Routes>
           </main>
         </div>
       </BrowserRouter>
     );
   }

   export default App;
   ```

2. **Sidebar Navigation**:
   ```typescript
   // src/components/Sidebar.tsx
   import React from 'react';
   import { Link } from 'react-router-dom';

   const Sidebar: React.FC = () => {
     return (
       <aside className="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700">
         <div className="p-4">
           <h1 className="text-xl font-bold">BEAR LLM AI</h1>
           <p className="text-sm text-gray-500">v0.0.20</p>
         </div>

         <nav className="mt-4">
           <Link to="/" className="block px-4 py-2 hover:bg-gray-100">
             Home
           </Link>
           <Link to="/settings" className="block px-4 py-2 hover:bg-gray-100">
             Settings
           </Link>
           <Link to="/about" className="block px-4 py-2 hover:bg-gray-100">
             About
           </Link>
         </nav>
       </aside>
     );
   };

   export default Sidebar;
   ```

3. **Placeholder Pages**:
   ```typescript
   // src/pages/Home.tsx
   const HomePage = () => {
     return (
       <div className="p-8">
         <h2 className="text-2xl font-bold mb-4">Welcome to BEAR LLM AI</h2>
         <p className="text-gray-600">
           This is a wireframe. Features will be added in subsequent phases.
         </p>
       </div>
     );
   };

   // src/pages/Settings.tsx
   const SettingsPage = () => {
     return (
       <div className="p-8">
         <h2 className="text-2xl font-bold mb-4">Settings</h2>
         <p className="text-gray-600">Settings UI will be implemented here.</p>
       </div>
     );
   };

   // src/pages/About.tsx
   const AboutPage = () => {
     return (
       <div className="p-8">
         <h2 className="text-2xl font-bold mb-4">About BEAR LLM AI</h2>
         <p className="text-gray-600">
           Version: 0.0.20<br />
           Privacy-first legal AI assistant<br />
           100% local processing
         </p>
       </div>
     );
   };
   ```

**Success Criteria**:
- Application window opens and displays
- Navigation between pages works
- UI is responsive and styled correctly
- No console errors

---

### Step 0.3: Database Setup & Migrations
**Priority**: Critical | **Effort**: Medium | **Risk**: Medium

**What**: Set up SQLite database with migration system.

**Implementation**:

1. **Add Database Dependencies**:
   ```toml
   # src-tauri/Cargo.toml
   [dependencies]
   sea-orm = { version = "0.12", features = ["sqlx-sqlite", "runtime-tokio-native-tls", "macros"] }
   sea-orm-migration = "0.12"
   sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio"] }
   tokio = { version = "1.36", features = ["full"] }
   ```

2. **Database Connection Manager**:
   ```rust
   // src-tauri/src/database/mod.rs
   use sea_orm::{Database, DatabaseConnection, DbErr};
   use std::sync::Arc;
   use tokio::sync::Mutex;

   pub struct DatabaseManager {
       connection: Arc<Mutex<Option<DatabaseConnection>>>,
   }

   impl DatabaseManager {
       pub fn new() -> Self {
           Self {
               connection: Arc::new(Mutex::new(None)),
           }
       }

       pub async fn initialize(&self, db_path: &str) -> Result<(), DbErr> {
           let db_url = format!("sqlite://{}?mode=rwc", db_path);
           let conn = Database::connect(&db_url).await?;

           // Run migrations
           migration::Migrator::up(&conn, None).await?;

           *self.connection.lock().await = Some(conn);
           Ok(())
       }

       pub async fn get_connection(&self) -> Option<DatabaseConnection> {
           self.connection.lock().await.clone()
       }
   }
   ```

3. **Initial Migration**:
   ```rust
   // src-tauri/migration/src/m20250101_000001_create_settings.rs
   use sea_orm_migration::prelude::*;

   #[derive(DeriveMigrationName)]
   pub struct Migration;

   #[async_trait::async_trait]
   impl MigrationTrait for Migration {
       async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
           manager
               .create_table(
                   Table::create()
                       .table(Settings::Table)
                       .if_not_exists()
                       .col(
                           ColumnDef::new(Settings::Id)
                               .integer()
                               .not_null()
                               .auto_increment()
                               .primary_key(),
                       )
                       .col(ColumnDef::new(Settings::Key).string().not_null().unique_key())
                       .col(ColumnDef::new(Settings::Value).string().not_null())
                       .col(
                           ColumnDef::new(Settings::CreatedAt)
                               .timestamp()
                               .not_null()
                               .default(Expr::current_timestamp()),
                       )
                       .col(
                           ColumnDef::new(Settings::UpdatedAt)
                               .timestamp()
                               .not_null()
                               .default(Expr::current_timestamp()),
                       )
                       .to_owned(),
               )
               .await
       }

       async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
           manager
               .drop_table(Table::drop().table(Settings::Table).to_owned())
               .await
       }
   }

   #[derive(Iden)]
   enum Settings {
       Table,
       Id,
       Key,
       Value,
       CreatedAt,
       UpdatedAt,
   }
   ```

4. **Database Initialization in main.rs**:
   ```rust
   // src-tauri/src/main.rs
   use tauri::Manager;
   mod database;

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
           .run(tauri::generate_context!())
           .expect("error while running tauri application");
   }
   ```

**Success Criteria**:
- Database file created in app data directory
- Migrations run successfully
- Settings table created
- Database connection available to Tauri commands

**Rust Files**:
- `src-tauri/src/database/mod.rs` - Database connection manager
- `src-tauri/src/database/models.rs` - SeaORM entity models
- `src-tauri/migration/src/lib.rs` - Migration runner
- `src-tauri/migration/src/m20250101_000001_create_settings.rs` - Settings table migration
- `src-tauri/migration/src/m20250102_000002_create_chats.rs` - Chats table migration
- `src-tauri/migration/src/m20250103_000003_create_messages.rs` - Messages table migration
- `src-tauri/migration/src/m20250104_000004_create_cases.rs` - Cases/matters table migration
- `src-tauri/entity/src/settings.rs` - Settings entity
- `src-tauri/entity/src/chats.rs` - Chats entity
- `src-tauri/entity/src/messages.rs` - Messages entity
- `src-tauri/entity/src/cases.rs` - Cases entity

---

### Step 0.4: i18n Framework Setup
**Priority**: High | **Effort**: Low | **Risk**: Low

**What**: Set up internationalization framework for multilingual support.

**Implementation**:

1. **i18n Configuration**:
   ```typescript
   // src/i18n/config.ts
   import i18n from 'i18next';
   import { initReactI18next } from 'react-i18next';
   import en from './locales/en.json';
   import nl from './locales/nl.json';
   import de from './locales/de.json';
   import fr from './locales/fr.json';
   import zh from './locales/zh.json';

   i18n
     .use(initReactI18next)
     .init({
       resources: {
         en: { translation: en },
         nl: { translation: nl },
         de: { translation: de },
         fr: { translation: fr },
         zh: { translation: zh },
       },
       lng: 'en',
       fallbackLng: 'en',
       interpolation: {
         escapeValue: false,
       },
     });

   export default i18n;
   ```

2. **Translation Files**:
   ```json
   // src/i18n/locales/en.json
   {
     "app": {
       "title": "BEAR LLM AI",
       "subtitle": "Privacy-First Legal Assistant"
     },
     "nav": {
       "home": "Home",
       "settings": "Settings",
       "about": "About",
       "newChat": "New Chat",
       "newCase": "New Case"
     },
     "settings": {
       "title": "Settings",
       "language": "Language",
       "theme": "Theme",
       "darkMode": "Dark Mode",
       "lightMode": "Light Mode",
       "systemTheme": "System Default"
     },
     "chat": {
       "inputPlaceholder": "Type your message...",
       "send": "Send",
       "attachFile": "Attach File",
       "processing": "Processing..."
     }
   }

   // src/i18n/locales/fr.json
   {
     "app": {
       "title": "BEAR LLM AI",
       "subtitle": "Assistant Juridique Respectueux de la Vie Privée"
     },
     "nav": {
       "home": "Accueil",
       "settings": "Paramètres",
       "about": "À propos",
       "newChat": "Nouvelle Discussion",
       "newCase": "Nouveau Dossier"
     },
     "settings": {
       "title": "Paramètres",
       "language": "Langue",
       "theme": "Thème",
       "darkMode": "Mode Sombre",
       "lightMode": "Mode Clair",
       "systemTheme": "Système par Défaut"
     },
     "chat": {
       "inputPlaceholder": "Saisissez votre message...",
       "send": "Envoyer",
       "attachFile": "Joindre un Fichier",
       "processing": "Traitement en cours..."
     }
   }

   // src/i18n/locales/zh.json
   {
     "app": {
       "title": "BEAR LLM AI",
       "subtitle": "隐私优先法律助手"
     },
     "nav": {
       "home": "主页",
       "settings": "设置",
       "about": "关于",
       "newChat": "新对话",
       "newCase": "新案例"
     },
     "settings": {
       "title": "设置",
       "language": "语言",
       "theme": "主题",
       "darkMode": "深色模式",
       "lightMode": "浅色模式",
       "systemTheme": "系统默认"
     },
     "chat": {
       "inputPlaceholder": "输入您的消息...",
       "send": "发送",
       "attachFile": "附加文件",
       "processing": "处理中..."
     }
   }
   ```

3. **Language Selector Component**:
   ```typescript
   // src/components/LanguageSelector.tsx
   import React from 'react';
   import { useTranslation } from 'react-i18next';

   const LanguageSelector: React.FC = () => {
     const { i18n } = useTranslation();

     return (
       <select
         value={i18n.language}
         onChange={(e) => i18n.changeLanguage(e.target.value)}
         className="border rounded px-2 py-1"
       >
         <option value="en">English</option>
         <option value="nl">Nederlands</option>
         <option value="de">Deutsch</option>
         <option value="fr">Français</option>
         <option value="zh">中文</option>
       </select>
     );
   };
   ```

**Success Criteria**:
- Language can be switched between EN/NL/DE/FR/ZH
- All UI text updates when language changes
- Language preference persists across app restarts
- All five languages fully translated

**Rust Files**:
- N/A (frontend-only feature)

---

### Step 0.4a: Theme Toggle (Dark/Light Mode)
**Priority**: High | **Effort**: Low | **Risk**: Low

**What**: Implement dark mode and light mode toggle with system preference detection.

**Implementation**:

1. **Theme Provider Setup**:
   ```typescript
   // src/contexts/ThemeContext.tsx
   import React, { createContext, useContext, useEffect, useState } from 'react';

   type Theme = 'light' | 'dark' | 'system';

   interface ThemeContextType {
     theme: Theme;
     setTheme: (theme: Theme) => void;
     effectiveTheme: 'light' | 'dark';
   }

   const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

   export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
     const [theme, setTheme] = useState<Theme>('system');
     const [effectiveTheme, setEffectiveTheme] = useState<'light' | 'dark'>('light');

     useEffect(() => {
       // Load saved theme preference
       const savedTheme = localStorage.getItem('theme') as Theme;
       if (savedTheme) setTheme(savedTheme);
     }, []);

     useEffect(() => {
       // Save theme preference
       localStorage.setItem('theme', theme);

       // Determine effective theme
       if (theme === 'system') {
         const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
         setEffectiveTheme(systemPrefersDark ? 'dark' : 'light');
       } else {
         setEffectiveTheme(theme);
       }
     }, [theme]);

     useEffect(() => {
       // Apply theme to document
       if (effectiveTheme === 'dark') {
         document.documentElement.classList.add('dark');
       } else {
         document.documentElement.classList.remove('dark');
       }
     }, [effectiveTheme]);

     return (
       <ThemeContext.Provider value={{ theme, setTheme, effectiveTheme }}>
         {children}
       </ThemeContext.Provider>
     );
   };

   export const useTheme = () => {
     const context = useContext(ThemeContext);
     if (!context) throw new Error('useTheme must be used within ThemeProvider');
     return context;
   };
   ```

2. **Theme Toggle Component**:
   ```typescript
   // src/components/ThemeToggle.tsx
   import React from 'react';
   import { useTheme } from '../contexts/ThemeContext';
   import { useTranslation } from 'react-i18next';

   const ThemeToggle: React.FC = () => {
     const { theme, setTheme } = useTheme();
     const { t } = useTranslation();

     return (
       <div className="flex items-center gap-2">
         <label className="text-sm font-medium">{t('settings.theme')}</label>
         <select
           value={theme}
           onChange={(e) => setTheme(e.target.value as any)}
           className="border rounded px-2 py-1 dark:bg-gray-800 dark:border-gray-600"
         >
           <option value="light">{t('settings.lightMode')}</option>
           <option value="dark">{t('settings.darkMode')}</option>
           <option value="system">{t('settings.systemTheme')}</option>
         </select>
       </div>
     );
   };

   export default ThemeToggle;
   ```

3. **Quick Theme Toggle Button** (for sidebar):
   ```typescript
   // src/components/QuickThemeToggle.tsx
   import React from 'react';
   import { useTheme } from '../contexts/ThemeContext';

   const QuickThemeToggle: React.FC = () => {
     const { effectiveTheme, setTheme } = useTheme();

     const toggleTheme = () => {
       setTheme(effectiveTheme === 'dark' ? 'light' : 'dark');
     };

     return (
       <button
         onClick={toggleTheme}
         className="p-2 rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
         aria-label="Toggle theme"
       >
         {effectiveTheme === 'dark' ? (
           <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
             <path fillRule="evenodd" d="M10 2a1 1 0 011 1v1a1 1 0 11-2 0V3a1 1 0 011-1zm4 8a4 4 0 11-8 0 4 4 0 018 0zm-.464 4.95l.707.707a1 1 0 001.414-1.414l-.707-.707a1 1 0 00-1.414 1.414zm2.12-10.607a1 1 0 010 1.414l-.706.707a1 1 0 11-1.414-1.414l.707-.707a1 1 0 011.414 0zM17 11a1 1 0 100-2h-1a1 1 0 100 2h1zm-7 4a1 1 0 011 1v1a1 1 0 11-2 0v-1a1 1 0 011-1zM5.05 6.464A1 1 0 106.465 5.05l-.708-.707a1 1 0 00-1.414 1.414l.707.707zm1.414 8.486l-.707.707a1 1 0 01-1.414-1.414l.707-.707a1 1 0 011.414 1.414zM4 11a1 1 0 100-2H3a1 1 0 000 2h1z" clipRule="evenodd" />
           </svg>
         ) : (
           <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
             <path d="M17.293 13.293A8 8 0 016.707 2.707a8.001 8.001 0 1010.586 10.586z" />
           </svg>
         )}
       </button>
     );
   };

   export default QuickThemeToggle;
   ```

4. **Tailwind Dark Mode Configuration**:
   ```javascript
   // tailwind.config.ts
   export default {
     darkMode: 'class', // Enable class-based dark mode
     content: [
       './index.html',
       './src/**/*.{js,ts,jsx,tsx}',
     ],
     theme: {
       extend: {
         colors: {
           // Custom color palette for legal theme
           primary: {
             light: '#3b82f6',
             dark: '#60a5fa',
           },
           background: {
             light: '#ffffff',
             dark: '#111827',
           },
           surface: {
             light: '#f9fafb',
             dark: '#1f2937',
           },
         },
       },
     },
   };
   ```

5. **Persist Theme in Rust Backend**:
   ```rust
   // src-tauri/src/commands/settings.rs
   use serde::{Deserialize, Serialize};

   #[derive(Debug, Serialize, Deserialize)]
   pub struct ThemeSettings {
       pub theme: String, // "light", "dark", or "system"
   }

   #[tauri::command]
   pub async fn get_theme_setting() -> Result<String, String> {
       // Load from database or config file
       // For now, return default
       Ok("system".to_string())
   }

   #[tauri::command]
   pub async fn save_theme_setting(theme: String) -> Result<(), String> {
       // Save to database or config file
       // Validate theme value
       if !["light", "dark", "system"].contains(&theme.as_str()) {
           return Err("Invalid theme value".to_string());
       }

       // TODO: Save to database
       Ok(())
   }
   ```

**Success Criteria**:
- Theme toggle works in settings
- Quick toggle button in sidebar functions correctly
- Theme persists across app restarts
- System theme preference detected and applied
- All UI components properly styled in both light and dark modes
- Smooth transitions between themes

**Rust Files**:
- `src-tauri/src/commands/settings.rs` - Theme settings persistence

---

### Step 0.5: Basic Tauri Commands
**Priority**: High | **Effort**: Low | **Risk**: Low

**What**: Implement basic Tauri commands for frontend-backend communication.

**Implementation**:

1. **Settings Commands**:
   ```rust
   // src-tauri/src/commands/settings.rs
   use tauri::State;
   use crate::database::DatabaseManager;

   #[tauri::command]
   pub async fn get_setting(
       key: String,
       db: State<'_, DatabaseManager>,
   ) -> Result<Option<String>, String> {
       let conn = db.get_connection().await
           .ok_or("Database not initialized")?;

       // Query setting from database
       // Implementation details...

       Ok(Some("value".to_string()))
   }

   #[tauri::command]
   pub async fn set_setting(
       key: String,
       value: String,
       db: State<'_, DatabaseManager>,
   ) -> Result<(), String> {
       let conn = db.get_connection().await
           .ok_or("Database not initialized")?;

       // Save setting to database
       // Implementation details...

       Ok(())
   }

   #[tauri::command]
   pub fn get_app_version() -> String {
       env!("CARGO_PKG_VERSION").to_string()
   }
   ```

2. **Register Commands**:
   ```rust
   // src-tauri/src/main.rs
   mod commands;

   #[tokio::main]
   async fn main() {
       // ... database setup ...

       tauri::Builder::default()
           .setup(|app| {
               // ... setup code ...
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
   ```

3. **Frontend Service Layer**:
   ```typescript
   // src/services/settings.ts
   import { invoke } from '@tauri-apps/api/tauri';

   export const settingsService = {
     async getSetting(key: string): Promise<string | null> {
       return await invoke('get_setting', { key });
     },

     async setSetting(key: string, value: string): Promise<void> {
       await invoke('set_setting', { key, value });
     },

     async getAppVersion(): Promise<string> {
       return await invoke('get_app_version');
     },
   };
   ```

**Success Criteria**:
- Frontend can call Rust backend commands
- Settings can be saved and retrieved
- App version displays correctly
- Error handling works properly

---

### Step 0.6: Build & Package Verification
**Priority**: Critical | **Effort**: Low | **Risk**: Low

**What**: Verify the application can be built and packaged for distribution.

**Implementation**:

1. **Development Build**:
   ```bash
   # Run in development mode
   npm run tauri dev

   # Verify:
   # - App window opens
   # - Navigation works
   # - No console errors
   # - Database created
   # - Settings persist
   ```

2. **Production Build**:
   ```bash
   # Build for production
   npm run tauri build

   # Verify output in src-tauri/target/release/:
   # - Executable binary
   # - Installer packages (.msi for Windows, .dmg for macOS, .deb/.AppImage for Linux)
   ```

3. **Test Installation**:
   - Install from generated package
   - Run installed application
   - Verify all features work
   - Check app data directory location
   - Verify uninstall works cleanly

**Success Criteria**:
- Development build runs without errors
- Production build completes successfully
- Installer packages generated for target platforms
- Installed app runs correctly
- App data stored in correct location

---

### Step 0.6a: Auto-Updater System (Privacy-Respecting)
**Priority**: High | **Effort**: Low | **Risk**: Low

**What**: Implement a privacy-respecting automatic updater that allows pushing updates to clients without compromising the local-first, no-telemetry philosophy.

**Why This Is Compatible with Privacy-First Design**:
- **No User Data Sent**: Only checks GitHub API for version numbers - no analytics, no user identification
- **Anonymous Downloads**: Updates downloaded from GitHub Releases without tracking
- **User Control**: Users can disable auto-update checks or require manual approval
- **Local Installation**: All updates applied locally, no server-side processing
- **Industry Standard**: Used by privacy-focused apps like VS Code, Signal Desktop, and Obsidian

**Implementation**:

1. **Enable Tauri Updater**:
   ```toml
   # src-tauri/Cargo.toml
   [dependencies]
   tauri = { version = "2.0", features = ["updater"] }
   ```

   ```json
   // src-tauri/tauri.conf.json
   {
     "bundle": {
       "updater": {
         "active": true,
         "endpoints": [
           "https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research/releases/latest/download/latest.json"
         ],
         "dialog": true,
         "pubkey": "YOUR_PUBLIC_KEY_HERE"
       }
     }
   }
   ```

2. **Generate Signing Keys** (for security):
   ```bash
   # Generate keypair for signing updates (one-time setup)
   npm run tauri signer generate -- -w ~/.tauri/bear-llm.key

   # This generates:
   # - Private key: Keep secret, use in CI/CD
   # - Public key: Add to tauri.conf.json
   ```

3. **Update Check Command**:
   ```rust
   // src-tauri/src/commands/updater.rs
   use tauri::updater::UpdateResponse;
   use tauri::Manager;

   #[tauri::command]
   pub async fn check_for_updates(app: tauri::AppHandle) -> Result<UpdateResponse, String> {
       // This only sends current version to GitHub API
       // No user data, no telemetry, no tracking
       match app.updater().check().await {
           Ok(update) => Ok(update),
           Err(e) => Err(format!("Failed to check for updates: {}", e)),
       }
   }

   #[tauri::command]
   pub async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
       match app.updater().check().await {
           Ok(update) => {
               if update.is_update_available() {
                   update.download_and_install().await
                       .map_err(|e| format!("Failed to install update: {}", e))?;
               }
               Ok(())
           }
           Err(e) => Err(format!("Update check failed: {}", e)),
       }
   }
   ```

4. **Update UI Component**:
   ```typescript
   // src/components/UpdateNotification.tsx
   import React, { useEffect, useState } from 'react';
   import { invoke } from '@tauri-apps/api/tauri';
   import { useTranslation } from 'react-i18next';

   interface UpdateInfo {
       available: boolean;
       current_version: string;
       latest_version: string;
       date: string;
       body: string;
   }

   export const UpdateNotification: React.FC = () => {
       const { t } = useTranslation();
       const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
       const [checking, setChecking] = useState(false);
       const [installing, setInstalling] = useState(false);

       const checkForUpdates = async () => {
           try {
               setChecking(true);
               const info = await invoke<UpdateInfo>('check_for_updates');
               if (info.available) {
                   setUpdateInfo(info);
               }
           } catch (error) {
               console.error('Update check failed:', error);
           } finally {
               setChecking(false);
           }
       };

       const installUpdate = async () => {
           try {
               setInstalling(true);
               await invoke('install_update');
               // App will restart after update
           } catch (error) {
               console.error('Update installation failed:', error);
               setInstalling(false);
           }
       };

       useEffect(() => {
           // Check for updates on app start (optional, user-controllable)
           const autoCheck = localStorage.getItem('autoCheckUpdates') !== 'false';
           if (autoCheck) {
               checkForUpdates();
           }
       }, []);

       if (!updateInfo) return null;

       return (
           <div className="fixed top-4 right-4 bg-blue-500 text-white p-4 rounded-lg shadow-lg max-w-md">
               <h3 className="font-bold mb-2">
                   {t('updater.newVersionAvailable')}
               </h3>
               <p className="text-sm mb-2">
                   {t('updater.version')}: {updateInfo.latest_version}
               </p>
               <p className="text-xs mb-4 opacity-90">
                   {updateInfo.body}
               </p>
               <div className="flex gap-2">
                   <button
                       onClick={installUpdate}
                       disabled={installing}
                       className="px-4 py-2 bg-white text-blue-500 rounded hover:bg-gray-100"
                   >
                       {installing ? t('updater.installing') : t('updater.install')}
                   </button>
                   <button
                       onClick={() => setUpdateInfo(null)}
                       className="px-4 py-2 bg-blue-600 rounded hover:bg-blue-700"
                   >
                       {t('updater.later')}
                   </button>
               </div>
           </div>
       );
   };
   ```

5. **Add Update Settings**:
   ```typescript
   // In Settings page
   <div className="setting-group">
       <h3>{t('settings.updates.title')}</h3>
       <label>
           <input
               type="checkbox"
               checked={autoCheckUpdates}
               onChange={(e) => {
                   setAutoCheckUpdates(e.target.checked);
                   localStorage.setItem('autoCheckUpdates', String(e.target.checked));
               }}
           />
           {t('settings.updates.autoCheck')}
       </label>
       <button onClick={checkForUpdates}>
           {t('settings.updates.checkNow')}
       </button>
   </div>
   ```

6. **i18n Translations**:
   ```json
   // src/i18n/locales/en-GB.json
   {
     "updater": {
       "newVersionAvailable": "New Version Available",
       "version": "Version",
       "install": "Install Update",
       "installing": "Installing...",
       "later": "Remind Me Later",
       "checkNow": "Check for Updates",
       "upToDate": "You're up to date!"
     },
     "settings": {
       "updates": {
         "title": "Updates",
         "autoCheck": "Automatically check for updates",
         "checkNow": "Check for Updates Now",
         "description": "Updates are downloaded from GitHub. No user data is sent."
       }
     }
   }
   ```

7. **CI/CD Integration** (GitHub Actions):
   ```yaml
   # .github/workflows/release.yml
   name: Release Build
   on:
     push:
       tags:
         - 'v*'

   jobs:
     release:
       strategy:
         matrix:
           platform: [windows-latest, ubuntu-latest, macos-latest]
       runs-on: ${{ matrix.platform }}
       steps:
         - uses: actions/checkout@v4
         - uses: actions/setup-node@v4
           with:
             node-version: 20
         - uses: dtolnay/rust-toolchain@stable

         - name: Install dependencies
           run: npm ci

         - name: Build and sign
           env:
             TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }}
             TAURI_KEY_PASSWORD: ${{ secrets.TAURI_KEY_PASSWORD }}
           run: npm run tauri build

         - name: Upload Release Assets
           uses: softprops/action-gh-release@v1
           with:
             files: |
               src-tauri/target/release/bundle/**/*.msi
               src-tauri/target/release/bundle/**/*.dmg
               src-tauri/target/release/bundle/**/*.AppImage
               src-tauri/target/release/bundle/**/*.deb
   ```

**Privacy Guarantees**:
- ✅ **No Analytics**: Zero tracking of who checks for updates
- ✅ **No User Data**: Only version comparison, no system info sent
- ✅ **No Phoning Home**: Updates come from GitHub, not custom servers
- ✅ **User Control**: Can be completely disabled
- ✅ **Transparent**: Open source code shows exactly what data is sent (version number only)
- ✅ **Secure**: Updates cryptographically signed to prevent tampering
- ✅ **Local First**: Update files downloaded and installed locally

**What Data Is Sent?**:
```
Request to GitHub API:
GET https://api.github.com/repos/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research/releases/latest

No headers identifying the user, no cookies, no tracking
GitHub sees: IP address (standard for any HTTP request), no other data
Response: JSON with version number, download URL, changelog
```

**Success Criteria**:
- Updater checks GitHub Releases without errors
- Update notification appears when new version available
- Update can be installed successfully
- Settings allow disabling auto-checks
- No user data sent (verify in network logs)
- Works on all target platforms (Windows, macOS, Linux)

**Documentation**:
- Add "Privacy: What Data We Send" section to README
- Explain updater in terms of service / privacy policy
- Document how to disable updater
- Show network traffic examples

---

### Step 0.7: Testing Setup
**Priority**: Medium | **Effort**: Low | **Risk**: Low

**What**: Set up testing infrastructure for both frontend and backend.

**Implementation**:

1. **Frontend Testing (Jest + React Testing Library)**:
   ```bash
   npm install -D jest @testing-library/react @testing-library/jest-dom
   npm install -D @testing-library/user-event
   npm install -D ts-jest @types/jest
   ```

   ```typescript
   // jest.config.js
   module.exports = {
     preset: 'ts-jest',
     testEnvironment: 'jsdom',
     setupFilesAfterEnv: ['<rootDir>/src/setupTests.ts'],
     moduleNameMapper: {
       '\\.(css|less|scss|sass)$': 'identity-obj-proxy',
     },
   };
   ```

2. **Example Frontend Test**:
   ```typescript
   // src/components/__tests__/Sidebar.test.tsx
   import { render, screen } from '@testing-library/react';
   import { BrowserRouter } from 'react-router-dom';
   import Sidebar from '../Sidebar';

   test('renders navigation links', () => {
     render(
       <BrowserRouter>
         <Sidebar />
       </BrowserRouter>
     );

     expect(screen.getByText('Home')).toBeInTheDocument();
     expect(screen.getByText('Settings')).toBeInTheDocument();
     expect(screen.getByText('About')).toBeInTheDocument();
   });
   ```

3. **Backend Testing (Rust)**:
   ```rust
   // src-tauri/src/commands/settings.rs
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_get_app_version() {
           let version = get_app_version();
           assert!(!version.is_empty());
       }
   }
   ```

4. **Run Tests**:
   ```bash
   # Frontend tests
   npm test

   # Backend tests
   cd src-tauri && cargo test
   ```

**Success Criteria**:
- Test frameworks configured
- Example tests pass
- Tests can be run from command line
- CI/CD can run tests automatically

---

### Step 0.8: Case/Matter Organization (Lawyer-Centric Architecture)
**Priority**: Critical | **Effort**: Medium | **Risk**: Low

**What**: Implement case/matter-based data organization from the start - this is how lawyers actually work.

**Implementation**:

1. **Database Schema for Cases/Matters**:
   ```rust
   // src-tauri/migration/src/m20250101_000002_create_cases.rs
   use sea_orm_migration::prelude::*;

   #[async_trait::async_trait]
   impl MigrationTrait for Migration {
       async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
           manager
               .create_table(
                   Table::create()
                       .table(Cases::Table)
                       .if_not_exists()
                       .col(
                           ColumnDef::new(Cases::Id)
                               .integer()
                               .not_null()
                               .auto_increment()
                               .primary_key(),
                       )
                       .col(ColumnDef::new(Cases::Name).string().not_null())
                       .col(ColumnDef::new(Cases::ClientName).string().not_null())
                       .col(ColumnDef::new(Cases::CaseNumber).string().unique_key())
                       .col(ColumnDef::new(Cases::Description).text())
                       .col(ColumnDef::new(Cases::Status).string().not_null().default("active"))
                       .col(ColumnDef::new(Cases::CreatedAt).timestamp().not_null())
                       .col(ColumnDef::new(Cases::UpdatedAt).timestamp().not_null())
                       .to_owned(),
               )
               .await?;

           // Create conversations table linked to cases
           manager
               .create_table(
                   Table::create()
                       .table(Conversations::Table)
                       .if_not_exists()
                       .col(
                           ColumnDef::new(Conversations::Id)
                               .integer()
                               .not_null()
                               .auto_increment()
                               .primary_key(),
                       )
                       .col(ColumnDef::new(Conversations::CaseId).integer().not_null())
                       .col(ColumnDef::new(Conversations::Title).string().not_null())
                       .col(ColumnDef::new(Conversations::CreatedAt).timestamp().not_null())
                       .foreign_key(
                           ForeignKey::create()
                               .name("fk_conversation_case")
                               .from(Conversations::Table, Conversations::CaseId)
                               .to(Cases::Table, Cases::Id)
                               .on_delete(ForeignKeyAction::Cascade)
                       )
                       .to_owned(),
               )
               .await?;

           // Create messages table
           manager
               .create_table(
                   Table::create()
                       .table(Messages::Table)
                       .if_not_exists()
                       .col(
                           ColumnDef::new(Messages::Id)
                               .integer()
                               .not_null()
                               .auto_increment()
                               .primary_key(),
                       )
                       .col(ColumnDef::new(Messages::ConversationId).integer().not_null())
                       .col(ColumnDef::new(Messages::Role).string().not_null()) // "user" | "assistant"
                       .col(ColumnDef::new(Messages::Content).text().not_null())
                       .col(ColumnDef::new(Messages::IsAiGenerated).boolean().not_null().default(false))
                       .col(ColumnDef::new(Messages::WasEdited).boolean().not_null().default(false))
                       .col(ColumnDef::new(Messages::CreatedAt).timestamp().not_null())
                       .foreign_key(
                           ForeignKey::create()
                               .name("fk_message_conversation")
                               .from(Messages::Table, Messages::ConversationId)
                               .to(Conversations::Table, Conversations::Id)
                               .on_delete(ForeignKeyAction::Cascade)
                       )
                       .to_owned(),
               )
               .await
       }
   }

   #[derive(Iden)]
   enum Cases {
       Table,
       Id,
       Name,
       ClientName,
       CaseNumber,
       Description,
       Status,
       CreatedAt,
       UpdatedAt,
   }

   #[derive(Iden)]
   enum Conversations {
       Table,
       Id,
       CaseId,
       Title,
       CreatedAt,
   }

   #[derive(Iden)]
   enum Messages {
       Table,
       Id,
       ConversationId,
       Role,
       Content,
       IsAiGenerated,
       WasEdited,
       CreatedAt,
   }
   ```

2. **Case Management UI**:
   ```typescript
   // src/pages/Cases.tsx
   const CasesPage = () => {
     const [cases, setCases] = useState([]);
     const [selectedCase, setSelectedCase] = useState(null);

     return (
       <div className="flex h-full">
         <CaseList cases={cases} onSelect={setSelectedCase} />
         {selectedCase && (
           <CaseDetail case={selectedCase} />
         )}
       </div>
     );
   };

   // src/components/CaseList.tsx
   const CaseList = ({ cases, onSelect }) => {
     return (
       <div className="w-80 border-r">
         <div className="p-4">
           <button className="btn-primary w-full">New Case</button>
         </div>
         <div className="divide-y">
           {cases.map(case => (
             <div
               key={case.id}
               className="p-4 hover:bg-gray-50 cursor-pointer"
               onClick={() => onSelect(case)}
             >
               <h3 className="font-medium">{case.name}</h3>
               <p className="text-sm text-gray-600">{case.clientName}</p>
               <p className="text-xs text-gray-500">{case.caseNumber}</p>
             </div>
           ))}
         </div>
       </div>
     );
   };
   ```

**Why This Matters**:
- Lawyers organize everything by case/client
- Ensures GDPR Purpose Limitation from the start
- Natural data isolation (each case is separate)
- Prepares for future features (all tied to specific cases)

**Success Criteria**:
- Can create, view, and select cases
- Database enforces case-based organization
- All conversations tied to specific cases
- UI clearly shows which case is active

---

### Step 0.9: Human-in-the-Loop Review UI Pattern
**Priority**: High | **Effort**: Low | **Risk**: Low

**What**: Implement the preview/review/approve workflow UI pattern that will be used for all AI operations (even though we don't have AI yet).

**Implementation**:

```typescript
// src/components/ReviewModal.tsx
interface ReviewModalProps {
  title: string;
  content: string;
  metadata?: {
    source?: 'ai' | 'user';
    model?: string;
    timestamp?: string;
  };
  onApprove: (edited: string) => void;
  onReject: () => void;
  onEdit: (edited: string) => void;
}

const ReviewModal: React.FC<ReviewModalProps> = ({
  title,
  content,
  metadata,
  onApprove,
  onReject,
  onEdit,
}) => {
  const [editedContent, setEditedContent] = useState(content);
  const [isEditing, setIsEditing] = useState(false);

  return (
    <Dialog>
      <div className="max-w-4xl p-6">
        <h2 className="text-xl font-bold mb-4">{title}</h2>

        {/* Show AI badge if applicable */}
        {metadata?.source === 'ai' && (
          <AIBadge model={metadata.model} />
        )}

        {/* Preview mode */}
        {!isEditing ? (
          <div className="prose max-w-none mb-6 p-4 bg-gray-50 rounded">
            {editedContent}
          </div>
        ) : (
          <textarea
            className="w-full h-64 p-4 border rounded mb-6"
            value={editedContent}
            onChange={(e) => setEditedContent(e.target.value)}
          />
        )}

        {/* Action buttons */}
        <div className="flex gap-3 justify-end">
          <button
            className="btn-secondary"
            onClick={onReject}
          >
            Reject
          </button>

          {!isEditing ? (
            <>
              <button
                className="btn-secondary"
                onClick={() => setIsEditing(true)}
              >
                Edit
              </button>
              <button
                className="btn-primary"
                onClick={() => onApprove(editedContent)}
              >
                Approve
              </button>
            </>
          ) : (
            <>
              <button
                className="btn-secondary"
                onClick={() => {
                  setEditedContent(content);
                  setIsEditing(false);
                }}
              >
                Cancel Edit
              </button>
              <button
                className="btn-primary"
                onClick={() => {
                  onEdit(editedContent);
                  setIsEditing(false);
                }}
              >
                Save Changes
              </button>
            </>
          )}
        </div>
      </div>
    </Dialog>
  );
};
```

**Why This Matters**:
- AI Act requires human-in-the-loop for all AI outputs
- Establish the pattern NOW (in wireframe)
- When AI is added in Phase 3, just plug into existing workflow
- Users get familiar with review process early

**Success Criteria**:
- Review modal can display content
- Edit functionality works
- Approve/Reject/Edit actions trigger callbacks
- UI pattern is established for future AI integration

---

### Step 0.10: AI Transparency Components (Placeholder)
**Priority**: Medium | **Effort**: Low | **Risk**: Low

**What**: Create AI badge and labeling components (even though there's no AI yet). These will be ready when AI is integrated.

**Implementation**:

```typescript
// src/components/AIBadge.tsx
interface AIBadgeProps {
  model?: string;
  timestamp?: string;
  wasEdited?: boolean;
  className?: string;
}

export const AIBadge: React.FC<AIBadgeProps> = ({
  model,
  timestamp,
  wasEdited,
  className,
}) => {
  return (
    <div className={`inline-flex items-center gap-2 px-3 py-1 rounded-full bg-blue-100 text-blue-800 text-sm ${className}`}>
      <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
        <path d="M10 2a8 8 0 100 16 8 8 0 000-16zm1 11H9v-2h2v2zm0-4H9V5h2v4z"/>
      </svg>

      <span className="font-medium">
        AI Generated
        {wasEdited && " (Edited)"}
      </span>

      {model && (
        <span className="text-xs opacity-75">
          {model}
        </span>
      )}

      {timestamp && (
        <span className="text-xs opacity-75">
          {new Date(timestamp).toLocaleString()}
        </span>
      )}
    </div>
  );
};

// Different badge variants
export const AIAssistedBadge = () => (
  <span className="px-2 py-1 rounded bg-purple-100 text-purple-800 text-xs">
    AI-Assisted
  </span>
);

export const HumanContentBadge = () => (
  <span className="px-2 py-1 rounded bg-gray-100 text-gray-800 text-xs">
    Human Written
  </span>
);

// Message component with AI badge
const Message = ({ role, content, isAiGenerated, wasEdited }) => {
  return (
    <div className={`message ${role === 'user' ? 'user' : 'assistant'}`}>
      {isAiGenerated && <AIBadge wasEdited={wasEdited} />}
      <div className="content">{content}</div>
    </div>
  );
};
```

**Why This Matters**:
- AI Act Article 52 requires clear AI content labeling
- Components are ready when AI is integrated
- Designers can see the intended UX now
- Compliance pattern established early

**Success Criteria**:
- AI badges render correctly (with placeholder data)
- Different badge variants for different scenarios
- Ready to use when AI is integrated in Phase 3

---

### Step 0.11: Basic Audit Log Structure
**Priority**: Medium | **Effort**: Low | **Risk**: Low

**What**: Create audit log database table and basic logging infrastructure (for future compliance).

**Implementation**:

```rust
// src-tauri/migration/src/m20250101_000003_create_audit_log.rs
use sea_orm_migration::prelude::*;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AuditLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuditLog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuditLog::Action).string().not_null())
                    .col(ColumnDef::new(AuditLog::CaseId).integer())
                    .col(ColumnDef::new(AuditLog::EntityType).string())
                    .col(ColumnDef::new(AuditLog::EntityId).integer())
                    .col(ColumnDef::new(AuditLog::Details).json())
                    .col(ColumnDef::new(AuditLog::Timestamp).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }
}

// src-tauri/src/services/audit.rs
use sea_orm::*;
use serde_json::Value;

pub struct AuditService;

impl AuditService {
    pub async fn log(
        db: &DatabaseConnection,
        action: &str,
        case_id: Option<i32>,
        entity_type: Option<&str>,
        entity_id: Option<i32>,
        details: Value,
    ) -> Result<(), DbErr> {
        // Insert audit log entry
        // For now, just structure - actual logging added in Phase 1
        Ok(())
    }
}
```

**Why This Matters**:
- GDPR Article 30 requires processing records
- Structure in place for Phase 1 compliance features
- All future operations can log to this table
- Foundation for compliance audits

**Success Criteria**:
- Audit log table created
- Basic logging infrastructure in place
- Ready for Phase 1 implementation
- Database schema supports all audit requirements

---

### Phase 0 Summary

**Deliverables**:
- ✅ Tauri + React + TypeScript project initialized
- ✅ Basic UI wireframe with navigation (inspired by Kaas, LM Studio, Jan AI)
- ✅ SQLite database with migrations
- ✅ **Case/Matter organization structure** (lawyers work by case)
- ✅ **Human-in-the-loop review UI pattern** (compliance foundation)
- ✅ **AI transparency label components** (ready for AI integration)
- ✅ **Basic audit log structure** (compliance foundation)
- ✅ i18n framework (EN/NL/DE/FR/ZH support)
- ✅ **Theme toggle (Dark/Light mode with system detection)**
- ✅ Basic Tauri commands working
- ✅ Build and packaging verified
- ✅ Testing infrastructure in place

**Technology Stack Verified**:
- Frontend: React 18 + TypeScript + Vite
- Backend: Rust + Tauri 2.0
- Database: SQLite + Sea-ORM
- Styling: Tailwind CSS (with dark mode support)
- i18n: i18next + react-i18next (5 languages)
- Testing: Jest + React Testing Library + Cargo test

**Compliance Patterns Established** (UI only, no AI yet):
- ✅ Preview/Review/Approve workflow UI
- ✅ AI badge components (placeholder for future AI features)
- ✅ Audit log table structure
- ✅ Case/matter isolation

**Complete Rust File Structure (Phase 0)**:
```
src-tauri/
├── src/
│   ├── main.rs                          # Application entry point
│   ├── lib.rs                           # Library exports
│   ├── commands/
│   │   ├── mod.rs                       # Command module exports
│   │   ├── settings.rs                  # Settings commands (theme, language)
│   │   ├── chat.rs                      # Chat management commands
│   │   ├── case.rs                      # Case/matter management commands
│   │   └── audit.rs                     # Audit logging commands
│   ├── database/
│   │   ├── mod.rs                       # Database manager
│   │   └── models.rs                    # SeaORM models
│   ├── services/
│   │   ├── mod.rs                       # Service exports
│   │   ├── db.rs                        # Database service
│   │   ├── cache.rs                     # Caching service
│   │   └── llm/                         # LLM services (Phase 3)
│   │       ├── mod.rs
│   │       ├── client.rs
│   │       ├── chat.rs
│   │       ├── models.rs
│   │       ├── types.rs
│   │       ├── utils.rs
│   │       └── providers/
│   │           ├── mod.rs
│   │           ├── types.rs
│   │           └── ollama/
│   │               ├── mod.rs
│   │               ├── config.rs
│   │               ├── chat.rs
│   │               └── models.rs
│   ├── core/
│   │   ├── mod.rs                       # Core exports
│   │   └── handle.rs                    # App handle management
│   ├── utils.rs                         # Utility functions
│   ├── errors.rs                        # Error types
│   ├── init.rs                          # Initialization logic
│   ├── log_utils.rs                     # Logging utilities
│   ├── process_helper.rs                # Process management
│   └── crash_handler.rs                 # Crash handling
├── migration/
│   ├── Cargo.toml                       # Migration dependencies
│   └── src/
│       ├── lib.rs                       # Migration runner
│       ├── m20250101_000001_create_settings.rs
│       ├── m20250102_000002_create_chats.rs
│       ├── m20250103_000003_create_messages.rs
│       ├── m20250104_000004_create_cases.rs
│       └── m20250105_000005_create_audit_logs.rs
├── entity/
│   ├── Cargo.toml                       # Entity dependencies
│   └── src/
│       ├── settings.rs                  # Settings entity
│       ├── chats.rs                     # Chats entity
│       ├── messages.rs                  # Messages entity
│       ├── cases.rs                     # Cases entity
│       └── audit_logs.rs                # Audit logs entity
├── Cargo.toml                           # Main dependencies
└── tauri.conf.json                      # Tauri configuration
```

**What's NOT Implemented Yet**:
- AI features (coming in Phase 3)
- Full GDPR compliance (coming in Phase 1)
- Encryption (coming in Phase 1)
- PII detection (coming in Phase 4)
- Legal research / RAG (coming in Phase 7 - for multi-client reuse)
- Any actual AI-powered legal assistance

**Next**: Phase 1 - GDPR Compliance (build on established patterns)

---

## 🏛️ Lawyer-Centric Architecture Note

**This application is designed around how lawyers actually work:**

### Client Work (Case/Matter-Specific)
**Phases 0-6**: Everything is organized by case/matter
- Each case is isolated (GDPR Purpose Limitation)
- Documents, conversations, and analysis tied to specific clients
- Privacy is paramount - no mixing of client data
- **Use case**: "Help me draft a confidentiality clause for the Johnson case"

### Legal Research (Multi-Client Reusable Knowledge)
**Phase 7+**: General legal knowledge for reuse
- NOT tied to specific clients
- Searchable across all your firm's knowledge
- Can be cited in multiple cases
- **Use case**: "What are the GDPR precedents for cookie consent?"
- **Implementation**: RAG (Retrieval-Augmented Generation) with vector database

**The distinction is clear:**
- **Before Phase 7**: All work is case-specific, client-confidential
- **Phase 7+**: Add capability for general legal research (still 100% local)

---

## Phase 1: GDPR Compliance (Priority: CRITICAL)
**Legal Foundation - Articles 5, 12-17, 25, 30, 32**

### Step 1: Data Minimization (Art. 5(1)(c))
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

## Phase 2: AI Act Compliance (Priority: HIGH)
**Legal Requirement - Article 52**

### Step 13: AI Transparency Labels (Art. 52)
**Priority**: Critical | **Effort**: Low | **Legal Risk**: High

**What**: Clearly label AI-generated or AI-assisted content in the interface.

**Implementation**:
- Add "AI Response" badge to all AI-generated content
- Add "AI-Assisted" badge to human-edited AI content
- Color-coded indicators (e.g., blue for AI, gray for human)
- Icons distinguishing AI from human content
- Persistent labels that cannot be hidden

**Technical Details**:
- Add metadata field `content_source: "ai" | "human" | "ai-assisted"`
- Create reusable badge components
- Implement in chat interface, document viewer, exports
- Add to print/export outputs
- Store source information in database

**Success Criteria**:
- All AI content visually distinct from human content
- Labels visible in all viewing contexts
- Labels persist in exports and prints
- Clear visual distinction between content types

---

### Step 14: AI Use Explanation (Art. 52)
**Priority**: Critical | **Effort**: Low | **Legal Risk**: Medium

**What**: Provide a short "About / Help" section explaining AI usage and local processing.

**Implementation**:
- Create "About AI" section in Help menu
- Explain clearly:
  - "This app uses local AI to assist with drafting, summarizing, and search"
  - "No data ever leaves your device"
  - "All processing happens on your computer"
  - "You maintain full control over all AI interactions"
- Include model information and capabilities
- Explain limitations and appropriate use cases
- Translate to all supported languages

**Technical Details**:
- Add Help menu with AI explanation
- Create onboarding tooltip explaining AI features
- Add contextual help bubbles in AI interfaces
- Include in first-run tutorial

**Success Criteria**:
- AI usage clearly explained in non-technical terms
- Accessible from Help menu
- Shown during onboarding
- Available in all languages

---

### Step 15: Output Provenance (Art. 52)
**Priority**: High | **Effort**: Medium | **Legal Risk**: Medium

**What**: Embed basic metadata in AI-generated outputs: model name, version/date, anonymization settings.

**Implementation**:
- Add metadata to all AI-generated content:
  - Model name (e.g., "Mistral-7B-v0.2")
  - Model version/date
  - Generation timestamp
  - Anonymization settings used
  - PII detection layer applied
  - User edits (if any)
- Store in document properties for exports
- Add optional footer to generated documents
- Include in audit logs

**Technical Details**:
- Create metadata schema for AI outputs
- Implement metadata embedding in:
  - PDF exports (document properties)
  - Markdown exports (YAML frontmatter)
  - Text exports (footer)
  - Database records
- Add metadata viewer in document properties dialog

**Metadata Example**:
```yaml
---
generated_by: BEAR LLM AI v0.0.18
model: mistral-7b-instruct-v0.2
generated_at: 2025-10-24T18:30:00Z
anonymization: layer1-regex
user_edited: true
edit_count: 3
---
```

**Success Criteria**:
- All AI outputs tagged with complete metadata
- Metadata embedded in exports
- Metadata viewable within application
- Metadata included in audit trail

---

### Step 16: Human-in-the-Loop (Art. 52)
**Priority**: Critical | **Effort**: Medium | **Legal Risk**: High

**What**: Always let the user review and optionally edit AI outputs before saving or exporting.

**Implementation**:
- No automatic saving of AI outputs
- Mandatory preview step for all AI generations
- Edit capability before accepting AI content
- Clear "Accept" vs "Edit" vs "Reject" options
- Regenerate option with different parameters
- Track human review in metadata

**Technical Details**:
- Create preview/edit modal for AI outputs
- Implement three-button workflow: Accept/Edit/Reject
- Add diff view showing AI changes
- Store review decisions in audit log
- Prevent bypass of review step (no "auto-accept" option)

**UI Flow**:
```
AI Generation → Preview Modal → User Reviews →
→ [Accept] → Save with metadata
→ [Edit] → Edit View → Save with "user-edited" flag
→ [Reject] → Discard
→ [Regenerate] → New generation with adjustable parameters
```

**Success Criteria**:
- Zero AI outputs saved without user review
- Edit functionality available for all outputs
- User review tracked in metadata
- No way to bypass review step

---

### Step 17: Label AI-Modified Data (Art. 52)
**Priority**: High | **Effort**: Low | **Legal Risk**: Medium

**What**: Add a visible note or metadata tag: "Processed by AI (local model)."

**Implementation**:
- Add footer to AI-processed documents:
  - "This document was processed using local AI assistance"
  - "Generated by BEAR LLM AI v[version]"
  - "Model: [model-name]"
  - "Date: [timestamp]"
- Make footer customizable (can be disabled)
- Add watermark option for highly sensitive documents
- Include in export formats: PDF, DOCX, TXT

**Technical Details**:
- Implement configurable footer templates
- Add footer insertion to export pipeline
- Create watermark option for PDFs
- Store footer preference in project settings
- Allow removal for final client deliverables (with audit log)

**Success Criteria**:
- AI processing clearly indicated in outputs
- Footer visible but not intrusive
- Configurable per project/export
- Removal tracked in audit log

---

### Step 18: Local Model Default (Art. 52)
**Priority**: Critical | **Effort**: High | **Legal Risk**: Medium

**What**: Run entirely offline with models stored locally. Only download model files after explicit user consent.

**Implementation**:
- No cloud AI services (OpenAI, Anthropic, etc.) by default
- All models downloaded and stored locally
- Explicit consent dialog before downloading models
- Show model size, disk space required, download time estimate
- Allow user to review and delete downloaded models
- Model verification (checksums) before use

**Technical Details**:
- Implement model manager in settings
- Show model list: installed, available, recommended
- Add download progress indicators
- Verify model integrity after download
- Implement model deletion functionality
- Store models in user-configurable location

**Model Download Consent Dialog**:
```
Download AI Model?

Model: Mistral-7B-Instruct-v0.2
Size: 4.1 GB
Required disk space: 8.2 GB (with safety margin)
Estimated download time: 15-45 minutes

This model will be stored on your device and used for local AI processing.
No data will ever be sent outside your computer.

[View Details] [Cancel] [Download]
```

**Success Criteria**:
- No models bundled with application
- Explicit consent before each download
- Clear information about disk space and capabilities
- Model management interface in settings
- Application remains functional without models (limited features)

---

### Step 19: Low-Risk by Design Classification (Art. 52, Recitals)
**Priority**: Medium | **Effort**: Low | **Legal Risk**: Low

**What**: Maintain minimal risk classification through user-in-control workflows.

**Implementation**:
- Ensure AI never makes autonomous decisions
- All AI outputs are suggestions, not final actions
- User maintains complete control over all operations
- No automated legal advice or binding decisions
- Clear disclaimers about AI limitations
- Document low-risk classification in compliance documentation

**Technical Details**:
- Add disclaimers to AI interfaces
- Implement mandatory review steps
- Prevent automation of critical workflows
- Document user-in-control architecture
- Create compliance documentation

**Disclaimer Example**:
```
⚠️ AI Assistant Notice

This AI assistant provides suggestions and drafts for your review.
It does not provide legal advice or make decisions on your behalf.
Always review and verify AI-generated content before use.
Consult qualified legal professionals for specific legal matters.
```

**Success Criteria**:
- Clear classification as minimal risk AI system
- User maintains control over all operations
- Disclaimers visible at all AI interaction points
- Documentation supports low-risk classification

---

## Phase 3: Local AI Infrastructure (Priority: HIGH)

### Step 20: Local Model Support with Candle and Hugging Face (Art. 52)
**Priority**: Critical | **Effort**: Very High | **Legal Risk**: Medium

**What**: Run all inference offline with Candle + Hugging Face. Allow manual model downloads.

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

## Phase 4: Advanced PII Protection (Priority: HIGH)

### Step 21: PII Layer 2 - Named Entity Recognition (NER)
**Priority**: High | **Effort**: High | **Legal Risk**: Medium

**What**: Add named-entity recognition for context-aware anonymization.

**Implementation**:
- Integrate local NER model (Hugging Face)
- Detect entities in context:
  - PERSON (names, titles)
  - ORGANIZATION (companies, firms, courts)
  - LOCATION (addresses, cities, countries)
  - DATE (specific dates, birth dates)
  - MONEY (amounts, financial info)
  - LAW (legal references - preserve these!)
  - CASE (case numbers, file references)
- Context-aware redaction (preserve legal citations)
- Smart anonymization (consistent replacement within document)
- Multi-language support (Dutch, German, English)

**NER Models**:
- English: `dslim/bert-base-NER`
- Multilingual: `xlm-roberta-large-finetuned-conll03-english`
- Dutch: `wietsedv/bert-base-dutch-cased-finetuned-conll2002-ner`
- German: `dbmdz/bert-large-cased-finetuned-conll03-german`
- Legal-specific: Custom fine-tuned model (future)

**Technical Details**:
- Download NER models to local storage
- Run inference locally using Candle or ONNX runtime
- Combine regex (Layer 1) + NER (Layer 2) for comprehensive coverage
- Implement entity linking (same person = same replacement)
- Create whitelist for legal terms and citations

**Smart Anonymization Example**:
```
Original:
"John Smith filed a complaint under Article 6 GDPR on 2024-03-15.
Mr. Smith claimed that Acme Corp violated his privacy rights."

After Layer 2 NER:
"[PERSON-A] filed a complaint under Article 6 GDPR on [DATE-1].
[PERSON-A] claimed that [ORGANIZATION-A] violated his privacy rights."

Note: "Article 6 GDPR" preserved as legal reference
```

**Performance Targets**:
- NER inference: >100 words/second
- Memory usage: <2GB
- Accuracy: >90% F1 score on legal documents
- Processing time: <5 seconds per page

**Success Criteria**:
- Context-aware entity detection
- Consistent anonymization within documents
- Legal references preserved
- Multi-language support
- Combined Layer 1 + Layer 2 detection rate >98%
- False positive rate <2%

**Rust Files (Phase 4 - Advanced PII with NER)**:
```
src-tauri/src/
├── pii/                                 # PII detection (expanded from Phase 1)
│   ├── mod.rs                           # PII module exports
│   ├── regex_detector.rs                # Layer 1: Regex patterns
│   ├── patterns.rs                      # Regex patterns library
│   ├── ner_detector.rs                  # Layer 2: NER detection (NEW)
│   ├── ner_models.rs                    # NER model management (NEW)
│   ├── entity_linker.rs                 # Cross-reference entities (NEW)
│   ├── redactor.rs                      # Redaction engine
│   ├── anonymizer.rs                    # Smart anonymization (NEW)
│   ├── whitelist.rs                     # Legal term whitelist (NEW)
│   └── multi_language.rs                # Multi-language support (NEW)
├── ml/
│   ├── mod.rs                           # ML module exports
│   ├── onnx_runtime.rs                  # ONNX runtime integration
│   ├── model_inference.rs               # ML inference
│   └── embeddings.rs                    # Text embeddings
└── commands/
    ├── pii.rs                           # PII detection commands (expanded)
    └── anonymization.rs                 # Anonymization commands (NEW)

migration/src/
├── m20250111_000011_add_ner_logs.rs     # NER detection logs
├── m20250112_000012_add_entity_map.rs   # Entity mapping table
└── m20250113_000013_add_whitelists.rs   # Legal term whitelists

entity/src/
├── pii_detections.rs                    # PII detection results
├── entities.rs                          # Detected entities
└── anonymization_maps.rs                # Anonymization mappings
```

---

## Phase 5: Optional Advanced Integration (Priority: MEDIUM)

### Step 22: PII Layer 3 - Optional Advanced Anonymization Bridge
**Priority**: Medium | **Effort**: High | **Legal Risk**: Low

**What**: Optional advanced anonymization plug-in (e.g., Presidio or custom compliance bridge).

**Implementation**:
- Optional plugin architecture for advanced anonymization
- Support Microsoft Presidio (local deployment)
- Support custom compliance tools
- Advanced features:
  - Cross-document entity resolution
  - Temporal reasoning (event timelines)
  - Relationship extraction
  - Risk scoring
  - Custom entity types
  - Industry-specific rules (legal, medical, financial)
- Remains fully local (no cloud services)
- User can enable/disable per project

**Presidio Integration** (Local Container):
- Run Presidio in local Docker container
- Expose REST API on localhost only
- No external network access
- Support for 50+ PII entity types
- Custom recognizer support
- Integration with existing Layer 1 + Layer 2

**Technical Details**:
- Plugin architecture using WebAssembly or shared libraries
- Local containerization (Docker or Podman)
- REST API communication (localhost only)
- Fallback to Layer 1 + Layer 2 if plugin unavailable
- Plugin management in settings

**Advanced Features**:
- **Cross-document analysis**: Track entities across multiple files
- **Risk scoring**: Automatically flag high-risk PII
- **Compliance rules**: Configurable rules per regulation (GDPR, HIPAA, etc.)
- **Custom dictionaries**: Add industry-specific terms
- **Anonymization strategies**:
  - Redaction
  - Replacement (synthetic data)
  - Tokenization
  - Masking
  - Encryption

**Success Criteria**:
- Optional installation (not required for core functionality)
- Fully local deployment
- Extends Layer 1 + Layer 2 capabilities
- User-friendly plugin management
- Performance impact <10% when enabled
- Support for custom compliance rules

---

### Step 23: Prompt Library & Template System
**Priority**: High | **Effort**: Medium | **Legal Risk**: Low

**What**: Local prompt library allowing users to import, organize, and reuse prompts via txt/md files. No telemetry, fully offline.

**Implementation**:

1. **Prompt Library Structure**:
   ```
   ~/BEAR_LLM_AI/prompts/
   ├── system/                    # Built-in system prompts
   │   ├── contract_review.md
   │   ├── gdpr_advisor.md
   │   ├── case_summarizer.md
   │   ├── legal_researcher.md
   │   └── compliance_checker.md
   ├── user/                      # User-created prompts
   │   ├── my_custom_prompt.md
   │   └── client_intake.txt
   ├── templates/                 # Document templates
   │   ├── nda_template.md
   │   ├── privacy_policy.md
   │   └── dpa_template.md
   └── shared/                    # Shared across cases (optional)
       └── common_clauses.md
   ```

2. **Prompt File Format** (.md or .txt):
   ```markdown
   ---
   name: Contract Review Assistant
   description: Analyzes contracts for potential risks and missing clauses
   category: contract_analysis
   language: en
   tags: [contract, review, risk-assessment]
   version: 1.0
   created: 2025-01-26
   author: User
   license_tier: basic  # basic, pro, enterprise, free
   ---

   # Contract Review Prompt

   You are a legal contract reviewer. Analyze the following contract for:
   1. Missing essential clauses
   2. Ambiguous language
   3. Potential liability issues
   4. GDPR compliance (if applicable)
   5. Unusual or non-standard terms

   Focus on Dutch/Belgian law where applicable.

   Contract to review:
   {CONTRACT_TEXT}
   ```

3. **Prompt Library UI**:
   - **Browse View**: Grid or list view of all prompts
   - **Categories**: Filter by category, language, tier
   - **Search**: Full-text search across prompts
   - **Preview**: Markdown preview before using
   - **Import**: Drag-and-drop .txt/.md files
   - **Export**: Export prompts for sharing (respecting licensing)
   - **Edit**: Built-in markdown editor
   - **Variables**: Support {VARIABLE_NAME} placeholders
   - **Version Control**: Track prompt versions

4. **Prompt Usage**:
   - Select prompt from library
   - Fill in variables (interactive form)
   - Preview final prompt
   - Execute with current case context
   - Save results to case

5. **Built-in System Prompts** (Included):
   - **Contract Reviewer**: Analyze contracts for risks
   - **GDPR Advisor**: Answer GDPR compliance questions
   - **Case Summarizer**: Summarize case files
   - **Legal Researcher**: Research legal questions (Pro tier with law library)
   - **Compliance Checker**: Check documents against regulations
   - **Citation Finder**: Find and verify legal citations
   - **Timeline Builder**: Extract chronological events
   - **Due Diligence**: M&A and due diligence analysis

6. **Tier-Based Prompt Access**:
   - **Free Tier**: 3 basic system prompts
   - **Basic Tier**: All system prompts + unlimited custom prompts
   - **Pro Tier**: All Basic + premium prompts with law library integration
   - **Enterprise Tier**: All Pro + team prompt sharing + centralized library

**Technical Details**:
- Store prompts in user data directory
- No cloud sync (100% local)
- Support .txt and .md formats
- YAML frontmatter for metadata
- Variable substitution engine
- Markdown rendering
- Full-text search index
- Category and tag system
- Tier-based access control

**Rust Backend**:
```rust
// src-tauri/src/prompts/mod.rs
pub struct PromptLibrary {
    prompts: Vec<Prompt>,
    system_prompts: Vec<SystemPrompt>,
    user_dir: PathBuf,
}

pub struct Prompt {
    id: String,
    name: String,
    description: String,
    category: String,
    content: String,
    variables: Vec<String>,
    tier: LicenseTier,
    metadata: PromptMetadata,
}

impl PromptLibrary {
    pub fn load_prompts() -> Result<Vec<Prompt>, Error>;
    pub fn import_prompt(path: PathBuf) -> Result<Prompt, Error>;
    pub fn save_prompt(prompt: &Prompt) -> Result<(), Error>;
    pub fn search_prompts(query: &str) -> Vec<Prompt>;
    pub fn get_by_category(category: &str) -> Vec<Prompt>;
    pub fn check_tier_access(prompt: &Prompt, user_tier: LicenseTier) -> bool;
}
```

**Success Criteria**:
- Import .txt/.md files with drag-and-drop
- Full-text search across 1000+ prompts in <500ms
- Variable substitution works correctly
- Tier-based access control functional
- No telemetry or cloud sync
- Markdown preview and editing
- Export functionality works

**Rust Files**:
```
src-tauri/src/
├── prompts/
│   ├── mod.rs                           # Prompt library manager
│   ├── parser.rs                        # Parse .md/.txt files with frontmatter
│   ├── variables.rs                     # Variable substitution engine
│   ├── search.rs                        # Full-text search
│   ├── categories.rs                    # Category management
│   ├── tier_control.rs                  # License tier access control
│   └── system_prompts.rs                # Built-in system prompts
├── templates/
│   ├── mod.rs                           # Template management
│   ├── renderer.rs                      # Markdown rendering
│   └── validator.rs                     # Template validation
└── commands/
    ├── prompts.rs                       # Prompt commands
    └── templates.rs                     # Template commands

migration/src/
├── m20250114_000014_add_prompts.rs      # Prompt library table
└── m20250115_000015_add_templates.rs    # Templates table

entity/src/
├── prompts.rs                           # Prompt entity
└── templates.rs                         # Template entity
```

---

## Phase 5a: Licensing & Payment System (Priority: HIGH)
**Revenue Model - Supporting Social Mission**

### Step 24: License Tier System
**Priority**: Critical | **Effort**: High | **Legal Risk**: Medium

**What**: Implement tiered licensing with local validation and payment integration (Stripe, Mollie, or Plug and Play).

**Pricing Tiers**:

#### 🆓 **Free Tier** (Social Mission)
**Target**: Social legal institutes (Rechtswinkels, sociale advocatuur, legal aid clinics)
**Price**: €0 / Free
**Features**:
- ✅ Basic chat interface
- ✅ Local AI inference (up to 7B model)
- ✅ 3 system prompts
- ✅ RAG on local files (up to 100 documents)
- ✅ Basic PII detection (Layer 1 - regex)
- ✅ Export to PDF/TXT
- ✅ GDPR & AI Act compliant
- ✅ All 5 languages (EN/NL/DE/FR/ZH)
- ❌ No prompt library access
- ❌ No law library RAG
- ❌ No advanced templates

**Verification Required**:
- Proof of non-profit status (KvK registration)
- Organization email (no personal emails)
- Application form with mission statement
- Renewable annually

---

#### 💼 **Basic Tier**
**Target**: Solo practitioners, freelance lawyers, small firms
**Price**: €9.99/month or €99/year (save 17%)
**USD**: $9.99/month or $99/year

**Features**:
- ✅ All Free Tier features
- ✅ **Full prompt library** (unlimited custom prompts)
- ✅ **All system prompts** (8+ built-in)
- ✅ **RAG on local files** (unlimited documents)
- ✅ **Document templates** (contracts, NDAs, privacy policies)
- ✅ Advanced PII detection (Layer 1 + Layer 2 NER)
- ✅ Case/matter organization
- ✅ Export to DOCX/PDF
- ✅ Dark mode
- ✅ Priority email support
- ❌ No law library RAG
- ❌ No team features

---

#### 🏆 **Pro Tier**
**Target**: Individual lawyers, boutique firms, specialists
**Price**: €19.99/month or €199/year (save 17%)
**USD**: $19.99/month or $199/year

**Features**:
- ✅ All Basic Tier features
- ✅ **Law Library RAG** (search across legal databases)
  - GDPR full text + case law
  - National laws (NL/BE/DE/FR)
  - EU regulations and directives
  - Case law databases (where available)
  - Legal doctrine and commentary
- ✅ **Advanced legal research prompts**
- ✅ **Citation verification**
- ✅ **Precedent finder**
- ✅ PII Layer 3 (Presidio integration - optional)
- ✅ Advanced document comparison
- ✅ Compliance checklists
- ✅ Timeline and evidence builder
- ✅ Priority support with 24h response
- ❌ No team/centralized billing

---

#### 🏢 **Enterprise Tier**
**Target**: Law firms, legal departments, multi-user teams
**Price**: €19.99/month per seat (minimum 3 seats)
**USD**: $19.99/month per seat

**Features**:
- ✅ All Pro Tier features (per user)
- ✅ **Centralized billing** (one invoice for all seats)
- ✅ **Team prompt library** (shared prompts across team)
- ✅ **Team template library**
- ✅ **Usage analytics** (aggregate, anonymized)
- ✅ **SSO integration** (optional)
- ✅ **Bulk license management**
- ✅ **Dedicated support** (priority + phone)
- ✅ **Custom onboarding**
- ✅ **Quarterly review calls**
- ✅ **SLA guarantee** (99.5% uptime for license validation)

**Minimum**: 3 seats
**Volume discounts**:
- 10+ seats: 10% discount
- 25+ seats: 15% discount
- 50+ seats: 20% discount
- 100+ seats: Contact for custom pricing

---

### Payment Gateway Integration

**Supported Payment Providers**:

1. **Stripe** (Primary - Global)
   - Credit/debit cards
   - SEPA Direct Debit (Europe)
   - iDEAL (Netherlands)
   - Bancontact (Belgium)
   - Apple Pay / Google Pay
   - Subscription management
   - Invoice generation

2. **Mollie** (Alternative - Europe)
   - All European payment methods
   - iDEAL, Bancontact, Sofort
   - SEPA Direct Debit
   - Credit cards
   - PayPal
   - Better European coverage

3. **Plug and Play** (Enterprise - Custom)
   - Custom payment flows
   - Invoice-based billing
   - Purchase orders
   - Wire transfers

**Implementation Strategy**:
- **Online activation** (preferred): Payment → instant license key
- **Offline activation** (optional): Purchase → manual license key (for air-gapped setups)
- **License validation**: Local validation with periodic online check (monthly)
- **Grace period**: 30 days if offline or payment fails
- **Automatic renewal**: Optional, user can disable

**Technical Flow**:
```
1. User selects tier → Payment page (Stripe/Mollie)
2. Payment successful → Generate license key
3. License key → Stored locally (encrypted)
4. Monthly validation check (non-blocking)
5. If validation fails → 30-day grace period
6. After grace → Downgrade to free tier (data preserved)
```

**License Key Format**:
```
Format: BEAR-XXXX-XXXX-XXXX-XXXX
Example: BEAR-PRO1-A3F9-K8L2-9X4M

Encoding:
- BEAR: Product identifier
- PRO1/BAS1/ENT1/FREE: Tier identifier
- Next 3 blocks: Encrypted data (tier, expiry, features)
- Signed with RSA private key (verified locally)
```

**Success Criteria**:
- Payment processing works in EUR and USD
- License activation completes in <30 seconds
- Offline activation works for air-gapped systems
- Tier-based feature gating functional
- Free tier verification process works
- Centralized billing for Enterprise
- License renewal automatic (if enabled)

**Rust Files (Licensing & Payment)**:
```
src-tauri/src/
├── licensing/
│   ├── mod.rs                           # License manager
│   ├── tier.rs                          # Tier definitions and features
│   ├── validator.rs                     # License key validation
│   ├── generator.rs                     # License key generation (server-side)
│   ├── activation.rs                    # License activation
│   ├── verification.rs                  # Periodic verification
│   ├── grace_period.rs                  # Grace period management
│   └── social_mission.rs                # Free tier verification
├── payment/
│   ├── mod.rs                           # Payment module
│   ├── stripe.rs                        # Stripe integration
│   ├── mollie.rs                        # Mollie integration
│   ├── plug_and_play.rs                 # Plug and Play integration
│   ├── webhook.rs                       # Payment webhooks
│   └── invoice.rs                       # Invoice generation
├── features/
│   ├── mod.rs                           # Feature flags
│   ├── tier_control.rs                  # Tier-based access control
│   ├── limits.rs                        # Usage limits per tier
│   └── restrictions.rs                  # Feature restrictions
└── commands/
    ├── licensing.rs                     # License commands
    ├── payment.rs                       # Payment commands
    └── subscription.rs                  # Subscription management

migration/src/
├── m20250116_000016_add_licenses.rs     # License table
├── m20250117_000017_add_subscriptions.rs # Subscription tracking
└── m20250118_000018_add_payments.rs     # Payment history

entity/src/
├── licenses.rs                          # License entity
├── subscriptions.rs                     # Subscription entity
└── payments.rs                          # Payment entity
```

**Privacy & Security**:
- ✅ **No telemetry**: Only license validation pings (once/month)
- ✅ **Encrypted license keys**: RSA-2048 signed
- ✅ **Local storage**: All payment info on Stripe/Mollie (not stored locally)
- ✅ **Offline mode**: 30-day grace period if no internet
- ✅ **GDPR compliant**: Minimal data collection
- ✅ **Transparent pricing**: No hidden fees
- ✅ **Cancel anytime**: No lock-in

**Social Mission Implementation**:

1. **Free Tier Application Process**:
   ```
   Apply → Submit Proof → Review (1-3 business days) → Approved → Free license key
   ```

2. **Required Documentation**:
   - KvK registration (Netherlands) or equivalent
   - Organization email domain
   - Mission statement (max 500 words)
   - Proof of non-profit status
   - Annual renewal form

3. **Verification**:
   - Manual review by BEAR LLM AI team
   - Check against public registries
   - Email verification
   - Optional: Reference from legal aid network

4. **Renewal**:
   - Annual renewal required
   - Automated email reminder 30 days before expiry
   - Re-verification process
   - Continued compliance check

**Organizations Eligible for Free Tier**:
- **Rechtswinkels** (Legal Advice Centers - NL)
- **Sociale advocatuur** (Social Legal Aid - NL/BE)
- **Juridisch Loket** (Legal Counter - NL)
- **Pro Bono partnerships**
- **University legal clinics**
- **Refugee legal aid**
- **Public interest law organizations**
- **Legal aid societies** (international)

---

## STRATEGIC DECISION POINT: Choose Your Path 🔀

**After completing GDPR compliance, AI Act compliance, and basic PII protection (Phases 1-5), you face a critical architectural decision that will shape the future of your legal AI system.**

### The Fork in the Road

Both paths lead to a **technologically independent, privacy-first legal AI system**, but they differ fundamentally in their approach to document workflows and integration philosophy.

---

### Path A: Markdown-First Architecture 📝
**The Plaintext Philosophy**

**Vision**: Make your entire legal practice "legible" to AI through plaintext formats, enabling unprecedented levels of AI assistance while maintaining complete control and future-proofing your data.

**Core Principles**:
- Everything in plaintext (Markdown, YAML, JSON, CSV, mbox)
- Future-proof formats that will never become obsolete
- Git version control for all legal work
- Full-text search and grep across entire practice
- AI can read and understand all firm data
- Complete independence from proprietary formats

**Key Benefits**:
- ✅ Maximum AI accessibility to all firm data
- ✅ Version control with Git (perfect audit trail)
- ✅ Future-proof (plaintext never obsolete)
- ✅ Powerful search (grep, semantic search)
- ✅ Easy backup, migration, archival
- ✅ No vendor lock-in
- ✅ Works perfectly offline

**Key Challenges**:
- ❌ Steeper learning curve for non-technical users
- ❌ Different from traditional legal workflows
- ❌ Requires custom PDF generation for professional output
- ❌ May face resistance from staff/clients expecting .docx

**Use Cases**:
- Law firms ready to embrace modern, tech-forward workflows
- Solo practitioners who control their entire workflow
- Tech-savvy legal professionals
- Firms prioritizing long-term data independence
- Teams comfortable with version control concepts

**→ Continue to Phase 6A for detailed Markdown-First implementation**

---

### Path B: Microsoft Word Integration with Advanced AI 📄
**The Familiar Workflow, AI-Enhanced**

**Vision**: Keep familiar Microsoft Word workflows while adding powerful local AI capabilities through a Word Add-in, agent-based automation, and multi-modal intelligence - all running locally for complete privacy.

**Core Principles**:
- Work stays in familiar Microsoft Word environment
- Local Word Add-in brings AnythingLLM-style AI into Word
- AI agents assist without disrupting existing workflows
- Multi-modal support (text, images, documents)
- Agentic workflows automate repetitive tasks
- All AI processing remains 100% local

**Key Features**:

#### 1. **Microsoft Word Add-in for Local LLM**
   - AnythingLLM-style interface embedded in Word
   - AI sidebar for conversational assistance while drafting
   - Context-aware suggestions based on current document
   - Local LLM integration (Mistral, Llama, etc.)
   - Multi-modal support (analyze images, tables, charts)
   - All processing 100% local and private

#### 2. **🦾 Workspace Agents**
   - **Web Research Agent**: Browse the web for legal research (with privacy controls)
   - **Document Analysis Agent**: Extract insights from multiple documents
   - **Citation Agent**: Find and verify legal citations
   - **Compliance Agent**: Check documents against regulatory requirements
   - **Translation Agent**: Multi-language document translation
   - **Summary Agent**: Generate executive summaries

#### 3. **🔄 Agentic Workflows (Zapier-like Automation)**
   - No-code workflow builder for legal tasks
   - Example workflows:
     - **Email → Document**: Auto-file client emails to correct matter folders
     - **Contract Review**: Extract clauses → Flag risks → Generate review memo
     - **Time Entry**: Track work → Generate descriptions → Create billing entries
     - **Document Assembly**: Template + Data → Generate → Review → Export
   - Trigger-action chains with AI decision points
   - All workflows run locally with privacy guarantees

#### 4. **🆕 Full MCP-Compatibility**
   - Model Context Protocol integration
   - Connect to local MCP servers (file systems, databases, tools)
   - Extensible architecture for custom integrations
   - AI agents can use MCP tools to access data sources
   - Privacy-preserving tool use (all local)

#### 5. **🆕 No-Code AI Agent Builder**
   - Visual interface for creating custom AI agents
   - Two approaches:
     - **Agent Flows** (No-code): Drag-and-drop workflow builder
       - Visual node editor
       - Pre-built components (prompts, conditions, actions)
       - Built for everyone - no coding required
       - Template library for common legal tasks
     - **Agent Skills** (Code-based): For power users
       - Write custom skills in JavaScript/TypeScript
       - Full API access to application features
       - Advanced customization and control
       - Community skill sharing (optional)

#### 6. **🖼️ Multi-Modal Support**
   - **Text**: All document formats (DOCX, PDF, TXT, etc.)
   - **Images**: Analyze diagrams, signatures, exhibits
   - **Tables**: Extract and analyze data from tables
   - **Scanned Documents**: OCR with local processing
   - **Audio**: Transcription and analysis (meetings, depositions)
   - Support for both:
     - **Closed-source models**: GPT-4V-equivalent (if available locally via Ollama/LM Studio)
     - **Open-source models**: Llama 3.2 Vision, BakLLaVA, etc.

#### 7. **Custom AI Agents**
   - **Agent Flows** (No-code approach):
     ```
     Workflow: Contract Risk Analysis
     ┌─────────────┐     ┌──────────────┐     ┌─────────────┐
     │ Load        │────▶│ Extract      │────▶│ Risk        │
     │ Contract    │     │ Clauses      │     │ Scoring     │
     └─────────────┘     └──────────────┘     └─────────────┘
                                                      │
                                                      ▼
     ┌─────────────┐     ┌──────────────┐     ┌─────────────┐
     │ Generate    │◀────│ Flag High    │◀────│ Categorize  │
     │ Report      │     │ Risk Items   │     │ by Type     │
     └─────────────┘     └──────────────┘     └─────────────┘
     ```

   - **Agent Skills** (Code-based approach):
     ```javascript
     // Example custom skill for clause extraction
     export const extractNonCompeteClauses = {
       name: "Extract Non-Compete Clauses",
       description: "Find and analyze non-compete clauses in employment contracts",
       async execute(document) {
         const clauses = await ai.extract({
           type: "non-compete",
           document: document,
           analyze: ["duration", "geography", "scope"]
         });
         return ai.summarize(clauses);
       }
     };
     ```

**Key Benefits**:
- ✅ Familiar workflow - no retraining needed
- ✅ Works with existing Word documents
- ✅ Gradual AI adoption - use as much or as little as needed
- ✅ Staff acceptance - looks like normal Word
- ✅ Client compatibility - delivers .docx files
- ✅ No-code options for non-technical users
- ✅ Multi-modal capabilities

**Key Challenges**:
- ❌ Dependency on Microsoft Word (vendor lock-in)
- ❌ Proprietary .docx format (less future-proof)
- ❌ More complex to maintain (Word API integration)
- ❌ Harder to version control (binary format)
- ❌ Limited to Windows/macOS (Word availability)

**Use Cases**:
- Established law firms with existing Word-based workflows
- Teams with staff trained on Microsoft Office
- Firms that regularly exchange .docx with clients
- Practices requiring gradual AI adoption
- Organizations prioritizing familiar tools

**→ Continue to Phase 6B for detailed MS Word Integration implementation**

---

### Decision Matrix

| Factor | Path A: Markdown | Path B: MS Word |
|--------|------------------|-----------------|
| **Learning Curve** | Steeper | Minimal |
| **Future-Proofing** | Excellent | Good |
| **Version Control** | Native (Git) | Limited |
| **Staff Adoption** | Challenging | Easy |
| **Client Compatibility** | Requires PDF export | Native .docx |
| **AI Accessibility** | Maximum | Very Good |
| **No-Code Options** | Limited initially | Extensive |
| **Search Capabilities** | Excellent (grep + semantic) | Good (semantic only) |
| **Vendor Independence** | Complete | Partial (Word dependency) |
| **Multi-Modal Support** | Via extensions | Native |
| **Automation** | Powerful (scripts) | User-friendly (flows) |

---

### Making the Decision

**Choose Path A (Markdown) if:**
- You're a solo practitioner or small tech-savvy team
- You value long-term data independence above all
- You're comfortable with Git and version control
- You want maximum AI accessibility to all data
- You're willing to train staff on new workflows
- You prioritize open formats and future-proofing

**Choose Path B (MS Word) if:**
- You have existing staff trained on Microsoft Office
- You regularly exchange .docx files with clients
- You want gradual, low-friction AI adoption
- You need visual, no-code workflow builders
- You prefer familiar tools with AI enhancements
- You want multi-modal support out of the box

**Or Choose Both (Hybrid Approach):**
- Internal work in Markdown (drafts, notes, research)
- Client-facing deliverables in Word (contracts, letters)
- Best of both worlds with conversion workflows
- Gradual transition from Word → Markdown over time

---

## Path A: Markdown-First Architecture

### Phase 6A: Making Your Firm "Legible" to AI
**Foundation for Advanced AI Assistance**

**Objective**: Create a system where AI can access and understand your entire legal practice while maintaining complete privacy and control.

#### Step 23: Plaintext-First Architecture
**Priority**: Medium | **Effort**: High

**What**: Design a plaintext-first data architecture that makes all firm data accessible to AI.

**Implementation**:
- Support plaintext formats for all document types:
  - Markdown for letters, memos, contracts
  - Plain text email storage (mbox, maildir formats)
  - Structured YAML for matter metadata
  - JSON for case timelines and evidence
  - CSV for time entries and billing

- Markdown-based legal document workflow:
  - Write in markdown with legal templates
  - Convert to professional PDF using custom Rust scripts
  - Maintain version control (Git integration)
  - Full-text search across all documents

- Email integration (plaintext formats):
  - Support for mutt, neomutt (Unix mail clients)
  - mbox and maildir format readers
  - Email-to-markdown conversion
  - Automatic filing by matter/case
  - AI-assisted email summarization

**Technical Details**:
```rust
// Example: Convert markdown letter to PDF
fn markdown_to_pdf(
    content: &str,
    template: &str,
    metadata: &LetterMetadata
) -> Result<Vec<u8>> {
    // Parse markdown with legal extensions
    // Apply professional template
    // Generate PDF with proper formatting
    // Include letterhead, signatures, page numbers
}
```

**Benefits**:
- AI can read and analyze all firm data
- Version control with Git
- Future-proof (plaintext never obsolete)
- Search and grep across entire practice
- Easy backup and migration

**Success Criteria**:
- 90% of firm data in plaintext formats
- Markdown-to-PDF conversion produces professional output
- Full-text search across all documents in <2 seconds
- Git version control for all matter files
- AI can access and understand all firm data

---

#### Step 24: Explicit Knowledge Capture
**Priority**: Medium | **Effort**: Medium

**What**: Make implicit knowledge explicit through structured documentation.

**Implementation**:
- Matter intake questionnaire (AI-assisted):
  - What is the case about?
  - What does success mean for the client?
  - Key deadlines and milestones
  - Budget and time constraints
  - Risk factors and concerns

- Structured matter files:
  ```markdown
  # Matter: [Client Name] - [Case Name]

  ## Objective
  [Clear statement of what success means]

  ## Background
  [AI-generated summary from intake conversation]

  ## Strategy
  [Legal strategy and approach]

  ## Timeline
  - [Deadline 1]: [Description]
  - [Deadline 2]: [Description]

  ## Success Criteria
  - [ ] Criterion 1
  - [ ] Criterion 2
  ```

- AI reads matter files for context-aware assistance
- Natural language instructions the AI can parse
- Human and AI both use same documentation

**Success Criteria**:
- Every matter has structured documentation
- Success criteria explicitly defined
- AI understands matter context
- Instructions readable by humans and AI

---

### Phase 7: Natural Language & Prompt-Based Workflows
**AI That Adapts to Your Practice**

#### Step 25: Prompt Library & Refinement
**Priority**: High | **Effort**: Medium

**What**: Build a library of prompts that can be refined over time.

**Implementation**:
- Prompt-based features with editable prompts:
  - Document summarization
  - Email triage and filing
  - Time entry generation
  - Task extraction from notes
  - Meeting preparation
  - Contract review checklists

- User-editable prompt templates:
  ```yaml
  name: "Email Summarization"
  prompt: |
    You are a legal assistant. Summarize this email for a lawyer.

    Focus on:
    - Key legal issues raised
    - Actions required
    - Deadlines mentioned
    - Client concerns

    Email:
    {email_content}

    Provide a 2-3 sentence summary.
  version: 1.2
  last_modified: 2025-10-24
  ```

- Prompt version control and A/B testing
- Metrics on prompt effectiveness
- Share prompts across firm (prompt library becomes crown jewels)

**Why This Matters**:
The prompts that work well for your practice become valuable intellectual property. They encode your firm's expertise and approach. With local AI, these prompts never leave your control.

**Success Criteria**:
- Library of 20+ specialized prompts
- Users can create and edit prompts
- Prompt versioning and rollback
- Metrics show improved accuracy over time
- Prompts shareable within firm only

---

#### Step 26: Conversational Workflows
**Priority**: Medium | **Effort**: High

**What**: Build workflows around natural conversation with AI.

**Implementation**:
- **Conversational Matter Intake**:
  ```
  AI: "Let's set up this new matter. What is the case about?"
  User: [Explains case]
  AI: "I understand. What does success look like for the client?"
  User: [Defines success]
  AI: "Got it. What are the key deadlines?"
  User: [Lists deadlines]
  AI: "Based on our conversation, here's the matter summary..."
  [User reviews and saves]
  ```

- **Conversational Document Review**:
  ```
  User: "Review this NDA for unusual clauses"
  AI: [Analyzes document]
  AI: "I found 3 unusual clauses: [lists them with explanations]"
  User: "Explain clause 2 in detail"
  AI: [Detailed explanation]
  User: "Suggest alternate language"
  AI: [Provides alternatives]
  ```

- **Conversational Legal Research**:
  ```
  User: "Find cases about GDPR violations involving cookies"
  AI: [Searches local legal database]
  AI: "I found 12 relevant cases. The most significant is..."
  User: "Summarize the key holdings"
  AI: [Provides summary]
  User: "Compare with Article 6 requirements"
  AI: [Provides comparison]
  ```

**Technical Details**:
- Conversation state management
- Multi-turn dialogue support
- Context retention across conversation
- Save conversation as matter documentation
- Export conversation transcripts

**Success Criteria**:
- Natural multi-turn conversations
- AI maintains context across turns
- Conversations can be saved and resumed
- Useful for intake, research, review workflows

---

### Phase 8: Small Tasks with Mandatory Validation
**Reliable AI Through Human Oversight**

#### Step 27: Validated Automation
**Priority**: High | **Effort**: Medium

**What**: Automate small tasks but always require human validation.

**Proven Workflows** (from the lawyer's practice):

1. **Automatic Email Filing**:
   - AI reads incoming email
   - Determines which matter it belongs to
   - Suggests filing location
   - User confirms or corrects
   - Creates time entry draft
   - Success Rate: ~85% correct classification

2. **Email Summarization**:
   - AI summarizes email in 2-3 sentences
   - Highlights action items
   - User reviews before saving
   - Summary added to matter file

3. **Task Extraction**:
   - AI reads meeting notes or emails
   - Extracts action items and deadlines
   - Proposes tasks with due dates
   - User reviews and adds to task list

4. **Time Entry Generation**:
   - AI monitors work activities
   - Drafts time entries with descriptions
   - Suggests appropriate time allocations
   - User reviews and approves before saving

5. **Document Summarization**:
   - AI summarizes long documents
   - Extracts key clauses and terms
   - User reviews for accuracy
   - Summary saved to matter file

**Validation Workflow**:
```
AI Processing → Draft Output → User Review → User Edits/Approves → Save
                                     ↓
                              [Always Required]
```

**Success Criteria**:
- Zero automatic saves without review
- 80%+ suggestions accepted with minor edits
- Time savings: 30-50% on administrative tasks
- User maintains full control
- Easy to correct AI errors

---

#### Step 28: Workflows NOT Automated
**Priority**: N/A | **Philosophy**

**What**: Explicit list of tasks to keep human-led.

**Never Automate**:
1. **Drafting Client Communications**
   - Emails to clients
   - Legal letters
   - Settlement proposals
   - Reason: Personal touch is essential to client relationships

2. **Legal Analysis & Advice**
   - Legal opinions
   - Risk assessments
   - Strategy decisions
   - Reason: Professional judgment required

3. **Court Filings**
   - Complaints
   - Motions
   - Briefs
   - Reason: Professional responsibility and liability

4. **Final Document Approval**
   - Contracts before signature
   - Legal documents before filing
   - Reason: Professional duty of care

**Philosophy**:
> "The fun part is helping other people, and I don't want an AI to come between that."

AI should handle the tasks lawyers don't enjoy (administrative work, document processing, research grunt work) so lawyers can focus on the meaningful parts: client relationships and strategic thinking.

---

### Phase 9: Knowledge Retrieval & Legal RAG
**Intelligent Legal Research Assistant**

#### Step 29: Local Legal Knowledge Base
**Priority**: High | **Effort**: Very High

**What**: Build a local RAG (Retrieval-Augmented Generation) system for legal knowledge.

**Implementation**:
- **Document Indexing**:
  - Index all firm documents (matters, memos, letters)
  - Index legal databases (if licensed)
  - Index statutes and regulations (GDPR, AI Act, national law)
  - Index case law and precedents
  - Index firm knowledge base and templates

- **Semantic Search**:
  - Convert documents to embeddings using local models
  - Store in local vector database (Qdrant, Meilisearch)
  - Semantic search across all documents
  - Find similar cases, clauses, arguments

- **Context-Aware Generation**:
  - Retrieve relevant documents for query
  - Provide to LLM as context
  - Generate answer grounded in firm knowledge
  - Cite sources for all claims

**Technical Architecture**:
```
User Query → Embedding Model → Vector Search →
→ Retrieve Top Documents → LLM + Context →
→ Generate Answer + Citations → User Review
```

**Embedding Models** (Local):
- `all-MiniLM-L6-v2` (22MB, fast)
- `multilingual-e5-large` (multilingual)
- Legal-specific embeddings (future)

**Vector Database**:
- Qdrant (Rust-based, embedded mode)
- Meilisearch (fast, typo-tolerant)
- Store locally, never sync to cloud

**Use Cases**:
1. **Legal Research**:
   - "Find all GDPR cases involving cookie consent"
   - "What are precedents for Article 17 right to erasure?"

2. **Clause Library**:
   - "Find NDA clauses we've used for tech companies"
   - "Show arbitration clauses from past employment contracts"

3. **Matter Precedents**:
   - "Find similar cases to [current matter]"
   - "How did we approach this issue in past matters?"

4. **Legal Analysis**:
   - "Analyze this contract against our standard terms"
   - "Compare this NDA to industry standards"

**Challenges** (from lawyer's experience):
- Hard to get right (quality of results)
- Requires high-quality legal corpus
- Embeddings need legal domain knowledge
- Results can be disappointing without careful tuning

**Success Criteria**:
- Semantic search finds relevant documents >80% of time
- Search results returned in <2 seconds
- Citations accurate and verifiable
- Better than keyword search for complex legal queries
- Continuous improvement as corpus grows

---

### Phase 10: Privacy, Security & Independence
**Fortress Architecture**

#### Step 30: Complete Air-Gapped Operation
**Priority**: High | **Effort**: Medium

**What**: Run AI server completely isolated from public internet.

**Implementation** (from lawyer's practice):
- **AI Server**: Runs in isolated environment (physical or VM)
  - No public internet access
  - Only connects to local network
  - Isolated from client data by default

- **Client Application**: Runs on lawyer's laptop
  - Connects to AI server via local network only
  - Also isolated from internet for sensitive operations
  - Can operate completely offline

- **Network Architecture**:
  ```
  Internet ──┐
             ├── Firewall ──→ Public-Facing Systems
             │
  Isolated   │
  Local      ├── AI Server (no internet)
  Network ───┤       ↕
             ├── Lawyer's Laptop (client)
             │       ↕
             └── Local Legal Database (optional)
  ```

**Security Layers**:
1. **Model Isolation**:
   - Don't trust downloaded models
   - Run in sandboxed environment
   - No access to file system or network

2. **Application Isolation**:
   - Client app runs in isolated environment
   - Limited access to system resources
   - All data encrypted at rest

3. **Network Isolation**:
   - No public internet access during sensitive operations
   - Local network only for AI communication
   - Firewall rules prevent data exfiltration

**Why This Matters**:
You're downloading and running gigabytes of code (AI models and software) that you haven't verified. Complete isolation ensures that even if compromised, damage is contained.

**Success Criteria**:
- AI server has no internet access
- Client can operate 100% offline
- All communication local network only
- Isolated environments verifiable
- Security audit confirms isolation

---

#### Step 31: Trust Nothing Architecture
**Priority**: High | **Effort**: High

**What**: Don't trust models or software - verify and isolate everything.

**Security Principles**:
1. **Don't trust AI models**:
   - Downloaded models may contain malicious code
   - Run in completely isolated environment
   - Use containerization (Docker, Firecracker)
   - Monitor resource usage and network access
   - Verify checksums before loading

2. **Don't trust AI-generated code**:
   - All code (including this AI development) is untrusted
   - Run thorough security audits
   - Penetration testing
   - Code review by security experts
   - Isolated execution environment

3. **Don't trust cloud services**:
   - No dependency on US-based AI companies
   - No reliance on cloud infrastructure
   - All critical operations must work offline
   - Independent operation guaranteed

**Implementation**:
- **Containerization**:
  ```rust
  // Run AI model in isolated container
  // No file system access
  // No network access
  // Resource limits enforced
  // Monitored and logged
  ```

- **Code Auditing**:
  - Security review of all dependencies
  - SBOM (Software Bill of Materials)
  - Vulnerability scanning
  - Regular updates and patching

- **Zero Trust Network**:
  - Verify all communications
  - Encrypt all data in transit
  - Authenticate all requests
  - Minimal privilege access

**Success Criteria**:
- AI models run in isolated containers
- Security audit shows no vulnerabilities
- Penetration testing confirms isolation
- Complete independence from cloud services
- Can operate with internet completely disabled

---

### Phase 11: Social Justice & Accessibility
**Making Law Accessible**

#### Step 32: Efficiency Gains → Lower Costs
**Priority**: Medium | **Effort**: Medium | **Mission**: Critical

**What**: Use AI efficiency gains to make legal services more accessible.

**Vision**:
> "If AI would only make law more efficient, I would find this a bit of a depressing future. But perhaps AI can help make law more accessible to people who cannot currently afford it. Using AI to contribute to social justice - now that's an AI-powered future I can believe in."

**Implementation**:
- **Tiered Service Model**:
  - Premium: Full-service legal representation
  - Standard: AI-assisted legal services (lower cost)
  - Pro Bono: AI-powered self-help for those who can't afford lawyers

- **AI-Assisted Pro Bono Work**:
  - Use AI to handle more pro bono cases
  - Efficiency gains allow helping more people
  - Simple cases can be handled faster with AI assistance
  - Complex cases get more lawyer time

- **Self-Service Legal Tools** (future):
  - AI-guided document preparation
  - Legal information chatbot
  - Template generation for common needs
  - Educational resources
  - Triaged referrals to human lawyers when needed

**Measuring Impact**:
- Track pro bono hours enabled by AI efficiency
- Number of people helped who couldn't afford traditional rates
- Cost reduction for standard legal services
- Access to justice metrics

**Philosophical Foundation**:
The goal is not to replace lawyers or just make law firms more profitable. The goal is to use technology to expand access to justice for those who need it most.

**Success Criteria**:
- 30%+ increase in pro bono capacity
- 20%+ reduction in costs for standard services
- Measurable increase in access to justice
- Maintain quality of legal services
- Positive social impact documented

---

## Implementation Priorities

### Phase 0 (Critical - Week 1-3)
**Focus**: Foundation & Wireframe + Compliance Patterns
**Timeline**: 2-3 weeks
- Step 0.1: Project initialization & setup
- Step 0.2: Basic UI wireframe
- Step 0.3: Database setup & migrations
- Step 0.4: i18n framework setup
- Step 0.5: Basic Tauri commands
- Step 0.6: Build & package verification
- Step 0.7: Testing setup
- **Step 0.8: Case/Matter organization** (lawyer-centric architecture) ⭐
- **Step 0.9: Human-in-the-loop review UI** (compliance pattern) ⭐
- **Step 0.10: AI transparency components** (ready for AI integration) ⭐
- **Step 0.11: Basic audit log structure** (compliance foundation) ⭐
- **Deliverable**: Working wireframe with case organization and compliance patterns
- **Success Metric**: App works case-by-case, review workflow functional, compliance UI ready

### Phase 1 (Critical - Q1 2025)
**Focus**: Legal Compliance - GDPR
**Timeline**: 8-10 weeks
- Steps 1-12: Complete GDPR compliance
  - Data minimization
  - Purpose limitation
  - Transparency & notice
  - Encryption at rest
  - PII Layer 1 (regex)
  - Access control
  - Data deletion (right to erasure)
  - Data correction
  - Storage limitation
  - Audit logging
  - Security by design
  - Privacy notice accessibility
- **Deliverable**: GDPR-compliant application
- **Success Metric**: Pass third-party GDPR compliance audit

### Phase 2 (Critical - Q1-Q2 2025)
**Focus**: Legal Compliance - AI Act
**Timeline**: 4-6 weeks
- Steps 13-19: Complete AI Act compliance
  - AI transparency labels
  - AI use explanation
  - Output provenance
  - Human-in-the-loop
  - Label AI-modified data
  - Local model default
  - Low-risk by design classification
- **Deliverable**: Fully compliant MVP (GDPR + AI Act)
- **Success Metric**: Legal review confirms compliance with EU AI Act Article 52

### Phase 3 (High - Q2 2025)
**Focus**: Local AI Infrastructure
**Timeline**: 8-12 weeks
- Step 20: Local model support (Candle + Hugging Face)
  - Candle inference engine integration
  - Support for Mistral, Llama, Phi-2 models
  - Model quantization (4-bit, 8-bit)
  - GPU acceleration (CUDA, Metal, ROCm)
  - CPU fallback
  - Model manager UI
- **Deliverable**: Working local AI with inference
- **Success Metric**: 7B model running at >10 tokens/sec on GPU, >2 tokens/sec on CPU

### Phase 4 (High - Q2-Q3 2025)
**Focus**: Advanced PII Protection
**Timeline**: 6-8 weeks
- Step 21: PII Layer 2 (NER - Named Entity Recognition)
  - Context-aware entity detection
  - Multi-language NER models
  - Smart anonymization
  - Legal citation preservation
- Step 22: PII Layer 3 (optional advanced anonymization)
  - Microsoft Presidio integration (local)
  - Custom compliance rules
  - Cross-document entity resolution
- **Deliverable**: Privacy-first AI features with >98% PII detection
- **Success Metric**: Combined Layer 1+2 detection rate >98%, false positives <2%

### Phase 5 (Medium - Q3 2025)
**Focus**: Strategic Path Selection
**Timeline**: 2 weeks planning
- **Decision Point**: Choose Path A (Markdown) or Path B (MS Word) or Hybrid
- Review decision matrix
- Assess team capabilities and user needs
- **Deliverable**: Strategic direction selected
- **Success Metric**: Clear decision made and documented

### Phase 6A (Medium - Q3-Q4 2025) - If Path A Selected
**Focus**: Markdown-First Architecture
**Timeline**: 10-12 weeks
- Steps 23-28: Plaintext architecture
  - Markdown workflow implementation
  - Git integration
  - PDF generation pipeline
  - Email integration (mbox/maildir)
  - Validated automation
- **Deliverable**: Efficient plaintext legal workflows
- **Success Metric**: 90% of firm data in plaintext formats, full Git version control

### Phase 6B (High - Q3-Q4 2025) - If Path B Selected
**Focus**: MS Word Integration with Advanced AI
**Timeline**: 12-16 weeks
- Steps 23B-28B: MS Word Add-in & Advanced Features
  - Microsoft Word Add-in development
  - Workspace agents (web research, document analysis, citations, compliance)
  - Agentic workflows (Zapier-like automation)
  - Full MCP-compatibility
  - No-code AI agent builder
  - Multi-modal support (vision, audio, OCR)
- **Deliverable**: AI-enhanced Word integration
- **Success Metric**: Word add-in installs successfully, agents complete tasks accurately

### Phase 7 (High - Q4 2025)
**Focus**: Knowledge Retrieval & Legal RAG
**Timeline**: 8-10 weeks
- Step 29: Local legal knowledge base
  - Document indexing
  - Vector database (Qdrant embedded)
  - Semantic search
  - RAG implementation
  - Citation system
- **Deliverable**: Intelligent legal research assistant
- **Success Metric**: Semantic search finds relevant documents >80% of time in <2 seconds

### Phase 8 (High - Q4 2025 - Q1 2026)
**Focus**: Privacy, Security & Independence
**Timeline**: 6-8 weeks
- Steps 30-31: Complete isolation and security
  - Air-gapped operation support
  - Trust nothing architecture
  - Model isolation (sandboxing)
  - Zero-trust networking
  - Security audit and penetration testing
- **Deliverable**: Fortress-level security
- **Success Metric**: Pass security audit, confirmed isolation, can operate with internet completely disabled

### Phase 9 (Future - 2026+)
**Focus**: Social Impact & Accessibility
**Timeline**: Ongoing
- Step 32: Accessibility and social justice
  - Tiered service model (premium/standard/pro bono)
  - AI-assisted pro bono work
  - Self-service legal tools
  - Access to justice metrics
- Expand to other legal markets
- **Deliverable**: Accessible legal AI for all
- **Success Metric**: 30%+ increase in pro bono capacity, 20% cost reduction for standard services

---

## Complete Development Timeline

**Total Time to MVP (Phase 0-2)**: ~4-5 months
**Total Time to Full Features (Phase 0-8)**: ~18-24 months

**Milestones**:
- ✅ **v0.0.20** (Current): Wireframe + basic features
- 🎯 **v0.1.0** (Phase 0 complete): Working foundation
- 🎯 **v0.2.0** (Phase 1 complete): GDPR compliant
- 🎯 **v0.3.0** (Phase 2 complete): AI Act compliant - **MVP READY**
- 🎯 **v0.4.0** (Phase 3 complete): Local AI working
- 🎯 **v0.5.0** (Phase 4 complete): Advanced PII protection
- 🎯 **v0.6.0** (Phase 5-6 complete): Strategic path implemented
- 🎯 **v1.0.0** (Phase 7-8 complete): **PRODUCTION READY**
- 🎯 **v2.0.0** (Phase 9+): Social impact features

---

## Path B: Microsoft Word Integration with Advanced AI

### Phase 6B: MS Word Add-in with Local LLM
**Bringing AI into Familiar Workflows**

**Objective**: Embed powerful local AI capabilities directly into Microsoft Word, enabling lawyers to work in familiar environments while benefiting from advanced AI assistance.

#### Step 23B: Microsoft Word Add-in Development
**Priority**: Critical | **Effort**: Very High | **Architecture**: Hybrid

**What**: Build a Microsoft Word Add-in that integrates local LLM processing with AnythingLLM-style interface.

**Implementation**:

1. **Add-in Architecture**:
   - **Technology Stack**:
     - Office Add-ins framework (JavaScript API)
     - Task pane for AI interface
     - Local backend server (Rust/Node.js) for LLM processing
     - WebSocket communication between Word and local AI server

   - **Components**:
     ```
     Microsoft Word ←→ Word Add-in (Task Pane) ←→ Local AI Server ←→ Local LLM
                           ↑
                      React UI (AI Chat Interface)
     ```

2. **AnythingLLM-Style Interface**:
   - Sidebar chat interface within Word
   - Context-aware suggestions based on active document
   - Multi-turn conversations with document context
   - Quick actions: Summarize, Analyze, Rephrase, Extract
   - Template library for common legal tasks
   - Document-wide or selection-specific analysis

3. **Core Features**:
   - **Document Analysis**:
     - "Analyze this contract for risk clauses"
     - "Extract all defined terms"
     - "Summarize key obligations"

   - **Drafting Assistance**:
     - "Draft a confidentiality clause"
     - "Rephrase this paragraph in plain language"
     - "Add a force majeure provision"

   - **Review & Editing**:
     - "Check for inconsistent terminology"
     - "Flag ambiguous language"
     - "Suggest improvements to clause 3"

4. **Technical Implementation**:
   ```javascript
   // Word Add-in manifest.xml
   <Host Name="Document">
     <DesktopFormFactor>
       <GetStarted>
         <Title>BEAR LLM AI Assistant</Title>
         <Description>Local AI assistance for legal drafting</Description>
       </GetStarted>
       <FunctionFile>functions.html</FunctionFile>
       <ExtensionPoint xsi:type="PrimaryCommandSurface">
         <CustomTab id="BearAI.Tab">
           <Group id="BearAI.Group">
             <Label>AI Assistant</Label>
             <Control xsi:type="Button" id="BearAI.ShowTaskpane">
               <Label>Open AI Assistant</Label>
               <Supertip>
                 <Title>BEAR AI Assistant</Title>
                 <Description>Get AI help with your document</Description>
               </Supertip>
               <Icon>
                 <bt:Image size="16" resid="Icon.16x16"/>
                 <bt:Image size="32" resid="Icon.32x32"/>
                 <bt:Image size="80" resid="Icon.80x80"/>
               </Icon>
               <Action xsi:type="ShowTaskpane">
                 <TaskpaneId>BearAI.Taskpane</TaskpaneId>
                 <SourceLocation resid="BearAI.Url"/>
               </Action>
             </Control>
           </Group>
         </CustomTab>
       </ExtensionPoint>
     </DesktopFormFactor>
   </Host>
   ```

   ```javascript
   // React component for AI task pane
   const AITaskPane = () => {
     const [messages, setMessages] = useState([]);
     const [context, setContext] = useState(null);

     useEffect(() => {
       // Get current document context
       Word.run(async (context) => {
         const selection = context.document.getSelection();
         selection.load('text');
         await context.sync();
         setContext({
           selectedText: selection.text,
           hasSelection: selection.text.length > 0
         });
       });
     }, []);

     const sendToAI = async (prompt) => {
       // Send to local AI server via WebSocket
       const response = await fetch('http://localhost:8765/api/chat', {
         method: 'POST',
         body: JSON.stringify({
           prompt,
           context: context.selectedText,
           documentType: 'legal'
         })
       });

       const aiResponse = await response.json();
       setMessages([...messages, { role: 'user', content: prompt }, aiResponse]);
     };

     return (
       <div className="ai-taskpane">
         <ChatInterface
           messages={messages}
           onSend={sendToAI}
           context={context}
         />
         <QuickActions
           onAction={(action) => handleQuickAction(action)}
         />
       </div>
     );
   };
   ```

5. **Local AI Server**:
   ```rust
   // Rust backend for LLM processing
   use actix_web::{web, App, HttpServer};
   use candle_core::{Device, Tensor};
   use candle_transformers::models::mistral;

   struct AIState {
       model: mistral::Model,
       tokenizer: Tokenizer,
   }

   async fn chat_endpoint(
       data: web::Json<ChatRequest>,
       state: web::Data<AIState>,
   ) -> impl Responder {
       let prompt = format!(
           "You are a legal AI assistant. Context: {}\n\nUser: {}\n\nAssistant:",
           data.context, data.prompt
       );

       let tokens = state.tokenizer.encode(&prompt, true)?;
       let response = state.model.generate(tokens, 512)?;

       Ok(web::Json(ChatResponse {
           content: response,
           model: "mistral-7b-instruct",
           timestamp: Utc::now(),
       }))
   }

   #[actix_web::main]
   async fn main() -> std::io::Result<()> {
       HttpServer::new(|| {
           App::new()
               .route("/api/chat", web::post().to(chat_endpoint))
               .route("/api/health", web::get().to(health_check))
       })
       .bind("127.0.0.1:8765")?  // Localhost only!
       .run()
       .await
   }
   ```

**Success Criteria**:
- Add-in installs and runs in Word 2016+
- Task pane opens and displays AI interface
- Local AI server processes requests in <2 seconds
- Document context correctly passed to AI
- Responses inserted back into Word document
- Works completely offline after model download
- No data sent to external servers

---

#### Step 24B: Workspace Agents Implementation
**Priority**: High | **Effort**: High

**What**: Implement specialized AI agents that can perform specific tasks within your legal workspace.

**Agent Types**:

1. **Web Research Agent** 🌐:
   ```javascript
   const WebResearchAgent = {
     name: "Web Research",
     description: "Search the web for legal information and precedents",

     async execute(query, privacyLevel = "strict") {
       // Use local privacy-preserving search
       const results = await search({
         query,
         filter: privacyLevel === "strict" ? "no-tracking" : "standard",
         localFirst: true  // Check local legal DB first
       });

       return {
         findings: results,
         sources: results.map(r => r.url),
         summary: await summarize(results)
       };
     },

     privacyOptions: {
       strict: "No tracking, VPN required, local cache",
       moderate: "Minimal tracking, encrypted",
       standard: "Normal web search"
     }
   };
   ```

2. **Document Analysis Agent** 📄:
   ```javascript
   const DocumentAnalysisAgent = {
     name: "Document Analyzer",
     description: "Deep analysis of legal documents",

     async execute(documents) {
       const analysis = {
         keyTerms: await extractKeyTerms(documents),
         clauses: await identifyClauses(documents),
         risks: await assessRisks(documents),
         comparisons: await compareDocuments(documents),
         timeline: await extractTimeline(documents)
       };

       return generateAnalysisReport(analysis);
     }
   };
   ```

3. **Citation Agent** 📚:
   ```javascript
   const CitationAgent = {
     name: "Citation Finder",
     description: "Find and verify legal citations",

     async execute(text) {
       const citations = await extractCitations(text);

       const verified = await Promise.all(
         citations.map(async (cite) => ({
           citation: cite,
           valid: await verifyCitation(cite),
           fullText: await fetchCitationText(cite, { localDB: true }),
           context: await getCitationContext(cite)
         }))
       );

       return {
         citations: verified,
         missing: verified.filter(v => !v.valid),
         recommendations: await suggestCitations(text)
       };
     }
   };
   ```

4. **Compliance Agent** ✅:
   ```javascript
   const ComplianceAgent = {
     name: "Compliance Checker",
     description: "Check documents against regulatory requirements",

     async execute(document, regulations = ["GDPR", "AI_ACT"]) {
       const checks = await Promise.all(
         regulations.map(async (reg) => ({
           regulation: reg,
           compliant: await checkCompliance(document, reg),
           issues: await findIssues(document, reg),
           suggestions: await generateComplianceSuggestions(document, reg)
         }))
       );

       return {
         overallCompliance: checks.every(c => c.compliant),
         detailedChecks: checks,
         actionItems: checks.flatMap(c => c.suggestions)
       };
     }
   };
   ```

**Success Criteria**:
- Each agent completes tasks in <30 seconds
- Agents can run in parallel when needed
- Privacy controls prevent data leakage
- Results are actionable and accurate
- Agents work offline with local data

---

#### Step 25B: Agentic Workflows (No-Code Automation)
**Priority**: High | **Effort**: Very High

**What**: Build a visual workflow builder for creating Zapier-like automation chains for legal tasks.

**Workflow Builder Architecture**:

```
┌─────────────────────────────────────────────────────────────┐
│              Visual Workflow Builder (React Flow)           │
├─────────────────────────────────────────────────────────────┤
│  Triggers    │  Conditions  │  Actions    │  AI Decisions   │
│  - Email     │  - If/Else   │  - File     │  - Classify     │
│  - File      │  - Contains  │  - Email    │  - Extract      │
│  - Schedule  │  - Matches   │  - Generate │  - Summarize    │
│  - Manual    │  - Compare   │  - Alert    │  - Decide       │
└─────────────────────────────────────────────────────────────┘
                           ↓
┌─────────────────────────────────────────────────────────────┐
│              Workflow Execution Engine (Rust)               │
│         - State management                                  │
│         - Error handling & retry                            │
│         - Audit logging                                     │
│         - Privacy validation                                │
└─────────────────────────────────────────────────────────────┘
```

**Example Workflows**:

1. **Email Auto-Filing**:
   ```yaml
   name: "Auto-File Client Emails"
   trigger:
     type: email_received
     filter: from_client

   steps:
     - ai_classify:
         prompt: "Which matter does this email belong to?"
         context: { email_content, subject, sender }
         output: matter_id

     - condition:
         if: confidence > 0.8
         then: auto_file
         else: suggest_to_user

     - file_document:
         destination: "matters/{{ matter_id }}/emails/"
         format: markdown

     - create_time_entry:
         matter: "{{ matter_id }}"
         description: "{{ ai_summary }}"
         duration: 0.1  # 6 minutes

     - notify_user:
         message: "Email filed to {{ matter_name }}"
   ```

2. **Contract Review Workflow**:
   ```yaml
   name: "Contract Risk Analysis"
   trigger:
     type: file_upload
     filter: "*.docx, *.pdf"
     folder: "contracts/review"

   steps:
     - extract_text:
         file: "{{ trigger.file }}"
         ocr_if_needed: true

     - ai_extract_clauses:
         types:
           - liability
           - termination
           - confidentiality
           - indemnification
           - arbitration

     - parallel:
         - ai_risk_score:
             clauses: "{{ extracted_clauses }}"
         - ai_missing_clauses:
             document: "{{ extracted_text }}"
         - ai_unusual_terms:
             document: "{{ extracted_text }}"

     - generate_review_memo:
         template: "contract_review_template.md"
         data:
           clauses: "{{ extracted_clauses }}"
           risks: "{{ risk_scores }}"
           missing: "{{ missing_clauses }}"
           unusual: "{{ unusual_terms }}"

     - human_review:
         reviewers: ["primary_attorney"]
         deadline: "+2 days"

     - on_approval:
         - file_final_memo
         - notify_client
         - create_time_entries
   ```

3. **Document Assembly Workflow**:
   ```yaml
   name: "NDA Generator"
   trigger:
     type: manual
     form:
       - field: client_name
       - field: counterparty_name
       - field: jurisdiction
         options: [NL, DE, US, UK]
       - field: mutual
         type: boolean

   steps:
     - load_template:
         template: "templates/nda_{{ jurisdiction }}.md"

     - ai_customize:
         template: "{{ loaded_template }}"
         variables: "{{ form_data }}"
         instructions: "Customize for {{ client_name }}"

     - ai_check_consistency:
         document: "{{ customized_doc }}"
         fix_pronouns: true
         fix_definitions: true

     - preview_to_user:
         format: pdf
         allow_edit: true

     - on_user_approval:
         - convert_to_pdf
         - save_to_matter
         - create_signature_request
         - log_completion
   ```

**Workflow Builder UI**:
```javascript
const WorkflowBuilder = () => {
  const [nodes, setNodes] = useState([]);
  const [edges, setEdges] = useState([]);

  const nodeTypes = {
    trigger: TriggerNode,
    ai_action: AIActionNode,
    condition: ConditionNode,
    action: ActionNode,
    parallel: ParallelNode,
    human_review: HumanReviewNode,
  };

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      nodeTypes={nodeTypes}
      onNodesChange={setNodes}
      onEdgesChange={setEdges}
    >
      <Background />
      <Controls />
      <MiniMap />
      <Panel position="top-right">
        <WorkflowToolbox />
      </Panel>
    </ReactFlow>
  );
};
```

**Success Criteria**:
- Visual workflow builder is intuitive (usable within 30 minutes)
- Library of 10+ pre-built workflow templates
- Workflows execute reliably with <1% failure rate
- Error handling and retry mechanisms work
- Audit logs capture all workflow executions
- Privacy validation prevents data leakage
- Human review steps cannot be bypassed

---

#### Step 26B: MCP Integration & No-Code Agent Builder
**Priority**: High | **Effort**: Very High

**What**: Implement Model Context Protocol support and build visual no-code agent builder.

**1. MCP Integration**:

```javascript
// MCP Server Configuration
const mcpServers = {
  filesystem: {
    type: "filesystem",
    path: "./legal-workspace",
    permissions: ["read", "write"],
    exclude: ["*.tmp", "*.lock"]
  },

  database: {
    type: "sqlite",
    path: "./data/legal.db",
    readonly: false,
    schema: "legal_schema.sql"
  },

  calendar: {
    type: "calendar",
    provider: "local",  // No cloud sync
    path: "./calendar.ics"
  },

  email: {
    type: "email",
    provider: "local",  // mbox/maildir
    path: "./email-archive"
  }
};

// MCP Tool Registry
const mcpTools = {
  "files:read": async (path) => readFile(path),
  "files:write": async (path, content) => writeFile(path, content),
  "files:search": async (query) => searchFiles(query),
  "db:query": async (sql) => executeQuery(sql),
  "calendar:events": async (date) => getEvents(date),
  "email:search": async (query) => searchEmails(query),
};

// AI Agent with MCP Access
const aiAgent = {
  async processRequest(userRequest) {
    const plan = await llm.plan({
      request: userRequest,
      availableTools: Object.keys(mcpTools)
    });

    const results = await executePlan(plan, mcpTools);
    return results;
  }
};
```

**2. No-Code Agent Builder**:

```javascript
// Agent Flow Builder (No-Code)
const AgentFlowBuilder = () => {
  const [flow, setFlow] = useState({
    name: "New Agent Flow",
    nodes: [],
    connections: []
  });

  const nodeLibrary = {
    inputs: [
      { type: "user_prompt", icon: "💬", name: "User Input" },
      { type: "document", icon: "📄", name: "Load Document" },
      { type: "selection", icon: "✂️", name: "Text Selection" }
    ],

    ai_operations: [
      { type: "summarize", icon: "📝", name: "Summarize" },
      { type: "extract", icon: "🔍", name: "Extract Info" },
      { type: "classify", icon: "🏷️", name: "Classify" },
      { type: "generate", icon: "✨", name: "Generate Text" },
      { type: "analyze", icon: "🔬", name: "Analyze" }
    ],

    logic: [
      { type: "if_condition", icon: "❓", name: "If/Else" },
      { type: "loop", icon: "🔄", name: "Loop" },
      { type: "parallel", icon: "⚡", name: "Parallel" }
    ],

    outputs: [
      { type: "insert_text", icon: "➕", name: "Insert to Doc" },
      { type: "show_result", icon: "👁️", name: "Show to User" },
      { type: "save_file", icon: "💾", name: "Save File" }
    ]
  };

  return (
    <div className="agent-flow-builder">
      <Toolbox nodeLibrary={nodeLibrary} />
      <Canvas flow={flow} onChange={setFlow} />
      <PropertiesPanel selectedNode={selectedNode} />
      <PreviewPanel flow={flow} />
    </div>
  );
};

// Example Agent Flow (Visual Representation)
const contractAnalysisFlow = {
  name: "Contract Risk Analyzer",
  nodes: [
    {
      id: "1",
      type: "document",
      config: { source: "current_document" }
    },
    {
      id: "2",
      type: "extract",
      config: {
        prompt: "Extract all liability and indemnification clauses",
        format: "structured"
      }
    },
    {
      id: "3",
      type: "analyze",
      config: {
        prompt: "Assess risk level for each clause (1-10)",
        factors: ["ambiguity", "one-sided", "unusual"]
      }
    },
    {
      id: "4",
      type: "if_condition",
      config: {
        condition: "any_risk > 7",
        true_path: "5",
        false_path: "6"
      }
    },
    {
      id: "5",
      type: "generate",
      config: {
        prompt: "Generate detailed risk report with recommendations"
      }
    },
    {
      id: "6",
      type: "generate",
      config: {
        prompt: "Generate summary: No high-risk clauses found"
      }
    },
    {
      id: "7",
      type: "show_result",
      config: {
        format: "markdown",
        allow_edit: true
      }
    }
  ],
  connections: [
    { from: "1", to: "2" },
    { from: "2", to: "3" },
    { from: "3", to: "4" },
    { from: "4", to: "5", condition: "true" },
    { from: "4", to: "6", condition: "false" },
    { from: "5", to: "7" },
    { from: "6", to: "7" }
  ]
};
```

**3. Agent Skills (Code-Based)**:

```typescript
// TypeScript SDK for Custom Agent Skills
import { AgentSkill, Document, AIContext } from '@bear-ai/sdk';

// Example: Custom Skill for GDPR Compliance Check
export class GDPRComplianceSkill implements AgentSkill {
  name = "GDPR Compliance Checker";
  description = "Checks documents for GDPR compliance requirements";
  version = "1.0.0";

  async execute(doc: Document, context: AIContext): Promise<SkillResult> {
    // Load GDPR requirements
    const requirements = await this.loadGDPRRequirements();

    // Extract relevant sections
    const sections = await context.ai.extract({
      document: doc,
      schema: {
        privacy_policy: "string",
        data_processing: "string[]",
        user_rights: "string[]",
        retention: "string"
      }
    });

    // Check each requirement
    const checks = await Promise.all(
      requirements.map(async (req) => ({
        requirement: req.name,
        article: req.article,
        compliant: await this.checkRequirement(sections, req),
        evidence: await this.findEvidence(sections, req),
        suggestions: await this.generateSuggestions(sections, req)
      }))
    );

    // Generate report
    return {
      overallCompliant: checks.every(c => c.compliant),
      checks: checks,
      report: await context.ai.generate({
        template: "gdpr_compliance_report",
        data: { checks, document: doc.name }
      })
    };
  }

  private async loadGDPRRequirements() {
    // Load from local knowledge base
    return await db.query('SELECT * FROM gdpr_requirements');
  }

  private async checkRequirement(sections, requirement) {
    return await ai.evaluate({
      prompt: `Does this document satisfy ${requirement.description}?`,
      context: sections,
      requirement: requirement
    });
  }
}

// Register skill
AgentSkillRegistry.register(new GDPRComplianceSkill());
```

**Success Criteria**:
- MCP servers connect and provide tools to AI
- No-code flow builder creates working agents
- Code-based skills can be developed and deployed
- Agent marketplace/library with 20+ pre-built agents
- Skills can be shared within firm (not externally)
- All agent execution logged for audit

---

#### Step 27B: Multi-Modal Support Implementation
**Priority**: High | **Effort**: Very High

**What**: Enable AI to process text, images, audio, and mixed-modality documents.

**Implementation**:

1. **Vision Models**:
   ```rust
   // Local vision model integration
   use candle_transformers::models::llava;

   async fn analyze_image(image_path: &str, question: &str) -> Result<String> {
       let model = llava::Model::load_local("models/bakllava-v1")?;
       let image = image::open(image_path)?;

       let response = model.generate_from_image(
           &image,
           &format!("User: {}\nAssistant:", question),
           512  // max tokens
       )?;

       Ok(response)
   }
   ```

2. **Multi-Modal Use Cases**:

   **A. Document with Diagrams**:
   ```javascript
   const analyzeContractWithDiagrams = async (docPath) => {
     const pages = await extractPages(docPath);

     const results = await Promise.all(
       pages.map(async (page) => {
         if (page.hasImages) {
           const imageAnalysis = await ai.vision({
             image: page.images,
             prompt: "Describe this diagram in the context of a legal contract"
           });

           return {
             pageNum: page.number,
             text: page.text,
             diagrams: imageAnalysis
           };
         }
         return { pageNum: page.number, text: page.text };
       })
     );

     return results;
   };
   ```

   **B. Signature Verification**:
   ```javascript
   const verifySignature = async (signatureImage) => {
     const analysis = await ai.vision({
       image: signatureImage,
       prompt: "Analyze this signature. Is it handwritten? Are there any anomalies?"
     });

     return {
       isHandwritten: analysis.handwritten,
       confidence: analysis.confidence,
       anomalies: analysis.anomalies,
       recommendation: analysis.confidence > 0.8 ? "accept" : "manual_review"
     };
   };
   ```

   **C. Exhibit Analysis**:
   ```javascript
   const analyzeExhibit = async (exhibitPath) => {
     const type = await detectDocumentType(exhibitPath);

     if (type === "image") {
       return await ai.vision({
         image: exhibitPath,
         prompt: "Describe this exhibit. What legal relevance might it have?"
       });
     } else if (type === "pdf") {
       const pages = await extractPages(exhibitPath);
       return await Promise.all(
         pages.map(page => analyzeContractWithDiagrams(page))
       );
     }
   };
   ```

3. **Audio Transcription**:
   ```rust
   // Local Whisper model for transcription
   use whisper_rs::{WhisperContext, FullParams};

   async fn transcribe_deposition(audio_path: &str) -> Result<Transcript> {
       let ctx = WhisperContext::new("models/whisper-large-v3")?;
       let params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });

       let audio = load_audio(audio_path)?;
       let transcript = ctx.full(params, &audio)?;

       // Post-process for legal terminology
       let corrected = correct_legal_terms(&transcript)?;

       // Add speaker diarization
       let with_speakers = identify_speakers(&corrected)?;

       Ok(Transcript {
           text: with_speakers,
           speakers: extract_speakers(&with_speakers),
           timestamps: extract_timestamps(&transcript),
           confidence: calculate_confidence(&transcript),
       })
   }
   ```

4. **Table Extraction**:
   ```javascript
   const extractTables = async (documentPath) => {
     const tables = await ai.vision({
       image: documentPath,
       prompt: "Extract all tables with their structure and data",
       outputFormat: "json"
     });

     return tables.map(table => ({
       headers: table.headers,
       rows: table.rows,
       analysis: ai.analyze({
         data: table.rows,
         prompt: "Summarize key insights from this table data"
       })
     }));
   };
   ```

**Supported Models**:
- **Vision**: BakLLaVA, Llama 3.2 Vision, Moondream
- **Audio**: Whisper Large V3 (local)
- **OCR**: Tesseract, PaddleOCR (local)
- **Table**: TableTransformer, LayoutLM

**Success Criteria**:
- Vision models process images in <5 seconds
- OCR accuracy >95% on legal documents
- Audio transcription accuracy >90%
- Table extraction preserves structure
- All processing 100% local
- Multi-modal workflows work end-to-end

---

### Phase 7B: Integration & Polish
**Final Steps for MS Word Path**

#### Step 28B: Unified Experience
**Priority**: High | **Effort**: Medium

**What**: Ensure all features work together seamlessly.

**Integration Points**:
1. Word Add-in ↔ Workspace Agents
2. Workflow Builder ↔ MCP Tools
3. Agent Flows ↔ Multi-Modal Processing
4. All features ↔ Audit Logging
5. All features ↔ Privacy Controls

**Success Criteria**:
- Features compose naturally
- No conflicts between systems
- Unified audit trail
- Consistent UI/UX
- Performance remains acceptable

---

## Technology Stack Summary (Both Paths)

### Core Technologies
- **Backend**: Rust (Tauri 2.0)
- **Frontend**: React 18 + TypeScript
- **Database**: SQLite + SQLCipher (encryption)
- **AI Inference**: Candle (Rust-native)
- **Models**: Hugging Face Transformers
- **Vector DB**: Qdrant (embedded mode)
- **Encryption**: ring, aes-gcm
- **Containerization**: Docker/Podman (for isolation)

### AI Models (Local)
- **LLM**: Mistral 7B, Llama 2, Phi-2
- **NER**: BERT-based multilingual NER
- **Embeddings**: all-MiniLM-L6-v2, e5-large
- **Quantization**: GGUF (4-bit, 8-bit)

### Hardware Requirements
- **Minimum**: 16GB RAM, 4-core CPU, 50GB storage
- **Recommended**: 32GB RAM, 8-core CPU, 200GB SSD
- **Optimal**: 64GB RAM, GPU (NVIDIA/AMD), 500GB NVMe
- **Server Setup**: RTX 4090/5090, 128GB RAM (for large models)

---

## Success Metrics

### Legal Compliance
- ✅ 100% GDPR compliance
- ✅ 100% AI Act compliance
- ✅ Third-party compliance audit passed
- ✅ Data protection impact assessment completed

### Privacy & Security
- ✅ 100% local operation (zero network calls)
- ✅ All data encrypted at rest
- ✅ Security audit: no critical vulnerabilities
- ✅ Penetration testing: isolation verified

### AI Performance
- ✅ Inference speed: >10 tokens/sec (7B model)
- ✅ PII detection: >98% accuracy
- ✅ Response quality: suitable for legal drafting
- ✅ Context retention: 8K+ tokens

### User Experience
- ✅ Startup time: <30 seconds
- ✅ UI responsive: <100ms interactions
- ✅ Learning curve: <2 hours to productivity
- ✅ User satisfaction: >4.5/5 stars

### Social Impact
- ✅ Pro bono capacity: +30%
- ✅ Cost reduction: 20% for standard services
- ✅ Access to justice: measurable increase
- ✅ Positive testimonials from beneficiaries

---

## Conclusion

This roadmap prioritizes **legal compliance and privacy** above all else, then builds toward a vision of **technologically independent legal practice** that uses AI to expand access to justice.

The journey starts with mandatory compliance (GDPR, AI Act), continues through building local AI infrastructure, and culminates in a vision where AI makes legal services more accessible to those who need them most.

**Core Principles**:
1. **Privacy First**: All data stays local, always
2. **User Control**: Human in the loop for all decisions
3. **Technological Independence**: No reliance on cloud AI services
4. **Social Justice**: Efficiency gains used to expand access
5. **Professional Responsibility**: AI assists, lawyers decide

**Timeline**: 18-24 months to complete all phases
**Outcome**: A privacy-first, locally-run legal AI that serves both professional excellence and social justice.

---

*This roadmap is a living document. It will evolve based on technological advances, regulatory changes, and user feedback. The core commitment to privacy, independence, and social justice remains constant.*
