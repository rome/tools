import {CSSParser, Tokens} from "@internal/css-parser/types";
import {AnyCSSValue, CSSAtRule, CSSRule, CSSSelector} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {parseSelectors} from "@internal/css-parser/parser/selectors";
import {descriptions} from "@internal/diagnostics";
import {parseKeyframe} from "@internal/css-parser/parser/keyframe";
import {parseDeclarationBlock} from "@internal/css-parser/parser/declaration";
import {parseComplexBlock} from "@internal/css-parser/parser/block";
import {parseComponentValue} from "@internal/css-parser/parser/value";
import {parseMediaList} from "@internal/css-parser/parser/media";

export function parseRules(
	parser: CSSParser,
	topLevel = false,
	endingTokenType?: keyof Tokens,
): Array<CSSAtRule | CSSRule> {
	const rules: Array<CSSAtRule | CSSRule> = [];
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
			const rule = parseRule(parser);
			rule && rules.push(rule);
			continue;
		}

		if (matchToken(parser, "AtKeyword")) {
			rules.push(parseAtRule(parser));
			continue;
		}

		const rule = parseRule(parser);
		if (rule !== undefined) {
			rules.push(rule);
		}
	}

	return rules;
}

function parseRule(parser: CSSParser): CSSRule | undefined {
	const start = parser.getPosition();
	let prelude: CSSSelector[] = [];
	while (!matchToken(parser, "EOF")) {
		if (matchToken(parser, "LeftCurlyBracket")) {
			return parser.finishNode(
				start,
				{
					type: "CSSRule",
					prelude,
					block: parseDeclarationBlock(parser),
				},
			);
		}
		prelude = parseSelectors(parser);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.UNEXPECTED_TOKEN,
		token: parser.getToken(),
	});
	return undefined;
}

export function parseAtRule(parser: CSSParser): CSSAtRule {
	const start = parser.getPosition();
	const previousToken = parser.getToken() as Tokens["AtKeyword"];
	const token = parser.expectToken("AtKeyword");
	const prelude: AnyCSSValue[] = [];
	const name = token.value;
	let block = undefined;
	while (true) {
		if (matchToken(parser, "Semi")) {
			break;
		}
		if (matchToken(parser, "EOF")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNTERMINATED_AT_RULE,
				token: parser.getToken(),
			});
			break;
		}
		if (previousToken.value === "media") {
			const value = parseMediaList(parser);
			if (value) {
				prelude.push(value);
			}
		}
		if (previousToken.value === "keyframes") {
			block = parseKeyframe(parser);
			break;
		}
		if (matchToken(parser, "LeftCurlyBracket")) {
			block = parseComplexBlock(parser);
			break;
		}
		const parsedValue = parseComponentValue(parser);
		if (parsedValue) {
			prelude.push(parsedValue);
		}
	}
	return parser.finishNode(
		start,
		{
			type: "CSSAtRule",
			name,
			prelude,
			block,
		},
	);
}
