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
**⚠️ NOTE: This step has been MOVED to Phase 1 (Step 1.0) - see lines ~2054-2068**

**This section remains here for reference during Phase 0 implementation.**

---

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
