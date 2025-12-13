// Allow dead code - these are API components that will be used from frontend
#![allow(dead_code)]

use super::types::NerModelInfo;

/// Registry of pre-configured NER models
pub struct NerModelRegistry {
    models: Vec<NerModelInfo>,
}

impl NerModelRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            models: Vec::new(),
        };

        registry.register_default_models();
        registry
    }

    /// Register default NER models
    fn register_default_models(&mut self) {
        // Register general-purpose models
        self.register_general_models();

        // Register legal-domain specific models by language
        self.register_legal_german_models();
        self.register_legal_english_models();
        self.register_legal_french_models();
        self.register_legal_dutch_models();
        self.register_legal_russian_models();
        self.register_legal_chinese_models();
    }

    /// Register general-purpose NER models
    fn register_general_models(&mut self) {
        // 1. BERT-base NER (dslim/bert-base-NER)
        self.models.push(NerModelInfo {
            model_id: "dslim/bert-base-NER".to_string(),
            name: "BERT-base NER".to_string(),
            description: "BERT-base fine-tuned on CoNLL-2003 for Named Entity Recognition. Supports Person, Organization, Location, and Miscellaneous entities.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "110M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/dslim/bert-base-NER/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/dslim/bert-base-NER/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/dslim/bert-base-NER/resolve/main/tokenizer.json".to_string(),
            file_size: 440_000_000, // ~440 MB
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.956), // F1 score on CoNLL-2003 test set
        });

        // 2. DistilBERT NER (lightweight)
        self.models.push(NerModelInfo {
            model_id: "dslim/distilbert-NER".to_string(),
            name: "DistilBERT NER (Fast)".to_string(),
            description: "DistilBERT fine-tuned for NER. Smaller and faster than BERT-base with competitive accuracy. Good for resource-constrained environments.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "distilbert".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "small".to_string(),
            parameters: "66M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/dslim/distilbert-NER/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/dslim/distilbert-NER/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/dslim/distilbert-NER/resolve/main/tokenizer.json".to_string(),
            file_size: 260_000_000, // ~260 MB
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.941), // F1 score
        });

        // 3. RoBERTa-base NER (high accuracy)
        self.models.push(NerModelInfo {
            model_id: "dslim/roberta-base-NER".to_string(),
            name: "RoBERTa-base NER (Accurate)".to_string(),
            description: "RoBERTa-base fine-tuned for NER. Higher accuracy than BERT-base, especially for complex entities. Best for legal and professional documents.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "roberta".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "125M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/dslim/roberta-base-NER/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/dslim/roberta-base-NER/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/dslim/roberta-base-NER/resolve/main/tokenizer.json".to_string(),
            file_size: 500_000_000, // ~500 MB
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.964), // F1 score - highest accuracy
        });

        // 4. XLM-RoBERTa NER (multilingual)
        self.models.push(NerModelInfo {
            model_id: "Davlan/xlm-roberta-base-ner-hrl".to_string(),
            name: "XLM-RoBERTa NER (Multilingual)".to_string(),
            description: "Multilingual NER model supporting 100+ languages. Ideal for non-English legal documents and multilingual workflows.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "xlm-roberta".to_string(),
            language: "multilingual".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "125M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/Davlan/xlm-roberta-base-ner-hrl/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/Davlan/xlm-roberta-base-ner-hrl/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/Davlan/xlm-roberta-base-ner-hrl/resolve/main/tokenizer.json".to_string(),
            file_size: 550_000_000, // ~550 MB
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.93), // Average F1 across languages
        });

        // 5. TinyBERT NER (ultra-fast, smallest)
        self.models.push(NerModelInfo {
            model_id: "mrm8488/bert-tiny-finetuned-ner".to_string(),
            name: "TinyBERT NER (Ultra-Fast)".to_string(),
            description: "Extremely small and fast BERT model for NER. Sacrifices some accuracy for speed. Perfect for real-time applications and low-resource devices.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(),
                "I-PER".to_string(),
                "B-ORG".to_string(),
                "I-ORG".to_string(),
                "B-LOC".to_string(),
                "I-LOC".to_string(),
                "B-MISC".to_string(),
                "I-MISC".to_string(),
            ],
            size: "small".to_string(),
            parameters: "15M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/mrm8488/bert-tiny-finetuned-ner/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/mrm8488/bert-tiny-finetuned-ner/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/mrm8488/bert-tiny-finetuned-ner/resolve/main/tokenizer.json".to_string(),
            file_size: 60_000_000, // ~60 MB
            checksum: None,
            license: "Apache 2.0".to_string(),
            accuracy: Some(0.87), // Lower accuracy, much faster
        });
    }

    /// Register legal-domain German NER models (de/de)
    fn register_legal_german_models(&mut self) {
        // German BERT Legal NER - Top performer for German legal texts
        self.models.push(NerModelInfo {
            model_id: "elenanereiss/bert-base-german-legal-ner".to_string(),
            name: "German BERT Legal NER".to_string(),
            description: "Fine-tuned German BERT on legal entity datasets. Consistently outperforms multilingual models in German case law with F1 scores exceeding 88% for legal entities.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "de".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "110M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/elenanereiss/bert-base-german-legal-ner/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/elenanereiss/bert-base-german-legal-ner/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/elenanereiss/bert-base-german-legal-ner/resolve/main/tokenizer.json".to_string(),
            file_size: 440_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.88), // F1 score on German legal texts
        });
    }

    /// Register legal-domain English NER models (en/gb)
    fn register_legal_english_models(&mut self) {
        // Legal-BERT - Specialized for legal English
        self.models.push(NerModelInfo {
            model_id: "nlpaueb/legal-bert-base-uncased".to_string(),
            name: "Legal-BERT".to_string(),
            description: "BERT adapted for legal English, optimized for contracts, case law, and regulatory filings. Superior context sensitivity for legal NER tasks.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "110M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/nlpaueb/legal-bert-base-uncased/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/nlpaueb/legal-bert-base-uncased/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/nlpaueb/legal-bert-base-uncased/resolve/main/tokenizer.json".to_string(),
            file_size: 440_000_000,
            checksum: None,
            license: "Apache 2.0".to_string(),
            accuracy: Some(0.92), // Estimated for legal documents
        });

        // spaCy Transformer for legal texts (en_core_web_trf equivalent)
        self.models.push(NerModelInfo {
            model_id: "spacy/en_core_web_trf".to_string(),
            name: "spaCy Transformer (Legal)".to_string(),
            description: "Transformer model with superior context sensitivity for legal NER, especially when integrated for PII detection in legal contexts.".to_string(),
            provider: "spaCy/HuggingFace".to_string(),
            model_type: "transformer".to_string(),
            language: "en".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "125M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/spacy/en_core_web_trf/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/spacy/en_core_web_trf/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/spacy/en_core_web_trf/resolve/main/tokenizer.json".to_string(),
            file_size: 500_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.94),
        });
    }

    /// Register legal-domain French NER models (fr/fr)
    fn register_legal_french_models(&mut self) {
        // CamemBERT-bio - Best for French legal and biomedical texts
        self.models.push(NerModelInfo {
            model_id: "almanach/camembert-bio-base".to_string(),
            name: "CamemBERT-bio".to_string(),
            description: "French BERT variant, consistently best for biomedical and clinical legal texts. Outperforms DrBERT and FlauBERT for French entity recognition including nested entities.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "camembert".to_string(),
            language: "fr".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "110M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/almanach/camembert-bio-base/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/almanach/camembert-bio-base/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/almanach/camembert-bio-base/resolve/main/tokenizer.json".to_string(),
            file_size: 440_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.91),
        });

        // CamemBERT-base for general French legal texts
        self.models.push(NerModelInfo {
            model_id: "camembert/camembert-base-ner".to_string(),
            name: "CamemBERT NER".to_string(),
            description: "General-purpose French BERT for NER tasks. Strong performance on French legal and business documents.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "camembert".to_string(),
            language: "fr".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "110M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/camembert/camembert-base-ner/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/camembert/camembert-base-ner/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/camembert/camembert-base-ner/resolve/main/tokenizer.json".to_string(),
            file_size: 440_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.89),
        });
    }

    /// Register legal-domain Dutch NER models (nl/nl)
    fn register_legal_dutch_models(&mut self) {
        // BERTje + CoNLL-2002 - Best for Dutch legal de-identification
        self.models.push(NerModelInfo {
            model_id: "wietsedv/bert-base-dutch-cased-finetuned-conll2002-ner".to_string(),
            name: "BERTje CoNLL-2002 NER".to_string(),
            description: "BERTje transformer trained on Dutch CoNLL-2002 corpus. Benchmarked as best for suppressing PII and de-identification in Dutch legal documents.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "nl".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "110M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/wietsedv/bert-base-dutch-cased-finetuned-conll2002-ner/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/wietsedv/bert-base-dutch-cased-finetuned-conll2002-ner/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/wietsedv/bert-base-dutch-cased-finetuned-conll2002-ner/resolve/main/tokenizer.json".to_string(),
            file_size: 440_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.90),
        });

        // RobBERT - Dutch-specific BERT variant
        self.models.push(NerModelInfo {
            model_id: "pdelobelle/robbert-v2-dutch-base".to_string(),
            name: "RobBERT NER".to_string(),
            description: "Dutch-specific BERT variant with robust token classification for legal text. Outperforms generic models for legislation references.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "roberta".to_string(),
            language: "nl".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "116M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/pdelobelle/robbert-v2-dutch-base/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/pdelobelle/robbert-v2-dutch-base/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/pdelobelle/robbert-v2-dutch-base/resolve/main/tokenizer.json".to_string(),
            file_size: 465_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.88),
        });
    }

    /// Register legal-domain Russian NER models (ru/ru)
    fn register_legal_russian_models(&mut self) {
        // RuBERT + RuLegalNER - Best for Russian legal texts
        self.models.push(NerModelInfo {
            model_id: "seara/rubert-base-cased-ru-legal-ner".to_string(),
            name: "RuBERT Legal NER".to_string(),
            description: "RuBERT fine-tuned on RuLegalNER legal dataset. Delivers strong performance for Russian court and statute texts with CRF/adapter extensions.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "ru".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "178M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/seara/rubert-base-cased-ru-legal-ner/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/seara/rubert-base-cased-ru-legal-ner/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/seara/rubert-base-cased-ru-legal-ner/resolve/main/tokenizer.json".to_string(),
            file_size: 715_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.86),
        });

        // Alternative: General RuBERT NER
        self.models.push(NerModelInfo {
            model_id: "alexyalunin/RuBERT-NER".to_string(),
            name: "RuBERT NER (General)".to_string(),
            description: "General-purpose Russian BERT for NER. Good fallback for Russian legal anonymization when domain-specific model is unavailable.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "ru".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "178M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/alexyalunin/RuBERT-NER/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/alexyalunin/RuBERT-NER/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/alexyalunin/RuBERT-NER/resolve/main/tokenizer.json".to_string(),
            file_size: 715_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: Some(0.84),
        });
    }

    /// Register legal-domain Chinese NER models (zh/hans and zh/hk)
    fn register_legal_chinese_models(&mut self) {
        // Lawformer - Specialized for long Chinese legal texts
        self.models.push(NerModelInfo {
            model_id: "thunlp/Lawformer".to_string(),
            name: "Lawformer".to_string(),
            description: "Pretrained language model specifically built for long Chinese legal texts (statutes, court decisions). Provides high accuracy for Chinese legal NER and contextual entity anonymization.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "longformer".to_string(),
            language: "zh".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "large".to_string(),
            parameters: "102M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/thunlp/Lawformer/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/thunlp/Lawformer/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/thunlp/Lawformer/resolve/main/tokenizer.json".to_string(),
            file_size: 410_000_000,
            checksum: None,
            license: "Apache 2.0".to_string(),
            accuracy: Some(0.90),
        });

        // BERT-base Chinese for general legal NER
        self.models.push(NerModelInfo {
            model_id: "ckiplab/bert-base-chinese-ner".to_string(),
            name: "BERT-base Chinese NER".to_string(),
            description: "General-purpose Chinese BERT for NER. Suitable for both Simplified (zh/hans) and Traditional Chinese (zh/hk) legal documents.".to_string(),
            provider: "HuggingFace".to_string(),
            model_type: "bert".to_string(),
            language: "zh".to_string(),
            entity_labels: vec![
                "O".to_string(),
                "B-PER".to_string(), "I-PER".to_string(),
                "B-ORG".to_string(), "I-ORG".to_string(),
                "B-LOC".to_string(), "I-LOC".to_string(),
                "B-MISC".to_string(), "I-MISC".to_string(),
            ],
            size: "medium".to_string(),
            parameters: "102M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://huggingface.co/ckiplab/bert-base-chinese-ner/resolve/main/model.safetensors".to_string(),
            config_url: "https://huggingface.co/ckiplab/bert-base-chinese-ner/resolve/main/config.json".to_string(),
            tokenizer_url: "https://huggingface.co/ckiplab/bert-base-chinese-ner/resolve/main/tokenizer.json".to_string(),
            file_size: 410_000_000,
            checksum: None,
            license: "Apache 2.0".to_string(),
            accuracy: Some(0.87),
        });
    }

    /// Get all registered models
    pub fn list_models(&self) -> &[NerModelInfo] {
        &self.models
    }

    /// Get model by ID
    pub fn get_model(&self, model_id: &str) -> Option<&NerModelInfo> {
        self.models.iter().find(|m| m.model_id == model_id)
    }

    /// Get recommended model for general use
    pub fn get_recommended_model(&self) -> Option<&NerModelInfo> {
        // BERT-base NER is a good balanced choice
        self.get_model("dslim/bert-base-NER")
    }

    /// Get fastest model
    pub fn get_fastest_model(&self) -> Option<&NerModelInfo> {
        self.get_model("mrm8488/bert-tiny-finetuned-ner")
    }

    /// Get most accurate model
    pub fn get_most_accurate_model(&self) -> Option<&NerModelInfo> {
        self.get_model("dslim/roberta-base-NER")
    }

    /// Get multilingual model
    pub fn get_multilingual_model(&self) -> Option<&NerModelInfo> {
        self.get_model("Davlan/xlm-roberta-base-ner-hrl")
    }

    /// Get recommended model for a specific language and legal domain
    ///
    /// Supports: de, en, fr, nl, ru, zh
    pub fn get_recommended_legal_model(&self, language: &str) -> Option<&NerModelInfo> {
        match language {
            "de" | "de-de" | "de/de" => self.get_model("elenanereiss/bert-base-german-legal-ner"),
            "en" | "en-gb" | "en/gb" | "en-us" => self.get_model("nlpaueb/legal-bert-base-uncased"),
            "fr" | "fr-fr" | "fr/fr" => self.get_model("almanach/camembert-bio-base"),
            "nl" | "nl-nl" | "nl/nl" => self.get_model("wietsedv/bert-base-dutch-cased-finetuned-conll2002-ner"),
            "ru" | "ru-ru" | "ru/ru" => self.get_model("seara/rubert-base-cased-ru-legal-ner"),
            "zh" | "zh-hans" | "zh/hans" | "zh-hk" | "zh/hk" => self.get_model("thunlp/Lawformer"),
            _ => None,
        }
    }

    /// Get all legal models for a specific language
    pub fn get_legal_models_by_language(&self, language: &str) -> Vec<&NerModelInfo> {
        let normalized_lang = match language {
            "de-de" | "de/de" => "de",
            "en-gb" | "en/gb" | "en-us" => "en",
            "fr-fr" | "fr/fr" => "fr",
            "nl-nl" | "nl/nl" => "nl",
            "ru-ru" | "ru/ru" => "ru",
            "zh-hans" | "zh/hans" | "zh-hk" | "zh/hk" => "zh",
            other => other,
        };

        self.models
            .iter()
            .filter(|m| {
                m.language == normalized_lang || m.language == "multilingual"
            })
            .collect()
    }

    /// Get all supported legal languages
    pub fn get_supported_legal_languages(&self) -> Vec<&str> {
        vec!["de", "en", "fr", "nl", "ru", "zh"]
    }

    /// Get model recommendations by use case
    pub fn get_recommendations_by_use_case(&self, use_case: &str) -> Vec<&NerModelInfo> {
        match use_case {
            "legal-german" => self.get_legal_models_by_language("de"),
            "legal-english" => self.get_legal_models_by_language("en"),
            "legal-french" => self.get_legal_models_by_language("fr"),
            "legal-dutch" => self.get_legal_models_by_language("nl"),
            "legal-russian" => self.get_legal_models_by_language("ru"),
            "legal-chinese" => self.get_legal_models_by_language("zh"),
            "fastest" => {
                vec![self.get_fastest_model()].into_iter().flatten().collect()
            }
            "most-accurate" => {
                vec![self.get_most_accurate_model()].into_iter().flatten().collect()
            }
            "multilingual" => {
                vec![self.get_multilingual_model()].into_iter().flatten().collect()
            }
            _ => Vec::new(),
        }
    }

    /// Get all models for a specific language
    pub fn get_models_by_language(&self, language: &str) -> Vec<&NerModelInfo> {
        let normalized_lang = match language {
            "de-de" | "de/de" => "de",
            "en-gb" | "en/gb" | "en-us" => "en",
            "fr-fr" | "fr/fr" => "fr",
            "nl-nl" | "nl/nl" => "nl",
            "ru-ru" | "ru/ru" => "ru",
            "zh-hans" | "zh/hans" | "zh-hk" | "zh/hk" => "zh",
            other => other,
        };

        self.models
            .iter()
            .filter(|m| m.language == normalized_lang || m.language == "multilingual")
            .collect()
    }

    /// Add a custom model to the registry
    pub fn add_model(&mut self, model: NerModelInfo) {
        self.models.push(model);
    }

    /// Remove a model from the registry by ID
    pub fn remove_model(&mut self, model_id: &str) -> bool {
        if let Some(pos) = self.models.iter().position(|m| m.model_id == model_id) {
            self.models.remove(pos);
            true
        } else {
            false
        }
    }
}

