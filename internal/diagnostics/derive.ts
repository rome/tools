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
	Diagnostics,
} from "./types";
import {
	ErrorFrames,
	StructuredError,
	getErrorStructure,
	getSourceLocationFromErrorFrame,
} from "@internal/v8";
import DiagnosticsNormalizer from "./DiagnosticsNormalizer";
import {diagnosticLocationToMarkupFilelink} from "./helpers";
import {RequiredProps} from "@internal/typescript-helpers";
import {Markup, isEmptyMarkup, markup} from "@internal/markup";

function normalizeArray<T>(val: undefined | Array<T>): Array<T> {
	if (Array.isArray(val)) {
		return val;
	} else {
		return [];
	}
}

export function mergeDiagnostics(
	rootDiag: Diagnostic,
	...diags: Array<Diagnostic>
): Diagnostic {
	let mergedAdvice: DiagnosticAdvice = [
		...normalizeArray(rootDiag.description.advice),
	];

	for (const diag of diags) {
		mergedAdvice = [
			...mergedAdvice,
			...deriveRootAdviceFromDiagnostic(diag).advice,
			...normalizeArray(diag.description.advice),
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
	const normalizer = new DiagnosticsNormalizer({
		stripPositions: true,
	});

	return JSON.stringify(normalizer.normalizeDiagnostic(diag));
}

export function deriveRootAdviceFromDiagnostic(
	diag: Diagnostic,
	opts: {
		skipFrame: boolean;
		includeHeaderInAdvice: boolean;
		outdated: boolean;
	} = {
		skipFrame: false,
		includeHeaderInAdvice: true,
		outdated: false,
	},
): {
	advice: DiagnosticAdvice;
	header: Markup;
} {
	const advice: DiagnosticAdvice = [];
	const {description, fixable, location} = diag;

	let header = diagnosticLocationToMarkupFilelink(location);

	if (diag.label !== undefined) {
		header = markup`<emphasis>${diag.label}</emphasis> ${header}`;

		if (description.category !== undefined) {
			header = markup`${header} <dim>${description.category}</dim>`;
		}
	} else {
		if (description.category !== undefined) {
			header = markup`${header} <emphasis>${description.category}</emphasis>`;
		}
	}

	if (fixable === true) {
		header = markup`${header} <inverse> FIXABLE </inverse>`;
	}

	if (opts.outdated === true) {
		header = markup`${header} <inverse> OUTDATED </inverse>`;
	}

	if (opts.includeHeaderInAdvice === true) {
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

	if (opts.skipFrame === false) {
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

	return {header, advice};
}

type DeriveErrorDiagnosticOptions = {
	description: RequiredProps<Partial<DiagnosticDescription>, "category">;
	label?: Markup;
	filename?: string;
	cleanFrames?: (frames: ErrorFrames) => ErrorFrames;
};

export function deriveDiagnosticFromErrorStructure(
	struct: StructuredError,
	opts: DeriveErrorDiagnosticOptions,
): Diagnostic {
	const {filename} = opts;

	let targetFilename: undefined | string = filename;
	let targetCode = undefined;
	let targetLoc = undefined;

	let {frames, message = "Unknown error"} = struct;

	const {cleanFrames} = opts;
	if (cleanFrames !== undefined) {
		frames = cleanFrames(frames);
	}

	// Point the target to the closest frame with a filename
	for (const frame of frames) {
		if (frame.filename === undefined) {
			continue;
		}

		targetFilename = frame.filename;
		targetLoc = getSourceLocationFromErrorFrame(frame);
		break;
	}

	const advice = getErrorStackAdvice({
		...struct,
		frames,
	});

	return {
		description: {
			message: markup`${message}`,
			...opts.description,
			advice: [...advice, ...(opts.description?.advice || [])],
		},
		location: {
			filename: targetFilename,
			start: targetLoc === undefined ? undefined : targetLoc.start,
			end: targetLoc === undefined ? undefined : targetLoc.end,
			sourceText: targetCode,
		},
		label: opts.label,
	};
}

export function deriveDiagnosticFromError(
	error: unknown,
	opts: DeriveErrorDiagnosticOptions,
): Diagnostic {
	return deriveDiagnosticFromErrorStructure(getErrorStructure(error), opts);
}

export function getErrorStackAdvice(
	error: StructuredError,
	title?: Markup,
): DiagnosticAdvice {
	const advice: DiagnosticAdvice = [];
	const {frames, stack} = error;

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

		const cleanStackList = cleanStack.split("\n").map((line) =>
			markup`${line.trim()}`
		);

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
				filename,
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
				filename,
				line: lineNumber,
				column: columnNumber,
			};
		});

		advice.push({
			type: "stacktrace",
			title,
			frames: adviceFrames,
		});
	}

	return advice;
}

export function addOriginsToDiagnostics(
	origins: Array<DiagnosticOrigin>,
	diagnostics: Diagnostics,
): Diagnostics {
	return diagnostics.map((diag) => {
		return addOriginsToDiagnostic(origins, diag);
	});
}

export function addOriginsToDiagnostic(
	origins: Array<DiagnosticOrigin>,
	diag: Diagnostic,
): Diagnostic {
	const newOrigins =
		diag.origins === undefined ? origins : [...origins, ...diag.origins];
	return {
		...diag,
		origins: newOrigins,
	};
}
