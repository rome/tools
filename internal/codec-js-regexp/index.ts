/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	BaseTokens,
	ComplexToken,
	ParserCore,
	ParserOptions,
	Position,
	TokenValues,
	ValueToken,
	createParser,
	isDigit,
	isESIdentifierChar,
	isESIdentifierStart,
} from "@internal/parser-core";
import {
	AnyJSRegExpBodyItem,
	AnyJSRegExpEscapedCharacter,
	AnyJSRegExpExpression,
	JSRegExpAlternation,
	JSRegExpCharSet,
	JSRegExpCharSetRange,
	JSRegExpGroupCapture,
	JSRegExpGroupNonCapture,
	JSRegExpQuantified,
	JSRegExpSubExpression,
} from "@internal/ast";
import {Diagnostics, descriptions} from "@internal/diagnostics";
import {Number0, ob1Add, ob1Get0} from "@internal/ob1";

type Operator =
	| "^"
	| "$"
	| "."
	| "["
	| "]"
	| "("
	| ")"
	| "?"
	| "{"
	| "}"
	| "+"
	| "*"
	| "|";

type Tokens = BaseTokens & {
	Operator: ValueToken<"Operator", Operator>;
	Character: ComplexToken<
		"Character",
		{
			value: string;
			escaped: boolean;
		}
	>;
	EscapedCharacter: ValueToken<
		"EscapedCharacter",
		"d" | "D" | "b" | "B" | "s" | "S" | "w" | "W"
	>;
	NumericBackReferenceCharacter: ComplexToken<
		"NumericBackReferenceCharacter",
		{
			value: number;
			escaped: boolean;
		}
	>;
	NamedBackReferenceCharacter: ComplexToken<
		"NamedBackReferenceCharacter",
		{
			value: string;
			escaped: boolean;
		}
	>;
};

type GroupModifiers =
	| {
			type: "NON_CAPTURE";
			kind: JSRegExpGroupNonCapture["kind"];
		}
	| {
			type: "NAMED_CAPTURE";
			name: string;
		};

type RegExpParserOptions = ParserOptions & {
	unicode: boolean;
};

function isHex(str: string): boolean {
	return !/[^0-9a-fA-F]/.test(str);
}

function isOct(str: string): boolean {
	const OCT_REGEX = /^[0-7]+$/;
	return OCT_REGEX.test(str);
}

function getCodePoint(char: string): number {
	if (char.length === 1) {
		const point = char.codePointAt(0);
		if (point !== undefined) {
			return point;
		}
	}

	throw new Error("Input was not 1 character long");
}

function readOctalCode(
	input: string,
	index: Number0,
	nextChar: string,
): {
	octalValue: number | undefined;
	end: Number0;
} {
	let char = nextChar;
	let octal = "";
	let nextIndex: Number0 = ob1Add(index, 1);
	while (isDigit(char)) {
		octal += char;
		// stop at max octal ascii in case of octal escape
		if (parseInt(octal) > 377) {
			octal = octal.slice(0, octal.length - 1);
			break;
		}
		nextIndex = ob1Add(nextIndex, 1);
		char = input[ob1Get0(nextIndex)];
	}
	if (octal === "") {
		return {octalValue: undefined, end: nextIndex};
	}
	const octalValue = parseInt(octal, 10);
	return {octalValue, end: nextIndex};
}

type RegExpParserTypes = {
	tokens: Tokens;
	state: {};
	options: RegExpParserOptions;
	meta: void;
};
type RegExpParser = ParserCore<RegExpParserTypes>;

