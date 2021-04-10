import {PartialExtensionHandler} from "./types";
import {parseCSS, tokenizeCSS} from "@internal/css-parser";

export const cssHandler: PartialExtensionHandler = {
	language: "css",
	hasTabs: true,
	mime: "text/css",
	capabilities: {
		lint: true,
		format: true,
	},

	async parse({integrity, path, file, worker}) {
		const sourceText = await worker.readFileText(file);
		const ast = parseCSS({
			input: sourceText,
			integrity,
			path,
		});
		return {
			sourceText,
			ast,
			astModifiedFromSource: false,
		};
	},

	async tokenize({integrity, path, file, worker}) {
		const sourceText = await worker.readFileText(file);
		const tokens = tokenizeCSS({
			input: sourceText,
			integrity,
			path,
		});
		return {
			sourceText,
			tokens,
		};
	},
};
