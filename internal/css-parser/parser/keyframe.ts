import {CSSParser, Tokens} from "@internal/css-parser/types";
import {
	CSSKeyframe,
	CSSKeyframeBlock,
	CSSKeyframeName,
	CSSKeyframeSelector,
	CSSPercentage,
	CSSRaw,
	CSSString,
} from "@internal/ast";
import {matchToken, nextToken, readToken} from "@internal/css-parser/tokenizer";
import {descriptions} from "@internal/diagnostics";
import {CSS_WIDE_KEYWORDS, isCustomIdent} from "@internal/css-parser/utils";
import {parseDeclarations} from "@internal/css-parser/parser/declaration";

const VALID_IDENTS = ["from", "to"];

function parseKeyframeName(parser: CSSParser): CSSKeyframeName | undefined {
	let value: CSSRaw | CSSString;
	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}

	if (!(parser.matchToken("Ident") || parser.matchToken("String"))) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.MISSING_KEYFRAME_NAME,
			token: parser.getToken(),
		});
		nextToken(parser);
		return undefined;
	}
	const start = parser.getPosition();

	if (parser.matchToken("String")) {
		const token = parser.getToken() as Tokens["String"];
		nextToken(parser);

		value = parser.finishNode(
			start,
			{
				type: "CSSString",
				value: token.value,
			},
		);
	} else {
		const token = parser.getToken() as Tokens["Ident"];
		if (!isCustomIdent(token)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_IDENTIFIER(
					token.value,
					CSS_WIDE_KEYWORDS,
				),
				token: parser.getToken(),
			});
			nextToken(parser);
			return undefined;
		}
		nextToken(parser);

		value = parser.finishNode(
			start,
			{
				type: "CSSRaw",
				value: token.value,
			},
		);
	}

	return parser.finishNode(
		start,
		{
			type: "CSSKeyframeName",
			value,
		},
	);
}

function parseKeyframeSelector(
	parser: CSSParser,
): CSSKeyframeSelector | undefined {
	let value: CSSRaw | CSSPercentage;
	const start = parser.getPosition();

	if (matchToken(parser, "Percentage")) {
		const percentage = (parser.getToken() as Tokens["Percentage"]).value;
		nextToken(parser);
		value = parser.finishNode(
			start,
			{
				type: "CSSPercentage",
				value: percentage,
			},
		);
	} else if (matchToken(parser, "Ident")) {
		const token = parser.getToken() as Tokens["Ident"];
		if (!VALID_IDENTS.includes(token.value)) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.INVALID_KEYFRAME_SELECTOR_NAME(
					token.value,
					VALID_IDENTS,
				),
				token: parser.getToken(),
			});
			nextToken(parser);
			return undefined;
		}
		nextToken(parser);
		value = parser.finishNode(
			start,
			{
				type: "CSSRaw",
				value: token.value,
			},
		);
	} else {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.UNKNOWN_KEYFRAME_SELECTOR_NAME,
			token: parser.getToken(),
		});
		nextToken(parser);
		return undefined;
	}

	return parser.finishNode(
		start,
		{
			type: "CSSKeyframeSelector",
			value,
		},
	);
}

function parseKeyframeBlocks(parser: CSSParser): CSSKeyframeBlock[] | undefined {
	const blocks: CSSKeyframeBlock[] = [];

	while (!matchToken(parser, "EOF")) {
		if (matchToken(parser, "RightCurlyBracket")) {
			nextToken(parser);
			break;
		}

		if (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
			continue;
		}
		const pos = parser.getPosition();
		const name = parseKeyframeSelector(parser);

		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}

		if (!matchToken(parser, "LeftCurlyBracket")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.EXPECTED_LBRACKET,
				token: parser.getToken(),
			});
			nextToken(parser);
			return undefined;
		}

		nextToken(parser);
		const value = parseDeclarations({
			parser,
			endingTokenType: "RightCurlyBracket",
		});

		if (name && value) {
			nextToken(parser);
			const block = parser.finishNode(
				pos,
				{
					type: "CSSKeyframeBlock",
					name,
					value,
				},
			);
			blocks.push(block);
		} else {
			nextToken(parser);
		}
	}

	return blocks;
}

export function parseKeyframe(parser: CSSParser): CSSKeyframe | undefined {
	const start = parser.getPosition();
	nextToken(parser);

	const name = parseKeyframeName(parser);
	if (!name) {
		return undefined;
	}

	while (matchToken(parser, "Whitespace")) {
		readToken(parser, "Whitespace");
	}

	if (!parser.matchToken("LeftCurlyBracket")) {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.EXPECTED_LBRACKET,
			token: parser.getToken(),
		});
		nextToken(parser);
		return undefined;
	}

	nextToken(parser);

	const value = parseKeyframeBlocks(parser);
	if (value === undefined) {
		return undefined;
	}

	return parser.finishNode(
		start,
		{
			type: "CSSKeyframe",
			name,
			value,
		},
	);
}