const createRegExpParser = createParser<RegExpParserTypes>({
	diagnosticCategory: "parse/regex",

	tokenize(parser, index) {
		const char = parser.getInputCharOnly(index);

		if (char === "\\") {
			let end = ob1Add(index, 2);

			const nextChar = parser.getInputCharOnly(index, 1);
			switch (nextChar) {
				case "t":
					return parser.finishComplexToken(
						"Character",
						{
							escaped: false,
							value: "\t",
						},
						end,
					);

				case "n":
					return parser.finishComplexToken(
						"Character",
						{
							escaped: false,
							value: "\n",
						},
						end,
					);

				case "r":
					return parser.finishComplexToken(
						"Character",
						{
							escaped: false,
							value: "\r",
						},
						end,
					);

				case "v":
					return parser.finishComplexToken(
						"Character",
						{
							escaped: false,
							value: "\x0b",
						},
						end,
					);

				case "f":
					return parser.finishComplexToken(
						"Character",
						{
							escaped: false,
							value: "\f",
						},
						end,
					);

				case "d":
				case "D":
				case "b":
				case "B":
				case "s":
				case "S":
				case "w":
				case "W":
					return parser.finishValueToken("EscapedCharacter", nextChar, end);

				case "k": {
					if (parser.options.unicode) {
						// named group back reference https://github.com/tc39/proposal-regexp-named-groups#backreferences
						let value = "";
						let [char, next] = parser.getInputChar(index, 2);

						if (char === "<") {
							while (!parser.isEOF(next)) {
								value += char;
								[char, next] = parser.getInputChar(index, 1);

								if (char === ">") {
									break;
								}
							}

							return parser.finishComplexToken(
								"NamedBackReferenceCharacter",
								{
									value,
									escaped: true,
								},
								index,
							);
						}
					}

					return parser.finishComplexToken(
						"Character",
						{
							value: "k",
							escaped: true,
						},
						end,
					);
				}

				case "p": {
					if (parser.options.unicode) {
						// TODO unicode property escapes https://github.com/tc39/proposal-regexp-unicode-property-escapes
					}

					return parser.finishComplexToken(
						"Character",
						{
							value: "p",
							escaped: true,
						},
						end,
					);
				}

				case "P": {
					if (parser.options.unicode) {
						// TODO unicode property escapes https://github.com/tc39/proposal-regexp-unicode-property-escapes
					}

					return parser.finishComplexToken(
						"Character",
						{
							value: "P",
							escaped: true,
						},
						end,
					);
				}

				case "c":
					// TODO???
					return parser.finishComplexToken(
						"Character",
						{
							value: "c",
							escaped: true,
						},
						end,
					);

				case "0": {
					const {octalValue, end: octalEnd} = readOctalCode(
						parser.input,
						index,
						nextChar,
					);
					if (octalValue !== undefined && isOct(octalValue.toString())) {
						const octal = parseInt(octalValue.toString(), 8);
						return parser.finishComplexToken(
							"Character",
							{
								value: String.fromCharCode(octal),
								escaped: true,
							},
							octalEnd,
						);
					}
					return parser.finishComplexToken(
						"Character",
						{
							value: String.fromCharCode(0),
							escaped: true,
						},
						end,
					);
				}

				case "x": {
					const [possibleHex] = parser.getInputRange(index, 3, 1);

					// \xhh
					if (possibleHex.length === 2 && isHex(possibleHex)) {
						end = ob1Add(end, 2);

						return parser.finishComplexToken(
							"Character",
							{
								value: String.fromCharCode(parseInt(possibleHex, 16)),
								escaped: true,
							},
							end,
						);
					}

					return parser.finishComplexToken(
						"Character",
						{
							value: "x",
							escaped: true,
						},
						end,
					);
				}

				case "u": {
					// Get the next 4 characters after \u
					const [possibleHex] = parser.getInputRange(index, 4, 2);

					// \uhhhh
					if (possibleHex.length === 4 && isHex(possibleHex)) {
						end = ob1Add(end, 4);

						return parser.finishComplexToken(
							"Character",
							{
								value: String.fromCharCode(parseInt(possibleHex, 16)),
								escaped: true,
							},
							end,
						);
					}

					if (parser.options.unicode) {
						// TODO \u{hhhh} or \u{hhhhh}
					}

					return parser.finishComplexToken(
						"Character",
						{
							value: "u",
							escaped: true,
						},
						end,
					);
				}

				// Redundant escaping
				default: {
					let {
						octalValue: referenceValue,
						end: referenceEnd,
					} = readOctalCode(parser.input, index, nextChar);
					if (referenceValue !== undefined) {
						let backReference = referenceValue.toString();
						// \8 \9 are treated as escape char
						if (referenceValue === 8 || referenceValue === 9) {
							return parser.finishComplexToken(
								"Character",
								{
									value: backReference,
									escaped: true,
								},
								referenceEnd,
							);
						}

						if (isOct(backReference)) {
							const octal = parseInt(backReference, 8);
							return parser.finishComplexToken(
								"Character",
								{
									value: String.fromCharCode(octal),
									escaped: true,
								},
								referenceEnd,
							);
						}

						// back reference allowed are 1 - 99
						if (referenceValue >= 1 && referenceValue <= 99) {
							return parser.finishComplexToken(
								"NumericBackReferenceCharacter",
								{
									value: parseInt(backReference, 10),
									escaped: true,
								},
								referenceEnd,
							);
						} else {
							backReference = backReference.slice(0, backReference.length - 1);
							referenceEnd = ob1Add(referenceEnd, -1);
							if (isOct(backReference)) {
								return parser.finishComplexToken(
									"Character",
									{
										value: String.fromCharCode(parseInt(backReference, 8)),
										escaped: true,
									},
									referenceEnd,
								);
							} else {
								return parser.finishComplexToken(
									"NumericBackReferenceCharacter",
									{
										value: parseInt(backReference, 10),
										escaped: true,
									},
									referenceEnd,
								);
							}
						}
					}

					return parser.finishComplexToken(
						"Character",
						{
							value: nextChar,
							escaped: true,
						},
						end,
					);
				}
			}
		}

		switch (char) {
			case "$":
			case "^":
			case ".":
			case "?":
			case "{":
			case "}":
			case "+":
			case "|":
			case "*":
			case "[":
			case "]":
			case "(":
			case ")":
				return parser.finishValueToken("Operator", char);
		}

		return parser.finishComplexToken(
			"Character",
			{
				value: char,
				escaped: false,
			},
		);
	},
});

