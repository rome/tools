import {RSERValue} from "./types";
import RSERBufferAssembler from "./RSERBufferAssembler";
import RSERBufferWriter from "./RSERBufferWriter";
import RSERStream from "./RSERStream";
import fs = require("fs");
import {markup} from "@internal/markup";
import {provideDiagnosticAdviceForError} from "@internal/diagnostics";
import {createUnknownPath} from "@internal/path";

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

export function encodeValueToRSERSingleMessageStream(
	val: RSERValue,
): ArrayBuffer {
	return encodeValueToRSERBuffer(val, true);
}

export function encodeValueToRSERMessage(val: RSERValue): ArrayBuffer {
	return encodeValueToRSERBuffer(val, false);
}

function encodeValueToRSERBuffer(val: RSERValue, isStream: boolean): ArrayBuffer {
	const assembler = new RSERBufferAssembler();
	if (isStream) {
		assembler.encodeStreamHeader();
	}
	assembler.encodeValue(val);
	const payloadLength = assembler.totalSize;
	assembler.encodeMessageHeader(payloadLength);
	const messageLength = assembler.totalSize;

	const buf = new RSERBufferWriter(new ArrayBuffer(messageLength), assembler);
	if (isStream) {
		assembler.encodeStreamHeader();
	}
	buf.encodeMessageHeader(payloadLength);
	buf.encodeValue(val);
	return buf.buffer;
}

type DecodedRSERSingleMessageStream =
	| {
			type: "INCOMPATIBLE";
		}
	| {
			type: "VALUE";
			value: RSERValue;
		};

export function decodeSingleMessageRSERStream(
	readStream: fs.ReadStream,
): Promise<DecodedRSERSingleMessageStream> {
	return new Promise((resolve, reject) => {
		let foundValue = false;
		const decodeStream = new RSERStream("file");

		function safeClose() {
			foundValue = true;
			readStream.close();
		}

		function handleError(err: Error) {
			reject(
				provideDiagnosticAdviceForError(
					err,
					{
						description: {
							message: markup`An error occured while decoding binary file ${createUnknownPath(
								String(readStream.path),
							)}`,
							category: "parse",
							categoryValue: "binary",
						},
					},
				),
			);
			safeClose();
		}

		readStream.on(
			"data",
			(chunk) => {
				const nodeBuffer: Buffer =
					typeof chunk === "string" ? Buffer.from(chunk) : chunk;
				const arrBuffer: ArrayBuffer = nodeBuffer.buffer;
				decodeStream.append(arrBuffer);
			},
		);

		readStream.on(
			"error",
			(err) => {
				handleError(err);
			},
		);

		readStream.on(
			"close",
			() => {
				if (!foundValue) {
					handleError(new Error("Stream ended and never received a value"));
				}
			},
		);

		decodeStream.incompatibleEvent.subscribe(() => {
			resolve({type: "INCOMPATIBLE"});
			safeClose();
		});

		decodeStream.errorEvent.subscribe((err) => {
			handleError(err);
		});

		decodeStream.valueEvent.subscribe((value) => {
			if (foundValue) {
				throw new Error("Stream contained multiple messages");
			}

			foundValue = true;
			resolve({
				type: "VALUE",
				value,
			});
		});
	});
}
