import { spawn } from "node:child_process";
import { connect, type Socket } from "node:net";

function getSocket(command: string): Promise<string> {
	return new Promise((resolve, reject) => {
		const process = spawn(command, ["__print_socket"], {
			stdio: "pipe",
		});

		process.on("error", reject);

		let pipeName = "";
		process.stdout.on("data", (data) => {
			pipeName += data.toString("utf-8");
		});

		process.on("exit", (code) => {
			if (code === 0) {
				resolve(pipeName.trimEnd());
			} else {
				reject(code);
			}
		});
	});
}

export async function createSocket(command: string): Promise<Socket> {
	const path = await getSocket(command);
	const socket = connect(path);

	await new Promise((resolve, reject) => {
		socket.once("error", reject);
		socket.once("ready", resolve);
	});

	return socket;
}
