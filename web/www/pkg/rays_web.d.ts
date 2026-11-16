/* tslint:disable */
/* eslint-disable */

/**
 * Chroma subsampling format
 */
export enum ChromaSampling {
    /**
     * Both vertically and horizontally subsampled.
     */
    Cs420 = 0,
    /**
     * Horizontally subsampled.
     */
    Cs422 = 1,
    /**
     * Not subsampled.
     */
    Cs444 = 2,
    /**
     * Monochrome.
     */
    Cs400 = 3,
}

/**
 * A scene on the GPU and the passes rendered so far.
 */
export class Renderer {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * The GPU's name and API.
     */
    adapter(): string;
    /**
     * Parse a scene (our JSON), flatten it and upload it; or say why it can't be rendered.
     * A width or height of 0 keeps the scene's.
     */
    static create(json: string, width: number, height: number): Promise<Renderer>;
    height(): number;
    /**
     * The image so far, finished and encoded for display as the command line's PNG is: RGBA,
     * top row first.
     */
    image(): Promise<Uint8Array>;
    /**
     * The image so far, finished but linear: RGB floats, top row first (for comparisons).
     */
    linear(): Promise<Float32Array>;
    /**
     * Queue the next pass: one sample of every pixel. Returns at once. Not while an `image`
     * or `linear` is awaited (an error then): the readback holds the renderer.
     */
    pass(): void;
    /**
     * Passes (samples per pixel) queued so far.
     */
    passes(): number;
    /**
     * The samples per pixel the scene asks for.
     */
    target(): number;
    width(): number;
}

/**
 * Supply a file a scene refers to, under the path the scene uses (eg. "./demo/teapot.obj").
 */
export function provide(path: string, bytes: Uint8Array | number[]): void;

export function start(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_renderer_free: (a: number, b: number) => void;
    readonly provide: (a: number, b: number, c: number, d: number) => void;
    readonly renderer_adapter: (a: number) => [number, number];
    readonly renderer_create: (a: number, b: number, c: number, d: number) => any;
    readonly renderer_height: (a: number) => number;
    readonly renderer_image: (a: number) => any;
    readonly renderer_linear: (a: number) => any;
    readonly renderer_pass: (a: number) => [number, number];
    readonly renderer_passes: (a: number) => number;
    readonly renderer_target: (a: number) => number;
    readonly renderer_width: (a: number) => number;
    readonly start: () => void;
    readonly wasm_bindgen_7c5a1d38c467384___convert__closures_____invoke___js_sys_e5596d40361065af___Function_fn_wasm_bindgen_7c5a1d38c467384___JsValue_____wasm_bindgen_7c5a1d38c467384___sys__Undefined___js_sys_e5596d40361065af___Function_fn_wasm_bindgen_7c5a1d38c467384___JsValue_____wasm_bindgen_7c5a1d38c467384___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_7c5a1d38c467384___convert__closures_____invoke___wasm_bindgen_7c5a1d38c467384___sys__JsNullable_wgpu_8aa7aa1458738504___backend__webgpu__webgpu_sys__gen_GpuAdapter__GpuAdapter___core_7d5f0a2ba6a62c33___result__Result_____wasm_bindgen_7c5a1d38c467384___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_7c5a1d38c467384___convert__closures_____invoke___wasm_bindgen_7c5a1d38c467384___sys__JsNullable_wgpu_8aa7aa1458738504___backend__webgpu__webgpu_sys__gen_GpuAdapter__GpuAdapter___core_7d5f0a2ba6a62c33___result__Result_____wasm_bindgen_7c5a1d38c467384___JsError___true__13: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_7c5a1d38c467384___convert__closures_____invoke___wasm_bindgen_7c5a1d38c467384___sys__JsNullable_wgpu_8aa7aa1458738504___backend__webgpu__webgpu_sys__gen_GpuAdapter__GpuAdapter___core_7d5f0a2ba6a62c33___result__Result_____wasm_bindgen_7c5a1d38c467384___JsError___true__14: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_7c5a1d38c467384___convert__closures_____invoke___wasm_bindgen_7c5a1d38c467384___sys__JsNullable_wgpu_8aa7aa1458738504___backend__webgpu__webgpu_sys__gen_GpuAdapter__GpuAdapter___core_7d5f0a2ba6a62c33___result__Result_____wasm_bindgen_7c5a1d38c467384___JsError___true__15: (a: number, b: number, c: any) => [number, number];
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
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
