/* eslint-disable unicorn/no-abusive-eslint-disable */
/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {string}
*/
export function str_input(input: string): string;
/**
* @param {string} input
* @returns {any}
*/
export function parse(input: string): any;
/**
* @param {any} input
* @returns {string}
*/
export function stringify(input: any): string;
/**
* @param {string} input
* @param {string} path
* @param {any} value
* @returns {string}
*/
export function edit(input: string, path: string, value: any): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly str_input: (a: number, b: number, c: number) => void;
  readonly parse: (a: number, b: number, c: number) => void;
  readonly stringify: (a: number, b: number) => void;
  readonly edit: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
