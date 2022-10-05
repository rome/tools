import { BackendKind } from "./index";
import { DaemonError } from "./daemon";
import { WasmError } from "./nodeWasm";

/**
 * Creates an error based on backend kind
 *
 * @param e
 * @param backendKind
 */
export function createError(
	e: any,
	backendKind: BackendKind,
): DaemonError | WasmError {
	if (backendKind === BackendKind.NODE) {
		return WasmError.fromError(e);
	} else {
		return DaemonError.fromError(e);
	}
}
