import {CSSParser, Tokens} from "@internal/css-parser/types";
import {CSSAtImport, CSSImport} from "@internal/ast";
import {parseAtImport} from "@internal/css-parser/parser/at-import";
import {parseImportRule} from "@internal/css-parser/parser/import-rule";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";

export function parseImport(
	parser: CSSParser,
	topLevel = false,
	endingTokenType?: keyof Tokens,
): Array<CSSImport | CSSAtImport> {
	const rules: Array<CSSAtImport | CSSImport> = [];
	while (!matchToken(parser, "EOF")) {
		if (endingTokenType && matchToken(parser, endingTokenType)) {
			nextToken(parser);
			break;
		}

		if (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
			continue;
		}

		if (matchToken(parser, "CDO") || matchToken(parser, "CDC")) {
			if (topLevel) {
				nextToken(parser);
				continue;
			}
			const rule = parseImportRule(parser);
			rule && rules.push(rule);
			continue;
		}

		if (matchToken(parser, "AtKeyword")) {
			rules.push(parseAtImport({parser}));
			continue;
		}

		const rule = parseImportRule(parser);
		if (rule !== undefined) {
			rules.push(rule);
		}
	}

	return rules;
}
