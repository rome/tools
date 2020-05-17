/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ParserOptions,
	Position,
	TokenValues,
	createParser,
	isAlpha,
} from "@romejs/parser-core";
import {
	ChildNode,
	Children,
	MarkupTagName,
	TagAttributes,
	TagNode,
	Tokens,
} from "./types";
import {isEscaped} from "@romejs/string-utils";
import {Number0, ob1Add, ob1Get0, ob1Inc} from "@romejs/ob1";
import {descriptions} from "@romejs/diagnostics";
import {unescapeTextValue} from "./escape";

const globalAttributes: Array<string> = ["emphasis", "dim"];

// Tags and their corresponding supported attributes
const tags: Map<MarkupTagName, Array<string>> = new Map();
tags.set("emphasis", []);
tags.set("number", ["approx", "pluralSuffix", "singularSuffix"]);
tags.set("grammarNumber", ["plural", "singular", "none"]);
tags.set("hyperlink", ["target"]);
tags.set("filelink", ["target", "column", "line"]);
tags.set("inverse", []);
tags.set("dim", []);
tags.set("filesize", []);
tags.set("duration", ["approx"]);
tags.set("italic", []);
tags.set("underline", []);
tags.set("strike", []);
tags.set("error", []);
tags.set("success", []);
tags.set("warn", []);
tags.set("info", []);
tags.set("command", []);
tags.set("color", ["fg", "bg"]);
tags.set("highlight", ["i"]);
tags.set("table", []);
tags.set("tr", []);
tags.set("td", ["align"]);
tags.set("hr", []);
tags.set("pad", ["width", "align"]);
tags.set("nobr", []);
tags.set("li", []);
tags.set("ul", []);
tags.set("ol", ["reversed", "start"]);

// Tags that only support certain other tags as their children
const tagsToOnlyChildren: Map<MarkupTagName, Array<MarkupTagName>> = new Map();
tagsToOnlyChildren.set("table", ["tr"]);
tagsToOnlyChildren.set("tr", ["td"]);
tagsToOnlyChildren.set("ol", ["li"]);
tagsToOnlyChildren.set("ul", ["li"]);

// Tags that should only be children of other tags
const tagsToOnlyParent: Map<MarkupTagName, Array<MarkupTagName>> = new Map();
tagsToOnlyParent.set("tr", ["table"]);
tagsToOnlyParent.set("td", ["tr"]);
tagsToOnlyParent.set("li", ["ol", "ul"]);

//
function isStringValueChar(char: string, index: Number0, input: string): boolean {
	if (char === '"' && !isEscaped(index, input)) {
		return false;
	}

	return true;
}

function isTextChar(char: string, index: Number0, input: string): boolean {
	return !isTagStartChar(index, input);
}

export function isTagStartChar(index: Number0, input: string): boolean {
	const i = ob1Get0(index);
	return input[i] === "<" && !isEscaped(index, input);
}

type State = {
	inTagHead: boolean;
};

type StringMarkupParserOptions = ParserOptions;

