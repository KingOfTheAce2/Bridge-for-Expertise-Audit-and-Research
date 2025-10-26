# Development Guide

This document provides detailed information for developers working on BEAR LLM AI.

## Table of Contents

- [Getting Started](#getting-started)
- [Architecture](#architecture)
- [Development Workflow](#development-workflow)
- [Database Management](#database-management)
- [Testing](#testing)
- [Code Style](#code-style)
- [Adding Features](#adding-features)
- [Troubleshooting](#troubleshooting)

## Getting Started

### Prerequisites

1. **Node.js v20+**: [Download](https://nodejs.org/)
2. **Rust** (latest stable): Install via [rustup](https://rustup.rs/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. **WebView2** (Windows only): Usually pre-installed on Windows 11

### First-Time Setup

```bash
# Clone the repository
git clone https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research.git
cd Bridge-for-Expertise-Audit-and-Research

# Install Node dependencies
npm install

# Run in development mode
npm run tauri:dev
```

The application will open automatically. Both frontend and backend will hot-reload on changes.

## Architecture

BEAR LLM uses a **Tauri architecture** - a Rust backend with a web-based frontend.

### High-Level Overview

```
┌─────────────────────────────────────────┐
│          Frontend (React)               │
│  - UI Components                        │
│  - State Management (Zustand)           │
│  - Routing (React Router)               │
│  - Styling (Tailwind CSS)               │
└─────────────┬───────────────────────────┘
              │ IPC (invoke)
┌─────────────┴───────────────────────────┐
│       Tauri Backend (Rust)              │
│  - Commands (API Endpoints)             │
│  - Business Logic (Services)            │
│  - Database (Sea-ORM + SQLite)          │
│  - File System Operations               │
└─────────────────────────────────────────┘
```

### Frontend Stack

- **React 18**: UI library
- **TypeScript**: Type safety
- **Vite**: Build tool and dev server
- **Tailwind CSS**: Styling
- **React Router**: Navigation
- **Zustand**: State management
- **i18next**: Internationalization

### Backend Stack

- **Rust**: System language
- **Tauri 2.0**: Framework for desktop apps
- **Sea-ORM**: ORM for database operations
- **SQLite**: Local database
- **Tokio**: Async runtime
- **Serde**: Serialization

### Project Structure

```
src/                          # Frontend
├── components/              # Reusable React components
├── pages/                   # Page-level components
├── contexts/                # React contexts
├── services/                # Frontend business logic
├── i18n/                    # Internationalization
└── styles/                  # Global styles

src-tauri/                   # Backend
├── src/
│   ├── main.rs             # Entry point
│   ├── commands/           # Tauri commands (exposed to frontend)
│   ├── database/           # DB connection management
│   └── services/           # Backend business logic
├── entity/                  # Database models (Sea-ORM entities)
├── migration/               # Database migrations
└── tauri.conf.json         # Tauri configuration
```

## Development Workflow

### Running the App

```bash
# Development mode with hot reload
npm run tauri:dev

# Frontend only (for UI work)
npm run dev

# Backend only (Rust)
cd src-tauri && cargo run
```

### Making Changes

#### Frontend Changes

1. Edit files in `src/`
2. Changes hot-reload automatically
3. Test in the running application

#### Backend Changes

1. Edit files in `src-tauri/src/`
2. Save files
3. Tauri will automatically rebuild and restart

### Building for Production

```bash
# Build the complete application
npm run tauri:build

# Output location: src-tauri/target/release/bundle/
```

## Database Management

BEAR LLM uses **Sea-ORM** with **SQLite** for data persistence.

### Database Location

**Development**: `%APPDATA%/com.bear.llm.ai/bear_llm.db` (Windows)

### Creating a New Entity

1. Create entity file in `src-tauri/entity/src/`:

```rust
// src-tauri/entity/src/my_entity.rs
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "my_entity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
```

2. Export in `src-tauri/entity/src/lib.rs`:

```rust
pub mod my_entity;
pub use my_entity::Entity as MyEntity;
```

### Creating a Migration

1. Create migration file in `src-tauri/migration/src/`:

```rust
// src-tauri/migration/src/m20250101_000004_create_my_entity.rs
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MyEntity::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MyEntity::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MyEntity::Name).string().not_null())
                    .col(
                        ColumnDef::new(MyEntity::CreatedAt)
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
            .drop_table(Table::drop().table(MyEntity::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum MyEntity {
    Table,
    Id,
    Name,
    CreatedAt,
}
```

2. Register in `src-tauri/migration/src/lib.rs`:

```rust
mod m20250101_000004_create_my_entity;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // ... existing migrations
            Box::new(m20250101_000004_create_my_entity::Migration),
        ]
    }
}
```

### Running Migrations

Migrations run automatically when the app starts. To manually test:

```bash
cd src-tauri
cargo run
```

## Testing

### Rust Tests

```bash
cd src-tauri
cargo test                    # Run all tests
cargo test --package entity   # Test specific package
cargo test test_name          # Run specific test
```

### Frontend Tests

```bash
npm test                      # Run Jest tests (when implemented)
npm run test:watch            # Watch mode
```

### End-to-End Testing

```bash
# Build and manually test the app
npm run tauri:build
# Install and run the built application
```

## Code Style

### Rust

Follow standard Rust conventions:

```bash
cd src-tauri
cargo fmt          # Format code
cargo clippy       # Lint code
```

**Guidelines**:
- Use meaningful variable names
- Add documentation comments for public APIs
- Handle errors properly (don't use `unwrap()` in production code)
- Use `Result<T, E>` for operations that can fail

### TypeScript/React

Follow standard React/TypeScript conventions:

```bash
npm run lint       # Lint code (when configured)
```

**Guidelines**:
- Use functional components with hooks
- Keep components small and focused
- Use TypeScript types for all props and state
- Follow naming: `PascalCase` for components, `camelCase` for functions

### Git Commits

Use **conventional commits**:

```
feat: Add new feature
fix: Fix bug
docs: Update documentation
style: Format code
refactor: Refactor code
test: Add tests
chore: Update dependencies
```

## Adding Features

### Adding a Tauri Command

1. Create command in `src-tauri/src/commands/`:

```rust
// src-tauri/src/commands/my_commands.rs
use tauri::State;
use crate::database::DatabaseManager;

#[tauri::command]
pub async fn my_command(
    param: String,
    db: State<'_, DatabaseManager>,
) -> Result<String, String> {
    // Your logic here
    Ok(format!("Result: {}", param))
}
```

2. Register in `src-tauri/src/main.rs`:

```rust
.invoke_handler(tauri::generate_handler![
    // ... existing commands
    commands::my_commands::my_command,
])
```

3. Call from frontend:

```typescript
import { invoke } from '@tauri-apps/api/tauri';

const result = await invoke('my_command', { param: 'value' });
```

### Adding a Route

1. Create page component in `src/pages/`:

```tsx
// src/pages/MyPage.tsx
export default function MyPage() {
  return (
    <div className="p-8">
      <h2 className="text-2xl font-bold">My Page</h2>
    </div>
  );
}
```

2. Add route in `src/App.tsx`:

```tsx
import MyPage from './pages/MyPage';

<Route path="/my-page" element={<MyPage />} />
```

3. Add navigation in `src/components/Sidebar.tsx`:

```tsx
<NavLink to="/my-page">My Page</NavLink>
```

### Adding Translations

1. Add keys to all locale files in `src/i18n/locales/`:

```json
{
  "myFeature": {
    "title": "My Feature",
    "description": "Feature description"
  }
}
```

2. Use in components:

```tsx
import { useTranslation } from 'react-i18next';

const { t } = useTranslation();
return <h1>{t('myFeature.title')}</h1>;
```

## Troubleshooting

### Common Issues

#### "Failed to load database"

**Cause**: Database migration failed or database is corrupted.

**Solution**:
1. Close the app
2. Delete the database: `%APPDATA%/com.bear.llm.ai/bear_llm.db`
3. Restart the app (migrations will run again)

#### "WebView2 not found" (Windows)

**Cause**: WebView2 runtime not installed.

**Solution**:
Download and install: https://developer.microsoft.com/microsoft-edge/webview2/

#### "Port 5173 already in use"

**Cause**: Another Vite dev server is running.

**Solution**:
```bash
# Find and kill the process
npx kill-port 5173

# Or specify a different port
npm run dev -- --port 5174
```

#### Rust compilation errors after pulling changes

**Cause**: Cargo.lock out of sync.

**Solution**:
```bash
cd src-tauri
cargo clean
cargo update
cargo build
```

### Debug Mode

Enable debug logging:

```bash
# Windows
$env:RUST_LOG="debug"
npm run tauri:dev

# Linux/Mac
RUST_LOG=debug npm run tauri:dev
```

### Database Inspection

Use a SQLite browser to inspect the database:
- [DB Browser for SQLite](https://sqlitebrowser.org/)
- [SQLite Viewer (VSCode extension)](https://marketplace.visualstudio.com/items?itemName=qwtel.sqlite-viewer)

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Run tests: `cargo test && npm test`
5. Commit with conventional commits: `git commit -m "feat: add my feature"`
6. Push: `git push origin feature/my-feature`
7. Open a Pull Request

## Resources

- [Tauri Documentation](https://tauri.app/)
- [Sea-ORM Documentation](https://www.sea-ql.org/SeaORM/)
- [React Documentation](https://react.dev/)
- [Rust Book](https://doc.rust-lang.org/book/)

## Getting Help

- **GitHub Issues**: Report bugs or request features
- **GitHub Discussions**: Ask questions or share ideas
- **Discord**: Join our community (coming soon)

---

Happy coding! 🚀
