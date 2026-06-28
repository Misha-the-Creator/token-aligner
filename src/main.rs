use serde::Serialize;
use tokenizers::tokenizer::Result;
mod aligner;
mod matching_algorithm;

#[derive(Serialize)]
struct ToFile {
    wp_bpe_comparison_str: Vec<String>,
}

fn main() -> Result<()> {
    let model_1 = aligner::aligner_define::Aligner::create_tokenizer("xlnet/xlnet-base-cased")?; // WP
    let model_2 =
        aligner::aligner_define::Aligner::create_tokenizer("google-bert/bert-base-cased")?; // BPE

    // dbg!(model_1.tokenization_algorithm);

    let out_mtrx =
        matching_algorithm::teacher_flow::Matrix::generate_output_matrix(model_2, model_1);
    // dbg!(out_mtrx);

    let mut output_str: Vec<String> = Vec::new();

    for (key, value) in out_mtrx.output {
        output_str.push(format!("{:?} — {:?}", key, value));
    }

    let my_obj = ToFile {
        wp_bpe_comparison_str: output_str,
    };
    let out = std::fs::File::create("out1.json").unwrap();
    serde_json::to_writer_pretty(out, &my_obj)?;

    Ok(())
}
