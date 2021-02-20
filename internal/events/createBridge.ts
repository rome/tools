import {Bridge} from ".";
import {
	BridgeDefinition,
	BridgeEventDeclaration,
	BridgeEventsDeclaration,
	BridgeType,
} from "./types";
import {WebSocketInterface} from "@internal/codec-websocket";
import {Socket} from "net";
import workerThreads = require("worker_threads");
import {RSERValue} from "@internal/codec-binary-serial";

export function createBridgeEventDeclaration<
	Param extends RSERValue,
	Ret extends RSERValue
>(): BridgeEventDeclaration<Param, Ret> {
	return {};
}

export class BridgeFactory<
	ListenEvents extends BridgeEventsDeclaration,
	CallEvents extends BridgeEventsDeclaration,
	SharedEvents extends BridgeEventsDeclaration
> {
	constructor(
		type: BridgeType,
		def: BridgeDefinition<{}, {}, SharedEvents>,
		listenEvents: ListenEvents,
		callEvents: CallEvents,
		SharedEvents: SharedEvents,
	) {
		this.listenEvents = listenEvents;
		this.callEvents = callEvents;
		this.SharedEvents = SharedEvents;
		this.type = type;
		this.def = def;
	}

	private listenEvents: ListenEvents;
	private callEvents: CallEvents;
	private SharedEvents: SharedEvents;

	private type: BridgeType;
	private def: BridgeDefinition<{}, {}, SharedEvents>;

	create(): Bridge<ListenEvents, CallEvents, SharedEvents> {
		return new Bridge(
			this.type,
			this.def,
			this.listenEvents,
			this.callEvents,
			this.SharedEvents,
		);
	}

	createFromWebSocketInterface(
		inf: WebSocketInterface,
	): Bridge<ListenEvents, CallEvents, SharedEvents> {
		const bridge = this.create();
		const {socket} = inf;
		const rser = bridge.attachRSER();

		rser.sendEvent.subscribe((buf) => {
			inf.send(Buffer.from(buf));
		});

		bridge.endEvent.subscribe(() => {
			socket.end();
		});

		inf.completeFrameEvent.subscribe((frame) => {
			rser.append(frame.payload.buffer);
		});

		socket.on(
			"error",
			(err) => {
				bridge.endWithError(err, false);
			},
		);

		socket.on(
			"end",
			() => {
				bridge.end("RPC WebSocket died", false);
			},
		);

		rser.init();

		return bridge;
	}

	createFromBrowserWebSocket(
		socket: WebSocket,
	): Bridge<ListenEvents, CallEvents, SharedEvents> {
		const bridge = this.create();
		const rser = bridge.attachRSER();

		rser.sendEvent.subscribe((buf) => {
			socket.send(buf);
		});

		bridge.endEvent.subscribe(() => {
			socket.close();
		});

		socket.binaryType = "arraybuffer";

		socket.onopen = () => {
			rser.init();
		};

		socket.onmessage = function(event) {
			const {data} = event;
			if (!(data instanceof ArrayBuffer)) {
				throw new Error("Expected ArrayBuffer");
			}
			rser.append(data);
		};

		socket.onclose = () => {
			bridge.end("RPC WebSocket disconnected", false);
		};

		return bridge;
	}

	createFromSocket(
		socket: Socket,
	): Bridge<ListenEvents, CallEvents, SharedEvents> {
		const bridge = this.create();
		const rser = bridge.attachRSER();

		rser.sendEvent.subscribe((buf) => {
			socket.write(new Uint8Array(buf));
		});

		bridge.endEvent.subscribe(() => {
			socket.end();
		});

		socket.on(
			"data",
			(chunk) => {
				rser.append(chunk.buffer);
			},
		);

		socket.on(
			"error",
			(err) => {
				bridge.endWithError(err, false);
			},
		);

		socket.on(
			"close",
			(hadError) => {
				bridge.end(hadError ? "Socket closed due to transmission error" : "Socket closed", false);
			},
		);

		if (socket.connecting) {
			socket.on("connect", () => {
				rser.init();
			});
		} else {
			rser.init();
		}

		return bridge;
	}

	createFromWorkerThread(
		worker: workerThreads.Worker,
	): Bridge<ListenEvents, CallEvents, SharedEvents> {
		const bridge = this.create();
		const rser = bridge.attachRSER();

		rser.sendEvent.subscribe((msg) => {
			worker.postMessage(msg, [msg]);
		});

		bridge.endEvent.subscribe(() => {
			worker.terminate();
		});

		worker.on(
			"message",
			(msg) => {
				rser.append(msg);
			},
		);

		worker.on(
			"messageerror",
			(err) => {
				bridge.endWithError(err, false);
			},
		);

		worker.on(
			"error",
			(err) => {
				bridge.endWithError(err, false);
			},
		);

		worker.on(
			"exit",
			(code) => {
				bridge.end(`Worker thread died with exit code ${code}`, false);
			},
		);

		rser.init();

		return bridge;
	}

	createFromWorkerThreadParentPort(): Bridge<
		ListenEvents,
		CallEvents,
		SharedEvents
	> {
		const {parentPort} = workerThreads;
		if (parentPort == null) {
			throw new Error("No worker_threads parentPort found");
		}

		const bridge = this.create();
		const rser = bridge.attachRSER();

		rser.sendEvent.subscribe((msg) => {
			parentPort.postMessage(msg, [msg]);
		});

		bridge.endEvent.subscribe(() => {
			parentPort.close();
			process.exit();
		});

		parentPort.on(
			"message",
			(msg) => {
				rser.append(msg);
			},
		);

		parentPort.on(
			"close",
			() => {
				bridge.end("Worker thread parent port closed", false);
			},
		);

		rser.init();

		return bridge;
	}
}

export class BridgeFactories<
	ClientEvents extends BridgeEventsDeclaration,
	ServerEvents extends BridgeEventsDeclaration,
	SharedEvents extends BridgeEventsDeclaration
> {
	constructor(def: BridgeDefinition<ClientEvents, ServerEvents, SharedEvents>) {
		this.Server = new BridgeFactory(
			"server",
			def,
			def.server,
			def.client,
			def.shared,
		);
		this.Client = new BridgeFactory(
			"client",
			def,
			def.client,
			def.server,
			def.shared,
		);
	}

	public Server: BridgeFactory<ServerEvents, ClientEvents, SharedEvents>;
	public Client: BridgeFactory<ClientEvents, ServerEvents, SharedEvents>;

	createFromLocal(): {
		server: Bridge<ServerEvents, ClientEvents, SharedEvents>;
		client: Bridge<ClientEvents, ServerEvents, SharedEvents>;
	} {
		const server = this.Server.create();
		server.sendMessageEvent.subscribe((data) => {
			client.handleMessage(data);
		});

		const client = this.Client.create();
		client.sendMessageEvent.subscribe((data) => {
			server.handleMessage(data);
		});

		return {server, client};
	}
}

export default function createBridge<
	ClientEvents extends BridgeEventsDeclaration,
	ServerEvents extends BridgeEventsDeclaration,
	SharedEvents extends BridgeEventsDeclaration
>(
	opts: BridgeDefinition<ClientEvents, ServerEvents, SharedEvents>,
): BridgeFactories<ClientEvents, ServerEvents, SharedEvents> {
	return new BridgeFactories(opts);
}
