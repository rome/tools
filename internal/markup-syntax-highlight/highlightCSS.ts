import {
	AnsiHighlightOptions,
	HighlightCodeResult,
} from "@internal/markup-syntax-highlight/types";
import {tokenizeCSS} from "@internal/css-parser";
import {reduceParserCore} from "@internal/markup-syntax-highlight/utils";

export default function highlightCSS(
	{input, path}: AnsiHighlightOptions,
): HighlightCodeResult {
	const tokens = tokenizeCSS({
		input,
		path,
	});

	return reduceParserCore(
		input,
		tokens,
		(token, value, prev, next) => {
			switch (token.type) {
				case "AtKeyword": {
					return {
						type: "atrule",
					};
				}
				case "LeftParen":
				case "LeftCurlyBracket":
				case "LeftSquareBracket":
				case "RightParen":
				case "RightCurlyBracket":
				case "RightSquareBracket":
				case "Colon":
				case "Semi":
				case "Comma": {
					return {
						type: "punctuation",
					};
				}
				case "Ident": {
					return {
						type: next?.type === "Colon" ? "property" : "string",
					};
				}
				case "Function": {
					return {
						type: "function",
					};
				}

				case "Comment": {
					return {
						type: "comment",
					};
				}
				case "Number": {
					return {
						type: "number",
					};
				}
				default:
					return undefined;
			}
		},
	);
}
