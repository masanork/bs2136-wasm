use wasm_bindgen::prelude::*;

const JOYOKANJI: &str = include_str!("joyokanji.txt");
const MAX_VALUE: u64 = 2136 * 2136 * 2136 * 2136 - 1;

#[wasm_bindgen]
pub fn encode_integer(n: u64) -> String {
    if n > MAX_VALUE {
        return "overflow".to_string();
    }

    let kanji_chars: Vec<char> = JOYOKANJI.chars().collect();
    let mut block = String::new();
    let mut number = n;

    for _ in 0..4 {
        let index = (number % 2136) as usize;
        block.insert(0, kanji_chars[index]);
        number /= 2136;
    }

    block
}

#[wasm_bindgen]
pub fn decode_integer(kanji_str: &str) -> u64 {
    let kanji_chars: Vec<char> = JOYOKANJI.chars().collect();
    let mut result = 0u64;

    for kanji in kanji_str.chars() {
        let index = kanji_chars.iter().position(|&c| c == kanji).expect("Kanji not found!");
        result = result * 2136 + index as u64;
    }

    result
}
