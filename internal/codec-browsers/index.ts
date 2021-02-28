import {Browser} from "@internal/browser-features/Browser";
import {resolveTargets} from "@internal/codec-browsers/resolve";
import {parseBrowserQuery} from "@internal/codec-browsers/parse";
import {ParserOptions} from "@internal/parser-core";

export function resolveBrowsers(
	queries: string | string[] | ParserOptions,
): Browser[] {
	const opt =
		typeof queries === "object"
			? queries as ParserOptions
			: {
					input: Array.isArray(queries) ? queries.join(", ") : queries,
				};

	return Array.from(resolveTargets(parseBrowserQuery(opt)));
}
