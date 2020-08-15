import {RSERValue} from "./types";
import RSERBufferAssembler from "./RSERBufferAssembler";
import RSERBufferWriter from "./RSERBufferWriter";

export {default as RSERBufferObserver} from "./RSERBufferAssembler";
export {default as RSERBufferParser} from "./RSERBufferParser";
export {default as RSERBufferWriter} from "./RSERBufferWriter";
export {default as RSERStream} from "./RSERStream";

export {
	AnyRSERFilePathMap,
	RSERAbsoluteFilePathMap,
	RSERArray,
	RSERMap,
	RSERObject,
	RSERRelativeFilePathMap,
	RSERSet,
	RSERUnknownFilePathMap,
	RSERValue,
} from "./types";

export function encodeRSERBuffer(val: RSERValue): ArrayBuffer {
	const observer = new RSERBufferAssembler();
	observer.encodeValue(val);
	const payloadLength = observer.totalSize;
	observer.encodeHeader(payloadLength);
	const messageLength = observer.totalSize;

	const buf = new RSERBufferWriter(
		new SharedArrayBuffer(messageLength),
		observer.references,
	);
	buf.encodeHeader(payloadLength);
	buf.encodeValue(val);
	return buf.buffer;
}
