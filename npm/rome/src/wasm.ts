import { main, Workspace } from "@rometools/wasm-nodejs";
import { FormatFileIntern } from "./types";
import { FormatDebugResult, FormatResult } from "./index";

export class Wasm {
	public workspace: Workspace;
	private constructor(workspace: Workspace) {
		this.workspace = workspace;
	}

	/**
     * It creates a new instance of the class {Rome}
     */
	public static async createWorkspace(): Promise<Wasm> {
		return new Wasm(await Wasm.loadWorkspace());
	}

	private static async loadWorkspace(): Promise<Workspace> {
		// load the web assembly module
		main();
		return Promise.resolve(new Workspace());
	}

	public async formatFile(
		params: FormatFileIntern,
	): Promise<FormatResult | FormatDebugResult> {
		let code;
		let ir = null;
		const { fileUpdated, currentFile, content, range, debug } = params;
		if (fileUpdated) {
			await this.workspace.change_file({
				content,
				version: currentFile.version,
				path: currentFile.path,
			});
		} else {
			await this.workspace.open_file({
				content,
				version: currentFile.version,
				path: currentFile.path,
			});
		}

		if (range) {
			const result = await this.workspace.format_range({
				path: currentFile.path,
				range,
			});
			code = result.code;
		} else {
			const result = await this.workspace.format_file({
				path: currentFile.path,
			});
			code = result.code;
		}
		if (debug) {
			ir = await this.workspace.get_formatter_ir({
				path: currentFile.path,
			});
		}

		if (ir) {
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
}
