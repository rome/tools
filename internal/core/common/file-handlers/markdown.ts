import {PartialExtensionHandler} from "./types";
import {parseMarkdown} from "@internal/markdown-parser";

export const markdownHandler: PartialExtensionHandler = {
	language: "markdown",
	hasTabs: true,
	mime: "text/markdown",
	capabilities: {
		lint: false,
		format: false,
	},

	async parse({integrity, path, file, worker}) {
		const sourceText = await worker.readFileText(file);
		const ast = parseMarkdown({
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
