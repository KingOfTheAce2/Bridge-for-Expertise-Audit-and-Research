# Third-Party Licenses

BEAR LLM AI incorporates code from several third-party open-source projects. We are grateful to the authors and contributors of these projects. This document provides the required attribution and license information for these components.

---

## Table of Contents

1. [Localization Files](#localization-files)
2. [JavaScript/TypeScript Libraries](#javascripttypescript-libraries)
3. [Rust Crates](#rust-crates)

---

## Localization Files

### Chinese Simplified (zh-Hans-CN) Translation

**File**: `src/i18n/locales/zh-Hans-CN.json`

**Copyright**: (c) 2024-present Frank Zhang

**License**: MIT License

**Source**: Adapted from [Kaas](https://github.com/0xfrankz/Kaas)

```
MIT License

Copyright (c) 2024-present Frank Zhang

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## JavaScript/TypeScript Libraries

The following npm packages are used in this project. All are licensed under permissive open-source licenses (MIT, Apache-2.0, BSD, etc.). For complete license information, see the LICENSE file in each package's directory in `node_modules/`.

### Core Framework
- **React** (MIT) - Copyright (c) Meta Platforms, Inc. and affiliates
- **React DOM** (MIT) - Copyright (c) Meta Platforms, Inc. and affiliates
- **React Router DOM** (MIT) - Copyright (c) Remix Software Inc.

### Build Tools
- **Vite** (MIT) - Copyright (c) 2019-present, Yuxi (Evan) You and Vite contributors
- **TypeScript** (Apache-2.0) - Copyright (c) Microsoft Corporation

### UI & Styling
- **Tailwind CSS** (MIT) - Copyright (c) Tailwind Labs, Inc.
- **PostCSS** (MIT) - Copyright (c) 2013 Andrey Sitnik
- **Autoprefixer** (MIT) - Copyright (c) 2013 Andrey Sitnik

### Radix UI Components
- **@radix-ui/react-dialog** (MIT) - Copyright (c) 2022 WorkOS
- **@radix-ui/react-select** (MIT) - Copyright (c) 2022 WorkOS

### State Management & Utilities
- **Zustand** (MIT) - Copyright (c) 2019 Paul Henschel
- **i18next** (MIT) - Copyright (c) 2022 i18next
- **react-i18next** (MIT) - Copyright (c) 2022 i18next

### Tauri
- **@tauri-apps/cli** (MIT/Apache-2.0) - Copyright (c) 2019-2024 Tauri Programme within The Commons Conservancy
- **@tauri-apps/api** (MIT/Apache-2.0) - Copyright (c) 2019-2024 Tauri Programme within The Commons Conservancy

### Development Dependencies
- **ESLint** (MIT) - Copyright OpenJS Foundation and other contributors
- **Prettier** (MIT) - Copyright (c) James Long and contributors
- **Jest** (MIT) - Copyright (c) Meta Platforms, Inc. and affiliates

---

## Rust Crates

The following Rust crates are used in this project. For complete license information, see the LICENSE file in each crate's source or on [crates.io](https://crates.io/).

### Core Framework
- **tauri** (MIT/Apache-2.0) - Tauri Programme within The Commons Conservancy
- **tokio** (MIT) - Tokio Contributors
- **serde** (MIT/Apache-2.0) - Erick Tryzelaar, David Tolnay
- **serde_json** (MIT/Apache-2.0) - Erick Tryzelaar, David Tolnay

### Database
- **sea-orm** (MIT/Apache-2.0) - SeaQL.org
- **sea-orm-migration** (MIT/Apache-2.0) - SeaQL.org
- **sqlx** (MIT/Apache-2.0) - Ryan Leckey, Austin Bonander, Chloe Ross

### Utilities
- **anyhow** (MIT/Apache-2.0) - David Tolnay
- **log** (MIT/Apache-2.0) - The Rust Project Developers
- **env_logger** (MIT/Apache-2.0) - The Rust Project Developers
- **async-trait** (MIT/Apache-2.0) - David Tolnay

---

## License Compliance

BEAR LLM AI complies with all license requirements of the above third-party components:

1. **Attribution**: All required copyright notices and attributions are preserved
2. **License Text**: Original license texts are included where required
3. **Source Availability**: Links to original sources are provided where applicable
4. **No Warranty**: All third-party software is used "as is" without warranty

## Important Note

While BEAR LLM AI incorporates these open-source components, **the overall application is proprietary software**. The licenses of these third-party components apply only to those specific components, and do not grant any rights to use, modify, or distribute BEAR LLM AI as a whole.

For the license governing BEAR LLM AI itself, see the [LICENSE](./LICENSE) file.

---

## Questions?

If you have questions about third-party licenses or compliance, please contact:
- Email: [Your contact email]
- GitHub Issues: [Repository Issues](https://github.com/KingOfTheAce2/Bridge-for-Expertise-Audit-and-Research/issues)

---

*Last updated: October 2025*
