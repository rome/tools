import {CSSParser} from "../index";
import {
	CSSClassSelector,
	CSSCombinator,
	CSSIdSelector,
	CSSPseudoClassSelector,
	CSSPseudoElementSelector,
	CSSSelector,
	CSSTypeSelector,
	Combinator,
} from "@internal/ast";
import {AnyCSSPattern} from "@internal/ast/css/unions";
import {Tokens} from "../types";
import {descriptions} from "@internal/diagnostics";

function parseTypeSelector(parser: CSSParser): CSSTypeSelector {
	const start = parser.getPosition();
	const token = parser.expectToken("Ident");
	parser.eatToken("Ident");
	return parser.finishNode(
		start,
		{
			type: "CSSTypeSelector",
			value: token.value,
		},
	);
}

function parseIdSelector(parser: CSSParser): CSSIdSelector {
	const start = parser.getPosition();
	const token = parser.expectToken("Hash");
	parser.eatToken("Hash");
	return parser.finishNode(
		start,
		{
			type: "CSSIdSelector",
			value: token.value,
		},
	);
}

function parseClassSelector(parser: CSSParser): CSSClassSelector | undefined {
	const start = parser.getPosition();
	parser.eatToken("Delim");
	if (parser.matchToken("Ident")) {
		const token = parser.eatToken("Ident") as Tokens["Ident"];
		return parser.finishNode(
			start,
			{
				type: "CSSClassSelector",
				value: token.value,
			},
		);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.EXPECTED_IDENTIFIER,
	});
	return undefined;
}

function parsePseudoSelector(
	parser: CSSParser,
): CSSPseudoClassSelector | CSSPseudoElementSelector | undefined {
	const start = parser.getPosition();
	if (parser.eatToken("Colon")) {
		if (parser.matchToken("Ident")) {
			const token = parser.eatToken("Ident") as Tokens["Ident"];
			return parser.finishNode(
				start,
				{
					type: "CSSPseudoClassSelector",
					value: token.value,
				},
			)
		} else if (parser.matchToken("Colon")) {
			const token = parser.eatToken("Ident") as Tokens["Ident"];
			return parser.finishNode(
				start,
				{
					type: "CSSPseudoElementSelector",
					value: token.value,
				}
			)
		}
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.EXPECTED_IDENTIFIER,
	});
	return undefined;
}

function tryParseCombinator(parser: CSSParser): CSSCombinator | undefined {
	const start = parser.getPosition();
	if (parser.eatToken("Whitespace")) {
		const nextCombinator = tryParseCombinator(parser);
		if (nextCombinator) {
			// Whitespace preceding the combinator is not a combinator.
			return nextCombinator;
		}
		return parser.finishNode(
			start,
			{
				type: "CSSCombinator",
				combinator: "descendant",
			},
		);
	}

	if (parser.matchToken("Delim")) {
		let combinator: Combinator | undefined;
		const value = (parser.getToken() as Tokens["Delim"]).value;
		if (value === ">") {
			combinator = "child";
		} else if (value === "+") {
			combinator = "nextSibiling";
		} else if (value === "~") {
			combinator = "subsequentSibiling";
		}

		if (combinator) {
			parser.eatToken("Delim");
			parser.eatToken("Whitespace"); // Eats trailing Whitespace after combinator.
			return parser.finishNode(
				start,
				{
					type: "CSSCombinator",
					combinator,
				},
			);
		}
	}
	return undefined;
}

function tryParseSelector(parser: CSSParser) {
	if (parser.matchToken("Colon")) {
		return parsePseudoSelector(parser);
	}  else if (parser.matchToken("Hash")) {
		return parseIdSelector(parser);
	} else if (parser.matchToken("Ident")) {
		return parseTypeSelector(parser);
	} else if (parser.matchToken("Delim")) {
		const token = parser.eatToken("Delim") as Tokens["Delim"];
		if (token.value === ".") {
			return parseClassSelector(parser);
		}
	}
	return undefined;
}

function parseSelector(parser: CSSParser): CSSSelector {
	const start = parser.getPosition();
	const patterns: AnyCSSPattern[] = [];

	while (
		!parser.matchToken("Comma") &&
		!parser.matchToken("LeftCurlyBracket") &&
		!parser.matchToken("EOF")
	) {
		const last = patterns[patterns.length - 1];
		const combinator = tryParseCombinator(parser);
		const selector = tryParseSelector(parser);

		if (selector && combinator && last && last.type !== "CSSCombinator") {
			patterns.push(combinator);
		}

		if (selector) {
			patterns.push(selector);
		}

		if (!selector && !combinator) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNEXPECTED_TOKEN,
			});
			break;
		}
	}

	return parser.finishNode(
		start,
		{
			type: "CSSSelector",
			patterns,
		},
	);
}

export function parseSelectors(parser: CSSParser): CSSSelector[] {
	const selectors = [];
	while (!parser.matchToken("LeftCurlyBracket") && !parser.matchToken("EOF")) {
		const selector = parseSelector(parser);
		selectors.push(selector);
		parser.eatToken("Comma");
	}
	return selectors;
}
