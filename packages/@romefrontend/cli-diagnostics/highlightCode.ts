/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// NB: Maybe this belongs in a dedicated package?
import {tokenizeJS} from "@romefrontend/js-parser";
import {Number0, ob1Get0} from "@romefrontend/ob1";
import {
	DiagnosticLanguage,
	DiagnosticSourceType,
} from "@romefrontend/diagnostics";
import {ConstJSSourceType} from "@romefrontend/ast";
import {tokenizeJSON} from "@romefrontend/codec-json";
import {UnknownFilePath} from "@romefrontend/path";
import {
	Markup,
	convertToMarkupFromRandomString,
	concatMarkup,
	markup,
	markupTag,
} from "@romefrontend/cli-layout";
import {MarkupTokenType} from "@romefrontend/cli-layout/types";
import {tokenizeHTML} from "@romefrontend/html-parser";
import {splitLines} from "./utils";

// Max file size to avoid doing expensive highlighting for massive files - 100KB
// NB: This should probably be lower
const FILE_SIZE_MAX = 100_000;

export type AnsiHighlightOptions = {
	path: UnknownFilePath;
	input: string;
	sourceTypeJS: undefined | DiagnosticSourceType;
	language: DiagnosticLanguage;
	highlight: boolean;
};

type TokenShape = {
	start: Number0;
	end: Number0;
};

type ReduceCallbackResult = {
	type?: MarkupTokenType;
	value?: Markup;
};

type ReduceCallback<Token extends TokenShape> = (
	token: Token,
	line: Markup,
	prev: undefined | Token,
	next: undefined | Token,
) => undefined | ReduceCallbackResult;

type HighlightCodeResult = Array<Markup>;

export default function highlightCode(
	opts: AnsiHighlightOptions,
): HighlightCodeResult {
	if (opts.input.length < FILE_SIZE_MAX && opts.highlight) {
		switch (opts.language) {
			case "js":
				return highlightJS(
					opts,
					// js-parser does not accept an "unknown" sourceType
					opts.sourceTypeJS === undefined || opts.sourceTypeJS === "unknown"
						? "script"
						: opts.sourceTypeJS,
				);

			case "html":
				return highlightHTML(opts);

			case "json":
				return highlightJSON(opts);
		}
	}

	return splitLines(opts.input).map((line) => markup`${line}`);
}

function reduceParserCore<Token extends TokenShape & {
	type: string;
}>(
	input: string,
	tokens: Array<Token>,
	callback: ReduceCallback<Token>,
): HighlightCodeResult {
	return reduce(
		input,
		tokens,
		(token, value, prev, next) => {
			switch (token.type) {
				case "Invalid":
					return invalidHighlight(value);

				// Will never be hit
				case "EOF":
				case "SOF":
					return {value: markup``};

				default:
					// We should have refined `token` to not include any of the base tokens
					return callback(token, value, prev, next);
			}
		},
	);
}

function reduce<Token extends TokenShape>(
	input: string,
	tokens: Array<Token>,
	callback: ReduceCallback<Token>,
): HighlightCodeResult {
	let prevEnd = 0;
	let parts: Array<Markup> = [];

	for (let i = 0; i < tokens.length; i++) {
		const token = tokens[i];
		const start = ob1Get0(token.start);
		const end = ob1Get0(token.end);
		let value = input.slice(start, end);

		// Add on text between tokens
		parts.push(markup`${input.slice(prevEnd, start)}`);
		prevEnd = end;

		// Print this token
		// We need to break up the token text into lines, so that we can easily split the highlighted newlines and have the ansi codes be unbroken
		const lines = splitLines(value);
		for (let i = 0; i < lines.length; i++) {
			const line = lines[i];

			if (line !== "") {
				const prev = tokens[i - 1];
				const next = tokens[i + 1];
				const escapedLine = markup`${line}`;
				const res = callback(token, escapedLine, prev, next);
				if (res === undefined) {
					parts.push(escapedLine);
				} else {
					const {value = escapedLine, type} = res;
					if (type === undefined) {
						parts.push(value);
					} else {
						parts.push(markupTag("token", value, {type}));
					}
				}
			}

			// Last element isn't a line break
			const isLast = i === lines.length - 1;
			if (!isLast) {
				parts.push(markup`\n`);
			}
		}
	}

	return splitLines(concatMarkup(parts).value).map((line) =>
		convertToMarkupFromRandomString(line)
	);
}

