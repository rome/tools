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
import { processResourceRoot } from "@internal/resources";

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

		bridge.resources.addSocket(socket);

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
				bridge.disconnected("RPC WebSocket died");
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

		bridge.resources.addWebSocket(socket);

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
			bridge.disconnected("RPC WebSocket disconnected");
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

		bridge.resources.addSocket(socket);

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
				bridge.disconnected(hadError ? "Socket closed due to transmission error" : "Socket closed");
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

		bridge.resources.addWorkerThread(worker);

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
				bridge.disconnected(`Worker thread died with exit code ${code}`);
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
		processResourceRoot.add(bridge);
		
		const rser = bridge.attachRSER();

		rser.sendEvent.subscribe((msg) => {
			parentPort.postMessage(msg, [msg]);
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
				bridge.disconnected("Worker thread parent port closed");
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
		server.endEvent.subscribe(() => {
			client.disconnected(`Server disconnected`);
		});

		const client = this.Client.create();
		client.sendMessageEvent.subscribe((data) => {
			server.handleMessage(data);
		});
		client.endEvent.subscribe(() => {
			server.disconnected(`Client disconnected`);
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
