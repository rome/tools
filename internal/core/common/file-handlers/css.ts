import {PartialExtensionHandler} from "./types";
import {parseCSS} from "@internal/css-parser";

export const cssHandler: PartialExtensionHandler = {
	language: "css",
	hasTabs: true,
	mime: "text/css",
	capabilities: {
		lint: false,
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
};
