interface FormatDebugOptions {
	/**
	 * If `true`, you'll be able to inspect the IR of the formatter
	 */
	debug: boolean;
}

interface FormatFilesOptions extends FormatDebugOptions {
	/**
	 * Writes the new content to disk
	 */
	write: boolean;
}

interface FormatContentOptions extends FormatDebugOptions {
	/**
	 * A virtual path of the file. You should add the extension,
	 * so Rome knows how to parse the content
	 */
	filePath: String;
	/**
	 * The range where to format the content
	 */
	range: [number, number];
}

interface FormatResult {
	// Not final
	content: String;
	// Not final
	errors: String[];
	// Available when in debug mode
	ir: String | null;
}

interface ParseOptions {
	/**
	 * A virtual path of the file. You should add the extension,
	 * so Rome knows how to parse the content
	 */
	filePath: String;
}

interface ParseResult {
	/**
	 * The CST of the code
	 */
	cst: String;
	/**
	 * The AST of the code
	 */
	ast: String;
	// Not final
	errors: String[];
}

export class Rome {
	async formatFiles(
		paths: String[],
		options: Partial<FormatFilesOptions> | undefined = undefined,
	): Promise<FormatResult> {
		paths;

		return {
			content: "",
			errors: [],
			ir: options?.debug ? "" : null,
		};
	}

	async formatContent(
		content: String,
		options: Partial<FormatContentOptions> | undefined = undefined,
	): Promise<FormatResult> {
		content;
		return {
			content: "",
			errors: [],
			ir: options?.debug ? "" : null,
		};
	}

	async parseContent(
		content: String,
		options: ParseOptions,
	): Promise<ParseResult> {
		content;
		options;
		return {
			ast: "",
			cst: "",
			errors: [],
		};
	}
}
