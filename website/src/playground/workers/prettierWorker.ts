import { formatWithPrettier } from "../utils";
import type { PlaygroundState } from "../types";

self.addEventListener("message", (e) => {
	switch (e.data.type) {
		case "format": {
			const {
				code,
				lineWidth,
				indentStyle,
				indentWidth,
				quoteStyle,
				quoteProperties,
				typescript: isTypeScript,
				trailingComma,
			} = e.data.playgroundState as PlaygroundState;
			const prettierOutput = formatWithPrettier(code, {
				lineWidth,
				indentStyle,
				indentWidth,
				language: isTypeScript ? "ts" : "js",
				quoteStyle,
				quoteProperties,
				trailingComma,
			});

			self.postMessage({
				type: "formatted",
				prettierOutput,
			});

			break;
		}

		default:
			console.error(`Unknown message ${e.data.type}.`);
	}
});
