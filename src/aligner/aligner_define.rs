use tokenizers::{Error, tokenizer::Tokenizer};
use std::collections::HashMap;

struct Aligner {
    pub tokenizer: Tokenizer,
    pub vocab: HashMap<String, u32>,
    pub exact_matrix: HashMap<Vec<String>, Vec<String>>
}

impl Aligner {
    pub fn create_tokenizer(&self, model_name: &str) -> Result<Tokenizer, Error> {
        let x = Tokenizer::from_pretrained("deepseek-ai/DeepSeek-V4-Pro", None)?;
        Ok(x)
    }
}