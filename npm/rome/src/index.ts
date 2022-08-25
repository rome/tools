import { NodeWasm } from "./nodeWasm";
import { Deamon } from "./daemon";
import { Diagnostic } from "@rometools/backend-jsonrpc";

export interface FormatFilesDebugOptions extends FormatFilesOptions {
	/**
	 * If `true`, you'll be able to inspect the IR of the formatter
	 */
	debug: boolean;
}

export interface FormatContentDebugOptions extends FormatContentOptions {
	/**
	 * If `true`, you'll be able to inspect the IR of the formatter
	 */
	debug: boolean;
}

export interface FormatFilesOptions {
	/**
	 * Writes the new content to disk
	 */
	write?: boolean;
}

export interface FormatContentOptions {
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

export interface FormatResult {
	/**
	 * The new formatted content
	 */
	content: string;
	/**
	 * A series of errors encountered while executing an operation
	 */
	diagnostics: Diagnostic[];
}

export interface FormatDebugResult {
	/**
	 * The new formatted content
	 */
	content: string;
	/**
	 * A series of errors encountered while executing an operation
	 */
	diagnostics: string[];
	/**
	 * The IR emitted by the formatter
	 */
	ir: string;
}

export interface ParseOptions {
	/**
	 * A virtual path of the file. You should add the extension,
	 * so Rome knows how to parse the content
	 */
	filePath: string;
}

export interface ParseResult {
	/**
	 * The CST of the code
	 */
	cst: string;
	/**
	 * The AST of the code
	 */
	ast: string;
	/**
	 * A series of errors encountered while executing an operation
	 */
	diagnostics: string[];
}

function isFormatFilesDebug(
	options: FormatFilesOptions | FormatFilesDebugOptions,
): options is FormatFilesDebugOptions {
	return "debug" in options && options.debug !== undefined;
}

function isFormatContentDebug(
	options: FormatContentOptions | FormatContentDebugOptions,
): options is FormatContentDebugOptions {
	return "debug" in options && options.debug !== undefined;
}

/**
 * What kind of client Rome should use to communicate with the binary
 */
export enum BackendKind {
	/**
	 * Use this if you want to communicate with the WebAssembly client built for Node.JS
	 */
	NODE,
	/**
	 * Use this if you want to communicate with the Daemon
	 */
	DAEMON,
}

type Backend = NodeWasm | Deamon;

export type RomeCreate =
	| {
		backendKind: BackendKind.NODE;
	}
	| {
		backendKind: BackendKind.DAEMON;
		pathToBinary?: string;
	};

export class Rome {
	private backend: Backend;

	private constructor(backend: Backend) {
		this.backend = backend;
	}

	/**
	 * It creates a new instance of the class {Rome}.
	 *
	 * When using the Daemon, an optional path to the Rome binary can be provided.
	 * This is useful for debugging/test purpose.
	 *
	 * @param options
	 */
	public static async create(options: RomeCreate): Promise<Rome> {
		switch (options.backendKind) {
			case BackendKind.DAEMON: {
				let client = await Deamon.createWorkspace(options.pathToBinary);
				return new Rome(client);
			}

			case BackendKind.NODE:
			default: {
				let client = await NodeWasm.createWorkspace();
				return new Rome(client);
			}
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
				diagnostics: [],
				ir: "",
			};
		}
		return {
			content: "",
			diagnostics: [],
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

	/**
	 * If formats some content.
	 *
	 * @param {String} content The content to format
	 * @param {FormatContentOptions | FormatContentDebugOptions} options Options needed when formatting some content
	 */
	async formatContent(
		content: string,
		options: FormatContentOptions | FormatContentDebugOptions,
	): Promise<FormatResult | FormatDebugResult> {
		let code;
		const file = {
			version: 0,
			path: {
				path: options.filePath,
				id: 0,
			},
		};

		await this.backend.workspace.openFile({
			content,
			version: file.version,
			path: file.path,
		});

		if (options.range) {
			const result = await this.backend.workspace.formatRange({
				path: file.path,
				// TODO: investigate following error:
				// Unknown Error: invalid type: map, expected a tuple of size 2
				// @ts-ignore The backend fails when passing an object, but it's ok when sending a tuple
				range: options.range,
			});
			code = result.code;
		} else {
			try {
				const result = await this.backend.workspace.formatFile({
					path: file.path,
				});
				code = result.code;
			} catch {
				const { diagnostics } = await this.backend.workspace.pullDiagnostics({
					path: file.path,
					categories: ["Syntax"],
				});
				return {
					content: content,
					diagnostics,
				};
			}
		}

		if (isFormatContentDebug(options)) {
			const ir = await this.backend.workspace.getFormatterIr({
				path: file.path,
			});

			await this.backend.workspace.closeFile({
				path: file.path,
			});
			return {
				content: code,
				diagnostics: [],
				ir,
			};
		}

		await this.backend.workspace.closeFile({
			path: file.path,
		});

		return {
			content: code,
			diagnostics: [],
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
			diagnostics: [],
		};
	}
}
