use serde::Serialize;
use std::collections::HashMap;
use tokenizers::tokenizer::Result;
mod aligner;

#[derive(Serialize)]
struct ToFile {
    wp_bpe_comparison_str: Vec<String>,
}

fn main() -> Result<()> {
    // needs http feature enabled

    let panic_str = "Something goes wrong";

    let mut model1 = aligner::aligner_define::Aligner {
        tokenizer: None,
        vocab: None,
    };
    model1.create_tokenizer("deepseek-ai/DeepSeek-V4-Pro");
    model1.create_vocab();

    let mut model2 = aligner::aligner_define::Aligner {
        tokenizer: None,
        vocab: None,
    };
    model2.create_tokenizer("bert-base-cased");
    model2.create_vocab();

    let mut exact_matrix: HashMap<Vec<String>, Vec<String>> = HashMap::new();

    let vocab_wp = model2.vocab.expect(panic_str);
    let vocab_bpe = model1.vocab.expect(panic_str);

    // let tokenizer_bpe = Tokenizer::from_pretrained("deepseek-ai/DeepSeek-V4-Pro", None)?;
    // let tokenizer_wp = Tokenizer::from_pretrained("bert-base-cased", None)?;
    for (token_str_wp, _) in vocab_wp {
        // 1. Start of the word case
        if token_str_wp.contains("##") == false {
            let supposed_bpe_token = format!("Ġ{token_str_wp}");
            let finding_key = vocab_bpe.contains_key(&supposed_bpe_token);

            // 1.1. Exact match case
            if finding_key == true {
                // println!("Индекс совпавшего слова ({}) у vocab_bpe — {:?}", token_str_wp, vocab_bpe.get(&supposed_bpe_token));
                exact_matrix.insert(vec![token_str_wp.to_string()], vec![supposed_bpe_token]);
            }
            // 1.2. Composite token case
            else {
                println!("========================================");
                println!("Несматченное слово — {token_str_wp}");
                let len_of_word = token_str_wp.len();
                println!("len_of_word={}", len_of_word);
                let mut token_sum: Vec<String> = Vec::new();
                let mut window_start = 0;
                'outer: while window_start != len_of_word {
                    let mut pop_counter = 0;

                    let new_window_start = window_start;
                    println!("new_window_start={}", new_window_start);
                    let slice = token_str_wp[new_window_start..len_of_word].to_string();
                    println!("slice3={slice}");
                    let mut poping_slice = token_str_wp[new_window_start..len_of_word].to_string();

                    for _ in slice.chars() {
                        let supposed_bpe_token = vocab_bpe.get(&poping_slice);
                        match supposed_bpe_token {
                            Some(_) => {
                                println!("Нашли токен {}", poping_slice);
                                token_sum.push(poping_slice.clone());
                                window_start = len_of_word - pop_counter;
                                break;
                            }
                            None => println!("Такого токена нет в соседнем словаре"),
                        }
                        println!(
                            "Слово для pop() — {}, его длина — {}",
                            poping_slice,
                            poping_slice.len()
                        );
                        let before_pop_len = &poping_slice.len();
                        poping_slice.pop();
                        if poping_slice == "" {
                            println!("Вышли из цикла по крайней причине");

                            break 'outer;
                        }
                        println!(
                            "Слово после pop() — {}, его длина — {}",
                            poping_slice,
                            poping_slice.len()
                        );
                        let after_pop_len = &poping_slice.len();
                        pop_counter += before_pop_len - after_pop_len;
                    }
                    println!("Полученный вектор токенов: {:?}", token_sum);
                }
                exact_matrix.insert(vec![token_str_wp.to_string()], token_sum);
            }
        }
        // 2. Middle of the word case
        else {
            let supposed_bpe_token = token_str_wp.replace("##", "");
            let finding_key = vocab_bpe.contains_key(&supposed_bpe_token);

            // 2.1 Exact match case
            if finding_key == true {
                // println!("Индекс совпавшего слова ({}) у vocab_bpe — {:?}", token_str_wp, vocab_bpe.get(&supposed_bpe_token));
                exact_matrix.insert(vec![token_str_wp.to_string()], vec![supposed_bpe_token]);
            }
            // 2.2 Composite token case
            else {
                println!("========================================");
                println!("Несматченное слово — {token_str_wp}");
                let len_of_word = token_str_wp.len();
                println!("len_of_word={}", len_of_word);
                let mut token_sum: Vec<String> = Vec::new();
                let mut window_start = 0;
                'outer: while window_start != len_of_word {
                    let mut pop_counter = 0;

                    let new_window_start = window_start;
                    println!("new_window_start={}", new_window_start);
                    let slice = token_str_wp[new_window_start..len_of_word].to_string();
                    println!("slice3={slice}");
                    let mut poping_slice = token_str_wp[new_window_start..len_of_word].to_string();

                    for _ in slice.chars() {
                        let supposed_bpe_token = vocab_bpe.get(&poping_slice);
                        match supposed_bpe_token {
                            Some(_) => {
                                println!("Нашли токен {}", poping_slice);
                                token_sum.push(poping_slice.clone());
                                window_start = len_of_word - pop_counter;
                                break;
                            }
                            None => println!("Такого токена нет в соседнем словаре"),
                        }
                        println!(
                            "Слово для pop() — {}, его длина — {}",
                            poping_slice,
                            poping_slice.len()
                        );
                        let before_pop_len = &poping_slice.len();
                        poping_slice.pop();
                        if poping_slice == "" {
                            println!("Вышли из цикла по крайней причине");

                            break 'outer;
                        }
                        println!(
                            "Слово после pop() — {}, его длина — {}",
                            poping_slice,
                            poping_slice.len()
                        );
                        let after_pop_len = &poping_slice.len();
                        pop_counter += before_pop_len - after_pop_len;
                    }
                    println!("Полученный вектор токенов: {:?}", token_sum);
                }
                exact_matrix.insert(vec![token_str_wp.to_string()], token_sum);
            }
        }
    }

    let mut output_str: Vec<String> = Vec::new();

    for (key, value) in exact_matrix {
        output_str.push(format!("{:?} — {:?}", key, value));
    }

    let my_obj = ToFile {
        wp_bpe_comparison_str: output_str,
    };
    let out = std::fs::File::create("out.json").unwrap();
    serde_json::to_writer_pretty(out, &my_obj)?;
    Ok(())
}
