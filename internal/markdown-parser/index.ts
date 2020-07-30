import {ParserOptionsWithRequiredPath} from "@internal/parser-core";
import {createMarkdownParser} from "@internal/markdown-parser/parse";

export function parseMarkdown(opts: ParserOptionsWithRequiredPath) {
	return createMarkdownParser(opts).parse();
}

export function tokenizeMarkdown(opts: ParserOptionsWithRequiredPath) {
	return createMarkdownParser(opts).tokenizeAll();
}
