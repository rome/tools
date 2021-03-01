/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticAdviceAction,
	DiagnosticAdviceStacktrace,
	DiagnosticDescription,
	DiagnosticOrigin,
	DiagnosticTags,
	Diagnostics,
} from "./types";
import {
	ErrorFrames,
	StructuredError,
	getDiagnosticLocationFromErrorFrame,
	getErrorStructure,
} from "@internal/errors";
import DiagnosticsNormalizer from "./DiagnosticsNormalizer";
import {appendAdviceToDiagnostic, prependAdviceToDiagnostic} from "./helpers";
import {StaticMarkup, isEmptyMarkup, markup} from "@internal/markup";
import {
	DiagnosticsError,
	createSingleDiagnosticError,
	getDiagnosticsFromError,
	isUserDiagnostic,
	isUserDiagnosticError,
} from "./error-wrappers";
import {MixedPathSet, Path, UNKNOWN_PATH, equalPaths} from "@internal/path";
import {RequiredProps} from "@internal/typescript-helpers";

export function derivePositionlessKeyFromDiagnostic(diag: Diagnostic): string {
	const normalizer = new DiagnosticsNormalizer(
		{},
		{
			stripPositions: true,
		},
	);

	return JSON.stringify(normalizer.normalizeDiagnostic(diag));
}

export function getActionAdviceFromDiagnostic(
	diag: Diagnostic,
): DiagnosticAdviceAction[] {
	return [
		...filterActionAdvice(diag.description.advice),
		...filterActionAdvice(diag.description.verboseAdvice),
	];
}

function filterActionAdvice(
	advice: undefined | DiagnosticAdvice,
): DiagnosticAdviceAction[] {
	if (advice === undefined) {
		return [];
	} else {
		const actions: DiagnosticAdviceAction[] = [];
		for (const item of advice) {
			if (item.type === "action") {
				actions.push(item);
			}
		}
		return actions;
	}
}

export type DeriveErrorDiagnosticOptions = {
	description: RequiredProps<Partial<DiagnosticDescription>, "category">;
	label?: StaticMarkup;
	// Passing in `internal: true` is redundant
	tags?: Omit<DiagnosticTags, "internal"> & {
		internal?: false;
	};
	path?: Path;
	cleanRelativeError?: Error;
	cleanFrames?: (frames: ErrorFrames) => ErrorFrames;
	stackAdviceOptions?: DeriveErrorStackAdviceOptions;
};

export function decorateErrorWithDiagnostics(
	error: Error,
	opts: DeriveErrorDiagnosticOptions,
): Error {
	if (isUserDiagnosticError(error)) {
		return error;
	}

	let diagnostics = getDiagnosticsFromError(error);
	if (diagnostics !== undefined) {
		// This is a diagnostics error so add on our intended advice (if any)
		let addAdvice: DiagnosticAdvice = [...(opts.description.advice || [])];
		if (opts.description.message !== undefined) {
			addAdvice.unshift({
				type: "log",
				category: "info",
				text: opts.description.message,
			});
		}

		return new DiagnosticsError(
			error.message,
			diagnostics.map((diag) => {
				// Could be mixed user and runtime diagnostics
				if (isUserDiagnostic(diag)) {
					return diag;
				}

				diag = appendAdviceToDiagnostic(diag, addAdvice);

				if (opts.tags !== undefined) {
					diag = {
						...diag,
						tags: {
							...diag.tags,
							...opts.tags,
						},
					};
				}

				if (opts.label !== undefined && diag.label === undefined) {
					diag = {
						...diag,
						label: opts.label,
					};
				}

				return diag;
			}),
		);
	}

	let diag = deriveDiagnosticFromError(error, opts);

	// If we specified a custom message then push on the original
	if (opts.description.message !== undefined) {
		diag = prependAdviceToDiagnostic(
			diag,
			[
				{
					type: "log",
					category: "none",
					text: markup`${getErrorStructure(error).message}`,
				},
			],
		);
	}

	return createSingleDiagnosticError(diag);
}

