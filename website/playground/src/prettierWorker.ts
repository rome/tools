import { formatWithPrettier } from "./utils";

self.addEventListener("message", (e) => {
	switch (e.data.type) {
		case "format": {
			const {
				code,
				lineWidth,
				indentStyle,
				indentWidth,
				quoteStyle,
				isTypeScript,
			} = e.data.playgroundState;
			const prettierOutput = formatWithPrettier(code, {
				lineWidth,
				indentStyle,
				indentWidth,
				language: isTypeScript ? "ts" : "js",
				quoteStyle,
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
