import init, { PlaygroundFormatOptions, run } from "../pkg/rome_playground";
import { IndentStyle, LoadingState } from "./types";

self.addEventListener("message", async (e) => {
	switch (e.data.type) {
		case "init": {
			try {
				await init();
				self.postMessage({ type: "init", loadingState: LoadingState.Success });
			} catch {
				self.postMessage({ type: "init", loadingState: LoadingState.Error });
			}

			break;
		}

		case "format": {
			const {
				code,
				lineWidth,
				indentStyle,
				indentWidth,
				quoteStyle,
				isTypeScript,
				isJsx,
				sourceType,
				cursorPosition,
			} = e.data.playgroundState;
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
				cursorPosition,
			);
			self.postMessage({
				type: "formatted",
				romeOutput: {
					ast: romeOutput.ast,
					cst: romeOutput.cst,
					errors: romeOutput.errors,
					formatted_code: romeOutput.formatted_code,
					formatter_ir: romeOutput.formatter_ir,
					control_flow_graph: romeOutput.control_flow_graph,
				},
			});

			break;
		}

		default:
			console.error(`Unknown message '${e.data.type}'.`);
	}
});
