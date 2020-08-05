/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {WebSocketInterface} from "@internal/codec-websocket";
import {JSONValue, consumeJSON} from "@internal/codec-json";
import {Consumer} from "@internal/consume";
import {ErrorCallback} from "@internal/typescript-helpers";

type InspectorSubscription = {
	once: boolean;
	callback: (params: Consumer) => void;
};

export class InspectorClientCloseError extends Error {
	constructor() {
		super("Inspector connection closed");
	}
}

export default class InspectorClient {
	constructor(socket: WebSocketInterface) {
		this.socket = socket;
		this.id = 0;

		this.subscriptions = new Map();
		this.callbacks = new Map();

		this.alive = true;

		this.init();
	}

	public alive: boolean;
	private id: number;
	private callbacks: Map<
		number,
		{
			resolve: (params: Consumer) => void;
			reject: ErrorCallback;
		}
	>;
	private subscriptions: Map<string, Set<InspectorSubscription>>;
	private socket: WebSocketInterface;

	public end() {
		this.socket.end();
	}

	private init() {
		const {socket} = this;

		socket.errorEvent.subscribe((err) => {
			this.alive = false;
			for (const [, {reject}] of this.callbacks) {
				reject(err);
			}
			this.callbacks.clear();
			this.end();
		});

		socket.endEvent.subscribe(() => {
			this.alive = false;
			for (const [, {reject}] of this.callbacks) {
				reject(new InspectorClientCloseError());
			}
			this.callbacks.clear();
		});

		socket.completeFrameEvent.subscribe((frame) => {
			const json = frame.payload.toString();
			const data = consumeJSON({
				input: json,
			});

			// Message reply
			const id = data.get("id").asNumberOrVoid();
			if (id !== undefined) {
				const handler = this.callbacks.get(id);
				if (handler !== undefined) {
					if (data.has("error")) {
						const errorMessage = data.get("error").get("message").asString();
						handler.reject(new Error(errorMessage));
					} else {
						handler.resolve(data.get("result"));
					}
					this.callbacks.delete(id);
				}
				return;
			}

			// Event
			const method = data.get("method").asStringOrVoid();
			if (method !== undefined) {
				const subs = this.subscriptions.get(method);
				if (subs !== undefined) {
					for (const sub of subs) {
						const {callback, once} = sub;
						callback(data.get("params"));
						if (once) {
							subs.delete(sub);
						}
					}
				}
			}
		});
	}

	private subscribe(method: string, sub: InspectorSubscription) {
		let subs = this.subscriptions.get(method);
		if (subs === undefined) {
			subs = new Set();
			this.subscriptions.set(method, subs);
		}
		subs.add(sub);
	}

	private assertAlive() {
		if (!this.alive) {
			throw new Error("InspectorClient has no active socket");
		}
	}

	public async wait(method: string): Promise<Consumer> {
		return new Promise((resolve) => {
			this.assertAlive();
			this.subscribe(
				method,
				{
					once: true,
					callback: resolve,
				},
			);
		});
	}

	public call(method: string, params?: JSONValue): Promise<Consumer> {
		const id = ++this.id;

		return new Promise((resolve, reject) => {
			this.assertAlive();
			this.callbacks.set(id, {resolve, reject});

			this.socket.sendJSON({
				id,
				method,
				params,
			});
		});
	}
}
