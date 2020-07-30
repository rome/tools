/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticOrigin, Diagnostics} from "./types";
import {addOriginsToDiagnostics} from "./derive";
import {DiagnosticsError, getDiagnosticsFromError} from "./errors";
import DiagnosticsProcessor from "./DiagnosticsProcessor";

type WrapResult<T> =
	| {
			readonly value: T;
			readonly diagnostics: undefined;
		}
	| {
			readonly value: undefined;
			readonly diagnostics: Diagnostics;
		};

export async function catchDiagnostics<T>(
	promise: () => Promise<T>,
	origin?: DiagnosticOrigin,
): Promise<WrapResult<T>> {
	try {
		const value = await promise();

		return {value, diagnostics: undefined};
	} catch (err) {
		const diagnostics = getDiagnosticsFromError(err);

		if (diagnostics) {
			return {
				value: undefined,
				diagnostics: origin === undefined
					? diagnostics
					: addOriginsToDiagnostics([origin], diagnostics),
			};
		} else {
			throw err;
		}
	}
}

export async function interceptDiagnostics<T>(
	promise: () => Promise<T>,
	process: (processor: DiagnosticsProcessor) => void,
	origin?: DiagnosticOrigin,
): Promise<T> {
	const res = await catchDiagnostics(promise, origin);

	if (res.diagnostics !== undefined) {
		const processor = new DiagnosticsProcessor();
		process(processor);
		processor.addDiagnostics(res.diagnostics);
		throw new DiagnosticsError(
			"Intercepted diagnostics",
			processor.getDiagnostics(),
		);
	}

	return res.value;
}

export function catchDiagnosticsSync<T>(
	callback: () => T,
	origin?: DiagnosticOrigin,
): WrapResult<T> {
	try {
		const value = callback();

		return {value, diagnostics: undefined};
	} catch (err) {
		const diagnostics = getDiagnosticsFromError(err);

		if (diagnostics) {
			return {
				value: undefined,
				diagnostics: origin === undefined
					? diagnostics
					: addOriginsToDiagnostics([origin], diagnostics),
			};
		} else {
			throw err;
		}
	}
}
