import {TokenValues} from "@romejs/parser-core";
import {CSSParserOptions, Tokens} from "./types";
import {createCSSParser} from "./parse";

export function tokenizeCSS(opts: CSSParserOptions): Array<TokenValues<Tokens>> {
	return createCSSParser(opts).tokenizeAll();
}
