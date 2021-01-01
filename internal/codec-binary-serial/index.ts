import {RSERValue} from "./types";
import RSERBufferAssembler from "./RSERBufferAssembler";
import RSERBufferWriter from "./RSERBufferWriter";
import RSERStream from "./RSERStream";
import fs = require("fs");

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

export function encodeValueToRSERBufferMessage(val: RSERValue): ArrayBuffer {
	const assembler = new RSERBufferAssembler();
	assembler.encodeValue(val);
	const payloadLength = assembler.totalSize;
	assembler.encodeMessageHeader(payloadLength);
	const messageLength = assembler.totalSize;

	const buf = new RSERBufferWriter(new ArrayBuffer(messageLength), assembler);
	buf.encodeMessageHeader(payloadLength);
	buf.encodeValue(val);
	return buf.buffer;
}

export function decodeSingleMessageRSERStream(
	readStream: fs.ReadStream,
): Promise<RSERValue> {
	return new Promise((resolve, reject) => {
		let foundValue = false;
		const decodeStream = new RSERStream("file");

		readStream.on(
			"data",
			(chunk) => {
				decodeStream.append(
					typeof chunk === "string" ? Buffer.from(chunk) : chunk,
				);
			},
		);

		readStream.on(
			"error",
			(err) => {
				reject(err);
			},
		);

		readStream.on(
			"close",
			() => {
				if (!foundValue) {
					reject(new Error("Stream ended and never received a value"));
				}
			},
		);

		decodeStream.errorEvent.subscribe((err) => {
			reject(err);
		});

		decodeStream.valueEvent.subscribe((value) => {
			if (foundValue) {
				throw new Error("Stream contained multiple messages");
			}

			foundValue = true;
			resolve(value);
		});
	});
}
