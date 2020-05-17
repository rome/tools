/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticOrigin, Diagnostics} from "./types";
import {addOriginsToDiagnostics} from "./derive";
import {getDiagnosticsFromError} from "./errors";

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
