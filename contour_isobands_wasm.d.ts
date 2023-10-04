declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* @param {Float64Array} data
	* @param {number} width
	* @param {number} height
	* @param {Float64Array} thresholds
	* @param {any} options
	* @returns {any}
	*/
	export function isolines(data: Float64Array, width: number, height: number, thresholds: Float64Array, options: any): any;
	/**
	* @param {Float64Array} data
	* @param {number} width
	* @param {number} height
	* @param {Float64Array} intervals
	* @param {any} options
	* @returns {any}
	*/
	export function isobands(data: Float64Array, width: number, height: number, intervals: Float64Array, options: any): any;
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly isolines: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly isobands: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
