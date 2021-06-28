import {AsyncVoidCallback} from "@internal/typescript-helpers";
import Resource from "./Resource";
import {
	FactoryResourceContainerOptions,
	FactoryResourceOptions,
	TimerResourceDetails,
} from "./types";
import {processResourceRoot} from "./index";
import workerThreads = require("worker_threads");
import childProcess = require("child_process");
import net = require("net");

export function createResourceFromCallback(
	name: string,
	callback: AsyncVoidCallback,
	opts?: FactoryResourceOptions,
): Resource {
	return new Resource({
		...opts,
		name,
		getDetails() {
			return {
				type: "callback",
			};
		},
		release: callback,
	});
}

type PrivateTimeout =
	| number
	| (NodeJS.Timeout & {
			_repeat?: null | number;
			_idleTimeout?: number;
		});

export function createResourceFromTimeout(
	name: string,
	timeout: PrivateTimeout,
	opts?: FactoryResourceOptions,
): Resource {
	let methodName = "setTimeout";
	let kind: TimerResourceDetails["kind"] = "unknown";
	let delay: undefined | number;

	// Extract private node properties for better name
	if (typeof timeout !== "number") {
		if (typeof timeout._repeat === "number") {
			methodName = "setInterval";
			kind = "interval";
		} else {
			kind = "timeout";
		}

		if (typeof timeout._idleTimeout === "number") {
			delay = timeout._idleTimeout;
		}
	}

	const wrappedName =
		delay === undefined
			? `${methodName}<${name}>`
			: `${methodName}<${name}, ${delay}ms>`;

	return new Resource({
		...opts,
		name: wrappedName,
		getDetails: () => ({
			type: "timer",
			kind,
			delay,
		}),
		release: () => {
			// TypeScript overloads are so weird and necessitate this
			if (typeof timeout === "number") {
				clearTimeout(timeout);
			} else {
				clearTimeout(timeout);
			}
		},
	});
}

export function createResourceFromChildProcess(
	proc: childProcess.ChildProcess,
	name: string = `ChildProcess<pid:${proc.pid}, ${proc.spawnfile}>`,
	opts?: FactoryResourceOptions,
): Resource {
	const resc = new Resource({
		...opts,
		name,
		getDetails: () => ({
			type: "process",
			self: false,
			pid: proc.pid,
			tid: 0,
			command: proc.spawnfile,
			args: proc.spawnargs,
		}),
		release: () => {
			proc.kill();
		},
	});
	proc.on(
		"close",
		() => {
			resc.release();
		},
	);
	return resc;
}

export function createResourceFromWorkerThread(
	worker: workerThreads.Worker,
	name: string = `WorkerThread<${String(worker.threadId)}>`,
	opts?: FactoryResourceOptions,
): Resource {
	const resc = new Resource({
		...opts,
		name,
		getDetails: () => ({
			type: "worker",
			worker,
		}),
		finalize: async () => {
			await worker.terminate();
		},
	});
	worker.on(
		"exit",
		() => {
			resc.release();
		},
	);
	return resc;
}

export function createResourceFromSocket(
	socket: net.Socket,
	name: string = "net.Socket",
	opts?: FactoryResourceOptions,
): Resource {
	const resc = new Resource({
		...opts,
		name,
		getDetails: () => ({
			type: "socket",
			localAddress: socket.localAddress,
			localPort: socket.localPort,
			remoteAddress: socket.remoteAddress,
			remoteFamily: socket.remoteFamily,
			remotePort: socket.remotePort,
		}),
		release: () => {
			socket.end();
		},
	});
	socket.on(
		"close",
		() => {
			resc.release();
		},
	);
	return resc;
}

export function createResourceFromServer(
	server: net.Server,
	name: string = "net.Server",
	opts?: FactoryResourceOptions,
): Resource {
	const resc = new Resource({
		...opts,
		name,
		getDetails: () => ({
			type: "server",
			...(server.address() as net.AddressInfo),
		}),
		finalize: () => {
			return new Promise((resolve, reject) => {
				server.close((err) => {
					if (err != null) {
						reject(err);
					} else {
						resolve();
					}
				});
			});
		},
	});
	server.on(
		"close",
		() => {
			resc.release();
		},
	);
	return resc;
}

export function createResourceFromWebSocket(
	socket: WebSocket,
	name: string = "WebSocket",
	opts?: FactoryResourceOptions,
): Resource {
	const resc = new Resource({
		...opts,
		name,
		getDetails: () => ({
			type: "websocket",
			url: socket.url,
		}),
		release: () => {
			socket.close();
		},
	});
	socket.addEventListener(
		"close",
		() => {
			resc.release();
		},
	);
	return resc;
}

export function createResourceRoot(
	name: string,
	callback?: AsyncVoidCallback,
): Resource {
	const resc = new Resource({
		name,
		getDetails: () => {
			return {
				type: "root",
			};
		},
		release: callback,
	});
	processResourceRoot.add(resc);
	return resc;
}

export function createResourceContainer(
	name: string,
	opts?: FactoryResourceContainerOptions,
): Resource {
	return new Resource({
		...opts,
		name,
		getDetails: () => ({
			type: "container",
		}),
	});
}
