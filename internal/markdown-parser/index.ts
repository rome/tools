import {ParserOptionsWithRequiredPath} from "@internal/parser-core";
import {createMarkdownParser} from "./parse";

export function parseMarkdown(opts: ParserOptionsWithRequiredPath) {
	return createMarkdownParser(opts).parse();
}

export function tokenizeMarkdown(opts: ParserOptionsWithRequiredPath) {
	return createMarkdownParser(opts).tokenizeAll();
}

export * from "./types";
export * from "./utils";
