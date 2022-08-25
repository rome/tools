import {
	createWorkspace,
	createWorkspaceWithBinary,
	Diagnostic,
	PullDiagnosticsParams,
	Workspace,
} from "@rometools/backend-jsonrpc";

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

	public async pullDiagnostics(
		params: PullDiagnosticsParams,
	): Promise<Diagnostic[]> {
		const result = await this.workspace.pullDiagnostics(params);
		return result.diagnostics;
	}
}
