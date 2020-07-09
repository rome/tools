import {ParserOptionsWithRequiredPath} from "@romefrontend/parser-core";
import {createMarkdownParser} from "@romefrontend/markdown-parser/parse";


export function parseMarkdown(opts: ParserOptionsWithRequiredPath) {
    return createMarkdownParser(opts).parse();
}

export function tokenizeMarkdown(opts: ParserOptionsWithRequiredPath) {
    return createMarkdownParser(opts).tokenizeAll();
}
