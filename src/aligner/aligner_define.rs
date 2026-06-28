use std::collections::HashMap;
use tokenizers::models::ModelWrapper;
use tokenizers::tokenizer::{Result, Tokenizer};

#[derive(Debug)]
pub enum TokenizationAlgorithm {
    BPE,
    WordPiece,
    Unigram,
    WordLevel,
}

pub struct Aligner {
    pub tokenizer: Tokenizer,
    pub vocab: HashMap<String, u32>,
    pub tokenization_algorithm: TokenizationAlgorithm,
}

impl Aligner {
    pub fn create_tokenizer(model_name: &str) -> Result<Aligner> {
        let tokenizer = Tokenizer::from_pretrained(model_name, None)?;
        let vocab = tokenizer.get_vocab(true);
        let tokenization_algorithm = match tokenizer.get_model().clone() {
            ModelWrapper::BPE(_) => TokenizationAlgorithm::BPE,
            ModelWrapper::WordPiece(_) => TokenizationAlgorithm::WordPiece,
            ModelWrapper::Unigram(_) => TokenizationAlgorithm::Unigram,
            ModelWrapper::WordLevel(_) => TokenizationAlgorithm::WordLevel,
        };
        Ok(Self {
            tokenizer,
            vocab,
            tokenization_algorithm,
        })
    }
}
