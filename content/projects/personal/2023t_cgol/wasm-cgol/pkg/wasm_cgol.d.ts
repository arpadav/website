/* tslint:disable */
/* eslint-disable */
/**
 * @returns {any}
 */
export function get_memory(): any;
export class GameOfLife {
  free(): void;
  /**
   * @param {number} w
   * @param {number} h
   * @param {boolean} wrap
   * @returns {GameOfLife}
   */
  static new(w: number, h: number, wrap: boolean): GameOfLife;
  /**
   * @param {number} w
   * @param {number} h
   * @param {Uint8Array} cells
   * @param {boolean} wrap
   * @returns {GameOfLife}
   */
  static load(w: number, h: number, cells: Uint8Array, wrap: boolean): GameOfLife;
  /**
   * @returns {number}
   */
  ptr(): number;
  clear(): void;
  /**
   * @param {number} row
   * @param {number} col
   */
  toggle_cell(row: number, col: number): void;
  /**
   * @param {boolean} val
   */
  wrap(val: boolean): void;
  tick(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_gameoflife_free: (a: number, b: number) => void;
  readonly gameoflife_new: (a: number, b: number, c: number) => number;
  readonly gameoflife_load: (a: number, b: number, c: number, d: number) => number;
  readonly gameoflife_ptr: (a: number) => number;
  readonly gameoflife_clear: (a: number) => void;
  readonly gameoflife_toggle_cell: (a: number, b: number, c: number) => void;
  readonly gameoflife_wrap: (a: number, b: number) => void;
  readonly gameoflife_tick: (a: number) => void;
  readonly get_memory: () => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
