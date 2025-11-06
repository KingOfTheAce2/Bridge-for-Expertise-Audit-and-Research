use anyhow::{Context, Result};
use candle_core::{Device, Tensor};
use std::path::Path;
use tokenizers::tokenizer::Tokenizer;

/// Tokenizer wrapper for NER tasks
pub struct NerTokenizer {
    tokenizer: Tokenizer,
    max_length: usize,
}

impl NerTokenizer {
    /// Load tokenizer from directory
    pub fn from_file(tokenizer_path: &Path, max_length: usize) -> Result<Self> {
        let tokenizer = Tokenizer::from_file(tokenizer_path)
            .map_err(|e| anyhow::anyhow!("Failed to load tokenizer: {}", e))?;

        Ok(Self {
            tokenizer,
            max_length,
        })
    }

    /// Tokenize text and return input tensors
    pub fn encode(&self, text: &str, device: &Device) -> Result<EncodingOutput> {
        // Encode text
        let encoding = self
            .tokenizer
            .encode(text, false)
            .map_err(|e| anyhow::anyhow!("Encoding error: {}", e))?;

        // Get token IDs
        let token_ids = encoding.get_ids();
        let attention_mask = encoding.get_attention_mask();
        let tokens = encoding.get_tokens().to_vec();
        let offsets = encoding.get_offsets().to_vec();

        // Truncate if necessary
        let length = token_ids.len().min(self.max_length);
        let token_ids = &token_ids[..length];
        let attention_mask = &attention_mask[..length];

        // Convert to tensors
        let input_ids = Tensor::from_slice(
            token_ids,
            (1, token_ids.len()),
            device,
        )?;

        let attention_mask = Tensor::from_slice(
            attention_mask,
            (1, attention_mask.len()),
            device,
        )?;

        // Token type IDs (all zeros for single sequence)
        let token_type_ids = Tensor::zeros((1, token_ids.len()), candle_core::DType::U32, device)?;

        Ok(EncodingOutput {
            input_ids,
            attention_mask,
            token_type_ids,
            tokens,
            offsets,
        })
    }

    /// Tokenize text in batches
    pub fn encode_batch(&self, texts: Vec<&str>, device: &Device) -> Result<Vec<EncodingOutput>> {
        texts
            .iter()
            .map(|text| self.encode(text, device))
            .collect()
    }

    /// Get special token IDs
    pub fn get_cls_token_id(&self) -> Option<u32> {
        self.tokenizer
            .token_to_id("[CLS]")
    }

    pub fn get_sep_token_id(&self) -> Option<u32> {
        self.tokenizer
            .token_to_id("[SEP]")
    }

    pub fn get_pad_token_id(&self) -> Option<u32> {
        self.tokenizer
            .token_to_id("[PAD]")
    }

    /// Decode token IDs back to text
    pub fn decode(&self, token_ids: &[u32], skip_special_tokens: bool) -> Result<String> {
        self.tokenizer
            .decode(token_ids, skip_special_tokens)
            .map_err(|e| anyhow::anyhow!("Decoding error: {}", e))
    }
}

/// Output from tokenization
pub struct EncodingOutput {
    pub input_ids: Tensor,
    pub attention_mask: Tensor,
    pub token_type_ids: Tensor,
    pub tokens: Vec<String>,
    pub offsets: Vec<(usize, usize)>, // Character offsets for each token
}

/// Align token predictions with original text
pub fn align_tokens_with_text(
    tokens: &[String],
    offsets: &[(usize, usize)],
    original_text: &str,
) -> Vec<TokenAlignment> {
    tokens
        .iter()
        .zip(offsets.iter())
        .filter_map(|(token, &(start, end))| {
            // Skip special tokens like [CLS], [SEP], [PAD]
            if token.starts_with('[') && token.ends_with(']') {
                return None;
            }

            // Skip subword tokens (starting with ##)
            let is_subword = token.starts_with("##");
            let clean_token = if is_subword {
                token.trim_start_matches("##")
            } else {
                token
            };

            Some(TokenAlignment {
                token: clean_token.to_string(),
                start,
                end,
                is_subword,
            })
        })
        .collect()
}

/// Token alignment information
#[derive(Debug, Clone)]
pub struct TokenAlignment {
    pub token: String,
    pub start: usize,
    pub end: usize,
    pub is_subword: bool,
}

/// Merge subword tokens into complete words
pub fn merge_subword_predictions(
    alignments: Vec<TokenAlignment>,
    predictions: Vec<(usize, f32)>, // (label_id, confidence)
) -> Vec<(String, usize, f32, usize, usize)> {
    // (text, label_id, confidence, start, end)
    let mut merged = Vec::new();
    let mut current_word = String::new();
    let mut current_label: Option<usize> = None;
    let mut current_confidence = 0.0;
    let mut current_start = 0;
    let mut current_end = 0;
    let mut token_count = 0;

    for (alignment, (label_id, confidence)) in alignments.iter().zip(predictions.iter()) {
        if alignment.is_subword {
            // Continue current word
            current_word.push_str(&alignment.token);
            current_confidence += confidence;
            current_end = alignment.end;
            token_count += 1;
        } else {
            // Start new word
            if !current_word.is_empty() {
                merged.push((
                    current_word.clone(),
                    current_label.unwrap_or(0),
                    current_confidence / token_count as f32,
                    current_start,
                    current_end,
                ));
            }

            current_word = alignment.token.clone();
            current_label = Some(*label_id);
            current_confidence = *confidence;
            current_start = alignment.start;
            current_end = alignment.end;
            token_count = 1;
        }
    }

    // Add last word
    if !current_word.is_empty() {
        merged.push((
            current_word,
            current_label.unwrap_or(0),
            current_confidence / token_count as f32,
            current_start,
            current_end,
        ));
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_alignment_subword() {
        let alignments = vec![
            TokenAlignment {
                token: "John".to_string(),
                start: 0,
                end: 4,
                is_subword: false,
            },
            TokenAlignment {
                token: "son".to_string(),
                start: 4,
                end: 7,
                is_subword: true,
            },
        ];

        let predictions = vec![(1, 0.9), (1, 0.85)];

        let merged = merge_subword_predictions(alignments, predictions);

        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].0, "Johnson");
        assert_eq!(merged[0].1, 1); // label_id
        assert!((merged[0].2 - 0.875).abs() < 0.01); // average confidence
    }

    #[test]
    fn test_merge_multiple_words() {
        let alignments = vec![
            TokenAlignment {
                token: "New".to_string(),
                start: 0,
                end: 3,
                is_subword: false,
            },
            TokenAlignment {
                token: "York".to_string(),
                start: 4,
                end: 8,
                is_subword: false,
            },
        ];

        let predictions = vec![(5, 0.9), (6, 0.85)]; // B-LOC, I-LOC

        let merged = merge_subword_predictions(alignments, predictions);

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].0, "New");
        assert_eq!(merged[1].0, "York");
    }
}
