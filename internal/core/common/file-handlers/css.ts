import {PartialExtensionHandler} from "./types";
import {parseCSS} from "@internal/css-parser";

export const cssHandler: PartialExtensionHandler = {
	language: "css",
	hasTabs: true,
	capabilities: {
		lint: false,
		format: true,
	},

	async parse({mtime, path, file, worker}) {
		const sourceText = await worker.readFile(file.real);
		const ast = parseCSS({
			input: sourceText,
			mtime,
			path,
		});
		return {
			sourceText,
			ast,
			astModifiedFromSource: false,
		};
	},
};
