/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {tokenizeJS} from "@romejs/js-parser";
import {Number0, ob1Get0} from "@romejs/ob1";
import {DiagnosticLanguage, DiagnosticSourceType} from "@romejs/diagnostics";
import {ConstSourceType} from "@romejs/ast";
import {tokenizeJSON} from "@romejs/codec-json";
import {UnknownFilePath, createUnknownFilePath} from "@romejs/path";
import {escapeMarkup, markupTag} from "@romejs/string-markup";
import {MarkupTokenType} from "@romejs/string-markup/types";

// 100KB
const FILE_SIZE_MAX = 100_000;

export type AnsiHighlightOptions = {
	path: UnknownFilePath;
	input: string;
	sourceType: undefined | DiagnosticSourceType;
	language: undefined | DiagnosticLanguage;
};

export default function highlightCode(opts: AnsiHighlightOptions): string {
	if (opts.input.length > FILE_SIZE_MAX) {
		return escapeMarkup(opts.input);
	}

	if (opts.language === "js") {
		// js-parser does not accept an "unknown" sourceType
		return highlightJS(
			opts.input,
			opts.sourceType === undefined || opts.sourceType === "unknown"
				? "script"
				: opts.sourceType,
		);
	}

	if (opts.language === "json") {
		return highlightJSON(opts.path, opts.input);
	}

	return escapeMarkup(opts.input);
}

function reduce<Token extends {
	start: Number0;
	end: Number0;
}>(
	input: string,
	tokens: Array<Token>,
	callback: (
		token: Token,
		line: string,
		prev: undefined | Token,
		next: undefined | Token,
	) => string,
): string {
	let prevEnd = 0;
	let buff = "";

	for (let i = 0; i < tokens.length; i++) {
		const token = tokens[i];
		const start = ob1Get0(token.start);
		const end = ob1Get0(token.end);
		let value = input.slice(start, end);

		// Add on text between tokens
		buff += escapeMarkup(input.slice(prevEnd, start));
		prevEnd = end;

		// We need to break up the token text into lines, so that we can easily split the highlighted newlines and have the ansi codes be unbroken
		const lines = value.split("\n");

		const values: Array<string> = lines.map((line) => {
			return line === ""
				? ""
				: callback(token, escapeMarkup(line), tokens[i - 1], tokens[i + 1]);
		});

		buff += values.join("\n");
	}

	return buff;
}

function invalidHighlight(line: string): string {
	return markupTag("emphasis", markupTag("color", line, {bg: "red"}));
}

function highlightJSON(path: UnknownFilePath, input: string): string {
	const tokens = tokenizeJSON({
		input,
		// Wont be used anywhere but activates JSON extensions if necessary
		path,
	});

	return reduce(
		input,
		tokens,
		(token, value) => {
			// Try to keep the highlighting in line with JS where possible
			switch (token.type) {
				case "BlockComment":
				case "LineComment":
					return markupTag("token", value, {type: "comment"});

				case "String":
					return markupTag("token", value, {type: "string"});

				case "Number":
					return markupTag("token", value, {type: "number"});

				case "Word":
					switch (token.value) {
						case "true":
						case "false":
						case "null":
							return markupTag("token", value, {type: "boolean"});

						default:
							return value;
					}

				case "Comma":
				case "Colon":
				case "Dot":
					return markupTag("token", value, {type: "operator"});

				case "BracketOpen":
				case "BracketClose":
				case "BraceOpen":
				case "BraceClose":
				case "Minus":
				case "Plus":
					return value;

				case "Invalid":
					return invalidHighlight(value);

				// Will never be hit
				case "EOF":
				case "SOF":
					return "";
			}
		},
	);
}

function highlightJS(input: string, sourceType: ConstSourceType): string {
	const tokens = tokenizeJS(
		input,
		{
			sourceType,
			// js-parser requires a filename. Doesn't really matter since we'll never be producing an AST or diagnostics
			path: createUnknownFilePath("unknown"),
		},
	);

	return reduce(
		input,
		tokens,
		(token, value, prev) => {
			const {type} = token;

			let tokenType: MarkupTokenType;

			switch (type.label) {
				case "break":
				case "case":
				case "catch":
				case "continue":
				case "debugger":
				case "default":
				case "do":
				case "else":
				case "finally":
				case "for":
				case "function":
				case "if":
				case "return":
				case "switch":
				case "throw":
				case "try":
				case "var":
				case "const":
				case "while":
				case "with":
				case "new":
				case "this":
				case "super":
				case "class":
				case "extends":
				case "export":
				case "import":
				case "null":
				case "true":
				case "false":
				case "in":
				case "instanceof":
				case "typeof":
				case "void":
				case "delete": {
					tokenType = "keyword";
					break;
				}

				case "num":
				case "bigint": {
					tokenType = "number";
					break;
				}

				case "regexp": {
					tokenType = "regex";
					break;
				}

				case "string":
				case "template":
				case "`": {
					tokenType = "string";
					break;
				}

				case "invalid":
					return invalidHighlight(value);

				case "comment": {
					tokenType = "comment";
					break;
				}

				case ",":
				case ";":
				case ":":
				case "::":
				case "${":
				case ".":
				case "?":
				case "?.":
				case "[":
				case "]":
				case "{":
				case "{|":
				case "}":
				case "|}":
				case "(":
				case ")": {
					tokenType = "punctuation";
					break;
				}

				case "name": {
					tokenType = value === "from" ? "keyword" : "variable";
					break;
				}

				case "jsxName": {
					tokenType =
						prev !== undefined &&
						(prev.type.label === "jsxTagStart" || prev.type.label === "/")
							? "variable"
							: "attr-name";
					break;
				}

				case "=>":
				case "...":
				case "@":
				case "#":
				case "=":
				case "_=":
				case "++/--":
				case "!":
				case "~":
				case "??":
				case "||":
				case "&&":
				case "|":
				case "^":
				case "&":
				case "==/!=":
				case "</>":
				case "<</>>":
				case "+/-":
				case "%":
				case "*":
				case "/":
				case "**": {
					tokenType = "operator";
					break;
				}

				case "jsxText":
				case "jsxTagStart":
				case "jsxTagEnd":
				case "eof":
					return value;
			}

			return markupTag("token", value, {type: tokenType});
		},
	);
}
