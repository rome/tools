/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {BuildFrameOpts, Frame} from "./types";

import crypto = require("crypto");

export function isCompleteFrame(frame: Frame): boolean {
	return Buffer.byteLength(frame.payload) >= frame.payloadLength;
}

export function unmaskPayload(
	payload: Buffer,
	mask: undefined | Buffer,
	offset: number,
) {
	if (mask === undefined) {
		return payload;
	}

	for (let i = 0; i < payload.length; i++) {
		payload[i] ^= mask[offset + i & 3];
	}

	return payload;
}

export function buildFrame(opts: BuildFrameOpts, shouldMask: boolean): Buffer {
	const {opcode, fin, data} = opts;

	let offset = shouldMask ? 6 : 2;
	let dataLength = data.length;

	if (dataLength >= 65_536) {
		offset += 8;
		dataLength = 127;
	} else if (dataLength > 125) {
		offset += 2;
		dataLength = 126;
	}

	const head = Buffer.allocUnsafe(offset);

	head[0] = fin ? opcode | 128 : opcode;
	head[1] = dataLength;

	if (dataLength === 126) {
		head.writeUInt16BE(data.length, 2);
	} else if (dataLength === 127) {
		head.writeUInt32BE(0, 2);
		head.writeUInt32BE(data.length, 6);
	}

	if (shouldMask) {
		const mask = crypto.randomBytes(4);
		head[1] |= 128;
		head[offset - 4] = mask[0];
		head[offset - 3] = mask[1];
		head[offset - 2] = mask[2];
		head[offset - 1] = mask[3];

		const masked = Buffer.alloc(dataLength);
		for (let i = 0; i < dataLength; ++i) {
			masked[i] = data[i] ^ mask[i & 3];
		}

		return Buffer.concat([head, masked]);
	} else {
		return Buffer.concat([head, data]);
	}
}

export function parseFrame(buffer: Buffer): Frame {
	const firstByte = buffer.readUInt8(0);
	const isFinalFrame: boolean = Boolean(firstByte >>> 7 & 1);
	const opcode: number = firstByte & 15;

	const [reserved1, reserved2, reserved3] = [
		(firstByte >>> 6 & 1) === 1,
		(firstByte >>> 5 & 1) === 1,
		(firstByte >>> 4 & 1) === 1,
	];
	reserved1;
	reserved2;
	reserved3;

	const secondByte: number = buffer.readUInt8(1);
	const isMasked: boolean = Boolean(secondByte >>> 7 & 1);

	// Keep track of our current position as we advance through the buffer
	let currentOffset = 2;
	let payloadLength = secondByte & 127;
	if (payloadLength > 125) {
		if (payloadLength === 126) {
			payloadLength = buffer.readUInt16BE(currentOffset);
			currentOffset += 2;
		} else if (payloadLength === 127) {
			const leftPart = buffer.readUInt32BE(currentOffset);
			currentOffset += 4;

			// The maximum safe integer in JavaScript is 2^53 - 1. An error is returned

			// if payload length is greater than this number.
			if (leftPart >= Number.MAX_SAFE_INTEGER) {
				throw new Error(
					"Unsupported WebSocket frame: payload length > 2^53 - 1",
				);
			}

			const rightPart = buffer.readUInt32BE(currentOffset);
			currentOffset += 4;

			payloadLength = leftPart * Math.pow(2, 32) + rightPart;
		} else {
			throw new Error("Unknown payload length");
		}
	}

	// Get the masking key if one exists
	let mask;
	if (isMasked) {
		mask = buffer.slice(currentOffset, currentOffset + 4);
		currentOffset += 4;
	}

	let payload = unmaskPayload(buffer.slice(currentOffset), mask, 0);

	return {
		fin: isFinalFrame,
		opcode,
		mask,
		payload,
		payloadLength,
	};
}
