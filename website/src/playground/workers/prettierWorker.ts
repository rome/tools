import {
	formatWithPrettier,
	isTypeScriptFilename,
	isJSONFilename,
} from "../utils";
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
				semicolons,
			} = settings;
			const code = e.data.code as string;
			const filename = e.data.filename as string;

			const prettierOutput = formatWithPrettier(code, {
				lineWidth,
				indentStyle,
				indentWidth,
				filepath: filename,
				quoteStyle,
				quoteProperties,
				trailingComma,
				semicolons,
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
