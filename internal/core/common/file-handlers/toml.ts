import {ExtensionHandler} from "./types";
import {parseToml} from "@internal/toml-parser";

export const markdownHandler: ExtensionHandler = {
	ext: "toml",
	language: "toml",
	hasTabs: true,
	capabilities: {
		lint: false,
		format: false,
	},

	async parse({mtime, path, file, worker}) {
		const sourceText = await worker.readFile(file.real);
		const ast = parseToml({
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
