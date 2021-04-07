import {CSSParser} from "@internal/css-parser/types";
import {
	CSSAtPage,
	CSSPageSelector,
	CSSPseudoPage,
	CSSPseudoPageValue,
} from "@internal/ast";
import {matchToken, readToken} from "@internal/css-parser/tokenizer";
import {parseDeclarationBlock} from "@internal/css-parser/parser/declaration";
import {descriptions} from "@internal/diagnostics";

const VALID_PSEUDO_PAGE = new Set(["left", "right", "first", "blank"]);

function parsePseudoPage(parser: CSSParser): CSSPseudoPage | undefined {
	const possibleIdent = parser.getToken();
	if (
		possibleIdent.type === "Ident" &&
		VALID_PSEUDO_PAGE.has(possibleIdent.value)
	) {
		const pos = parser.getPosition();
		parser.nextToken();
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
	parser.nextToken();
	return undefined;
}

function parsePageSelector(parser: CSSParser): CSSPageSelector | undefined {
	const token = parser.getToken();
	const start = parser.getPosition();
	let ident;
	let pseudo: CSSPseudoPage | undefined = undefined;
	if (token.type === "Ident") {
		ident = token.value;
		parser.nextToken();
		if (parser.getToken().type === "Colon") {
			parser.nextToken();
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
		parser.nextToken();
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
	parser.nextToken();
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
		const block = parseDeclarationBlock(parser);
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
			parser.nextToken();
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
			const block = parseDeclarationBlock(parser);
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
