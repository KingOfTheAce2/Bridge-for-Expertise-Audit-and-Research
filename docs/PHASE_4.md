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

