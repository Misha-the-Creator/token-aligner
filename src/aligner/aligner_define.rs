use std::collections::HashMap;
use tokenizers::tokenizer::Tokenizer;

pub struct Aligner {
    pub tokenizer: Option<Tokenizer>,
    pub vocab: Option<HashMap<String, u32>>,
}

impl Aligner {
    pub fn create_tokenizer(&mut self, model_name: &str) {
        let tokenizer = Tokenizer::from_pretrained(model_name, None)
            .expect("Something goes wrong while getting tokenizer");
        self.tokenizer = Some(tokenizer);
    }

    pub fn create_vocab(&mut self) {
        let vocab = self
            .tokenizer
            .as_ref()
            .expect("Smth goes wrong")
            .get_vocab(true);
        self.vocab = Some(vocab);
    }
}
