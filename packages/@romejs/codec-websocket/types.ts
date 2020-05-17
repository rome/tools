/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export type BuildFrameOpts = {
	opcode: number;
	fin: boolean;
	data: Buffer;
};

export type Frame = {
	fin: boolean;
	opcode: number;
	mask: undefined | Buffer;
	payload: Buffer;
	payloadLength: number;
};

export const OPCODES = {
	CONTINUATION: 0,
	TEXT: 1,
	BINARY: 2,
	TERMINATE: 8,
	PING: 9,
	PONG: 10,
};

export const GUID = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
