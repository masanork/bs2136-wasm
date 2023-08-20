use wasm_bindgen::prelude::*;

const JOYOKANJI: &str = include_str!("joyokanji.txt");
const CHUNK_SIZE_BITS: u64 = 44;
const MAX_CHUNK_VALUE: u64 = (1 << CHUNK_SIZE_BITS) - 1;

fn encode_single_block(mut n: u64) -> String {
    let kanji_chars: Vec<char> = JOYOKANJI.chars().collect();
    let mut block = String::new();

    for _ in 0..4 {
        let index = (n % 2136) as usize;
        block.insert(0, kanji_chars[index]);
        n /= 2136;
    }

    block
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
pub fn encode_integer(n: u64) -> String {
    let mut number = n;
    let mut result = String::new();

    while number > 0 {
        let chunk = number & MAX_CHUNK_VALUE;
        result = encode_single_block(chunk) + &result;
        number >>= CHUNK_SIZE_BITS;
    }

    if result.is_empty() {
        return encode_single_block(0);
    }

    result
}

#[wasm_bindgen]
pub fn decode_integer(encoded: &str) -> u64 {
    let kanji_vec = encoded.chars().collect::<Vec<_>>();
    let blocks = kanji_vec.chunks(4);
    let mut result = 0u64;

    for block in blocks {
        let block_str: String = block.iter().collect();
        result <<= CHUNK_SIZE_BITS;
        result += decode_block(&block_str);
    }

    result
}
