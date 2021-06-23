/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BaseTokens,
	NodeBase,
	NumberToken,
	ParserCore,
	ParserOptions,
	StringToken,
	ValueToken,
	createParser,
} from "@internal/parser-core";
import {isEscaped} from "@internal/string-utils";
import {ZeroIndexed} from "@internal/numbers";
import {descriptions} from "@internal/diagnostics";

type Tokens = BaseTokens & {
	Hashes: NumberToken<"Hashes">;
	CodeBlock: ValueToken<
		"CodeBlock",
		{
			text: string;
			language: undefined | string;
		}
		>;
	TextLine: StringToken<"TextLine">;
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

function isCodeBlockEnd(index: ZeroIndexed, input: string): boolean {
	return (
		input[index.valueOf()] === "`" &&
		!isEscaped(index, input) &&
		input[index.add(1).valueOf()] === "`" &&
		input[index.add(2).valueOf()] === "`"
	);
}

function isInCodeBlock(char: string, index: ZeroIndexed, input: string): boolean {
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

// TODO just use markdown-parser
export const snapshotParser = createParser<SnapshotParserTypes>({
	diagnosticLanguage: "markdown",
	ignoreWhitespaceTokens: true,

	tokenize(parser, tokenizer) {
		if (tokenizer.startsWith("#")) {
			const hashes = tokenizer.read(isHash);
			const level = hashes.length;
			return tokenizer.finishValueToken("Hashes", level);
		}

		if (tokenizer.consume("```")) {
			let language: undefined | string;
			if (!tokenizer.startsWith("\n")) {
				language = tokenizer.read(isntNewline);
			}

			// Expect the first offset character to be a newline
			if (!tokenizer.consume("\n")) {
				throw parser.unexpected({
					description: descriptions.SNAPSHOTS.MISSING_NEWLINE_AFTER_CODE_BLOCK,
					start: tokenizer.getPosition(),
				});
			}

			let code = tokenizer.read(isInCodeBlock);

			if (isCodeBlockEnd(tokenizer.index, parser.input)) {
				// Check for trailing newline
				if (code[code.length - 1] === "\n") {
					// Trim trailing newline
					code = code.slice(0, -1);

					tokenizer.assert("```");

					return tokenizer.finishValueToken(
						"CodeBlock",
						{
							language,
							text: unescapeTicks(code),
						},
					);
				} else {
					throw parser.unexpected({
						description: descriptions.SNAPSHOTS.MISSING_NEWLINE_BEFORE_CODE_BLOCK,
						start: tokenizer.getPosition(),
					});
				}
			} else {
				throw parser.unexpected({
					description: descriptions.SNAPSHOTS.UNCLOSED_CODE_BLOCK,
					start: tokenizer.getPosition(),
				});
			}
		}

		const text = tokenizer.read(isntNewline);
		return tokenizer.finishValueToken("TextLine", text);
	},
});

export function parseSnapshot(parser: SnapshotParser): Node[] {
	const nodes: Node[] = [];

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
