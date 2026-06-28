use crate::aligner::aligner_define::{Aligner, TokenizationAlgorithm};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::Write;

fn predicats_start_of_word_teacher(
    tok_algo_teacher: &TokenizationAlgorithm,
    teacher_token_str: &String,
) -> bool {
    match tok_algo_teacher {
        &TokenizationAlgorithm::BPE => teacher_token_str.starts_with("Ġ"),
        &TokenizationAlgorithm::WordPiece => !teacher_token_str.starts_with("##"),
        &TokenizationAlgorithm::Unigram => teacher_token_str.starts_with("▁"),
        &TokenizationAlgorithm::WordLevel => false,
    }
}

fn predicats_start_of_word_student(
    tok_algo_teacher: &TokenizationAlgorithm,
    tok_algo_student: &TokenizationAlgorithm,
    teacher_token_str: &String,
) -> String {
    match tok_algo_teacher {
        &TokenizationAlgorithm::BPE => match tok_algo_student {
            &TokenizationAlgorithm::BPE => teacher_token_str.to_string(),
            &TokenizationAlgorithm::WordPiece => {
                teacher_token_str.strip_prefix("Ġ").unwrap().to_string()
            }
            &TokenizationAlgorithm::Unigram => {
                let mut mutable_teacher_token_str =
                    teacher_token_str.strip_prefix("Ġ").unwrap().to_string();
                mutable_teacher_token_str.insert_str(0, "▁");
                mutable_teacher_token_str
            }
            &TokenizationAlgorithm::WordLevel => "WordLevelCase".to_string(),
        },
        &TokenizationAlgorithm::WordPiece => match tok_algo_student {
            &TokenizationAlgorithm::BPE => {
                let mut mutable_teacher_token_str = String::from(teacher_token_str);
                mutable_teacher_token_str.insert_str(0, "Ġ");
                mutable_teacher_token_str.to_string()
            }
            &TokenizationAlgorithm::WordPiece => teacher_token_str.to_string(),
            &TokenizationAlgorithm::Unigram => {
                let mut mutable_teacher_token_str = String::from(teacher_token_str);
                mutable_teacher_token_str.insert_str(0, "▁");
                mutable_teacher_token_str.to_string()
            }
            &TokenizationAlgorithm::WordLevel => "WordLevelCase".to_string(),
        },
        &TokenizationAlgorithm::Unigram => match tok_algo_student {
            &TokenizationAlgorithm::BPE => {
                let mutable_teacher_token_str = teacher_token_str.strip_prefix("▁");
                match mutable_teacher_token_str {
                    Some(p) => {
                        let mut new_p = p.to_string();
                        new_p.insert_str(0, "Ġ");
                        new_p
                    }
                    None => {
                        let none_val = "Aint do shit".to_string();
                        none_val
                    }
                }
            }
            &TokenizationAlgorithm::WordPiece => {
                teacher_token_str.strip_prefix("▁").unwrap().to_string()
            }
            &TokenizationAlgorithm::Unigram => teacher_token_str.to_string(),
            &TokenizationAlgorithm::WordLevel => "WordLevelCase".to_string(),
        },
        &TokenizationAlgorithm::WordLevel => "WordLevelCase".to_string(),
    }
}

fn predicats_middle_of_word_student(
    tok_algo_teacher: &TokenizationAlgorithm,
    tok_algo_student: &TokenizationAlgorithm,
    teacher_token_str: &String,
) -> String {
    match tok_algo_teacher {
        &TokenizationAlgorithm::BPE => match tok_algo_student {
            &TokenizationAlgorithm::BPE => teacher_token_str.to_string(),
            &TokenizationAlgorithm::WordPiece => {
                let mut mutable_teacher_token_str = String::from(teacher_token_str);
                mutable_teacher_token_str.insert_str(0, "##");
                mutable_teacher_token_str.to_string()
            }
            &TokenizationAlgorithm::Unigram => teacher_token_str.to_string(),
            &TokenizationAlgorithm::WordLevel => "WordLevelCase".to_string(),
        },
        &TokenizationAlgorithm::WordPiece => match tok_algo_student {
            &TokenizationAlgorithm::BPE => {
                teacher_token_str.strip_prefix("##").unwrap().to_string()
            }
            &TokenizationAlgorithm::WordPiece => teacher_token_str.to_string(),
            &TokenizationAlgorithm::Unigram => {
                teacher_token_str.strip_prefix("##").unwrap().to_string()
            }
            &TokenizationAlgorithm::WordLevel => "WordLevelCase".to_string(),
        },
        &TokenizationAlgorithm::Unigram => match tok_algo_student {
            &TokenizationAlgorithm::BPE => teacher_token_str.to_string(),
            &TokenizationAlgorithm::WordPiece => {
                let mut mutable_teacher_token_str = String::from(teacher_token_str);
                mutable_teacher_token_str.insert_str(0, "##");
                mutable_teacher_token_str.to_string()
            }
            &TokenizationAlgorithm::Unigram => teacher_token_str.to_string(),
            &TokenizationAlgorithm::WordLevel => "WordLevelCase".to_string(),
        },
        &TokenizationAlgorithm::WordLevel => "WordLevelCase".to_string(),
    }
}

fn save_as_txt<T: Serialize + Debug>(arr: &Vec<T>, name_of_file: &str) {
    let list_as_json = serde_json::to_string(arr).unwrap();
    let mut file = File::create(name_of_file).expect("Could not create file!");

    for elem in arr {
        writeln!(file, "{:?}", elem).unwrap();
    }
}

#[derive(Debug)]
pub struct Matrix {
    pub output: HashMap<(String, u32), Vec<(String, u32)>>,
}

