/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BaseTokens,
	NodeBase,
	ParserCore,
	ParserOptions,
	ValueToken,
	createParser,
} from "@internal/parser-core";
import {isEscaped} from "@internal/string-utils";
import {Number0, ob1Add, ob1Get0} from "@internal/ob1";
import {descriptions} from "@internal/diagnostics";

type Tokens = BaseTokens & {
	Hashes: ValueToken<"Hashes", number>;
	CodeBlock: ValueToken<
		"CodeBlock",
		{
			text: string;
			language: undefined | string;
		}
	>;
	TextLine: ValueToken<"TextLine", string>;
};

type HeadingNode = NodeBase & {
	type: "Heading";
	text: string;
	level: number;
};

type CodeBlockNode = NodeBase & {
	type: "CodeBlock";
	language: undefined | string;
	text: string;
};

type TextLineNode = NodeBase & {
	type: "TextLine";
	text: string;
};

type Node = HeadingNode | CodeBlockNode | TextLineNode;

function isHash(char: string): boolean {
	return char === "#";
}

function isCodeBlockEnd(index: Number0, input: string): boolean {
	return (
		input[ob1Get0(index)] === "`" &&
		!isEscaped(index, input) &&
		input[ob1Get0(ob1Add(index, 1))] === "`" &&
		input[ob1Get0(ob1Add(index, 2))] === "`"
	);
}

function isInCodeBlock(char: string, index: Number0, input: string): boolean {
	return !isCodeBlockEnd(index, input);
}

function isntNewline(char: string): boolean {
	return char !== "\n";
}

function unescapeTicks(code: string): string {
	return code;
}

type SnapshotParserTypes = {
	tokens: Tokens;
	options: ParserOptions;
	state: {};
	meta: void;
};

type SnapshotParser = ParserCore<SnapshotParserTypes>;

export const createSnapshotParser = createParser<SnapshotParserTypes>({
	diagnosticCategory: "parse/snapshots",
	ignoreWhitespaceTokens: true,

	tokenize(parser, index) {
		const char = parser.getInputCharOnly(index);

		switch (char) {
			case "#": {
				const [hashes] = parser.readInputFrom(index, isHash);
				const level = hashes.length;
				return parser.finishValueToken("Hashes", level, ob1Add(index, level));
			}

			case "`": {
				const nextChar = parser.getInputCharOnly(index, 1);
				const nextNextChar = parser.getInputCharOnly(index, 2);

				if (nextChar === "`" && nextNextChar === "`") {
					let codeOffset = ob1Add(index, 3);

					let language: undefined | string;
					if (parser.getInputCharOnly(codeOffset) !== "\n") {
						[language, codeOffset] = parser.readInputFrom(
							codeOffset,
							isntNewline,
						);
					}

					// Expect the first offset character to be a newline
					if (parser.getInputCharOnly(codeOffset) === "\n") {
						// Skip leading newline
						codeOffset = ob1Add(codeOffset, 1);
					} else {
						throw parser.unexpected({
							description: descriptions.SNAPSHOTS.MISSING_NEWLINE_AFTER_CODE_BLOCK,
							start: parser.getPositionFromIndex(codeOffset),
						});
					}

					let [code] = parser.readInputFrom(codeOffset, isInCodeBlock);

					let end = ob1Add(codeOffset, code.length);

					if (isCodeBlockEnd(end, parser.input)) {
						// Check for trailing newline
						if (code[code.length - 1] === "\n") {
							// Trim trailing newline
							code = code.slice(0, -1);

							// Skip closing ticks
							end = ob1Add(end, 3);

							return parser.finishValueToken(
								"CodeBlock",
								{
									language,
									text: unescapeTicks(code),
								},
								end,
							);
						} else {
							throw parser.unexpected({
								description: descriptions.SNAPSHOTS.MISSING_NEWLINE_BEFORE_CODE_BLOCK,
								start: parser.getPositionFromIndex(end),
							});
						}
					} else {
						throw parser.unexpected({
							description: descriptions.SNAPSHOTS.UNCLOSED_CODE_BLOCK,
							start: parser.getPositionFromIndex(end),
						});
					}
				}
			}
		}

		const [text, end] = parser.readInputFrom(index, isntNewline);
		return parser.finishValueToken("TextLine", text, end);
	},
});

export function parseSnapshot(parser: SnapshotParser): Array<Node> {
	const nodes: Array<Node> = [];

	while (!parser.matchToken("EOF")) {
		const start = parser.getPosition();
		const token = parser.getToken();

		switch (token.type) {
			case "Hashes": {
				const level = token.value;
				parser.nextToken();
				const text = parser.expectToken("TextLine").value;
				nodes.push({
					type: "Heading",
					level,
					text,
					loc: parser.finishLoc(start),
				});
				break;
			}

			case "CodeBlock": {
				nodes.push({
					type: "CodeBlock",
					...token.value,
					loc: parser.finishLoc(start),
				});
				parser.nextToken();
				break;
			}

			case "TextLine": {
				nodes.push({
					type: "TextLine",
					text: token.value,
					loc: parser.finishLoc(start),
				});
				parser.nextToken();
				break;
			}

			default:
				throw parser.unexpected();
		}
	}

	return nodes;
}
