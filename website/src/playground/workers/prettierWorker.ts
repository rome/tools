import { formatWithPrettier, isTypeScriptFilename } from "../utils";
import { defaultPlaygroundState, PlaygroundSettings } from "../types";

let settings = defaultPlaygroundState.settings;

self.addEventListener("message", (e) => {
	switch (e.data.type) {
		case "updateSettings": {
			settings = e.data.settings as PlaygroundSettings;
			break;
		}

		case "format": {
			const {
				lineWidth,
				indentStyle,
				indentWidth,
				quoteStyle,
				quoteProperties,
				trailingComma,
			} = settings;
			const code = e.data.code as string;
			const filename = e.data.filename as string;

			const prettierOutput = formatWithPrettier(code, {
				lineWidth,
				indentStyle,
				indentWidth,
				language: isTypeScriptFilename(filename) ? "ts" : "js",
				quoteStyle,
				quoteProperties,
				trailingComma,
			});

			self.postMessage({
				type: "formatted",
				filename,
				prettierOutput,
			});

			break;
		}

		default:
			console.error(`Unknown message ${e.data.type}.`);
	}
});
