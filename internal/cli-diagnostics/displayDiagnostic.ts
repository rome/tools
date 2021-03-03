import {
	Diagnostic,
	DiagnosticAdvice,
	diagnosticLocationToMarkupFilelink,
	formatCategoryDescription,
} from "@internal/diagnostics";
import {
	Markup,
	StaticMarkup,
	isEmptyMarkup,
	joinMarkup,
	markup,
} from "@internal/markup";
import {equalPositions} from "@internal/parser-core";
import {MixedPathSet, equalPaths} from "@internal/path";
import {DiagnosticsPrinterFlags} from "./types";

type BuildOptions = {
	diagnostic: Diagnostic;
	flags: DiagnosticsPrinterFlags;
	outdatedPaths: MixedPathSet;
	missingPaths: MixedPathSet;
};

export function buildDisplayDiagnostic(
	opts: BuildOptions,
): {
	header: StaticMarkup;
	advice: DiagnosticAdvice[];
} {
	return {
		header: buildDisplayHeader(opts),
		advice: buildCompleteDisplayAdvice(opts),
	};
}

function buildDisplayHeader(
	{diagnostic, missingPaths, outdatedPaths}: BuildOptions,
): StaticMarkup {
	const {description, location, label, tags = {}} = diagnostic;

	const headerParts: Markup[] = [];

	if (label !== undefined) {
		headerParts.push(markup`<emphasis>${label}</emphasis>`);
	}

	headerParts.push(diagnosticLocationToMarkupFilelink(location));
	headerParts.push(
		markup`<emphasis>${formatCategoryDescription(description)}</emphasis>`,
	);

	if (tags.internal) {
		headerParts.push(markup`<inverse><error> INTERNAL </error></inverse>`);
	}
	if (tags.fixable) {
		headerParts.push(markup`<inverse> FIXABLE </inverse>`);
	}
	if (outdatedPaths.size > 0) {
		headerParts.push(markup`<inverse><warn> OUTDATED </warn></inverse>`);
	}
	if (missingPaths.has(location.path)) {
		headerParts.push(markup`<inverse><warn> MISSING </warn></inverse>`);
	}
	if (tags.fatal) {
		headerParts.push(markup`<inverse><error> FATAL </error></inverse>`);
	}

	return joinMarkup(headerParts, markup` `);
}

function buildCompleteDisplayAdvice(
	{diagnostic, missingPaths, outdatedPaths, flags}: BuildOptions,
): DiagnosticAdvice[] {
	const {location} = diagnostic;
	const {start, end, path} = location;

	const mainAdvice = filterMainAdvice(diagnostic);

	// Determine if we should skip showing the frame at the top of the diagnostic output
	// We check if there are any frame advice entries that match us exactly, this is
	// useful for simplifying stacktraces
	let skipFrame = false;
	if (start !== undefined && end !== undefined) {
		adviceLoop: for (const item of mainAdvice) {
			if (
				item.type === "frame" &&
				equalPaths(item.location.path, path) &&
				equalPositions(item.location.start, start) &&
				equalPositions(item.location.end, end)
			) {
				skipFrame = true;
				break;
			}

			if (item.type === "stacktrace") {
				for (const frame of item.frames) {
					if (equalPaths(frame.path, path) && equalPositions(frame, start)) {
						skipFrame = true;
						break adviceLoop;
					}
				}
			}
		}
	}
	if (missingPaths.has(path)) {
		skipFrame = true;
	}

	return [
		...buildMessageAdvice(diagnostic, skipFrame),
		...mainAdvice,
		...buildTagsAdvice(diagnostic),
		...buildOutdatedAdvice(diagnostic, missingPaths, outdatedPaths),
		...(flags.verboseDiagnostics ? buildVerboseAdvice(diagnostic) : []),
	];
}