function getGroupModifiers(parser: RegExpParser): undefined | GroupModifiers {
	const token = parser.getToken();

	if (token.type === "Character") {
		switch (token.value) {
			case ":": {
				parser.nextToken();
				return {
					type: "NON_CAPTURE",
					kind: undefined,
				};
			}

			case "=": {
				parser.nextToken();
				return {
					type: "NON_CAPTURE",
					kind: "positive-lookahead",
				};
			}

			case "!": {
				parser.nextToken();
				return {
					type: "NON_CAPTURE",
					kind: "negative-lookahead",
				};
			}

			case "<": {
				const nextToken = parser.lookaheadToken();

				if (nextToken.type === "Character") {
					switch (nextToken.value) {
						case "!": {
							parser.nextToken();
							parser.nextToken();
							return {
								type: "NON_CAPTURE",
								kind: "negative-lookbehind",
							};
						}

						case "=": {
							parser.nextToken();
							parser.nextToken();
							return {
								type: "NON_CAPTURE",
								kind: "positive-lookbehind",
							};
						}
					}

					if (isESIdentifierStart(nextToken.value)) {
						let name = "";

						// 1 is for the <
						let skipCount = 1;
						let targetToken: TokenValues<Tokens> = nextToken;
						while (
							targetToken.type === "Character" &&
							isESIdentifierChar(targetToken.value)
						) {
							name += targetToken.value;
							targetToken = parser.lookaheadToken(targetToken.end);
							skipCount++;
						}

						if (targetToken.type === "Character" && targetToken.value === ">") {
							// Skip through all the name tokens including >
							skipCount++;

							// This is kinda a hacky solution, and slower than it could be
							for (let i = 0; i < skipCount; i++) {
								parser.nextToken();
							}

							return {
								type: "NAMED_CAPTURE",
								name,
							};
						}
					}
				}
			}
		}
	}

	parser.unexpectedDiagnostic({
		description: descriptions.REGEX_PARSER.INVALID_CAPTURE_GROUP_MODIFIER,
		token,
	});

	return undefined;
}

function matchOperator(parser: RegExpParser, op: string): boolean {
	const token = parser.getToken();
	return token.type === "Operator" && token.value === op;
}

function eatOperator(parser: RegExpParser, op: string): boolean {
	if (matchOperator(parser, op)) {
		parser.nextToken();
		return true;
	} else {
		return false;
	}
}

