import {CSSParser, Tokens} from "@internal/css-parser/types";
import {matchToken, nextToken} from "@internal/css-parser/tokenizer";
import {isCustomProperty} from "@internal/css-parser/utils";
import {ValueToken} from "@internal/parser-core";
import {parseSimpleBlock} from "@internal/css-parser/parser/block";
import {parseFunction} from "@internal/css-parser/parser/function";
import {parseUrl} from "@internal/css-parser/parser/url";
import {AnyCSSValue, CSSCustomProperty} from "@internal/ast";
import {parseTemplateAreas} from "@internal/css-parser/parser/grid/parseTemplateAreas";
import {parseGridArea} from "@internal/css-parser/parser/grid/parseGridArea";

export function parseComponentValue(
	parser: CSSParser,
	declarationName?: string | CSSCustomProperty,
): AnyCSSValue | undefined {
	if (
		matchToken(parser, "LeftCurlyBracket") ||
		matchToken(parser, "LeftParen") ||
		matchToken(parser, "LeftSquareBracket")
	) {
		return parseSimpleBlock(parser);
	}

	if (matchToken(parser, "Function")) {
		return parseFunction(parser);
	}

	if (matchToken(parser, "URL") || matchToken(parser, "BadURL")) {
		return parseUrl(parser);
	}

	const start = parser.getPosition();

	if (matchToken(parser, "Whitespace")) {
		nextToken(parser);
		return undefined;
	}

	if (matchToken(parser, "Dimension")) {
		const unit = (parser.getToken() as Tokens["Dimension"]).unit;
		const value = (parser.getToken() as Tokens["Dimension"]).value;
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSDimension",
				unit,
				value,
			},
		);
	}

	if (matchToken(parser, "Percentage")) {
		const value = (parser.getToken() as Tokens["Percentage"]).value;
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSPercentage",
				value,
			},
		);
	}

	if (matchToken(parser, "Ident")) {
		if (declarationName && typeof declarationName === "string") {
			if (declarationName === "grid-area") {
				const gridArea = parseGridArea(parser);
				if (gridArea) {
					return gridArea;
				}
				// there's been an error, so we exist from the function
				return undefined;
			}
		}
		const value = (parser.getToken() as Tokens["Ident"]).value;
		nextToken(parser);
		if (isCustomProperty(value)) {
			return parser.finishNode(
				parser.getPosition(),
				{
					type: "CSSCustomProperty",
					value,
				},
			);
		}

		return parser.finishNode(
			start,
			{
				type: "CSSIdentifier",
				value,
			},
		);
	}

	if (matchToken(parser, "Number")) {
		if (declarationName && typeof declarationName === "string") {
			if (declarationName === "grid-area") {
				const gridArea = parseGridArea(parser);
				if (gridArea) {
					return gridArea;
				}
			}
		}
		const numberToken = parser.getToken() as Tokens["Number"];
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSNumber",
				value: numberToken.value,
				raw: numberToken.raw,
			},
		);
	}

	if (matchToken(parser, "Colon")) {
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSRaw",
				value: ":",
			},
		);
	}

	if (matchToken(parser, "Comma")) {
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSComma",
			},
		);
	}

	if (matchToken(parser, "Hash")) {
		const hashToken = parser.getToken() as Tokens["Hash"];
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSHash",
				value: `${hashToken.value}`,
			},
		);
	}

	if (matchToken(parser, "String")) {
		// grid syntax is a bit more , strict and its values are a bit more strict
		if (declarationName && typeof declarationName === "string") {
			if (declarationName === "grid-template-areas") {
				const stringToken = parseTemplateAreas(parser);
				if (stringToken) {
					nextToken(parser);
					return parser.finishNode(
						start,
						{
							type: "CSSString",
							value: stringToken.value,
						},
					);
				}
			}
		}

		const stringToken = parser.getToken() as Tokens["String"];
		nextToken(parser);
		return parser.finishNode(
			start,
			{
				type: "CSSString",
				value: stringToken.value,
			},
		);
	}

	const value = (parser.getToken() as ValueToken<string, string>).value;
	nextToken(parser);
	return parser.finishNode(
		start,
		{
			type: "CSSRaw",
			value,
		},
	);
}
