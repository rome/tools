import {tokenizeJS} from "@romefrontend/js-parser";
import {AnsiHighlightOptions, HighlightCodeResult} from "./types";
import {ConstJSSourceType} from "@romefrontend/ast";
import {invalidHighlight, reduce} from "./utils";
import {readMarkup} from "@romefrontend/markup/escape";

export default function highlightJS(
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
					const word = readMarkup(value);
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
