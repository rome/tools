import { main, Workspace, RomePath } from "@rometools/wasm-nodejs";

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
	/**
	 * The new formatted content
	 */
	content: string;
	// Not final
	errors: string[];
}

interface FormatDebugResult {
	/**
	 * The new formatted content
	 */
	content: string;
	// Not final
	errors: string[];
	/**
	 * The IR emitted by the formatter
	 */
	ir: string;
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

interface CurrentFile {
	version: number;
	path: RomePath;
}

export class Rome {
	private workspace: Workspace;

	private constructor(workspace: Workspace) {
		this.workspace = workspace;
	}

	/**
	 * It creates a new instance of the class {Rome}
	 */
	public static async create(): Promise<Rome> {
		return new Rome(await Rome.loadWorkspace());
	}

	private static async loadWorkspace(): Promise<Workspace> {
		// load the web assembly module
		main();
		return Promise.resolve(new Workspace());
	}

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
		const path: RomePath = {
			id: 0,
			path: options.filePath,
		};
		await this.workspace.openFile({
			content,
			version: 0,
			path,
		});

		let code;
		if (options.range) {
			const result = await this.workspace.formatRange({
				path: path,
				// @ts-expect-error For some reason, passing the tuple works but. Need to understand what's going on
				range: options.range,
			});
			code = result.code;
		} else {
			const result = await this.workspace.formatFile({
				path,
			});
			code = result.code;
		}

		if (isFormatContentDebug(options)) {
			const ir = await this.workspace.getFormatterIr({
				path,
			});
			this.workspace.closeFile({ path });
			return {
				content: code,
				errors: [],
				ir,
			};
		}
		this.workspace.closeFile({ path });
		return {
			content: code,
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