impl Default for NerModelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_models() {
        let registry = NerModelRegistry::new();
        assert!(registry.list_models().len() > 0);
    }

    #[test]
    fn test_get_model_by_id() {
        let registry = NerModelRegistry::new();
        let model = registry.get_model("dslim/bert-base-NER");
        assert!(model.is_some());
        assert_eq!(model.unwrap().name, "BERT-base NER");
    }

    #[test]
    fn test_get_models_by_language() {
        let registry = NerModelRegistry::new();
        let en_models = registry.get_models_by_language("en");
        assert!(en_models.len() > 0);
    }

    #[test]
    fn test_recommended_models() {
        let registry = NerModelRegistry::new();

        assert!(registry.get_recommended_model().is_some());
        assert!(registry.get_fastest_model().is_some());
        assert!(registry.get_most_accurate_model().is_some());
        assert!(registry.get_multilingual_model().is_some());
    }

    #[test]
    fn test_add_custom_model() {
        let mut registry = NerModelRegistry::new();
        let initial_count = registry.list_models().len();

        let custom_model = NerModelInfo {
            model_id: "custom/test-model".to_string(),
            name: "Test Model".to_string(),
            description: "A test model".to_string(),
            provider: "Custom".to_string(),
            model_type: "bert".to_string(),
            language: "en".to_string(),
            entity_labels: vec!["O".to_string()],
            size: "small".to_string(),
            parameters: "10M".to_string(),
            format: "safetensors".to_string(),
            model_url: "https://example.com/model".to_string(),
            config_url: "https://example.com/config".to_string(),
            tokenizer_url: "https://example.com/tokenizer".to_string(),
            file_size: 10_000_000,
            checksum: None,
            license: "MIT".to_string(),
            accuracy: None,
        };

        registry.add_model(custom_model);
        assert_eq!(registry.list_models().len(), initial_count + 1);
    }

    #[test]
    fn test_remove_model() {
        let mut registry = NerModelRegistry::new();
        let initial_count = registry.list_models().len();

        let removed = registry.remove_model("dslim/bert-base-NER");
        assert!(removed);
        assert_eq!(registry.list_models().len(), initial_count - 1);
    }
}
