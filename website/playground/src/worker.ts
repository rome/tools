import init, { PlaygroundFormatOptions, run } from "../pkg/rome_playground";
import { formatWithPrettier } from "./utils";
import { IndentStyle, TreeStyle, LoadingState } from "./types";

init()
	.then(() => {
		self.postMessage({ type: "init", loadingState: LoadingState.Success });
	})
	.catch(() => {
		self.postMessage({ type: "init", loadingState: LoadingState.Error });
	});

self.addEventListener("message", (e) => {
	if (e.data.type === "format") {
		const {
			code,
			lineWidth,
			indentStyle,
			indentWidth,
			quoteStyle,
			isTypeScript,
			isJsx,
			sourceType,
			treeStyle,
		} = e.data.playgroundState;

		console.time("rome");
		const romeOutput = run(
			code,
			new PlaygroundFormatOptions(
				lineWidth,
				indentStyle === IndentStyle.Space ? indentWidth : undefined,
				quoteStyle,
			),
			isTypeScript,
			isJsx,
			sourceType,
			treeStyle === TreeStyle.Json,
		);
		console.timeEnd("rome");
		const prettierOutput = formatWithPrettier(code, {
			lineWidth,
			indentStyle,
			indentWidth,
			language: isTypeScript ? "ts" : "js",
			quoteStyle,
		});

		self.postMessage({
			type: "formatted",
			romeOutput: {
				ast: romeOutput.ast,
				cst: romeOutput.cst,
				errors: romeOutput.errors,
				formatted_code: romeOutput.formatted_code,
				formatter_ir: romeOutput.formatter_ir,
			},
			prettierOutput,
		});
	}
});
