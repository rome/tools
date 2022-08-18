interface FormatFilesDebugOptions extends FormatFilesOptions {
	/**
	 * If `true`, you'll be able to inspect the IR of the formatter
	 */
	debug: boolean;
}

interface FormatContentDebugOptions extends FormatContentOptions {
	/**
	 * If `true`, you'll be able to inspect the IR of the formatter
	 */
	debug: boolean;
}

interface FormatFilesOptions {
	/**
	 * Writes the new content to disk
	 */
	write?: boolean;
}

interface FormatContentOptions {
	/**
	 * A virtual path of the file. You should add the extension,
	 * so Rome knows how to parse the content
	 */
	filePath: string;
	/**
	 * The range where to format the content
	 */
	range?: [number, number];
}

interface FormatResult {
	// Not final
	content: string;
	// Not final
	errors: string[];
}

interface FormatDebugResult {
	// Not final
	content: string;
	// Not final
	errors: string[];
	// Available when in debug mode
	ir: string | null;
}

interface ParseOptions {
	/**
	 * A virtual path of the file. You should add the extension,
	 * so Rome knows how to parse the content
	 */
	filePath: string;
}

interface ParseResult {
	/**
	 * The CST of the code
	 */
	cst: string;
	/**
	 * The AST of the code
	 */
	ast: string;
	// Not final
	errors: string[];
}

function isFormatFilesDebug(
	options: FormatFilesOptions | FormatFilesDebugOptions,
): options is FormatFilesDebugOptions {
	return "debug" in options && options.debug !== undefined;
}

function isFormatContentDebug(
	options: any,
): options is FormatContentDebugOptions {
	return options?.debug !== undefined;
}

export class Rome {
	async formatFiles(paths: string[]): Promise<FormatResult>;
	async formatFiles(
		paths: string[],
		options?: FormatFilesOptions,
	): Promise<FormatResult>;
	async formatFiles(
		paths: string[],
		options?: FormatFilesDebugOptions,
	): Promise<FormatDebugResult>;
	async formatFiles(
		paths: string[],
		options?: FormatFilesOptions | FormatFilesDebugOptions,
	): Promise<FormatResult | FormatDebugResult> {
		paths;

		if (options && isFormatFilesDebug(options)) {
			return {
				content: "",
				errors: [],
				ir: "",
			};
		}
		return {
			content: "",
			errors: [],
		};
	}

	async formatContent(
		content: string,
		options: FormatContentOptions,
	): Promise<FormatResult>;
	async formatContent(
		content: string,
		options: FormatContentDebugOptions,
	): Promise<FormatDebugResult>;
	async formatContent(
		content: string,
		options: FormatContentOptions | FormatContentDebugOptions,
	): Promise<FormatResult | FormatDebugResult> {
		content;
		options.filePath;
		if (isFormatContentDebug(options)) {
			return {
				content: "",
				errors: [],
				ir: "",
			};
		}
		return {
			content: "",
			errors: [],
		};
	}

	async parseContent(
		content: string,
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
