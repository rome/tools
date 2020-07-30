/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ParserCoreState,
	ParserOptions,
	Position,
	TokenValues,
	createParser,
	isAlpha,
} from "@romefrontend/parser-core";
import {
	MarkupParsedAttributes,
	MarkupParsedChild,
	MarkupParsedChildren,
	MarkupParsedTag,
	MarkupTagName,
	Tokens,
} from "./types";
import {isEscaped} from "@romefrontend/string-utils";
import {Number0, ob1Add, ob1Dec, ob1Get0, ob1Inc} from "@romefrontend/ob1";
import {descriptions} from "@romefrontend/diagnostics";
import {unescapeTextValue} from "./escape";
import {createEmptyAttributes} from "./util";
import {
	globalAttributes,
	tags,
	tagsToOnlyChildren,
	tagsToOnlyParent,
} from "./tags";

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

type State = ParserCoreState & {
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
			state: State,
		):
			| undefined
			| {
					token: TokenValues<Tokens>;
					state: State;
				} {
			const escaped = isEscaped(index, this.input);
			const char = this.getInputCharOnly(index);

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
						token: this.finishValueToken(
							"String",
							unescapeTextValue(value),
							end,
						),
					};
				}

				if (char === ">") {
					return {
						state: {
							...state,
							inTagHead: false,
						},
						token: this.finishToken("Greater"),
					};
				}
			}

			if (isTagStartChar(index, this.input)) {
				return {
					state: {
						...state,
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
		): MarkupParsedTag {
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

			const attributes: MarkupParsedAttributes = createEmptyAttributes();
			const children: MarkupParsedChildren = [];
			let selfClosing = false;

			// Parse attributes
			while (!this.matchToken("EOF") && !this.matchToken("Greater")) {
				const keyToken = this.getToken();

				let valueToken: TokenValues<Tokens>;
				let key;
				if (keyToken.type === "Word") {
					key = keyToken.value;

					const validator =
						allowedAttributes.get(key) || globalAttributes.get(key);
					if (validator === undefined) {
						throw this.unexpected({
							description: descriptions.STRING_MARKUP.INVALID_ATTRIBUTE_NAME_FOR_TAG(
								tagName,
								key,
								[...allowedAttributes.keys(), ...globalAttributes.keys()],
							),
						});
					}

					this.nextToken();

					let rawValue;

					// Shorthand properties
					if (
						this.matchToken("Word") ||
						this.matchToken("Slash") ||
						this.matchToken("Greater")
					) {
						rawValue = key;
						valueToken = keyToken;
					} else {
						this.expectToken("Equals");

						valueToken = this.expectToken("String");
						rawValue = valueToken.value;
					}

					const value = validator(rawValue, key);

					if (value === undefined) {
						throw this.unexpected({
							token: valueToken,
							description: descriptions.STRING_MARKUP.INVALID_ATTRIBUTE_VALUE(
								tagName,
								key,
								rawValue,
							),
						});
					}

					attributes.get(
						key,
						{
							getDiagnosticLocation: (target) => {
								switch (target) {
									case "key":
										return this.getDiagnosticLocation({token: keyToken});

									case "value":
										return this.getDiagnosticLocation({token: valueToken});

									case "inner-value":
										if (valueToken === keyToken) {
											// Shorthand
											return this.getDiagnosticLocation({token: keyToken});
										} else {
											// Remove string quotes
											return this.getDiagnosticLocation({
												startIndex: ob1Inc(keyToken.start),
												endIndex: ob1Dec(valueToken.end),
											});
										}

									case "all":
										return this.getDiagnosticLocation({
											startIndex: keyToken.start,
											endIndex: valueToken.end,
										});
								}
							},
						},
					).setValue(value);
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

		parseChild(
			parentTagName: undefined | MarkupTagName,
		): undefined | MarkupParsedChild {
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
					source: true,
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

		parse(): MarkupParsedChildren {
			const children: MarkupParsedChildren = [];
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

export function parseMarkup(input: string, opts: ParserOptions = {}) {
	try {
		return createStringMarkupParser({...opts, input}).parse();
	} catch (err) {
		throw err;
	}
}
