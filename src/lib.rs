use wasm_bindgen::prelude::*;

const JOYOKANJI: &str = include_str!("joyokanji.txt");

fn encode_block(mut n: u64) -> String {
    let kanji_chars: Vec<char> = JOYOKANJI.chars().collect();
    let mut result = String::new();

    for _ in 0..4 {
        let index = (n % 2136) as usize;
        result.insert(0, kanji_chars[index]);
        n /= 2136;
    }

    result
}

fn decode_block(kanji_str: &str) -> u64 {
    let kanji_chars: Vec<char> = JOYOKANJI.chars().collect();
    let mut result = 0u64;

    for kanji in kanji_str.chars() {
        let index = kanji_chars.iter().position(|&c| c == kanji).expect("Kanji not found!");
        result = result * 2136 + index as u64;
    }

    result
}

#[wasm_bindgen]
pub fn encode_to_kanji(input: &[u8]) -> String {
    let blocks = input.chunks(8);
    let mut output = String::new();

    for block in blocks {
        let mut number = [0u8; 8];
        for (i, byte) in block.iter().enumerate() {
            number[i] = *byte;
        }
        let number = u64::from_be_bytes(number);
        output.push_str(&encode_block(number));
    }

    output
}

#[wasm_bindgen]
pub fn decode_from_kanji(kanji_str: &str) -> Vec<u8> {
    let kanji_vec = kanji_str.chars().collect::<Vec<_>>();
    let blocks = kanji_vec.chunks(4);
    let mut output = Vec::new();

    for block in blocks {
        let block_str: String = block.iter().collect();
        let number = decode_block(&block_str);
        output.extend(&number.to_be_bytes());
    }

    output
}