function parseGroupCapture(
	parser: RegExpParser,
): JSRegExpGroupCapture | JSRegExpGroupNonCapture {
	const start = parser.getPosition();
	parser.nextToken();

	let modifiers: undefined | GroupModifiers;
	if (eatOperator(parser, "?")) {
		modifiers = getGroupModifiers(parser);
	}

	const expression = parseExpression(parser, () => !matchOperator(parser, ")"));

	if (!eatOperator(parser, ")")) {
		parser.unexpectedDiagnostic({
			description: descriptions.REGEX_PARSER.UNCLOSED_GROUP,
			start,
		});
	}

	if (modifiers !== undefined && modifiers.type === "NON_CAPTURE") {
		return {
			type: "JSRegExpGroupNonCapture",
			expression,
			kind: modifiers.kind,
			loc: parser.finishLoc(start),
		};
	} else {
		let name = modifiers !== undefined ? modifiers.name : undefined;
		return {
			type: "JSRegExpGroupCapture",
			expression,
			name,
			loc: parser.finishLoc(start),
		};
	}
}

function parseCharSet(parser: RegExpParser): JSRegExpCharSet {
	const start = parser.getPosition();
	parser.nextToken();

	const body: JSRegExpCharSet["body"] = [];
	const invert = eatOperator(parser, "^");

	while (!parser.matchToken("EOF") && !matchOperator(parser, "]")) {
		const part = parseCharacterOrRange(parser);
		body.push(part);
	}

	if (!eatOperator(parser, "]")) {
		parser.unexpectedDiagnostic({
			description: descriptions.REGEX_PARSER.UNCLOSED_CHAR_SET,
			start,
		});
	}

	return {
		type: "JSRegExpCharSet",
		invert,
		body,
		loc: parser.finishLoc(start),
	};
}

function getCharacterFromToken(
	parser: RegExpParser,
	token: TokenValues<Tokens>,
): string {
	switch (token.type) {
		case "Character":
		case "Operator":
			return token.value;

		case "SOF":
		case "EOF":
		case "Invalid":
			throw new Error("Unnecessary");

		default:
			throw new Error("Never");
	}
}

function parseCharacter(parser: RegExpParser): AnyJSRegExpEscapedCharacter {
	const token = parser.getToken();

	if (token.type === "Character") {
		parser.nextToken();
		return {
			type: "JSRegExpCharacter",
			value: token.value,
			loc: parser.finishLocFromToken(token),
		};
	}

	if (token.type === "NumericBackReferenceCharacter") {
		parser.nextToken();

		return {
			type: "JSRegExpNumericBackReference",
			value: token.value,
			loc: parser.finishLocFromToken(token),
		};
	}

	if (token.type === "NamedBackReferenceCharacter") {
		const start = parser.input.slice(0, ob1Get0(token.start));
		parser.nextToken();

		if (token.value[token.value.length - 1] !== ">") {
			parser.unexpectedDiagnostic({
				description: descriptions.REGEX_PARSER.UNCLOSED_NAMED_CAPTURE,
				loc: parser.finishLocFromToken(token),
			});
		}

		if (!start.includes(token.value)) {
			parser.unexpectedDiagnostic({
				description: descriptions.REGEX_PARSER.INVALID_NAMED_CAPTURE,
				loc: parser.finishLocFromToken(token),
			});
		}

		const name = token.value.slice(1, token.value.length - 1);
		return {
			type: "JSRegExpNamedBackReference",
			name,
			loc: parser.finishLocFromToken(token),
		};
	}

	if (token.type === "EscapedCharacter") {
		parser.nextToken();

		const loc = parser.finishLocFromToken(token);
		switch (token.value) {
			case "d":
				return {
					type: "JSRegExpDigitCharacter",
					loc,
				};

			case "D":
				return {
					type: "JSRegExpNonDigitCharacter",
					loc,
				};

			case "b":
				return {
					type: "JSRegExpWordBoundaryCharacter",
					loc,
				};

			case "B":
				return {
					type: "JSRegExpNonWordBoundaryCharacter",
					loc,
				};

			case "s":
				return {
					type: "JSRegExpWhiteSpaceCharacter",
					loc,
				};

			case "S":
				return {
					type: "JSRegExpNonWhiteSpaceCharacter",
					loc,
				};

			case "w":
				return {
					type: "JSRegExpWordCharacter",
					loc,
				};

			case "W":
				return {
					type: "JSRegExpNonWordCharacter",
					loc,
				};
		}
	}

	parser.nextToken();
	return {
		type: "JSRegExpCharacter",
		value: getCharacterFromToken(parser, token),
		loc: parser.finishLocFromToken(token),
	};
}

