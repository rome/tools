import {TokenValues} from "@internal/parser-core";
import {
	DelimiterRun,
	Emphasis,
	MarkdownParser,
	MarkdownParserState,
	Strong,
	Tokens,
	canBeLeftFlankingDelimiter,
	canBeRightFlankingDelimiter,
	hasBlockTokens,
} from "@internal/markdown-parser";
import {Number0, ob1Add, ob1Get0, ob1Sub} from "@internal/ob1";
import {
	AnyMarkdownInlineNode,
	MarkdownBoldInline,
	MarkdownEmphasisInline,
	MarkdownText,
} from "@internal/ast";

type OnUnknownToken = (
	token: TokenValues<Tokens>,
) => AnyMarkdownInlineNode | Array<AnyMarkdownInlineNode> | undefined;

// TODO: to handle the case of **something **else** is** broken
// NOTE: at the moment the code detects the first closing tag, the one beside "else**"
// HINT: review logic when checking the right flanking and interrogate state and update the delimiters
// HINT: keep pinpoints

/**
 * This function supports recursion, in case we have nested emphasis/strong
 * @param parser
 * @param token
 * @param onUnknownToken
 */
export function parseTextWrapping(
	parser: MarkdownParser,
	token: Emphasis | Strong,
	onUnknownToken: OnUnknownToken,
): MarkdownEmphasisInline | MarkdownText | MarkdownBoldInline {
	let children: Array<AnyMarkdownInlineNode> = [];

	// let inlineNode: MarkdownEmphasisInline | undefined = undefined;
	const {leftFlankingDelimiter, closingIndexOfDelimiter} = token;

	// the token can potentially open an inline style, let's start checking the next tokens
	// until we find a potential closing token
	if (leftFlankingDelimiter && closingIndexOfDelimiter !== undefined) {
		const start = parser.getPosition();

		parser.nextToken();
		let exit = false;
		while (
			!parser.matchToken("EOF") &&
			!(parser.matchToken("Emphasis") || parser.matchToken("Strong")) &&
			parser.getToken().start <= closingIndexOfDelimiter &&
			!exit
		) {
			const currentToken = parser.getToken();
			if (currentToken.type === "Emphasis" || currentToken.type === "Strong") {
				const possibleChild = parseTextWrapping(
					parser,
					currentToken,
					onUnknownToken,
				);

				if (possibleChild) {
					children.push(possibleChild);
				}

				parser.nextToken();
			} else {
				const nodeOrNodes = onUnknownToken(currentToken);
				parser.nextToken();
				if (nodeOrNodes === undefined) {
					exit = true;
				} else if (Array.isArray(nodeOrNodes)) {
					children.push(...nodeOrNodes);
				} else {
					children.push(nodeOrNodes);
				}
			}
		}
		if (token.type === "Emphasis" || token.type === "Strong") {
			return parser.finishNode(
				start,
				{
					type: "MarkdownEmphasisInline",
					value: children,
				},
			);
		}
		return parser.finishNode(
			start,
			{
				type: "MarkdownBoldInline",
				value: children,
			},
		);
	}
	return parser.finishNode(
		parser.getPosition(),
		{
			type: "MarkdownText",
			value: token.value,
		},
	);
}

type TokenizeInline = {
	state: MarkdownParserState;
	token: Tokens["Text"] | Tokens["Emphasis"] | Tokens["Strong"];
};

export function tokenizeTextWrapping(
	parser: MarkdownParser,
	state: MarkdownParserState,
	charToCheck: "*" | "_",
	index: Number0,
): TokenizeInline | undefined {
	const [valueOfInlineToken, endIndexOfDelimiter] = parser.readInputFrom(
		index,
		(char1) => char1 === charToCheck,
	);

	const leftFlankingDelimiter = canBeLeftFlankingDelimiter({
		startIndex: index,
		endIndex: ob1Sub(endIndexOfDelimiter, 1),
		input: parser.input,
	});
	const rightFlankingDelimiter = canBeRightFlankingDelimiter({
		startIndex: index,
		endIndex: ob1Sub(endIndexOfDelimiter, 1),
		input: parser.input,
	});

	const tokenType = valueOfInlineToken.length === 1 ? "Emphasis" : "Strong";
	if (leftFlankingDelimiter) {
		let rightFlankingDelimiterFound = false;
		let isEndOfParagraph = false;
		const [, closingIndex, endOfInput] = parser.readInputFrom(
			index,
			(char, indexToCheck, input) => {
				if (hasBlockTokens(char, indexToCheck, input)) {
					// found list item ahead, let's exit
					isEndOfParagraph = true;
					return false;
				}

				// the right flanking check should be done only when there's a
				// ending character that matches the starting character
				if (char !== charToCheck || indexToCheck === index) {
					// continue, no need to do further checks
					return true;
				}

				let endIndex = indexToCheck;

				const nextChar = parser.getInputCharOnly(ob1Add(index, 1));
				if (valueOfInlineToken.length > 1) {
					// we found a character that matches but we need to make sure that also the next character
					// is the same
					if (nextChar !== charToCheck) {
						return true;
					}
				}

				rightFlankingDelimiterFound = canBeRightFlankingDelimiter({
					startIndex: indexToCheck,
					endIndex,
					input: parser.input,
				});

				// we should stop only if the characters that we found are a right flanking delimiter
				return !rightFlankingDelimiterFound;
			},
		);

		if (!rightFlankingDelimiterFound || endOfInput) {
			return {
				token: parser.finishValueToken(
					"Text",
					valueOfInlineToken,
					endIndexOfDelimiter,
				),
				state: {
					...state,
					isParagraph: endOfInput || isEndOfParagraph
						? false
						: state.isParagraph,
				},
			};
		}

		const nextChar = parser.getInputCharOnly(ob1Add(closingIndex, 2));
		const [, closingIndexOfDelimiter] = parser.readInputFrom(
			closingIndex,
			(char1, index, input) => {
				const prevChar = input[ob1Get0(index) - 1];
				return !(prevChar !== " " && char1 === charToCheck);
			},
		);

		return {
			state: {
				...state,
				// if next after two characters we still have a new line, it means we need to start a new paragraph
				isParagraph: nextChar === "\n" ? false : state.isParagraph,
			},

			token: parser.finishComplexToken<typeof tokenType, DelimiterRun>(
				tokenType,
				{
					closingIndexOfDelimiter,
					leftFlankingDelimiter,
					rightFlankingDelimiter,
					value: valueOfInlineToken,
				},
				endIndexOfDelimiter,
			),
		};
	}

	if (rightFlankingDelimiter) {
		const nextChar = parser.getInputCharOnly(ob1Add(endIndexOfDelimiter, 2));

		return {
			state: {
				...state,
				// if next after two characters we still have a new line, it means we need to start a new paragraph
				isParagraph: nextChar === "\n" ? false : state.isParagraph,
			},
			token: parser.finishComplexToken<typeof tokenType, DelimiterRun>(
				tokenType,
				{
					leftFlankingDelimiter,
					rightFlankingDelimiter,
					value: valueOfInlineToken,
				},
				endIndexOfDelimiter,
			),
		};
	}
	return undefined;
}
