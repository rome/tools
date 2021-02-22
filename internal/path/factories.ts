import AbsoluteFilePath from "./classes/AbsoluteFilePath";
import RelativePath from "./classes/RelativePath";
import UIDPath from "./classes/UIDPath";
import URLPath from "./classes/URLPath";
import {
	AnyParsedPath,
	PathTypeHint,
	parsePathSegments,
	splitPathSegments,
	ParsePathSegmentsOverrides,
} from "./parse";
import {AnyFilePath, AnyPath} from "./types";

function createPathFromParsed(parsed: AnyParsedPath): AnyPath {
	switch (parsed.type) {
		case "windows-drive":
		case "windows-unc":
		case "unix":
			return new AbsoluteFilePath(parsed);

		case "url":
			return new URLPath(parsed);

		case "uid":
			return new UIDPath(parsed);

		case "relative":
			return new RelativePath(parsed);
	}
}

export function createPathFromSegments(
	segments: string[],
	hint: PathTypeHint,
	overrides?: ParsePathSegmentsOverrides,
): AnyPath {
	const parsed = parsePathSegments(segments, hint, overrides);
	return createPathFromParsed(parsed);
}

export function createRelativePath(filename: string): RelativePath {
	return createAnyPath(filename, "relative").assertRelative();
}

export function createURLPath(filename: string): URLPath {
	return createAnyPath(filename, "any").assertURL();
}

export function createAbsoluteFilePath(filename: string): AbsoluteFilePath {
	return createAnyPath(filename, "absolute").assertAbsolute();
}

export function createUIDPath(filename: string): UIDPath {
	return createAnyPath(filename, "uid").assertUID();
}

export function createFilePath(filename: string): AnyFilePath {
	return createAnyPath(filename, "absolute").assertFilePath();
}

export function createAnyPath(
	param: string,
	hint: PathTypeHint = "any",
): AnyPath {
	const segments = splitPathSegments(param);
	const parsed = parsePathSegments(segments, hint);
	return createPathFromParsed(parsed);
}