function parseCharacterOrRange(
	parser: RegExpParser,
): AnyJSRegExpEscapedCharacter | JSRegExpCharSetRange {
	const startPos = parser.getPosition();
	let start = parseCharacter(parser);

	// Range
	const nextToken = parser.getToken();
	if (
		start.type === "JSRegExpCharacter" &&
		nextToken.type === "Character" &&
		nextToken.value === "-" &&
		!nextToken.escaped
	) {
		const lookaheadToken = parser.lookaheadToken();
		if (lookaheadToken.type === "Character") {
			// Skip dash
			parser.nextToken();

			let end = parseCharacter(parser);

			const loc = parser.finishLoc(startPos);

			if (
				start.type === "JSRegExpCharacter" &&
				end.type === "JSRegExpCharacter" &&
				getCodePoint(end.value) < getCodePoint(start.value)
			) {
				parser.unexpectedDiagnostic({
					description: descriptions.REGEX_PARSER.REVERSED_CHAR_SET_RANGE,
					loc,
				});
				const _end = end;
				end = start;
				start = _end;
			}

			return {
				type: "JSRegExpCharSetRange",
				loc,
				start,
				end,
			};
		}
	}

	return start;
}

function parseDigits(parser: RegExpParser): undefined | number {
	let digits = "";
	let token = parser.getToken();
	while (token.type === "Character" && isDigit(token.value)) {
		digits += token.value;
		token = parser.nextToken();
	}

	if (digits.length === 0) {
		return undefined;
	} else {
		return Number(digits);
	}
}

function parseQuantifier(
	parser: RegExpParser,
):
	| undefined
	| {
			min: number;
			max?: number;
		} {
	if (eatOperator(parser, "?")) {
		return {
			min: 0,
			max: 1,
		};
	}

	if (eatOperator(parser, "*")) {
		return {
			min: 0,
			max: undefined,
		};
	}

	if (eatOperator(parser, "+")) {
		return {
			min: 1,
			max: undefined,
		};
	}

	if (matchOperator(parser, "{")) {
		const snapshot = parser.save();

		parser.nextToken();

		const start = parser.getPosition();
		const min = parseDigits(parser);

		if (min !== undefined) {
			const nextToken = parser.getToken();
			if (nextToken.type === "Character" && nextToken.value === ",") {
				parser.nextToken();

				const max = parseDigits(parser);
				const end = parser.getPosition();

				const endToken = parser.getToken();
				if (endToken.type === "Operator" && endToken.value === "}") {
					parser.nextToken();

					if (max !== undefined && min > max) {
						parser.unexpectedDiagnostic({
							description: descriptions.REGEX_PARSER.REVERSED_QUANTIFIER_RANGE,
							start,
							end,
						});
						return {
							max: min,
							min: max,
						};
					}

					return {
						min,
						max,
					};
				}
			} else if (nextToken.type === "Operator" && nextToken.value === "}") {
				parser.nextToken();
				return {
					min,
					max: min,
				};
			}
		}

		parser.restore(snapshot);
	}

	return undefined;
}

function parseBodyItem(parser: RegExpParser): undefined | AnyJSRegExpBodyItem {
	const start = parser.getPosition();

	const prefix = parseBodyItemPrefix(parser);
	if (prefix === undefined) {
		return undefined;
	}

	let target = prefix;

	while (true) {
		const quantifier = parseQuantifier(parser);
		if (quantifier === undefined) {
			break;
		}

		const lazy = eatOperator(parser, "?");

		const quantified: JSRegExpQuantified = {
			type: "JSRegExpQuantified",
			loc: parser.finishLoc(start),
			target,
			lazy,
			...quantifier,
		};

		target = quantified;
	}

	return target;
}

