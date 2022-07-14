import { formatWithPrettier } from "./utils";

let timeout: number;
self.addEventListener("message", (e) => {
	clearTimeout(timeout);

	if (e.data.type === "format") {
		const {
			code,
			lineWidth,
			indentStyle,
			indentWidth,
			quoteStyle,
			isTypeScript,
		} = e.data.playgroundState;
		timeout = setTimeout(() => {
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
		}, 500);
	}
});
