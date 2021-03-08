import {FSReadStream} from "@internal/fs";
import {getUTF8ByteLength} from "./utf8";

export * from "./utf8";

type BufferSourceLike = string | BufferSource;

const textEncoder = new TextEncoder();
const textDecoder = new TextDecoder();

export function isBufferSource(val: unknown): val is BufferSource {
	return ArrayBuffer.isView(val) || val instanceof ArrayBuffer;
}

export function encodeTextToArrayBuffer(str: string): ArrayBuffer {
	return textEncoder.encode(str).buffer;
}

export function getByteLength(val: BufferSourceLike | FSReadStream): number {
	if (ArrayBuffer.isView(val)) {
		return val.byteLength - val.byteOffset;
	} else if (val instanceof ArrayBuffer) {
		return val.byteLength;
	} else if (typeof val === "string") {
		return getUTF8ByteLength(val);
	} else {
		return val.bytesRead;
	}
}

export function toDataView(val: BufferSourceLike): DataView {
	if (val instanceof DataView) {
		return val;
	} else if (ArrayBuffer.isView(val)) {
		return new DataView(val.buffer, val.byteOffset, val.byteLength);
	} else if (val instanceof ArrayBuffer) {
		return new DataView(val);
	} else {
		return new DataView(encodeTextToArrayBuffer(val));
	}
}

export function toUintArray8(val: BufferSourceLike): Uint8Array {
	if (val instanceof Uint8Array) {
		return val;
	} else if (ArrayBuffer.isView(val)) {
		return new Uint8Array(val.buffer, val.byteOffset, val.byteLength);
	} else if (val instanceof ArrayBuffer) {
		return new Uint8Array(val);
	} else {
		return new Uint8Array(encodeTextToArrayBuffer(val));
	}
}

export function decodeUTF8(val: BufferSourceLike): string {
	if (typeof val === "string") {
		return val;
	} else {
		return textDecoder.decode(val);
	}
}

export function toNodeBuffer(
	val: BufferSourceLike,
	encoding?: BufferEncoding,
): Buffer {
	if (typeof val === "string") {
		return Buffer.from(val, encoding);
	} else if (val instanceof ArrayBuffer) {
		return Buffer.from(val);
	} else {
		return Buffer.from(val.buffer, val.byteOffset, val.byteLength);
	}
}

export function encodeBase64(val: BufferSourceLike): string {
	return toNodeBuffer(val).toString("base64");
}

export function decodeBase64(val: string): ArrayBuffer {
	return toNodeBuffer(val, "base64").buffer;
}