impl Matrix {
    pub fn generate_output_matrix(tokenizer_student: Aligner, tokenizer_teacher: Aligner) -> Self {
        let mut debug_vect: Vec<(String, String, u32)> = Vec::new();
        let mut debug_vect_middle: Vec<(String, String, Vec<(String, u32)>)> = Vec::new();
        let teacher_vocab = tokenizer_teacher.vocab;
        let student_vocab = tokenizer_student.vocab;
        let mut output_matrix = Matrix {
            output: HashMap::new(),
        };
        let mut debug_vec3: Vec<String> = Vec::new();
        // let vect_for_middle_words: Vec<String> = Vec::new();
        for (teacher_token_str, teacher_token_idx) in teacher_vocab {
            // 1. Start of the word case
            if predicats_start_of_word_teacher(
                &tokenizer_teacher.tokenization_algorithm,
                &teacher_token_str,
            ) == true
            {
                let clones = teacher_token_str.clone();
                debug_vec3.push(String::from(clones));
                let supposed_student_token = predicats_start_of_word_student(
                    &tokenizer_teacher.tokenization_algorithm,
                    &tokenizer_student.tokenization_algorithm,
                    &teacher_token_str,
                );
                let is_such_token = student_vocab.contains_key(&supposed_student_token);

                // 1.1. Exact match case
                if is_such_token == true {
                    // println!("Индекс совпавшего слова ({}) у vocab_bpe — {:?}", token_str_wp, vocab_bpe.get(&supposed_student_token));
                    let student_tokex_idx = *student_vocab.get(&supposed_student_token).unwrap();
                    let temp_str = supposed_student_token.clone();
                    let temp_str2 = teacher_token_str.clone();
                    debug_vect.push((temp_str2, temp_str, student_tokex_idx));
                    output_matrix.output.insert(
                        (teacher_token_str, teacher_token_idx),
                        vec![(supposed_student_token, student_tokex_idx)],
                    );
                }
                // 1.2. Composite token case
                else {
                    println!("========================================");
                    println!("Несматченное слово — {supposed_student_token}");
                    let len_of_teacher_word: usize = supposed_student_token.len();
                    println!("len_of_word={}", len_of_teacher_word);
                    let mut token_sum: Vec<(String, u32)> = Vec::new();

                    let mut window_start = 0;
                    'outer: while window_start != len_of_teacher_word {
                        let mut pop_counter = 0;

                        let new_window_start = window_start;
                        println!("new_window_start={}", new_window_start);
                        let slice = supposed_student_token[new_window_start..len_of_teacher_word]
                            .to_string();
                        println!("slice3={slice}");
                        let mut poping_slice = supposed_student_token
                            [new_window_start..len_of_teacher_word]
                            .to_string();

                        for _ in slice.chars() {
                            let supposed_student_token = student_vocab.get(&poping_slice);
                            match supposed_student_token {
                                Some(index) => {
                                    println!("Нашли токен {}", poping_slice);
                                    token_sum.push((poping_slice.clone(), *index));
                                    window_start = len_of_teacher_word - pop_counter;
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
                    let temp_str2 = teacher_token_str.clone();
                    let temp_str = supposed_student_token.clone();
                    let temp_str_3 = token_sum.clone();
                    debug_vect_middle.push((temp_str2, temp_str, temp_str_3));
                    output_matrix
                        .output
                        .insert((teacher_token_str, teacher_token_idx), token_sum);
                }
            }
            // 2. Middle of the word case
            else {
                // let clones = teacher_token_str.clone();
                // debug_vec3.push(String::from(clones));
                let supposed_student_token = predicats_middle_of_word_student(
                    &tokenizer_teacher.tokenization_algorithm,
                    &tokenizer_student.tokenization_algorithm,
                    &teacher_token_str,
                );
                let is_such_token = student_vocab.contains_key(&supposed_student_token);

                // 2.1 Exact match case
                if is_such_token == true {
                    // println!("Индекс совпавшего слова ({}) у vocab_bpe — {:?}", token_str_wp, vocab_bpe.get(&supposed_student_token));
                    let student_tokex_idx = *student_vocab.get(&supposed_student_token).unwrap();
                    output_matrix.output.insert(
                        (teacher_token_str, teacher_token_idx),
                        vec![(supposed_student_token, student_tokex_idx)],
                    );
                }
                // 2.2 Composite token case
                else {
                    println!("========================================");
                    println!("Несматченное слово — {}", supposed_student_token.clone());
                    let len_of_teacher_word: usize = supposed_student_token.len();
                    println!("len_of_word={}", len_of_teacher_word);
                    let mut token_sum: Vec<(String, u32)> = Vec::new();
                    let mut window_start = 0;
                    'outer: while window_start != len_of_teacher_word {
                        let mut pop_counter = 0;

                        let new_window_start = window_start;
                        println!("new_window_start={}", new_window_start);
                        let slice = supposed_student_token[new_window_start..len_of_teacher_word]
                            .to_string();
                        println!("slice3={slice}");
                        let mut poping_slice = supposed_student_token
                            [new_window_start..len_of_teacher_word]
                            .to_string();

                        for _ in slice.chars() {
                            let supposed_student_token = student_vocab.get(&poping_slice);
                            match supposed_student_token {
                                Some(index) => {
                                    println!("Нашли токен {}", poping_slice);
                                    token_sum.push((poping_slice.clone(), *index));
                                    window_start = len_of_teacher_word - pop_counter;
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
                    output_matrix
                        .output
                        .insert((teacher_token_str, teacher_token_idx), token_sum);
                }
            }
        }
        save_as_txt(&debug_vect, "debug_1.txt");
        save_as_txt(&debug_vec3, "debug_2.txt");
        save_as_txt(&debug_vect_middle, "debug_3.txt");
        return output_matrix;
    }
}
