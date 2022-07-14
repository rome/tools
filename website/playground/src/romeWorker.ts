import init, { PlaygroundFormatOptions, run } from "../pkg/rome_playground";
import { IndentStyle, TreeStyle, LoadingState } from "./types";

init()
	.then(() => {
		self.postMessage({ type: "init", loadingState: LoadingState.Success });
	})
	.catch(() => {
		self.postMessage({ type: "init", loadingState: LoadingState.Error });
	});

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
			isJsx,
			sourceType,
			treeStyle,
		} = e.data.playgroundState;
		timeout = setTimeout(() => {
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
			self.postMessage({
				type: "formatted",
				romeOutput: {
					ast: romeOutput.ast,
					cst: romeOutput.cst,
					errors: romeOutput.errors,
					formatted_code: romeOutput.formatted_code,
					formatter_ir: romeOutput.formatter_ir,
				},
			});
		}, 500);
	}
});
