/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BaseTokens,
	NodeBase,
	ParserOptions,
	ValueToken,
	createParser,
} from "@romejs/parser-core";
import {isEscaped} from "@romejs/string-utils";
import {Number0, ob1Add, ob1Get0} from "@romejs/ob1";
import {descriptions} from "@romejs/diagnostics";

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

export const createSnapshotParser = createParser((ParserCore) =>
	class SnapshotParser extends ParserCore<Tokens> {
		constructor(opts: ParserOptions) {
			super(opts, "parse/snapshots", {});
			this.ignoreWhitespaceTokens = true;
		}

		tokenize(index: Number0, input: string) {
			const char = input[ob1Get0(index)];

			switch (char) {
				case "#": {
					const [hashes] = this.readInputFrom(index, isHash);
					const level = hashes.length;
					return this.finishValueToken("Hashes", level, ob1Add(index, level));
				}

				case "`": {
					const nextChar = input[ob1Get0(ob1Add(index, 1))];
					const nextNextChar = input[ob1Get0(ob1Add(index, 2))];

					if (nextChar === "`" && nextNextChar === "`") {
						let codeOffset = ob1Add(index, 3);

						let language: undefined | string;
						if (input[ob1Get0(codeOffset)] !== "\n") {
							[language, codeOffset] = this.readInputFrom(
								codeOffset,
								isntNewline,
							);
						}

						// Expect the first offset character to be a newline
						if (input[ob1Get0(codeOffset)] === "\n") {
							// Skip leading newline
							codeOffset = ob1Add(codeOffset, 1);
						} else {
							throw this.unexpected({
								description: descriptions.SNAPSHOTS.MISSING_NEWLINE_AFTER_CODE_BLOCK,
								start: this.getPositionFromIndex(codeOffset),
							});
						}

						let [code] = this.readInputFrom(codeOffset, isInCodeBlock);

						let end = ob1Add(codeOffset, code.length);

						if (isCodeBlockEnd(end, input)) {
							// Check for trailing newline
							if (code[code.length - 1] === "\n") {
								// Trim trailing newline
								code = code.slice(0, -1);

								// Skip closing ticks
								end = ob1Add(end, 3);

								return this.finishValueToken(
									"CodeBlock",
									{
										language,
										text: unescapeTicks(code),
									},
									end,
								);
							} else {
								throw this.unexpected({
									description: descriptions.SNAPSHOTS.MISSING_NEWLINE_BEFORE_CODE_BLOCK,
									start: this.getPositionFromIndex(end),
								});
							}
						} else {
							throw this.unexpected({
								description: descriptions.SNAPSHOTS.UNCLOSED_CODE_BLOCK,
								start: this.getPositionFromIndex(end),
							});
						}
					}
				}
			}

			const [text, end] = this.readInputFrom(index, isntNewline);
			return this.finishValueToken("TextLine", text, end);
		}

		parse(): Array<Node> {
			const nodes: Array<Node> = [];

			while (!this.matchToken("EOF")) {
				const start = this.getPosition();
				const token = this.getToken();

				switch (token.type) {
					case "Hashes": {
						const level = token.value;
						this.nextToken();
						const text = this.expectToken("TextLine").value;
						nodes.push({
							type: "Heading",
							level,
							text,
							loc: this.finishLoc(start),
						});
						break;
					}

					case "CodeBlock": {
						nodes.push({
							type: "CodeBlock",
							...token.value,
							loc: this.finishLoc(start),
						});
						this.nextToken();
						break;
					}

					case "TextLine": {
						nodes.push({
							type: "TextLine",
							text: token.value,
							loc: this.finishLoc(start),
						});
						this.nextToken();
						break;
					}

					default:
						throw this.unexpected();
				}
			}

			return nodes;
		}
	}
);
