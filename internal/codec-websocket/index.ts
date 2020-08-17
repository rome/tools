/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BuildFrameOpts, Frame, GUID, OPCODES} from "./types";
import {buildFrame, isCompleteFrame, parseFrame, unmaskPayload} from "./frame";
import {Event} from "@internal/events";
import crypto = require("crypto");

import url = require("url");

import http = require("http");

import net = require("net");

import {Reporter} from "@internal/cli-reporter";
import {markup} from "@internal/markup";
import prettyFormat from "@internal/pretty-format";
import {NodeSystemError} from "@internal/node";

export function createKey(key: string): string {
	return crypto.createHash("sha1").update(`${key}${GUID}`).digest("base64");
}

export type WebSocketType = "client" | "server";

export type WebSocketInterfaceOptions = {
	type: WebSocketType;
	socket: net.Socket;
	reporter?: Reporter;
};

export class WebSocketInterface {
	constructor({type, socket, reporter}: WebSocketInterfaceOptions) {
		// When a frame is set here then any additional continuation frames payloads will be appended
		this.unfinishedFrame = undefined;

		// When a frame is set here, all additional chunks will be appended until we reach the correct payloadLength
		this.incompleteFrame = undefined;

		this.reporter = reporter;
		this.socket = socket;
		this.alive = true;
		this.type = type;

		this.completeFrameEvent = new Event({name: "WebSocketInterface.message"});
		this.errorEvent = new Event({name: "WebSocketInterface.error"});
		this.endEvent = new Event({name: "WebSocketInterface.end", serial: true});

		socket.on(
			"data",
			(buff) => {
				this.addBuffer(buff);
			},
		);

		socket.on(
			"error",
			(err: NodeSystemError) => {
				if (err.code === "ECONNRESET") {
					this.endEvent.send();
				} else {
					this.errorEvent.send(err);
				}
			},
		);

		socket.on(
			"close",
			() => {
				this.end();
			},
		);
	}

	private alive: boolean;
	private type: WebSocketType;
	private incompleteFrame: undefined | Frame;
	private unfinishedFrame: undefined | Frame;
	public socket: net.Socket;
	private reporter: undefined | Reporter;

	public completeFrameEvent: Event<Frame, void>;
	public errorEvent: Event<Error, void>;
	public endEvent: Event<void, void>;

	public end() {
		if (!this.alive) {
			return;
		}

		this.alive = false;
		this.endEvent.send();
		this.socket.end();
	}

	public send(buff: string | Buffer) {
		if (typeof buff === "string") {
			this.sendFrame({
				opcode: OPCODES.TEXT,
				fin: true,
				data: Buffer.from(buff),
			});
		} else if (buff instanceof Buffer) {
			this.sendFrame({
				opcode: OPCODES.BINARY,
				fin: true,
				data: buff,
			});
		} else {
			throw new Error("Don't know how to send this");
		}
	}

	public sendJSON(val: unknown) {
		this.send(String(JSON.stringify(val)));
	}

	private sendFrame(frameOpts: BuildFrameOpts) {
		if (this.reporter !== undefined) {
			this.reporter.info(
				markup`Sending frame ${() =>
					prettyFormat({
						fin: frameOpts.fin,
						opcode: frameOpts.opcode,
						msg: frameOpts.data.toString(),
					})
				}`,
			);
		}
		this.socket.write(
			Buffer.from(buildFrame(frameOpts, this.type === "client")),
		);
	}

	private completeFrame(frame: Frame): void {
		// If we have an unfinished frame then only allow continuations
		const {unfinishedFrame} = this;
		if (unfinishedFrame !== undefined) {
			if (frame.opcode === OPCODES.CONTINUATION) {
				unfinishedFrame.payload = Buffer.concat([
					unfinishedFrame.payload,
					unmaskPayload(
						frame.payload,
						unfinishedFrame.mask,
						unfinishedFrame.payload.length,
					),
				]);

				if (frame.fin) {
					this.unfinishedFrame = undefined;
					this.completeFrame(unfinishedFrame);
				}
				return;
			} else {
				// Silently ignore the previous frame...
				this.unfinishedFrame = undefined;
				/*throw new Error(
          `We're waiting for a frame to finish so only allow continuation frames. Received frame: ${JSON.stringify(
            frame,
          )} Unfinished frame: ${JSON.stringify(unfinishedFrame)}`,
        );*/
			}
		}

		if (frame.fin) {
			if (frame.opcode === OPCODES.PING) {
				this.sendFrame({
					opcode: OPCODES.PONG,
					fin: true,
					data: frame.payload,
				});
			} else {
				// Trim off any excess payload
				let excess;
				if (frame.payload.byteLength > frame.payloadLength) {
					excess = frame.payload.slice(frame.payloadLength);
					frame.payload = frame.payload.slice(0, frame.payloadLength);
				}

				if (this.reporter !== undefined) {
					this.reporter.info(
						markup`Received complete frame ${() =>
							prettyFormat({
								opcode: frame.opcode,
								length: frame.payloadLength,
								msg: frame.payload.toString(),
							})
						}`,
					);
				}

				this.completeFrameEvent.send(frame);

				if (excess !== undefined) {
					this.addBuffer(excess);
				}
			}
		} else {
			this.unfinishedFrame = frame;
		}
	}

	private addBufferToIncompleteFrame(incompleteFrame: Frame, buff: Buffer) {
		incompleteFrame.payload = Buffer.concat([
			incompleteFrame.payload,
			unmaskPayload(buff, incompleteFrame.mask, incompleteFrame.payload.length),
		]);

		if (isCompleteFrame(incompleteFrame)) {
			this.incompleteFrame = undefined;
			this.completeFrame(incompleteFrame);
		}
	}

	private addBuffer(buff: Buffer): void {
		// Check if we're still waiting for the rest of a payload
		const {incompleteFrame} = this;
		if (incompleteFrame !== undefined) {
			this.addBufferToIncompleteFrame(incompleteFrame, buff);
			return;
		}

		const frame = parseFrame(buff);

		if (isCompleteFrame(frame)) {
			// Frame has been completed!
			this.completeFrame(frame);
		} else {
			this.incompleteFrame = frame;
		}
	}
}

export async function createClient(
	rawUrl: string,
	reporter?: Reporter,
): Promise<WebSocketInterface> {
	const parts = url.parse(rawUrl);

	return new Promise((resolve, reject) => {
		const key = crypto.randomBytes(16).toString("base64");
		const digest = createKey(key);

		const req = http.request({
			hostname: parts.hostname,
			port: parts.port,
			path: parts.path,
			method: "GET",
			headers: {
				Connection: "Upgrade",
				Upgrade: "websocket",
				"Sec-WebSocket-Key": key,
				"Sec-WebSocket-Version": "13",
			},
		});

		req.on(
			"upgrade",
			(res, socket, head) => {
				if (res.headers["sec-websocket-accept"] !== digest) {
					socket.end();
					reject(
						new Error(
							`Digest mismatch ${digest} !== ${res.headers["sec-websocket-accept"]}`,
						),
					);
					return;
				}

				const client = new WebSocketInterface({type: "client", socket, reporter});
				//client.addBuffer(head);
				head;
				resolve(client);
			},
		);

		req.on(
			"error",
			(err) => {
				reject(err);
			},
		);

		req.end();
	});
}
