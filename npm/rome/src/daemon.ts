import {
	createWorkspace,
	createWorkspaceWithBinary,
	Workspace,
} from "@rometools/backend-jsonrpc";

/**
 * Class responsible to communicate with the Rome daemon.
 *
 * A daemon is a long term server where multiple clients can communicate with it using JSON-RPC.
 * A daemon is spawn via CLI.
 */
export class Deamon {
	public workspace: Workspace;
	private constructor(workspace: Workspace) {
		this.workspace = workspace;
	}

	/**
	 * It creates a new instance of a workspace connected to the Daemon
	 */
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
}
