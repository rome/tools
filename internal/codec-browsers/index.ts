import {Browser} from "@internal/browser-features/Browser";
import {ResolveOptions, resolveTargets} from "./resolve";
import {parseBrowserQuery} from "./parse";
import {ParserOptions} from "@internal/parser-core";

export function resolveBrowsers(
	queries: string | string[] | ParserOptions,
	resolveOptions?: ResolveOptions,
): Browser[] {
	let opt: ParserOptions;

	if (typeof queries === "string" || Array.isArray(queries)) {
		opt = {
			input: Array.isArray(queries) ? queries.join(", ") : queries,
		};
	} else {
		opt = queries;
	}

	return Array.from(resolveTargets(parseBrowserQuery(opt), resolveOptions));
}
