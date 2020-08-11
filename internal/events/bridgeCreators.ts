/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {WebSocketInterface} from "@internal/codec-websocket";
import {BridgeCreatorOptions, BridgeMessage} from "./types";
import Bridge from "./Bridge";
import {Socket} from "net";
import {Class} from "@internal/typescript-helpers";
import workerThreads = require("worker_threads");
import {RSERSharedBuffer, encodeToBuffer} from "@internal/codec-binary-serial";

export function createBridgeFromWebSocketInterface<B extends Bridge>(
	CustomBridge: Class<B>,
	inf: WebSocketInterface,
	opts: BridgeCreatorOptions,
): B {
	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			inf.send(Buffer.from(encodeToBuffer(data)));
		},
	});

	const {socket} = inf;
	const buf = bridge.attachRSER();

	bridge.endEvent.subscribe(() => {
		socket.end();
	});

	inf.completeFrameEvent.subscribe((frame) => {
		buf.append(frame.payload);
	});

	socket.on(
		"error",
		(err) => {
			bridge.endWithError(err);
		},
	);

	socket.on(
		"end",
		() => {
			bridge.end("RPC WebSocket died");
		},
	);

	return bridge;
}

export function createBridgeFromBrowserWebSocket<B extends Bridge>(
	CustomBridge: Class<B>,
	socket: WebSocket,
	opts: BridgeCreatorOptions,
): B {
	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			socket.send(encodeToBuffer(data));
		},
	});

	const buf = bridge.attachRSER();

	bridge.endEvent.subscribe(() => {
		socket.close();
	});

	socket.binaryType = "arraybuffer";

	socket.onmessage = function(event) {
		const {data} = event;
		if (!(data instanceof ArrayBuffer)) {
			throw new Error("Expected ArrayBuffer");
		}
		buf.append(new Uint8Array(data));
	};

	socket.onclose = () => {
		bridge.end("RPC WebSocket disconnected");
	};

	return bridge;
}

export function createBridgeFromSocket<B extends Bridge>(
	CustomBridge: Class<B>,
	socket: Socket,
	opts: BridgeCreatorOptions,
): B {
	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			socket.write(new Uint8Array(encodeToBuffer(data)));
		},
	});

	const buf = bridge.attachRSER();

	bridge.endEvent.subscribe(() => {
		socket.end();
	});

	socket.on(
		"data",
		(chunk) => {
			buf.append(chunk);
		},
	);

	socket.on(
		"error",
		(err) => {
			bridge.endWithError(err);
		},
	);

	socket.on(
		"end",
		() => {
			bridge.end("Socket disconnected");
		},
	);

	return bridge;
}

export function createBridgeFromLocal<B extends Bridge>(
	CustomBridge: Class<B>,
	opts: Omit<BridgeCreatorOptions, "type">,
): B {
	const bridge = new CustomBridge({
		...opts,
		type: "server&client",
		sendMessage: (msg: BridgeMessage) => {
			bridge.handleMessage(msg);
		},
	});

	return bridge;
}

export function createBridgeFromWorkerThread<B extends Bridge>(
	CustomBridge: Class<B>,
	worker: workerThreads.Worker,
	opts: BridgeCreatorOptions,
): B {
	const shared = RSERSharedBuffer.create(500_000, 500_000);

	let ready = false;
	let readyQueue: Array<ArrayBuffer> = [];

	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			shared.send(data);
		},
	});

	shared.valueEvent.subscribe((msg) => {
		bridge.handleMessage((msg as BridgeMessage));
	});

	shared.sendEvent.subscribe((msg) => {
		if (ready) {
			worker.postMessage(msg);
		} else {
			readyQueue.push(msg);
		}
	});

	bridge.endEvent.subscribe(() => {
		worker.terminate();
	});

	worker.once(
		"message",
		(msg) => {
			if (msg === "READY") {
				worker.on(
					"message",
					(msg) => {
						shared.processOutBand(msg);
					},
				);

				worker.postMessage(shared.getFlippedBuffers());
				ready = true;

				for (const msg of readyQueue) {
					worker.postMessage(msg);
				}

				readyQueue = [];
			} else {
				throw new Error(
					`Expected READY as the first worker message but got ${msg}`,
				);
			}
		},
	);

	worker.on(
		"messageerror",
		(err) => {
			bridge.endWithError(err);
		},
	);

	worker.on(
		"error",
		(err) => {
			bridge.endWithError(err);
		},
	);

	worker.on(
		"exit",
		(code) => {
			bridge.end(`Worker thread died with exit code ${code}`);
		},
	);

	return bridge;
}

export async function createBridgeFromWorkerThreadParentPort<B extends Bridge>(
	CustomBridge: Class<B>,
	opts: BridgeCreatorOptions,
): Promise<B> {
	const {parentPort} = workerThreads;
	if (parentPort == null) {
		throw new Error("No worker_threads parentPort found");
	}

	const shared: RSERSharedBuffer = await new Promise((resolve) => {
		parentPort.once(
			"message",
			(buffers) => {
				resolve(new RSERSharedBuffer(buffers));
			},
		);

		parentPort.postMessage("READY");
	});

	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			shared.send(data);
		},
	});

	shared.valueEvent.subscribe((msg) => {
		bridge.handleMessage((msg as BridgeMessage));
	});

	shared.sendEvent.subscribe((msg) => {
		parentPort.postMessage(msg);
	});

	bridge.endEvent.subscribe(() => {
		parentPort.close();
	});

	parentPort.on(
		"message",
		(msg) => {
			shared.processOutBand(msg);
		},
	);

	parentPort.on(
		"close",
		() => {
			bridge.end("Worker thread parent port closed");
		},
	);

	return bridge;
}