function buildMessageAdvice(
	diagnostic: Diagnostic,
	skipFrame: boolean,
): DiagnosticAdvice[] {
	const advice: DiagnosticAdvice[] = [];

	const {description, location} = diagnostic;
	const {message} = description;

	if (isEmptyMarkup(message)) {
		advice.push({
			type: "log",
			category: "none",
			text: markup`<dim>no diagnostic message provided</dim>`,
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
				location,
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

	return advice;
}

function filterMainAdvice(diagnostic: Diagnostic): DiagnosticAdvice[] {
	let advice: DiagnosticAdvice[] = [...diagnostic.description.advice];
	const {path, start} = diagnostic.location;

	// Remove stacktrace from beginning if it contains only one frame that matches the root diagnostic location
	const firstAdvice = advice[0];
	if (firstAdvice?.type === "stacktrace" && firstAdvice.frames.length === 1) {
		const frame = firstAdvice.frames[0];
		if (equalPaths(frame.path, path) && equalPositions(frame, start)) {
			advice.shift();
		}
	}

	return advice;
}

function buildTagsAdvice(diagnostic: Diagnostic): DiagnosticAdvice[] {
	const advice: DiagnosticAdvice[] = [];
	const {tags} = diagnostic;
	if (tags === undefined) {
		return advice;
	}

	if (tags.fatal) {
		advice.push({
			type: "log",
			category: "warn",
			text: markup`Rome exited as this error could not be handled and resulted in a fatal error. Please report if necessary.`,
		});
	} else if (tags.internal) {
		advice.push({
			type: "log",
			category: "warn",
			text: markup`This diagnostic was derived from an internal Rome error. Potential bug, please report if necessary.`,
		});
	}

	return advice;
}

function buildOutdatedAdvice(
	diagnostic: Diagnostic,
	missingPaths: MixedPathSet,
	outdatedPaths: MixedPathSet,
): DiagnosticAdvice[] {
	const {path} = diagnostic.location;
	const advice: DiagnosticAdvice[] = [];

	if (missingPaths.size === 1 && missingPaths.has(path)) {
		advice.push({
			type: "log",
			category: "warn",
			text: markup`This diagnostic refers to a file that no longer exists`,
		});
	} else if (outdatedPaths.size > 0) {
		advice.push({
			type: "log",
			category: "warn",
			text: markup`This diagnostic relies on the following files that have been deleted`,
		});

		advice.push({
			type: "list",
			list: Array.from(missingPaths),
		});
	}

	if (outdatedPaths.size === 1 && outdatedPaths.has(path)) {
		advice.push({
			type: "log",
			category: "warn",
			text: markup`This file has been changed since the diagnostic was produced and may be out of date`,
		});
	} else if (outdatedPaths.size > 0) {
		advice.push({
			type: "log",
			category: "warn",
			text: markup`This diagnostic may be out of date as it relies on the following files that have been changed`,
		});

		advice.push({
			type: "list",
			list: Array.from(outdatedPaths),
		});
	}

	return advice;
}

function buildVerboseAdvice(diagnostic: Diagnostic): DiagnosticAdvice[] {
	let verboseAdvice: DiagnosticAdvice[] = [
		...(diagnostic.description.verboseAdvice ?? []),
	];

	const {origins} = diagnostic;
	if (origins !== undefined && origins.length > 0) {
		verboseAdvice.push({
			type: "log",
			category: "info",
			text: markup`Why are you seeing this diagnostic?`,
		});

		verboseAdvice.push({
			type: "list",
			ordered: true,
			list: origins.map((origin) => {
				let res = markup`<emphasis>${origin.category}</emphasis>`;
				if (origin.message !== undefined) {
					res = markup`${res}: ${origin.message}`;
				}
				return res;
			}),
		});
	}

	// Display advice as a group to separate it from regular advice
	if (verboseAdvice.length > 0) {
		verboseAdvice = [
			{
				type: "group",
				title: markup`Verbose advice`,
				advice: verboseAdvice,
			},
		];
	}

	return verboseAdvice;
}