export function deriveDiagnosticFromErrorStructure(
	struct: Partial<StructuredError>,
	opts: DeriveErrorDiagnosticOptions,
): Diagnostic {
	let targetPath: Path = opts.path ?? UNKNOWN_PATH;
	let targetLoc = undefined;

	let {frames = [], message = "Unknown error"} = struct;

	const {cleanFrames, cleanRelativeError} = opts;
	if (cleanFrames !== undefined) {
		frames = cleanFrames(frames);
	}
	if (cleanRelativeError !== undefined) {
		// We consider the last two frames as possible candidates to allow for easy construction
		const refFrames = getErrorStructure(cleanRelativeError).frames.slice(0, 2);

		frameLoop: for (let i = 0; i < frames.length; i++) {
			const frame = frames[i];
			for (const refFrame of refFrames) {
				if (
					equalPaths(frame.path, refFrame.path) &&
					frame.lineNumber === refFrame.lineNumber
				) {
					frames = frames.slice(0, i - 1);
					break frameLoop;
				}
			}
		}
	}

	// Point the target to the closest frame with a filename
	for (const frame of frames) {
		if (frame.path === undefined) {
			continue;
		}

		targetPath = frame.path;
		targetLoc = getDiagnosticLocationFromErrorFrame(frame);
		break;
	}

	const advice = getErrorStackAdvice(
		{
			...struct,
			frames,
		},
		opts.stackAdviceOptions,
	);

	return {
		description: {
			message: markup`${message}`,
			...opts.description,
			advice: [...advice, ...(opts.description?.advice || [])],
		},
		location: {
			path: targetPath,
			start: targetLoc === undefined ? undefined : targetLoc.start,
			end: targetLoc === undefined ? undefined : targetLoc.end,
		},
		label: opts.label,
		tags: {
			internal: true,
			...opts.tags,
		},
	};
}

export function deriveDiagnosticFromError(
	error: unknown,
	opts: DeriveErrorDiagnosticOptions,
): Diagnostic {
	return deriveDiagnosticFromErrorStructure(getErrorStructure(error), opts);
}

export type DeriveErrorStackAdviceOptions = {
	title?: StaticMarkup;
	importantPaths?: MixedPathSet;
};

export function getErrorStackAdvice(
	error: Partial<StructuredError>,
	{title, importantPaths}: DeriveErrorStackAdviceOptions = {},
): DiagnosticAdvice {
	const advice: DiagnosticAdvice = [];
	const {frames = [], stack} = error;

	if (frames.length === 0 && stack !== undefined) {
		// Just in case we didn't get the frames for some reason
		if (title !== undefined) {
			advice.push({
				type: "log",
				category: "info",
				text: title,
			});
		}

		// Remove the `message` from the `stack`
		let cleanStack = stack;
		let removeMessage = `${error.name}: ${error.message}`;
		if (cleanStack.startsWith(removeMessage)) {
			cleanStack = cleanStack.slice(removeMessage.length);
		}
		cleanStack = cleanStack.trim();

		const cleanStackList = cleanStack.replace(/\n+/g, "\n").split("\n").map((
			line,
		) => markup`${line.trim()}`);

		if (cleanStackList.length === 1 && isEmptyMarkup(cleanStackList[0])) {
			advice.push({
				type: "log",
				category: "warn",
				text: markup`We did not receive any frames and no raw stack trace found`,
			});
		} else {
			advice.push({
				type: "log",
				category: "warn",
				text: markup`Raw stack trace is being displayed as we did not receive any frames`,
			});

			advice.push({
				type: "list",
				list: cleanStackList,
			});
		}
	} else {
		const adviceFrames: DiagnosticAdviceStacktrace["frames"] = frames.map((
			frame,
		) => {
			const {
				typeName,
				functionName,
				methodName,
				path,
				lineNumber,
				columnNumber,
				isEval,
				isNative,
				isConstructor,
				isAsync,
			} = frame;

			const prefixes = [];
			if (isAsync) {
				prefixes.push("await");
			}
			if (isEval) {
				prefixes.push("eval");
			}
			if (isConstructor) {
				prefixes.push("new");
			}
			const prefix = prefixes.length === 0 ? undefined : prefixes.join(" ");

			let object = typeName;
			let property = "<anonymous>";
			if (functionName !== undefined) {
				property = functionName;
			}
			if (methodName !== undefined) {
				property = methodName;
			}

			let suffix;
			if (isNative) {
				suffix = "native";
			}

			return {
				language: "unknown",
				suffix,
				prefix,
				object,
				property,
				path,
				line: lineNumber,
				column: columnNumber,
			};
		});

		advice.push({
			type: "stacktrace",
			title,
			frames: adviceFrames,
			importantPaths,
		});
	}

	return advice;
}

export function addOriginsToDiagnostics(
	origins: DiagnosticOrigin[],
	diagnostics: Diagnostics,
): Diagnostics {
	if (origins.length === 0) {
		return diagnostics;
	}

	return diagnostics.map((diag) => {
		return addOriginsToDiagnostic(origins, diag);
	});
}

export function addOriginsToDiagnostic(
	origins: DiagnosticOrigin[],
	diag: Diagnostic,
): Diagnostic {
	if (origins.length === 0) {
		return diag;
	}

	const newOrigins =
		diag.origins === undefined ? origins : [...origins, ...diag.origins];
	return {
		...diag,
		origins: newOrigins,
	};
}
