import {RSERValue} from "./types";
import RSERBufferAssembler from "./RSERBufferAssembler";
import RSERBufferWriter from "./RSERBufferWriter";

export {default as RSERBufferObserver} from "./RSERBufferAssembler";
export {default as RSERBufferParser} from "./RSERBufferParser";
export {default as RSERBufferWriter} from "./RSERBufferWriter";
export {default as RSERStream} from "./RSERStream";
export {default as RSERSharedBuffer} from "./RSERSharedBuffer";

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

export function encodeToBuffer(val: RSERValue): ArrayBuffer {
	const {payloadLength, messageLength} = RSERBufferAssembler.measure(val);
	const buf = RSERBufferWriter.allocate(messageLength);
	buf.encodeHeader(payloadLength);
	buf.encodeValue(val);
	return buf.buffer;
}
