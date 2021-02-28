import {FSReadStream} from "@internal/fs";
import {utf8Count} from "./utf8";

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
  if (isBufferSource(val)) {
    return val.byteLength;
  } else if (typeof val === "string") {
    return utf8Count(val);
  } else {
    return val.bytesRead;
  }
}

export function getArrayBuffer(val: BufferSourceLike): ArrayBuffer {
  if (val instanceof ArrayBuffer) {
    return val;
  } else if (ArrayBuffer.isView(val)) {
    return val.buffer;
  } else {
    return encodeTextToArrayBuffer(val);
  }
}

export function getArrayBufferView(val: BufferSourceLike): ArrayBufferView {
  if (ArrayBuffer.isView(val)) {
    return val;
  } else if (val instanceof ArrayBuffer) {
    return new DataView(val);
  } else {
    return new DataView(encodeTextToArrayBuffer(val));
  }
}

export function decodeUTF8(val: BufferSourceLike): string {
  if (typeof val === "string") {
    return val;
  } else {
    return textDecoder.decode(val);
  }
}

export function getNodeBuffer(val: BufferSourceLike, encoding?: BufferEncoding): Buffer {
  if (typeof val === "string") {
    return Buffer.from(val, encoding);
  } else if (val instanceof ArrayBuffer) {
    return Buffer.from(val);
  } else {
    return Buffer.from(val.buffer);
  }
}

export function encodeBase64(val: BufferSourceLike): string {
  return getNodeBuffer(val).toString("base64");
}

export function decodeBase64(val: string): ArrayBuffer {
  return getNodeBuffer(val, "base64").buffer;
}