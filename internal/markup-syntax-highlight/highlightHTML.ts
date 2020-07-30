import {reduceParserCore} from "./utils";
import {tokenizeHTML} from "@internal/html-parser";
import {AnsiHighlightOptions, HighlightCodeResult} from "./types";

export default function highlightHTML(
	{input, path}: AnsiHighlightOptions,
): HighlightCodeResult {
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
