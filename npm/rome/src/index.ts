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
	version: number, path: RomePath
}

export class Rome {
	private workspace: Workspace
	private currentFile: CurrentFile;

	constructor() {
		// load the web assembly module
		main();
		this.workspace = new Workspace();
		this.currentFile = {
			version: 0,
			path: {
				path: "",
				id: 0
			}
		};
	}

	private async getWorkspace(): Promise<Workspace> {
		return Promise.resolve(this.workspace)
	}

	/**
	 * It updates the current file. If `true`, the file was correctly updated.
	 * If `false`, a new version will be created.
	 * @param path
	 * @param workspace
	 * @private
	 */
	private updateCurrentFile(path: RomePath): boolean {
		if (path.path === this.currentFile.path.path) {
			// same path, let's just update the version
			this.currentFile = {
				version: this.currentFile.version++,
				...this.currentFile.path
			}
			return true;
		} else {
			// no same path, let's create a new one
			this.currentFile = {
				version: 0,
				path
			}
			return false;
		}
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
		const workspace = await this.getWorkspace();
		const updated = this.updateCurrentFile({
			path: options.filePath,
			id: 1
		});
		if (updated) {
			workspace.change_file({
				content,
				version: this.currentFile.version,
				path: this.currentFile.path
			})
		} else {
			await workspace.open_file({
				content,
				version: this.currentFile.version,
				path: this.currentFile.path
			});
		}

		let code;
		if (options.range) {
			const result = workspace.format_range({
				path: this.currentFile.path,
				range: options.range
			});
			code = result.code;
		} else {
			const result = await workspace.format_file({
				path: this.currentFile.path
			});
			code = result.code;
		}


		if (isFormatContentDebug(options)) {
			const ir = workspace.get_formatter_ir({
				path: this.currentFile.path,
			});
			return {
				content: code,
				errors: [],
				ir,
			};
		}
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
