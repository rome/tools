/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Diagnostic,
	DiagnosticAdvice,
	DiagnosticDescription,
	DiagnosticOrigin,
	Diagnostics,
} from "./types";
import {escapeMarkup} from "@romefrontend/cli-layout";
import {
	ErrorFrames,
	StructuredError,
	getErrorStructure,
	getSourceLocationFromErrorFrame,
} from "@romefrontend/v8";
import {createBlessedDiagnosticMessage} from "./descriptions";
import DiagnosticsNormalizer from "./DiagnosticsNormalizer";
import {diagnosticLocationToMarkupFilelink} from "./helpers";
import {RequiredProps} from "@romefrontend/typescript-helpers";

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
	header: string;
} {
	const advice: DiagnosticAdvice = [];
	const {description, fixable, location} = diag;

	let header = diagnosticLocationToMarkupFilelink(location);

	if (diag.label !== undefined) {
		header = `<emphasis>${diag.label}</emphasis> ${header}`;

		if (description.category !== undefined) {
			header += ` <dim>${description.category}</dim>`;
		}
	} else {
		if (description.category !== undefined) {
			header += ` <emphasis>${description.category}</emphasis>`;
		}
	}

	if (fixable === true) {
		header += " <inverse> FIXABLE </inverse>";
	}

	if (opts.outdated === true) {
		header += " <inverse> OUTDATED </inverse>";
	}

	if (opts.includeHeaderInAdvice === true) {
		advice.push({
			type: "log",
			category: "none",
			text: header,
		});
	}

	const message = description.message.value;

	if (message === "") {
		advice.push({
			type: "log",
			category: "none",
			text: "<dim>no diagnostic message specified</dim>",
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

type DeriveErrorDiagnosticOpts = {
	description: RequiredProps<Partial<DiagnosticDescription>, "category">;
	label?: string;
	filename?: string;
	cleanFrames?: (frames: ErrorFrames) => ErrorFrames;
};

export function deriveDiagnosticFromErrorStructure(
	struct: StructuredError,
	opts: DeriveErrorDiagnosticOpts,
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
			message: createBlessedDiagnosticMessage(escapeMarkup(message)),
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
	opts: DeriveErrorDiagnosticOpts,
): Diagnostic {
	return deriveDiagnosticFromErrorStructure(getErrorStructure(error), opts);
}

export function getErrorStackAdvice(
	error: StructuredError,
	title?: string,
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

		advice.push({
			type: "log",
			category: "warn",
			text: "Raw stack trace is being displayed as we did not receive any frames",
		});

		advice.push({
			type: "list",
			list: cleanStack.split("\n").map((line) => escapeMarkup(line.trim())),
		});
	} else {
		const adviceFrames = frames.map((frame) => {
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
