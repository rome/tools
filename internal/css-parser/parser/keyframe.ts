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
import {
	CSS_WIDE_KEYWORDS,
	getBlockEndTokenType,
	getBlockStartTokenValue,
	isCustomIdent,
} from "@internal/css-parser/utils";
import {parseDeclarationBlock} from "@internal/css-parser/parser/declaration";

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
		parser.nextToken();
		return undefined;
	}
	const start = parser.getPosition();

	if (parser.matchToken("String")) {
		const token = parser.getToken() as Tokens["String"];
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
			parser.nextToken();
			return undefined;
		}

		value = parser.finishNode(
			start,
			{
				type: "CSSRaw",
				value: token.value,
			},
		);
	}
	parser.nextToken();

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

	if (matchToken(parser, "Percentage")) {
		const pos = parser.getPosition();
		value = parser.finishNode(
			pos,
			{
				type: "CSSPercentage",
				value: (parser.getToken() as Tokens["Percentage"]).value,
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
			parser.nextToken();
			return undefined;
		}
		value = parser.finishNode(
			parser.getPosition(),
			{
				type: "CSSRaw",
				value: token.value,
			},
		);
	} else {
		parser.unexpectedDiagnostic({
			description: descriptions.CSS_PARSER.UNKNOW_KEYFRAME_SELECTOR_NAME,
			token: parser.getToken(),
		});
		parser.nextToken();
		return undefined;
	}

	const start = parser.getPosition();
	parser.nextToken();
	return parser.finishNode(
		start,
		{
			type: "CSSKeyframeSelector",
			value,
		},
	);
}

function parseKeyframeBlocks(
	parser: CSSParser,
	endingTokenType: keyof Tokens,
): CSSKeyframeBlock[] | undefined {
	const blocks: CSSKeyframeBlock[] = [];

	while (!matchToken(parser, "EOF")) {
		if (endingTokenType && matchToken(parser, endingTokenType)) {
			nextToken(parser);
			break;
		}

		if (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
			continue;
		}

		const name = parseKeyframeSelector(parser);
		while (matchToken(parser, "Whitespace")) {
			readToken(parser, "Whitespace");
		}
		if (!matchToken(parser, "LeftCurlyBracket")) {
			parser.unexpectedDiagnostic({
				description: descriptions.CSS_PARSER.EXPECTED_LBRACKET,
				token: parser.getToken(),
			});
			parser.nextToken();
			return undefined;
		}
		const value = parseDeclarationBlock(parser);
		if (name && value) {
			const block: CSSKeyframeBlock = parser.finishNode(
				parser.getPosition(),
				{
					type: "CSSKeyframeBlock",
					name,
					value,
				},
			);
			blocks.push(block);
		}

		parser.nextToken();
	}

	return blocks;
}

export function parseKeyframe(parser: CSSParser): CSSKeyframe | undefined {
	const start = parser.getPosition();
	parser.nextToken();
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
		parser.nextToken();
		return undefined;
	}
	const startingToken = parser.getToken();
	const startingTokenValue = getBlockStartTokenValue(parser, startingToken);
	const endingTokenType = getBlockEndTokenType(parser, startingToken);

	if (!endingTokenType) {
		return undefined;
	}

	nextToken(parser);

	const value = parseKeyframeBlocks(parser, endingTokenType);

	if (!value) {
		return undefined;
	}
	return parser.finishNode(
		start,
		{
			type: "CSSKeyframe",
			name,
			value,
			startingTokenValue,
		},
	);
}
