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
	public static async connectToDaemon(pathToBinary?: string): Promise<Deamon> {
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

interface ErrorFromDaemon {
	code: number;
	data: string;
	message: string;
}

/**
 * Error generated when communicating with the daemon
 */
export class DaemonError extends Error {
	/**
	 * The code of the error
	 */
	code: number;
	// @ematipico Not sure how to document it
	data: string;
	/**
	 * The reason why there's been an error
	 */
	message: string;

	private constructor({ code, data, message }: ErrorFromDaemon) {
		super();
		this.code = code;
		this.data = data;
		this.message = message;
	}

	static fromError(e: any): DaemonError {
		return new DaemonError(e as ErrorFromDaemon);
	}
}
