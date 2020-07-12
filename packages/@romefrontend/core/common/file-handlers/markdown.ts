import {ExtensionHandler} from "./types";
import {parseMarkdown} from "@romefrontend/markdown-parser";

export const markdownHandler: ExtensionHandler = {
	ext: "md",
	canLint: true,
	canFormat: true,

	async parse({stat, path, file, worker}) {
		const sourceText = await worker.readFile(file.real);
		const ast = parseMarkdown({
			input: sourceText,
			mtime: stat.mtimeMs,
			path,
		});
		return {
			sourceText,
			ast,
			astModifiedFromSource: false,
		};
	},
};
