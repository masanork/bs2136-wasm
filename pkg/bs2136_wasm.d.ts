/* tslint:disable */
/* eslint-disable */
/**
* @param {string} n
* @returns {string}
*/
export function encode_integer_for_wasm(n: string): string;
/**
* @param {string} encoded
* @returns {string}
*/
export function decode_integer_for_wasm(encoded: string): string;
/**
* @param {Uint8Array} input
* @returns {string}
*/
export function encode_bytestream_for_wasm(input: Uint8Array): string;
/**
* @param {string} encoded
* @returns {Uint8Array}
*/
export function decode_bytestream_for_wasm(encoded: string): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly encode_integer_for_wasm: (a: number, b: number, c: number) => void;
  readonly decode_integer_for_wasm: (a: number, b: number, c: number) => void;
  readonly encode_bytestream_for_wasm: (a: number, b: number, c: number) => void;
  readonly decode_bytestream_for_wasm: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
