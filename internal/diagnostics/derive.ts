/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
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
import {
	appendAdviceToDiagnostic,
	diagnosticLocationToMarkupFilelink,
	formatCategoryDescription,
	prependAdviceToDiagnostic,
} from "./helpers";
import {StaticMarkup, isEmptyMarkup, markup} from "@internal/markup";
import {
	DiagnosticsError,
	createSingleDiagnosticError,
	getDiagnosticsFromError,
	isUserDiagnosticError,
	isUserDiagnostic,
} from "./error-wrappers";
import {AnyPath, MixedPathSet, UNKNOWN_PATH, equalPaths} from "@internal/path";
import { RequiredProps } from "@internal/typescript-helpers";

function normalizeArray<T>(val: undefined | (T[])): T[] {
	if (Array.isArray(val)) {
		return val;
	} else {
		return [];
	}
}

export function mergeDiagnostics(
	rootDiag: Diagnostic,
	...diags: Diagnostic[]
): Diagnostic {
	let mergedAdvice: DiagnosticAdvice = [
		...normalizeArray(rootDiag.description.advice),
	];

	for (const diag of diags) {
		const derived = deriveRootAdviceFromDiagnostic(diag);
		mergedAdvice = [
			...mergedAdvice,
			...derived.advice,
			...normalizeArray(diag.description.advice),
			...derived.lastAdvice,
		];
	}

	return {
		...rootDiag,
		description: {
			...rootDiag.description,
			advice: mergedAdvice,
		},
	};
}

export function derivePositionlessKeyFromDiagnostic(diag: Diagnostic): string {
	const normalizer = new DiagnosticsNormalizer(
		{},
		{
			stripPositions: true,
		},
	);

	return JSON.stringify(normalizer.normalizeDiagnostic(diag));
}

export function deriveRootAdviceFromDiagnostic(
	diag: Diagnostic,
	{
		skipFrame = false,
		includeHeaderInAdvice = true,
		isOutdated = false,
		isMissing = false,
	}: {
		skipFrame?: boolean;
		includeHeaderInAdvice?: boolean;
		isOutdated?: boolean;
		isMissing?: boolean;
	} = {},
): {
	advice: DiagnosticAdvice;
	lastAdvice: DiagnosticAdvice;
	header: StaticMarkup;
} {
	const advice: DiagnosticAdvice = [];
	const {description, tags = {}, location} = diag;

	let header = diagnosticLocationToMarkupFilelink(location);

	if (diag.label !== undefined) {
		header = markup`<emphasis>${diag.label}</emphasis> ${header}`;
	}

	header = markup`${header} <emphasis>${formatCategoryDescription(description)}</emphasis>`;

	if (tags.internal) {
		header = markup`${header} <inverse><error> INTERNAL </error></inverse>`;
	}

	if (tags.fixable) {
		header = markup`${header} <inverse> FIXABLE </inverse>`;
	}

	if (isOutdated) {
		header = markup`${header} <inverse><warn> OUTDATED </warn></inverse>`;
	}

	if (isMissing) {
		header = markup`${header} <inverse><warn> MISSING </warn></inverse>`;
	}

	if (tags.fatal) {
		header = markup`${header} <inverse><error> FATAL </error></inverse>`;
	}

	if (includeHeaderInAdvice) {
		advice.push({
			type: "log",
			category: "none",
			text: header,
		});
	}

	const message = description.message;

	if (isEmptyMarkup(message)) {
		advice.push({
			type: "log",
			category: "none",
			text: markup`<dim>no diagnostic message specified</dim>`,
		});
	} else {
		advice.push({
			type: "log",
			category: "error",
			text: message,
		});
	}

	if (!skipFrame) {
		if (location.start !== undefined && location.end !== undefined) {
			advice.push({
				type: "frame",
				location: diag.location,
			});
		} else if (location.marker !== undefined) {
			// If we have no start/end, but we do have a marker then output is a log error
			advice.push({
				type: "log",
				category: "error",
				text: location.marker,
			});
		}
	}

	const lastAdvice: DiagnosticAdvice = [];

	if (tags.fatal) {
		lastAdvice.push({
			type: "log",
			category: "warn",
			text: markup`Rome exited as this error could not be handled and resulted in a fatal error. Please report if necessary.`,
		});
	} else if (tags.internal) {
		lastAdvice.push({
			type: "log",
			category: "warn",
			text: markup`This diagnostic was derived from an internal Rome error. Potential bug, please report if necessary.`,
		});
	}

	return {header, advice, lastAdvice};
}

export type DeriveErrorDiagnosticOptions = {
	description: RequiredProps<Partial<DiagnosticDescription>, "category">;
	label?: StaticMarkup;
	// Passing in `internal: true` is redundant
	tags?: Omit<DiagnosticTags, "internal"> & {
		internal?: false;
	};
	path?: AnyPath;
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
		let addAdvice: DiagnosticAdvice = [
			...(opts.description.advice || [])
		];
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
	let targetPath: AnyPath = opts.path ?? UNKNOWN_PATH;
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
