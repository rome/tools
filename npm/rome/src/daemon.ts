import {
	createWorkspace,
	createWorkspaceWithBinary,
	Workspace,
} from "@rometools/backend-jsonrpc";
import { FormatFileIntern } from "./types";
import { FormatDebugResult, FormatResult } from "./index";

export class Deamon {
	public workspace: Workspace;
	private constructor(workspace: Workspace) {
		this.workspace = workspace;
	}

	public static async createWorkspace(pathToBinary?: string): Promise<Deamon> {
		if (pathToBinary) {
			let workspace = await createWorkspaceWithBinary(pathToBinary);
			if (workspace) {
				return new Deamon(workspace);
			}
		} else {
			let workspace = await createWorkspace();
			if (workspace) {
				return new Deamon(workspace);
			}
		}
		throw new Error("could not connect to the daemon");
	}

	public async formatFile(
		params: FormatFileIntern,
	): Promise<FormatResult | FormatDebugResult> {
		let code;
		let ir = null;
		const { fileUpdated, currentFile, content, range, debug } = params;
		if (fileUpdated) {
			await this.workspace.changeFile({
				content,
				version: currentFile.version,
				path: currentFile.path,
			});
		} else {
			await this.workspace.openFile({
				content,
				version: currentFile.version,
				path: currentFile.path,
			});
		}

		if (range) {
			const result = await this.workspace.formatRange({
				path: currentFile.path,
				range,
			});
			code = result.code;
		} else {
			const result = await this.workspace.formatFile({
				path: currentFile.path,
			});
			code = result.code;
		}
		if (debug) {
			ir = await this.workspace.getFormatterIr({
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
