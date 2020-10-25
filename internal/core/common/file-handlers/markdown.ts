import {ExtensionHandler} from "./types";
import {parseMarkdown} from "@internal/markdown-parser";

export const markdownHandler: ExtensionHandler = {
	ext: "md",
	language: "md",
	hasTabs: true,
	capabilities: {
		lint: false,
		format: true,
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
