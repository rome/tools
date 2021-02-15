import {AnyCSSValue, CSSParser, Tokens} from "@internal/css-parser/types";
import {matchToken, nextToken} from "@internal/css-parser/tokenizer";
import {isCustomProperty} from "@internal/css-parser/utils";
import {ValueToken} from "@internal/parser-core";
import {parseSimpleBlock} from "@internal/css-parser/parser/block";
import {parseFunction} from "@internal/css-parser/parser/function";
import {parseUrl} from "@internal/css-parser/parser/url";

export function parseComponentValue(parser: CSSParser): AnyCSSValue | undefined {
	console.log("value", parser.getToken());
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
		if (hashToken.hashType === "id") {
			nextToken(parser);
			return parser.finishNode(
				start,
				{
					type: "CSSHash",
					value: `${hashToken.value}`,
				},
			);
		}
	}

	if (matchToken(parser, "String")) {
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