function invalidHighlight(line: Markup): ReduceCallbackResult {
	return {
		value: markupTag("emphasis", markupTag("color", line, {bg: "red"})),
	};
}

function highlightJSON({input, path}: AnsiHighlightOptions): HighlightCodeResult {
	const tokens = tokenizeJSON({
		input,
		// Wont be used anywhere but activates JSON extensions if necessary
		path,
	});

	return reduceParserCore(
		input,
		tokens,
		(token) => {
			// Try to keep the highlighting in line with JS where possible
			switch (token.type) {
				case "BlockComment":
				case "LineComment":
					return {type: "comment"};

				case "String":
					return {type: "string"};

				case "Number":
					return {type: "number"};

				case "Word":
					switch (token.value) {
						case "true":
						case "false":
						case "null":
							return {type: "boolean"};

						default:
							return undefined;
					}

				case "Comma":
				case "Colon":
				case "Dot":
					return {type: "operator"};

				case "BracketOpen":
				case "BracketClose":
				case "BraceOpen":
				case "BraceClose":
				case "Minus":
				case "Plus":
					return {type: "punctuation"};

				default:
					return undefined;
			}
		},
	);
}

function highlightHTML({input, path}: AnsiHighlightOptions): HighlightCodeResult {
	const tokens = tokenizeHTML({
		input,
		path,
	});

	return reduceParserCore(
		input,
		tokens,
		(token, value, prev) => {
			// All these tokens appear only inside of tags
			switch (token.type) {
				case "Equals":
					return {type: "attr-equals"};

				case "Identifier":
					return {
						type: prev !== undefined && prev.type === "TagStartOpen"
							? "tag"
							: "attr-name",
					};

				case "String":
					return {type: "attr-value"};

				case "TagEndOpen":
				case "TagEnd":
				case "TagSelfClosing":
				case "TagStartOpen":
					return {type: "punctuation"};

				default:
					return undefined;
			}
		},
	);
}

function highlightJS(
	{input, path}: AnsiHighlightOptions,
	sourceType: ConstJSSourceType,
): HighlightCodeResult {
	const tokens = tokenizeJS({
		input,
		sourceType,
		path,
	});

	return reduce(
		input,
		tokens,
		(token, value, prev, next) => {
			const {type} = token;

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
				case "in":
				case "instanceof":
				case "typeof":
				case "void":
				case "delete":
					return {type: "keyword"};

				case "true":
				case "false":
				case "null":
					return {type: "boolean"};

				case "num":
				case "bigint":
					return {type: "number"};

				case "regexp":
					return {type: "regex"};

				case "string":
				case "template":
				case "`":
					return {type: "string"};

				case "invalid":
					return invalidHighlight(value);

				case "comment":
					return {type: "comment"};

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
				case ")":
					return {type: "punctuation"};

				case "name": {
					if (next !== undefined && next.type.label === "(") {
						return {type: "function"};
					}

					// These are contextual keywords
					const word = value.value;
					if (
						word === "from" ||
						word === "let" ||
						word === "async" ||
						word === "await"
					) {
						return {type: "keyword"};
					}

					return {type: "variable"};
				}

				case "jsxName":
					return {
						type: prev !== undefined &&
						(prev.type.label === "jsxTagStart" || prev.type.label === "/")
							? "variable"
							: "attr-name",
					};

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
				case "**":
					return {type: "operator"};

				default:
					return undefined;
			}
		},
	);
}
