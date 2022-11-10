import type { Workspace } from "@rometools/backend-jsonrpc";

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
		const { createWorkspace, createWorkspaceWithBinary } = await import(
			"@rometools/backend-jsonrpc"
		);

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
	/**
	 * A better representation of the error, which might contain the stack trace of the error.
	 *
	 * This is useful for debug purpose
	 */
	data?: any;
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
