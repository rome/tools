import {Browser} from "@internal/browser-features/Browser";
import {resolveTargets} from "@internal/codec-browsers/resolve";
import {parseBrowserQuery} from "@internal/codec-browsers/parse";
import {ParserOptions} from "@internal/parser-core";

export function resolveBrowsers(
	queries: string | string[] | ParserOptions,
): Browser[] {
	let opt: ParserOptions;

	if (typeof queries === "string" || Array.isArray(queries)) {
		opt = {
			input: Array.isArray(queries) ? queries.join(", ") : queries,
		};
	} else {
		opt = queries;
	}

	return Array.from(resolveTargets(parseBrowserQuery(opt)));
}