function parseOperator(
	parser: RegExpParser,
	token: Tokens["Operator"],
): undefined | AnyJSRegExpBodyItem {
	switch (token.value) {
		case "$": {
			parser.nextToken();
			return {
				type: "JSRegExpEndCharacter",
				loc: parser.finishLocFromToken(token),
			};
		}

		case "^": {
			parser.nextToken();
			return {
				type: "JSRegExpStartCharacter",
				loc: parser.finishLocFromToken(token),
			};
		}

		case ".": {
			parser.nextToken();
			return {
				type: "JSRegExpAnyCharacter",
				loc: parser.finishLocFromToken(token),
			};
		}

		case "[":
			return parseCharSet(parser);

		case "(":
			return parseGroupCapture(parser);

		case ")": {
			parser.nextToken();
			parser.unexpectedDiagnostic({
				description: descriptions.REGEX_PARSER.UNOPENED_GROUP,
				token,
			});
			return;
		}

		case "{": {
			const start = parser.getPosition();
			const unmatchedQuantifier = parseQuantifier(parser);
			if (unmatchedQuantifier === undefined) {
				// Quantifier is undefined and eaten tokens were restored
				// Return a '{' token as a RegexpCharacter, parseBodyItem() will handle parsing of subsequent quantifiers
				return parseCharacter(parser);
			} else {
				// If quantifier is defined, then syntax error: Nothing to repeat
				const end = parser.getPosition();
				parser.unexpectedDiagnostic({
					description: descriptions.REGEX_PARSER.NO_TARGET_QUANTIFIER,
					start,
					end,
				});
				return;
			}
		}

		case "?":
		case "*":
		case "+": {
			parser.nextToken();
			parser.unexpectedDiagnostic({
				description: descriptions.REGEX_PARSER.INVALID_QUANTIFIER_TARGET,
				token,
			});
			return;
		}

		case "]":
		case "}":
			return parseCharacter(parser);

		default:
			return undefined;
	}
}

function parseBodyItemPrefix(
	parser: RegExpParser,
): undefined | AnyJSRegExpBodyItem {
	const token = parser.getToken();

	switch (token.type) {
		case "Operator":
			return parseOperator(parser, token);

		case "EscapedCharacter":
		case "Character":
		case "NumericBackReferenceCharacter":
		case "NamedBackReferenceCharacter":
			return parseCharacter(parser);
	}

	parser.unexpectedDiagnostic({
		description: descriptions.REGEX_PARSER.UNKNOWN_REGEX_PART,
		token,
	});

	return undefined;
}

function parseExpression(
	parser: RegExpParser,
	whileCallback?: () => boolean,
): JSRegExpSubExpression | JSRegExpAlternation {
	const alternations: Array<{
		start: Position;
		end: Position;
		body: Array<AnyJSRegExpBodyItem>;
	}> = [];
	let body: Array<AnyJSRegExpBodyItem> = [];

	const start = parser.getPosition();
	let alternateStart = start;

	while (
		!parser.matchToken("EOF") &&
		(whileCallback === undefined || whileCallback())
	) {
		if (eatOperator(parser, "|")) {
			alternations.push({
				start: alternateStart,
				end: parser.getPosition(),
				body,
			});
			alternateStart = parser.getPosition();
			body = [];
			continue;
		}

		const part = parseBodyItem(parser);
		if (part !== undefined) {
			body.push(part);
		}
	}

	alternations.push({
		body,
		start: alternateStart,
		end: parser.getPosition(),
	});

	let expression: undefined | JSRegExpSubExpression | JSRegExpAlternation;

	while (alternations.length > 0) {
		const alternation = alternations.shift()!;

		const sub: JSRegExpSubExpression = {
			type: "JSRegExpSubExpression",
			body: alternation.body,
			loc: parser.finishLocAt(alternation.start, alternation.end),
		};

		if (expression === undefined) {
			expression = sub;
		} else {
			const alternationNode: JSRegExpAlternation = {
				type: "JSRegExpAlternation",
				left: expression,
				right: sub,
				loc: parser.finishLocAt(
					parser.getLoc(expression).start,
					alternation.end,
				),
			};

			expression = alternationNode;
		}
	}

	if (expression === undefined) {
		throw new Error(
			"Impossible. We should always have at least one alternation that will set parser.",
		);
	}

	return expression;
}

export function parseRegExp(
	opts: RegExpParserOptions,
): {
	expression: AnyJSRegExpExpression;
	diagnostics: Diagnostics;
} {
	const parser = createRegExpParser(opts);
	return {
		expression: parseExpression(parser),
		diagnostics: parser.getDiagnostics(),
	};
}
