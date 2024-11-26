let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}
/**
 * @returns {any}
 */
export function get_memory() {
    const ret = wasm.get_memory();
    return ret;
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

const GameOfLifeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gameoflife_free(ptr >>> 0, 1));

export class GameOfLife {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GameOfLife.prototype);
        obj.__wbg_ptr = ptr;
        GameOfLifeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GameOfLifeFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gameoflife_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get width() {
        const ret = wasm.__wbg_get_gameoflife_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set width(arg0) {
        wasm.__wbg_set_gameoflife_width(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get height() {
        const ret = wasm.__wbg_get_gameoflife_height(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set height(arg0) {
        wasm.__wbg_set_gameoflife_height(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} w
     * @param {number} h
     * @returns {GameOfLife}
     */
    static new(w, h) {
        const ret = wasm.gameoflife_new(w, h);
        return GameOfLife.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    size() {
        const ret = wasm.gameoflife_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    render() {
        const ret = wasm.gameoflife_render(this.__wbg_ptr);
        return ret >>> 0;
    }
    tick() {
        wasm.gameoflife_tick(this.__wbg_ptr);
    }
    partick() {
        wasm.gameoflife_partick(this.__wbg_ptr);
    }
    /**
     * @param {number} row
     * @param {number} col
     */
    toggle_cell(row, col) {
        wasm.gameoflife_toggle_cell(this.__wbg_ptr, row, col);
    }
}

export const __wbg_random_ff204240120aa46e = typeof Math.random == 'function' ? Math.random : notDefined('Math.random');

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_memory() {
    const ret = wasm.memory;
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