const createStringMarkupParser = createParser((ParserCore) =>
	class StringMarkupParser extends ParserCore<Tokens, State> {
		constructor(opts: StringMarkupParserOptions) {
			super(opts, "parse/stringMarkup", {inTagHead: false});
		}

		tokenizeWithState(
			index: Number0,
			input: string,
			state: State,
		):
			 | undefined
			| {
					token: TokenValues<Tokens>;
					state: State;
				} {
			const escaped = isEscaped(index, input);
			const char = input[ob1Get0(index)];

			if (!escaped && state.inTagHead) {
				if (char === " ") {
					return this.lookahead(ob1Inc(index));
				}

				if (char === "=") {
					return {
						state,
						token: this.finishToken("Equals"),
					};
				}

				if (char === "/") {
					return {
						state,
						token: this.finishToken("Slash"),
					};
				}

				if (isAlpha(char)) {
					const [value, end] = this.readInputFrom(index, isAlpha);
					return {
						state,
						token: this.finishValueToken("Word", value, end),
					};
				}

				if (char === '"') {
					const [value, stringValueEnd, unclosed] = this.readInputFrom(
						ob1Inc(index),
						isStringValueChar,
					);

					if (unclosed) {
						throw this.unexpected({
							description: descriptions.STRING_MARKUP.UNCLOSED_STRING,
							start: this.getPositionFromIndex(stringValueEnd),
						});
					}

					const end = ob1Add(stringValueEnd, 1);
					return {
						state,
						token: this.finishValueToken("String", unescapeTextValue(value), end),
					};
				}

				if (char === ">") {
					return {
						state: {
							inTagHead: false,
						},
						token: this.finishToken("Greater"),
					};
				}
			}

			if (isTagStartChar(index, input)) {
				return {
					state: {
						inTagHead: true,
					},
					token: this.finishToken("Less"),
				};
			}

			// Keep eating text until we hit a <
			const [value, end] = this.readInputFrom(index, isTextChar);
			return {
				state,
				token: {
					type: "Text",
					value: unescapeTextValue(value),
					start: index,
					end,
				},
			};
		}

		atTagEnd(): boolean {
			return this.matchToken("Less") && this.lookahead().token.type === "Slash";
		}

		parseTag(
			headStart: Position,
			parentTagName: undefined | MarkupTagName,
		): TagNode {
			const nameToken = this.expectToken("Word");
			const tagName = (nameToken.value as MarkupTagName);

			const allowedAttributes = tags.get(tagName);
			if (allowedAttributes === undefined) {
				throw this.unexpected({
					description: descriptions.STRING_MARKUP.UNKNOWN_TAG_NAME(tagName),
					token: nameToken,
				});
			}

			// Check if this tag is restricted to certain parents
			const onlyAllowedAsChild = tagsToOnlyParent.get(tagName);
			if (onlyAllowedAsChild !== undefined) {
				if (
					parentTagName === undefined ||
					!onlyAllowedAsChild.includes(parentTagName)
				) {
					throw this.unexpected({
						description: descriptions.STRING_MARKUP.RESTRICTED_CHILD(
							tagName,
							onlyAllowedAsChild,
							parentTagName,
						),
						token: nameToken,
					});
				}
			}

			// Check if the parent only allows certain children
			if (parentTagName !== undefined) {
				const onlyAllowedAsParent = tagsToOnlyChildren.get(parentTagName);
				if (
					onlyAllowedAsParent !== undefined &&
					!onlyAllowedAsParent.includes(tagName)
				) {
					throw this.unexpected({
						description: descriptions.STRING_MARKUP.RESTRICTED_PARENT(
							parentTagName,
							onlyAllowedAsParent,
							tagName,
						),
						token: nameToken,
					});
				}
			}

			const attributes: TagAttributes = {};
			const children: Children = [];
			let selfClosing = false;

			// Parse attributes
			while (!this.matchToken("EOF") && !this.matchToken("Greater")) {
				const keyToken = this.getToken();

				let key;
				if (keyToken.type === "Word") {
					key = keyToken.value;

					if (!allowedAttributes.includes(key) && !globalAttributes.includes(key)) {
						throw this.unexpected({
							description: descriptions.STRING_MARKUP.INVALID_ATTRIBUTE_NAME_FOR_TAG(
								tagName,
								key,
								[...allowedAttributes, ...globalAttributes],
							),
						});
					}

					this.nextToken();

					// Shorthand properties
					if (
						this.matchToken("Word") ||
						this.matchToken("Slash") ||
						this.matchToken("Greater")
					) {
						attributes[key] = "true";
						continue;
					}

					this.expectToken("Equals");

					const valueToken = this.expectToken("String");
					if (valueToken.type !== "String") {
						throw new Error("Expected String");
					}
					const value = valueToken.value;

					attributes[key] = value;
				} else if (keyToken.type === "Slash") {
					this.nextToken();
					selfClosing = true;
				} else {
					throw this.unexpected({
						description: descriptions.STRING_MARKUP.EXPECTED_ATTRIBUTE_NAME,
					});
				}
			}

			this.expectToken("Greater");

			const headEnd = this.getPosition();

			// Verify closing tag
			if (!selfClosing) {
				while (
					// Build children
					!this.matchToken("EOF") &&
					!this.atTagEnd()
				) {
					const child = this.parseChild(tagName);
					if (child !== undefined) {
						children.push(child);
					}
				}

				if (this.matchToken("EOF")) {
					throw this.unexpected({
						description: descriptions.STRING_MARKUP.UNCLOSED_TAG(
							tagName,
							this.finishLocAt(headStart, headEnd),
						),
					});
				} else {
					this.expectToken("Less");
					this.expectToken("Slash");

					const name = this.getToken();
					if (name.type === "Word") {
						if (name.value !== tagName) {
							throw this.unexpected({
								description: descriptions.STRING_MARKUP.INCORRECT_CLOSING_TAG_NAME(
									tagName,
									name.value,
								),
							});
						}

						this.nextToken();
					} else {
						throw this.unexpected({
							description: descriptions.STRING_MARKUP.EXPECTED_CLOSING_TAG_NAME,
						});
					}

					this.expectToken("Greater");
				}
			}

			return {
				type: "Tag",
				attributes,
				name: tagName,
				children,
			};
		}

		parseChild(parentTagName: undefined | MarkupTagName): undefined | ChildNode {
			const start = this.getPosition();
			const token = this.getToken();
			this.nextToken();

			if (token.type === "Text") {
				// If this tag has restricted children then no text is allowed
				if (parentTagName !== undefined) {
					const onlyAllowedAsParent = tagsToOnlyChildren.get(parentTagName);
					if (onlyAllowedAsParent !== undefined) {
						// Ignore text that's just whitespace
						if (token.value.trim() === "") {
							return undefined;
						} else {
							throw this.unexpected({
								description: descriptions.STRING_MARKUP.RESTRICTED_PARENT_TEXT(
									parentTagName,
								),
								token,
							});
						}
					}
				}

				return {
					type: "Text",
					value: token.value,
				};
			} else if (token.type === "Less") {
				return this.parseTag(start, parentTagName);
			} else {
				throw this.unexpected({
					description: descriptions.STRING_MARKUP.UNKNOWN_START,
				});
			}
		}

		parse(): Children {
			const children: Children = [];
			while (!this.matchToken("EOF")) {
				const child = this.parseChild(undefined);
				if (child !== undefined) {
					children.push(child);
				}
			}
			return children;
		}
	}
);

export function parseMarkup(input: string) {
	try {
		return createStringMarkupParser({input}).parse();
	} catch (err) {
		throw err;
	}
}
