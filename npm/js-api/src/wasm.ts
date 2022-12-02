export type WasmBundler = typeof import("@rometools/wasm-bundler");
export type WasmNodejs = typeof import("@rometools/wasm-nodejs");
export type WasmWeb = typeof import("@rometools/wasm-web");

export type WasmModule = WasmBundler | WasmNodejs | WasmWeb;

/**
 * What kind of client Rome should use to communicate with the binary
 */
export enum Distribution {
	/**
	 * Use this if you want to communicate with the WebAssembly client built for bundlers
	 */
	BUNDLER,
	/**
	 * Use this if you want to communicate with the WebAssembly client built for Node.JS
	 */
	NODE,
	/**
	 * Use this if you want to communicate with the WebAssembly client built for the Web
	 */
	WEB,
}

const isInitialized = {
	[Distribution.BUNDLER]: false,
	[Distribution.NODE]: false,
	[Distribution.WEB]: false,
};

export async function loadModule(dist: Distribution): Promise<WasmModule> {
	let modulePromise: Promise<WasmModule>;

	switch (dist) {
		case Distribution.BUNDLER: {
			modulePromise = import("@rometools/wasm-bundler");
			break;
		}
		case Distribution.NODE: {
			modulePromise = import("@rometools/wasm-nodejs");
			break;
		}
		case Distribution.WEB: {
			modulePromise = import("@rometools/wasm-web");
			break;
		}
	}

	const module = await modulePromise;

	if (!isInitialized[dist]) {
		isInitialized[dist] = true;
		module.main();
	}

	return module;
}

/**
 * The error generated when communicating with WebAssembly
 */
class WasmError extends Error {
	/**
	 * The stack trace of the error.
	 *
	 * It might be useful, but the first like of the stack trace contains the error
	 */
	public stackTrace: string;
	private constructor(stackTrace: string) {
		super();
		this.stackTrace = stackTrace;
	}

	static fromError(e: unknown): WasmError {
		return new WasmError(e as string);
	}
}

/**
 * Creates wrap a WebAssembly error into a native JS Error
 *
 * @param e
 */
export function wrapError(e: unknown): WasmError {
	return WasmError.fromError(e);
}
