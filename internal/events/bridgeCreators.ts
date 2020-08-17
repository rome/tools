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

type BridgeClass<B> = Class<B, ConstructorParameters<typeof Bridge>>;

export function createBridgeFromWebSocketInterface<B extends Bridge>(
	CustomBridge: BridgeClass<B>,
	inf: WebSocketInterface,
	opts: BridgeCreatorOptions,
): B {
	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			rser.send(data);
		},
	});

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
	CustomBridge: BridgeClass<B>,
	socket: WebSocket,
	opts: BridgeCreatorOptions,
): B {
	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			rser.send(data);
		},
	});

	const rser = bridge.attachRSER();

	rser.sendEvent.subscribe((buf) => {
		socket.send(buf);
	});

	bridge.endEvent.subscribe(() => {
		socket.close();
	});

	socket.binaryType = "arraybuffer";

	socket.onmessage = function(event) {
		const {data} = event;
		if (!(data instanceof ArrayBuffer)) {
			throw new Error("Expected ArrayBuffer");
		}
		rser.append(data);
	};

	socket.onclose = () => {
		bridge.end("RPC WebSocket disconnected");
	};

	return bridge;
}

export function createBridgeFromSocket<B extends Bridge>(
	CustomBridge: BridgeClass<B>,
	socket: Socket,
	opts: BridgeCreatorOptions,
): B {
	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			rser.send(data);
		},
	});

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
	CustomBridge: BridgeClass<B>,
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
	CustomBridge: BridgeClass<B>,
	worker: workerThreads.Worker,
	opts: BridgeCreatorOptions,
): B {
	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			rser.send(data);
		},
	});

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

export function createBridgeFromWorkerThreadParentPort<B extends Bridge>(
	CustomBridge: BridgeClass<B>,
	opts: BridgeCreatorOptions,
): B {
	const {parentPort} = workerThreads;
	if (parentPort == null) {
		throw new Error("No worker_threads parentPort found");
	}

	const bridge = new CustomBridge({
		...opts,
		sendMessage: (data: BridgeMessage) => {
			rser.send(data);
		},
	});

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
			bridge.end("Worker thread parent port closed");
		},
	);

	return bridge;
}
