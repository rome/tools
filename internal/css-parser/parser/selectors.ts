import {
	AttributeMatcher,
	AttributeModifier,
	CSSAttributeSelector,
	CSSClassSelector,
	CSSCombinator,
	CSSIdSelector,
	CSSIdentifier,
	CSSPseudoClassSelector,
	CSSPseudoElementSelector,
	CSSSelector,
	CSSString,
	CSSTypeSelector,
	Combinator,
} from "@internal/ast";
import {AnyCSSPattern} from "@internal/ast/css/unions";
import {CSSParser, Tokens} from "../types";
import {matchToken, readToken} from "../tokenizer";
import {descriptions} from "@internal/diagnostics";

function parseTypeSelector(parser: CSSParser): CSSTypeSelector {
	const start = parser.getPosition();
	const token = parser.expectToken("Ident");
	readToken(parser, "Ident");
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
	readToken(parser, "Hash");
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
	readToken(parser, "Delim");
	if (matchToken(parser, "Ident")) {
		const token = readToken(parser, "Ident") as Tokens["Ident"];
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
	if (readToken(parser, "Colon")) {
		if (matchToken(parser, "Ident")) {
			const token = readToken(parser, "Ident") as Tokens["Ident"];
			return parser.finishNode(
				start,
				{
					type: "CSSPseudoClassSelector",
					value: token.value,
				},
			);
		} else if (readToken(parser, "Colon")) {
			if (matchToken(parser, "Ident")) {
				const token = readToken(parser, "Ident") as Tokens["Ident"];
				return parser.finishNode(
					start,
					{
						type: "CSSPseudoElementSelector",
						value: token.value,
					},
				);
			}
		}
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.EXPECTED_IDENTIFIER,
	});
	return undefined;
}

function tryParseCombinator(parser: CSSParser): CSSCombinator | undefined {
	const start = parser.getPosition();
	if (readToken(parser, "Whitespace")) {
		const nextCombinator = tryParseCombinator(parser);
		if (nextCombinator) {
			readToken(parser, "Whitespace");
			// Whitespace preceding the combinator is not a combinator.
			return nextCombinator;
		}
		if (matchToken(parser, "LeftCurlyBracket") || matchToken(parser, "Comma")) {
			return undefined;
		}
		return parser.finishNode(
			start,
			{
				type: "CSSCombinator",
				combinator: "descendant",
			},
		);
	}

	if (matchToken(parser, "Delim")) {
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
			readToken(parser, "Delim");
			readToken(parser, "Whitespace"); // Eats trailing Whitespace after combinator.
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

function parseAttributeMatcher(parser: CSSParser): AttributeMatcher | undefined {
	let matcher: string | undefined;
	if (matchToken(parser, "Delim")) {
		const first = (parser.getToken() as Tokens["Delim"]).value;
		if (first === "=") {
			matcher = "=";
			parser.nextToken();
		} else if (["~", "^", "$", "*"].includes(first)) {
			matcher = first;
			parser.nextToken();

			const second = parser.getToken();
			if (second.type !== "Delim" || second.value !== "=") {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.UNKNOWN_ATTRIBUTE_MATCHER,
				});
				return undefined;
			}
			matcher += second.value;
			parser.nextToken();
		} else {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.UNKNOWN_ATTRIBUTE_MATCHER,
			});
			return undefined;
		}
	}
	return matcher as (AttributeMatcher | undefined);
}

function parseAttributeValue(
	parser: CSSParser,
): CSSIdentifier | CSSString | undefined {
	const start = parser.getPosition();
	let value: CSSIdentifier | CSSString | undefined;
	if (matchToken(parser, "Ident")) {
		const token = readToken(parser, "Ident") as Tokens["Ident"];
		value = parser.finishNode(
			start,
			{
				type: "CSSIdentifier",
				value: token.value,
			},
		);
	}
	if (matchToken(parser, "String")) {
		const token = readToken(parser, "String") as Tokens["String"];
		value = parser.finishNode(
			start,
			{
				type: "CSSString",
				value: token.value,
			},
		);
	}
	return value;
}

