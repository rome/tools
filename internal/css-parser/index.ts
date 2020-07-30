import {TokenValues} from "@internal/parser-core";
import {CSSParserOptions, Tokens} from "./types";
import {createCSSParser} from "./parse";
import {CSSRoot} from "@internal/ast";

export function parseCSS(opts: CSSParserOptions): CSSRoot {
	return createCSSParser(opts).parse();
}

export function tokenizeCSS(opts: CSSParserOptions): Array<TokenValues<Tokens>> {
	return createCSSParser(opts).tokenizeAll();
}
