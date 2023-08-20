import init, { encode_integer, decode_integer } from './pkg/bs2136_wasm.js';

async function run() {
    await init();
}

run();

function encode() {
    const number = BigInt(document.getElementById("numberInput").value);
    const encoded = encode_integer(number.toString());
    document.getElementById("encodedOutput").textContent = encoded;
}

function decode() {
    const kanji = document.getElementById("kanjiInput").value;
    const decoded = decode_integer(kanji);
    document.getElementById("decodedOutput").textContent = decoded.toString();
}

window.encode = encode;
window.decode = decode;
