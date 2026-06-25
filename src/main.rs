use serde::{Serialize};
use std::{collections::HashMap, fmt::format};
use tokenizers::tokenizer::{Result, Tokenizer};
mod aligner;


#[derive(Serialize)]
struct ToFile {
    wp_bpe_comparison_str: Vec<String>,
}


fn main() -> Result<()> {
    // needs http feature enabled
    let tokenizer_bpe = Tokenizer::from_pretrained("deepseek-ai/DeepSeek-V4-Pro", None)?;
    let tokenizer_wp = Tokenizer::from_pretrained("bert-base-cased", None)?;

    let vocab_bpe = tokenizer_bpe.get_vocab(true);
    let vocab_wp = tokenizer_wp.get_vocab(true);

    let mut exact_matrix:HashMap<Vec<String>, Vec<String>>  = HashMap::new();
    
    for (token_str_wp, token_id_wp) in &vocab_wp {
        
        // 1. Start of the word case
        if token_str_wp.contains("##") == false {
            let supposed_bpe_token = format!("Ġ{token_str_wp}");
            let finding_key = vocab_bpe.contains_key(&supposed_bpe_token);
            
            // 1.1. Exact match case
            if finding_key == true {
                // println!("Индекс совпавшего слова ({}) у vocab_bpe — {:?}", token_str_wp, vocab_bpe.get(&supposed_bpe_token));
                exact_matrix.insert(vec![token_str_wp.to_string()], vec![supposed_bpe_token]);
            }
        
        // 2. Middle of the word case
        else {
            let supposed_bpe_token = token_str_wp.replace("##", "");
            let finding_key = vocab_bpe.contains_key(&supposed_bpe_token);
            if finding_key == true {
                // println!("Индекс совпавшего слова ({}) у vocab_bpe — {:?}", token_str_wp, vocab_bpe.get(&supposed_bpe_token));
                exact_matrix.insert(vec![token_str_wp.to_string()], vec![supposed_bpe_token]);
            }
        }
            
    }}

    // dbg!(exact_matrix.keys());
    // dbg!(&exact_matrix);
    let mut vec1: Vec<Vec<String>> = Vec::new();
    let mut vec2: Vec<Vec<String>> = Vec::new();

    for (key,value) in exact_matrix {
        vec1.push(key.to_vec());
        vec2.push(value.to_vec());
    }

    let my_obj = ToFile{wp_vector: vec1, bpe_vector: vec2};
    let out = std::fs::File::create("out.txt").unwrap();
    serde_json::to_writer(out, &my_obj)?;
    Ok(())

}