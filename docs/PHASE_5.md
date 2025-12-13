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

