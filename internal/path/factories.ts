import AbsoluteFilePath from "./classes/AbsoluteFilePath";
import RelativePath from "./classes/RelativePath";
import UIDPath from "./classes/UIDPath";
import URLPath from "./classes/URLPath";
import DataURIPath from "./classes/DataURIPath";
import {
	PathTypeHint,
	normalizeRelativeSegments,
	parsePath,
	parseRelativePathSegments,
} from "./parse";
import {FilePath, ParsedPath, Path} from "./types";

export function createPathFromParsed(parsed: ParsedPath): Path {
	switch (parsed.type) {
		case "absolute-windows-drive":
		case "absolute-windows-unc":
		case "absolute-unix":
			return new AbsoluteFilePath(parsed);

		case "data":
			return new DataURIPath(parsed);

		case "url":
			return new URLPath(parsed);

		case "uid":
			return new UIDPath(parsed);

		case "relative":
			return new RelativePath(parsed);
	}
}

export function createRelativePathFromSegments(segments: string[]): RelativePath {
	return new RelativePath(parseRelativePathSegments(segments));
}

export function createRelativePath(str: string): RelativePath {
	return createPath(str, "relative").assertRelative();
}

export function createURLPath(str: string): URLPath {
	return createPath(str, "url").assertURL();
}

export function createDataURIPath(str: string): DataURIPath {
	return createPath(str, "url").assertDataURI();
}

export function createAbsoluteFilePath(str: string): AbsoluteFilePath {
	return createPath(str, "absolute").assertAbsolute();
}

export function createUIDPath(str: string): UIDPath {
	return createPath(str, "uid").assertUID();
}

export function createUIDPathFromSegments(relativeSegments: string[]): UIDPath {
	return new UIDPath({
		type: "uid",
		...normalizeRelativeSegments(relativeSegments),
	});
}

export function createFilePath(str: string): FilePath {
	return createPath(str, "absolute").assertFilePath();
}

export function createPath(param: string, hint: PathTypeHint = "any"): Path {
	return createPathFromParsed(parsePath(param, hint));
}
