import {PartialExtensionHandler} from "./types";
import {parseMarkdown} from "@internal/markdown-parser";

export const markdownHandler: PartialExtensionHandler = {
	language: "md",
	hasTabs: true,
	capabilities: {
		lint: false,
		format: false,
	},

	async parse({mtime, path, file, worker}) {
		const sourceText = await worker.readFile(file.real);
		const ast = parseMarkdown({
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
