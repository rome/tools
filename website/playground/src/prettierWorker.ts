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
		console.log("Prettier Format Request");

		timeout = setTimeout(() => {
			console.time("prettier");
			const prettierOutput = formatWithPrettier(code, {
				lineWidth,
				indentStyle,
				indentWidth,
				language: isTypeScript ? "ts" : "js",
				quoteStyle,
			});
			console.timeEnd("prettier");
			self.postMessage({
				type: "formatted",
				prettierOutput,
			});
		}, 500);
	}
});
