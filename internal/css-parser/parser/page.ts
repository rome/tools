import {CSSParser} from "@internal/css-parser/types";
import {
	CSSAtPage,
	CSSPageSelector,
	CSSPseudoPage,
	CSSPseudoPageValue,
} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {parseDeclarationBlock} from "@internal/css-parser/parser/declaration";
import {descriptions} from "@internal/diagnostics";
import {
	ALLOWED_PAGE_MARGIN_PROPERTIES,
	ALLOWED_PAGE_PROPERTIES,
} from "@internal/css-parser/utils";

const VALID_PSEUDO_PAGE = new Set(["left", "right", "first", "blank"]);

const ALLOWED_AT_RULES = [
	"top-left-corner",
	"top-left",
	"top-center",
	"top-right",
	"top-right-corner",
	"bottom-left-corner",
	"bottom-left",
	"bottom-center",
	"bottom-right",
	"bottom-right-corner",
	"left-top",
	"left-middle",
	"left-bottom",
	"right-top",
	"right-middle",
	"right-bottom",
];

function parsePseudoPage(parser: CSSParser): CSSPseudoPage | undefined {
	const possibleIdent = parser.getToken();
	if (
		possibleIdent.type === "Ident" &&
		VALID_PSEUDO_PAGE.has(possibleIdent.value)
	) {
		const pos = parser.getPosition();
		nextToken(parser);
		return parser.finishNode(
			pos,
			{
				type: "CSSPseudoPage",
				value: possibleIdent.value as CSSPseudoPageValue,
			},
		);
	}
	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.AT_PAGE_INVALID_PSEUDO_PAGE,
		token: parser.getToken(),
	});
	nextToken(parser);
	return undefined;
}

function parsePageSelector(parser: CSSParser): CSSPageSelector | undefined {
	const token = parser.getToken();
	const start = parser.getPosition();
	let ident;
	let pseudo: CSSPseudoPage | undefined = undefined;
	if (token.type === "Ident") {
		ident = token.value;
		nextToken(parser);
		if (parser.getToken().type === "Colon") {
			nextToken(parser);
			pseudo = parsePseudoPage(parser);
		}
		return parser.finishNode(
			start,
			{
				type: "CSSPageSelector",
				ident,
				pseudo,
			},
		);
	} else if (token.type === "Colon") {
		nextToken(parser);
		pseudo = parsePseudoPage(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSPageSelector",
				ident,
				pseudo,
			},
		);
	}

	parser.unexpectedDiagnostic({
		description: descriptions.CSS_PARSER.AT_PAGE_MALFORMED,
		token: parser.getToken(),
	});
	nextToken(parser);
	return undefined;
}

export function parseAtPage(parser: CSSParser): CSSAtPage | undefined {
	const start = parser.getPosition();
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}
	const token = parser.getToken();
	if (token.type === "LeftCurlyBracket") {
		// in case @page doesn't have any prelude
		const block = parseDeclarationBlock({
			parser,
			/**
			 * This callback is used to check the at-rule keywords inside @page
			 * are correct
			 *
			 * https://www.w3.org/TR/css-page-3/#syntax-page-selector
			 * @param atKeywordToken
			 */
			onAtKeyword: (atKeywordToken) => {
				return ALLOWED_AT_RULES.includes(atKeywordToken.value);
			},
			/**
			 * This Function checks whether the properties inside @page and other valid at rules (@top-left, @top-center, etc)
			 * are correct. Only a certain number of properties are accepted.
			 *
			 * https://www.w3.org/TR/css-page-3/#syntax-page-selector
			 * @param declaration
			 * @param previousAtKeywordToken
			 */
			onAtDeclaration: function(declaration, previousAtKeywordToken) {
				if (
					previousAtKeywordToken &&
					ALLOWED_AT_RULES.includes(previousAtKeywordToken.value)
				) {
					if (
						!ALLOWED_PAGE_MARGIN_PROPERTIES.includes(
							declaration.value.toLowerCase(),
						)
					) {
						parser.unexpectedDiagnostic({
							description: descriptions.CSS_PARSER.AT_PAGE_AT_RULE_INVALID_DECLARATION(
								previousAtKeywordToken.value,
								declaration.value,
								ALLOWED_PAGE_MARGIN_PROPERTIES,
							),
							token: declaration,
						});
						return false;
					}
				} else if (
					!ALLOWED_PAGE_PROPERTIES.includes(declaration.value.toLowerCase())
				) {
					parser.unexpectedDiagnostic({
						description: descriptions.CSS_PARSER.AT_PAGE_INVALID_DECLARATION(
							declaration.value,
							ALLOWED_PAGE_PROPERTIES,
						),
						token: declaration,
					});
					return false;
				}
				return true;
			},
		});
		if (block) {
			return parser.finishNode(
				start,
				{
					type: "CSSAtPage",
					block,
				},
			);
		}
	} else {
		let prelude: CSSPageSelector[] | undefined = undefined;

		const selector = parsePageSelector(parser);
		if (selector) {
			prelude = [];
			prelude.push(selector);
		}
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}

		if (parser.getToken().type === "Comma") {
			nextToken(parser);
			while (true) {
				while (matchToken(parser, "Whitespace")) {
					readToken(parser, "Whitespace");
				}

				if (parser.matchToken("LeftCurlyBracket") || parser.matchToken("EOF")) {
					break;
				}

				const selector = parsePageSelector(parser);

				if (selector && prelude) {
					prelude.push(selector);
				}
			}
		}

		if (prelude) {
			const selectorList = parser.finishNode(
				start,
				{
					type: "CSSPageSelectorList",
					value: prelude,
				},
			);
			while (matchToken(parser, "Whitespace")) {
				readToken(parser, "Whitespace");
			}
			const block = parseDeclarationBlock({parser});
			if (block) {
				return parser.finishNode(
					start,
					{
						type: "CSSAtPage",
						prelude: selectorList,
						block,
					},
				);
			}
		}
	}

	return undefined;
}
