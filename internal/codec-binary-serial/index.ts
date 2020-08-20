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
	RSERUnknownPathMap,
	RSERValue,
} from "./types";

export function encodeRSERBuffer(val: RSERValue): ArrayBuffer {
	const assembler = new RSERBufferAssembler();
	assembler.encodeValue(val);
	const payloadLength = assembler.totalSize;
	assembler.encodeHeader(payloadLength);
	const messageLength = assembler.totalSize;

	const buf = new RSERBufferWriter(new ArrayBuffer(messageLength), assembler);
	buf.encodeHeader(payloadLength);
	buf.encodeValue(val);
	return buf.buffer;
}
