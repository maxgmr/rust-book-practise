#![allow(unused)]

use std::cmp;
use std::collections::HashMap;

pub fn start() {
    let v1 = vec![25, 50, 34, 45, 7, 46, 33, 57, 50, 7, 25, 26, 25];
    let v2 = vec![6];
    let v3 = vec![1, 2, 3, 5, 3, 5, 6, 7, 4, 4, 2, 6, 1, 5, 3];
    let v4: Vec<i32> = vec![];

    let median1 = median(&v1).unwrap_or(0);
    let median2 = median(&v2).unwrap_or(0);
    let median3 = median(&v3).unwrap_or(0);
    let median4 = median(&v4).unwrap_or(0);

    let mode1 = mode(&v1).unwrap_or(0);
    let mode2 = mode(&v2).unwrap_or(0);
    let mode3 = mode(&v3).unwrap_or(0);
    let mode4 = mode(&v4).unwrap_or(0);

    println!("median1: {median1}, median2: {median2}, median3: {median3}, median4: {median4}");
    println!("mode1: {mode1}, mode2: {mode2}, mode3: {mode3}, mode4: {mode4}");

    let str1 = String::from("All human beings are born free and equal in dignity and rights. They are endowed with reason and conscience and should act towards one another in a spirit of brotherhood.");
    let str2 = String::from("So do not fear, for I am with you; do not be dismayed, for I am your God. I will strengthen you and help you; I will uphold you with my righteous right hand.");
    println!("{}", to_pig_latin(&str1));
    println!("{}", to_pig_latin(&str2));
}

fn to_pig_latin(str: &str) -> String {
    fn word_to_pig_latin(word: &str) -> String {
        let mut chars = word.chars();

        let first_char = match chars.next() {
            Some(c) => c,
            None => return String::new(),
        };

        match first_char {
            'A' | 'a' | 'E' | 'e' | 'I' | 'i' | 'O' | 'o' | 'U' | 'u' => match chars.next_back() {
                Some(last_char) => match last_char {
                    '.' | '\'' | '"' | ',' | ';' | '-' | '?' | '!' | ':' => {
                        format!("{}{}-hay{}", first_char, chars.as_str(), last_char)
                    }
                    _ => format!("{}-hay", word),
                },
                None => format!("{}-hay", word),
            },
            _ => match chars.next_back() {
                Some(last_char) => match last_char {
                    '.' | '\'' | '"' | ',' | ';' | '-' | '?' | '!' | ':' => {
                        format!("{}-{}ay{}", chars.as_str(), first_char, last_char)
                    }
                    _ => format!("{}{}-{}ay", chars.as_str(), last_char, first_char),
                },
                None => format!("{}-hay", word),
            },
        }
    }

    fn folder(mut current: String, next: String) -> String {
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(&next);
        current
    }

    str.split_whitespace()
        .map(word_to_pig_latin)
        .fold(String::new(), folder)
}

fn median(vec: &Vec<i32>) -> Option<i32> {
    let mut sorted_vec = vec.to_vec();
    sorted_vec.sort();
    match sorted_vec.get(sorted_vec.len() / 2) {
        Some(v) => Some(*v),
        None => None,
    }
}

fn mode(vec: &Vec<i32>) -> Option<i32> {
    let mut count_map = HashMap::new();
    for v in vec {
        let count = count_map.entry(v).or_insert(0);
        *count += 1;
    }
    let mut max_count = -1;
    let mut max_key = None;
    for (key, count) in &count_map {
        if *count > max_count {
            max_count = *count;
            max_key = Some(**key);
        }
    }
    max_key
}
