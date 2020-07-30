import {reduceParserCore} from "./utils";
import {tokenizeJSON} from "@internal/codec-json";
import {AnsiHighlightOptions, HighlightCodeResult} from "./types";

export default function highlightJSON(
	{input, path}: AnsiHighlightOptions,
): HighlightCodeResult {
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
