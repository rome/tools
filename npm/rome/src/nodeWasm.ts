import { main, Workspace } from "@rometools/wasm-nodejs";

/**
 * Class responsible to connect with the WebAssembly backend of Rome.
 */
export class NodeWasm {
	public workspace: Workspace;
	private constructor(workspace: Workspace) {
		this.workspace = workspace;
	}

	/**
     * It creates a new instance of a workspace connected to the WebAssembly backend
     */
	public static async loadWebAssembly(): Promise<NodeWasm> {
		return new NodeWasm(await NodeWasm.loadWorkspace());
	}

	private static async loadWorkspace(): Promise<Workspace> {
		// load the web assembly module
		main();
		return Promise.resolve(new Workspace());
	}
}
