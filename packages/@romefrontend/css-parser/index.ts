import {TokenValues} from "@romefrontend/parser-core";
import {CSSParserOptions, Tokens} from "./types";
import {createCSSParser} from "./parse";
import {CSSStylesheet} from "@romefrontend/ast";

export function parseCSS(opts: CSSParserOptions): CSSStylesheet {
	return createCSSParser(opts).parse();
}

export function tokenizeCSS(opts: CSSParserOptions): Array<TokenValues<Tokens>> {
	return createCSSParser(opts).tokenizeAll();
}
