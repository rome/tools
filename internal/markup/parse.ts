/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	ParserCore,
	ParserOptions,
	Position,
	TokenValues,
	createParser,
	isAlpha,
} from "@internal/parser-core";
import {
	MarkupParsedAttributes,
	MarkupParsedChild,
	MarkupParsedChildren,
	MarkupParsedTag,
	MarkupTagName,
	Tokens,
} from "./types";
import {isEscaped} from "@internal/string-utils";
import {Number0, ob1Add, ob1Dec, ob1Get0, ob1Inc} from "@internal/ob1";
import {descriptions} from "@internal/diagnostics";
import {
	AnyMarkup,
	StaticMarkup,
	readMarkup,
	serializeLazyMarkup,
	unescapeTextValue,
} from "./escape";
import {createEmptyAttributes, isSingleEscaped} from "./util";
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

type State = {
	inTagHead: boolean;
};

type MarkupParserTypes = {
	tokens: Tokens;
	state: State;
	options: ParserOptions;
	meta: void;
};

type MarkupParser = ParserCore<MarkupParserTypes>;

const createStringMarkupParser = createParser<MarkupParserTypes>({
	diagnosticCategory: "parse/stringMarkup",
	getInitialState: () => ({inTagHead: false}),
	tokenizeWithState(parser, index, state) {
		const escaped = isEscaped(index, parser.input);
		const char = parser.getInputCharOnly(index);

		if (!escaped && state.inTagHead) {
			if (char === " ") {
				return parser.lookahead(ob1Inc(index));
			}

			if (char === "=") {
				return {
					state,
					token: parser.finishToken("Equals"),
				};
			}

			if (char === "/") {
				return {
					state,
					token: parser.finishToken("Slash"),
				};
			}

			if (isAlpha(char)) {
				const [value, end] = parser.readInputFrom(index, isAlpha);
				return {
					state,
					token: parser.finishValueToken("Word", value, end),
				};
			}

			if (char === '"') {
				const [value, stringValueEnd, unclosed] = parser.readInputFrom(
					ob1Inc(index),
					isStringValueChar,
				);

				if (unclosed) {
					throw parser.unexpected({
						description: descriptions.STRING_MARKUP.UNCLOSED_STRING,
						start: parser.getPositionFromIndex(stringValueEnd),
					});
				}

				const end = ob1Add(stringValueEnd, 1);
				return {
					state,
					token: parser.finishValueToken(
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
					token: parser.finishToken("Greater"),
				};
			}
		}

		if (isTagStartChar(index, parser.input)) {
			return {
				state: {
					...state,
					inTagHead: true,
				},
				token: parser.finishToken("Less"),
			};
		}

		// Keep eating text until we hit a <
		const [value, end] = parser.readInputFrom(index, isTextChar);
		return {
			state,
			token: {
				type: "Text",
				value: unescapeTextValue(value),
				start: index,
				end,
			},
		};
	},
});

function atTagEnd(parser: MarkupParser): boolean {
	return parser.matchToken("Less") && parser.lookahead().token.type === "Slash";
}

function parseTag(
	parser: MarkupParser,
	headStart: Position,
	parentTagName: undefined | MarkupTagName,
): MarkupParsedTag {
	const nameToken = parser.expectToken("Word");
	const tagName = (nameToken.value as MarkupTagName);

	const allowedAttributes = tags.get(tagName);
	if (allowedAttributes === undefined) {
		throw parser.unexpected({
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
			throw parser.unexpected({
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
			throw parser.unexpected({
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
	while (!parser.matchToken("EOF") && !parser.matchToken("Greater")) {
		const keyToken = parser.getToken();

		let valueToken: TokenValues<Tokens>;
		let key;
		if (keyToken.type === "Word") {
			key = keyToken.value;

			const validator = allowedAttributes.get(key) || globalAttributes.get(key);
			if (validator === undefined) {
				throw parser.unexpected({
					description: descriptions.STRING_MARKUP.INVALID_ATTRIBUTE_NAME_FOR_TAG(
						tagName,
						key,
						[...allowedAttributes.keys(), ...globalAttributes.keys()],
					),
				});
			}

			parser.nextToken();

			let rawValue;

			// Shorthand properties
			if (
				parser.matchToken("Word") ||
				parser.matchToken("Slash") ||
				parser.matchToken("Greater")
			) {
				rawValue = key;
				valueToken = keyToken;
			} else {
				parser.expectToken("Equals");

				valueToken = parser.expectToken("String");
				rawValue = valueToken.value;
			}

			const value = validator(rawValue, key);

			if (value === undefined) {
				throw parser.unexpected({
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
								return parser.getDiagnosticLocation({token: keyToken});

							case "value":
								return parser.getDiagnosticLocation({token: valueToken});

							case "inner-value":
								if (valueToken === keyToken) {
									// Shorthand
									return parser.getDiagnosticLocation({token: keyToken});
								} else {
									// Remove string quotes
									return parser.getDiagnosticLocation({
										startIndex: ob1Inc(keyToken.start),
										endIndex: ob1Dec(valueToken.end),
									});
								}

							case "all":
								return parser.getDiagnosticLocation({
									startIndex: keyToken.start,
									endIndex: valueToken.end,
								});
						}
					},
				},
			).setValue(value);
		} else if (keyToken.type === "Slash") {
			parser.nextToken();
			selfClosing = true;
		} else {
			throw parser.unexpected({
				description: descriptions.STRING_MARKUP.EXPECTED_ATTRIBUTE_NAME,
			});
		}
	}

	parser.expectToken("Greater");

	const headEnd = parser.getPosition();

	// Verify closing tag
	if (!selfClosing) {
		while (
			// Build children
			!parser.matchToken("EOF") &&
			!atTagEnd(parser)
		) {
			const child = parseChild(parser, tagName);
			if (child !== undefined) {
				children.push(child);
			}
		}

		if (parser.matchToken("EOF")) {
			throw parser.unexpected({
				description: descriptions.STRING_MARKUP.UNCLOSED_TAG(
					tagName,
					parser.finishLocAt(headStart, headEnd),
				),
			});
		} else {
			parser.expectToken("Less");
			parser.expectToken("Slash");

			const name = parser.getToken();
			if (name.type === "Word") {
				if (name.value !== tagName) {
					throw parser.unexpected({
						description: descriptions.STRING_MARKUP.INCORRECT_CLOSING_TAG_NAME(
							tagName,
							name.value,
						),
					});
				}

				parser.nextToken();
			} else {
				throw parser.unexpected({
					description: descriptions.STRING_MARKUP.EXPECTED_CLOSING_TAG_NAME,
				});
			}

			parser.expectToken("Greater");
		}
	}

	return {
		type: "Tag",
		attributes,
		name: tagName,
		children,
	};
}

function parseChild(
	parser: MarkupParser,
	parentTagName: undefined | MarkupTagName,
): undefined | MarkupParsedChild {
	const start = parser.getPosition();
	const token = parser.getToken();
	parser.nextToken();

	if (token.type === "Text") {
		// If this tag has restricted children then no text is allowed
		if (parentTagName !== undefined) {
			const onlyAllowedAsParent = tagsToOnlyChildren.get(parentTagName);
			if (onlyAllowedAsParent !== undefined) {
				// Ignore text that's just whitespace
				if (token.value.trim() === "") {
					return undefined;
				} else {
					throw parser.unexpected({
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
		return parseTag(parser, start, parentTagName);
	} else {
		throw parser.unexpected({
			description: descriptions.STRING_MARKUP.UNKNOWN_START,
		});
	}
}

const parseCache: WeakMap<StaticMarkup, MarkupParsedChildren> = new WeakMap();
export function parseMarkup(
	input: string | AnyMarkup,
	opts: ParserOptions = {},
): MarkupParsedChildren {
	let cacheKey: undefined | StaticMarkup;
	let children: undefined | MarkupParsedChildren;

	if (typeof input !== "string") {
		cacheKey = serializeLazyMarkup(input);

		const cached = parseCache.get(cacheKey);
		if (cached !== undefined) {
			return cached;
		}

		// Don't need to parse a single escaped
		if (isSingleEscaped(cacheKey)) {
			children = [
				{
					type: "Text",
					value: cacheKey.parts[0],
					source: true,
				},
			];
		}

		input = readMarkup(input);
	}

	if (children === undefined) {
		children = [];
		const parser = createStringMarkupParser({...opts, input});
		while (!parser.matchToken("EOF")) {
			const child = parseChild(parser, undefined);
			if (child !== undefined) {
				children.push(child);
			}
		}
	}

	if (cacheKey !== undefined) {
		parseCache.set(cacheKey, children);
	}
	return children;
}
