import {ParserCoreTokenizeState, TokenValues} from "@internal/parser-core";
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
import {ZeroIndexed} from "@internal/numbers";
import {
	AnyMarkdownInlineNode,
	MarkdownBoldInline,
	MarkdownEmphasisInline,
	MarkdownText,
} from "@internal/ast";
import {MarkdownParserTypes} from "../types";

type OnUnknownToken = (
	token: TokenValues<Tokens>,
) => AnyMarkdownInlineNode | (AnyMarkdownInlineNode[]) | undefined;

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
export function parseInline(
	parser: MarkdownParser,
	token: Emphasis | Strong,
	onUnknownToken: OnUnknownToken,
): MarkdownEmphasisInline | MarkdownText | MarkdownBoldInline | undefined {
	let children: AnyMarkdownInlineNode[] = [];

	const {leftFlankingDelimiter, closingIndexOfDelimiter} = token;

	// the token can potentially open an inline style, let's start checking the next tokens
	// until we find a potential closing token
	if (leftFlankingDelimiter && closingIndexOfDelimiter !== undefined) {
		const start = parser.getPosition();

		parser.nextToken();
		let exit = false;
		while (
			!(parser.matchToken("EOF") ||
			parser.matchToken("Emphasis") ||
			parser.matchToken("Strong")) &&
			parser.getToken().start <= closingIndexOfDelimiter &&
			!exit
		) {
			const currentToken = parser.getToken();
			if (currentToken.type === "Emphasis" || currentToken.type === "Strong") {
				const possibleChild = parseInline(parser, currentToken, onUnknownToken);

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

export function tokenizeInline(
	parser: MarkdownParser,
	state: MarkdownParserState,
	charToCheck: "*" | "_",
	index: ZeroIndexed,
): ParserCoreTokenizeState<MarkdownParserTypes> | undefined {
	const [valueOfInlineToken, endIndexOfDelimiter] = parser.readInputFrom(
		index,
		(char1) => char1 === charToCheck,
	);

	const leftFlankingDelimiter = canBeLeftFlankingDelimiter({
		startIndex: index,
		endIndex: endIndexOfDelimiter.subtract(1),
		input: parser.input,
	});
	const rightFlankingDelimiter = canBeRightFlankingDelimiter({
		startIndex: index,
		endIndex: endIndexOfDelimiter.subtract(1),
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

				const nextChar = parser.getInputCharOnly(index.add(1));
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
			return [
				{
					isParagraph: endOfInput || isEndOfParagraph
						? false
						: state.isParagraph,
				},
				parser.finishValueToken("Text", valueOfInlineToken, endIndexOfDelimiter),
			];
		}

		const nextChar = parser.getInputCharOnly(closingIndex.add(2));
		const [, closingIndexOfDelimiter] = parser.readInputFrom(
			closingIndex,
			(char1, index, input) => {
				const prevChar = input[index.valueOf() - 1];
				return !(prevChar !== " " && char1 === charToCheck);
			},
		);

		return [
			{
				// if next after two characters we still have a new line, it means we need to start a new paragraph
				isParagraph: nextChar === "\n" ? false : state.isParagraph,
			},

			parser.finishComplexToken<typeof tokenType, DelimiterRun>(
				tokenType,
				{
					closingIndexOfDelimiter,
					leftFlankingDelimiter,
					rightFlankingDelimiter,
					value: valueOfInlineToken,
				},
				endIndexOfDelimiter,
			),
		];
	}

	if (rightFlankingDelimiter) {
		const nextChar = parser.getInputCharOnly(endIndexOfDelimiter.add(2));

		return [
			{
				// if next after two characters we still have a new line, it means we need to start a new paragraph
				isParagraph: nextChar === "\n" ? false : state.isParagraph,
			},
			parser.finishComplexToken<typeof tokenType, DelimiterRun>(
				tokenType,
				{
					leftFlankingDelimiter,
					rightFlankingDelimiter,
					value: valueOfInlineToken,
				},
				endIndexOfDelimiter,
			),
		];
	}

	return undefined;
}
