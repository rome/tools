import {
	Diagnostic,
	main,
	PullDiagnosticsParams,
	Workspace,
} from "@rometools/wasm-nodejs";

export class NodeWasm {
	public workspace: Workspace;
	private constructor(workspace: Workspace) {
		this.workspace = workspace;
	}

	/**
     * It creates a new instance of the class {Rome}
     */
	public static async createWorkspace(): Promise<NodeWasm> {
		return new NodeWasm(await NodeWasm.loadWorkspace());
	}

	private static async loadWorkspace(): Promise<Workspace> {
		// load the web assembly module
		main();
		return Promise.resolve(new Workspace());
	}

	public async pullDiagnostics(
		params: PullDiagnosticsParams,
	): Promise<Diagnostic[]> {
		const result = await this.workspace.pullDiagnostics(params);
		return result.diagnostics;
	}
}