function parseAttributeSelector(
	parser: CSSParser,
): CSSAttributeSelector | undefined {
	if (!matchToken(parser, "LeftSquareBracket")) {
		return undefined;
	}

	const start = parser.getPosition();
	parser.nextToken();
	readToken(parser, "Whitespace");

	if (matchToken(parser, "Ident")) {
		const ident = parser.getToken() as Tokens["Ident"];
		const idStart = parser.getPosition();
		parser.nextToken();
		const attribute = parser.finishNode(
			idStart,
			{
				type: "CSSIdentifier",
				value: ident.value,
			},
		);

		readToken(parser, "Whitespace");

		const matcher = parseAttributeMatcher(parser);
		readToken(parser, "Whitespace");

		const value = matcher && parseAttributeValue(parser);
		readToken(parser, "Whitespace");

		let modifier: AttributeModifier | undefined;
		if (matchToken(parser, "Ident")) {
			const ident = parser.getToken() as Tokens["Ident"];
			if (ident.value === "i") {
				modifier = ident.value;
				parser.nextToken();
			} else {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.UNKNOWN_ATTRIBUTE_MODIFIER,
				});
				return undefined;
			}
		}

		readToken(parser, "Whitespace");

		if (!matchToken(parser, "RightSquareBracket")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.EXPECTED_CLOSING_ATTRIBUTE_SELECTOR,
			});
			return undefined;
		}
		parser.nextToken();
		return parser.finishNode(
			start,
			{
				type: "CSSAttributeSelector",
				value,
				attribute,
				matcher,
				modifier,
			},
		);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.EXPECTED_IDENTIFIER,
	});

	return undefined;
}

function tryParseSelector(parser: CSSParser) {
	if (matchToken(parser, "Colon")) {
		return parsePseudoSelector(parser);
	} else if (matchToken(parser, "Hash")) {
		return parseIdSelector(parser);
	} else if (matchToken(parser, "Ident")) {
		return parseTypeSelector(parser);
	} else if (matchToken(parser, "Delim")) {
		const token = readToken(parser, "Delim") as Tokens["Delim"];
		if (token.value === ".") {
			return parseClassSelector(parser);
		}
	} else if (matchToken(parser, "LeftSquareBracket")) {
		return parseAttributeSelector(parser);
	}
	return undefined;
}

function parseSelector(parser: CSSParser): CSSSelector {
	const start = parser.getPosition();
	const patterns: AnyCSSPattern[] = [];

	readToken(parser, "Comma");
	readToken(parser, "Whitespace");

	while (
		!matchToken(parser, "EOF") &&
		!matchToken(parser, "Comma") &&
		!matchToken(parser, "LeftCurlyBracket")
	) {
		const selectorStart = parser.getPosition();
		const last = patterns[patterns.length - 1];
		const combinator = tryParseCombinator(parser);
		const selector = tryParseSelector(parser);

		if (combinator) {
			if (!selector || !last || last.type === "CSSCombinator") {
				parser.unexpectedDiagnostic({
					description: descriptions.CSS_PARSER.EXPECTED_SELECTOR,
					start: last ? undefined : selectorStart,
				});
				break;
			} else {
				patterns.push(combinator);
			}
		}

		if (selector) {
			patterns.push(selector);
		}

		if (
			!selector &&
			!combinator &&
			!matchToken(parser, "Comma") &&
			!matchToken(parser, "LeftCurlyBracket")
		) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.EXPECTED_LBRACKET,
				start: selectorStart,
			});
			parser.nextToken();
			break;
		}
	}

	if (patterns.length <= 0) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.UNEXPECTED_EMPTY_SELECTOR,
			start,
		});
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
	while (!matchToken(parser, "LeftCurlyBracket") && !matchToken(parser, "EOF")) {
		const selector = parseSelector(parser);
		selectors.push(selector);
	}
	return selectors;
}
