import {RSERValue} from "./types";
import RSERWriterMaterial from "./RSERWriterMaterial";
import RSERStream from "./RSERStream";
import fs = require("fs");
import {markup} from "@internal/markup";
import {
	DIAGNOSTIC_CATEGORIES,
	decorateErrorWithDiagnostics,
} from "@internal/diagnostics";
import {createAbsoluteFilePath} from "@internal/path";
import RSERWriterCounter from "./RSERWriterCounter";
import RSERWriterHasher from "./RSERWriterHasher";
import {sha256} from "@internal/string-utils";
import { getArrayBuffer } from "../helpers";

export function encodeValueToRSERSingleMessageStream(
	val: RSERValue,
): ArrayBuffer {
	return encodeValueToRSERBuffer(val, true);
}

export function encodeValueToRSERMessage(val: RSERValue): ArrayBuffer {
	return encodeValueToRSERBuffer(val, false);
}

export function hashRSERValue(val: RSERValue): string {
	if (typeof val === "string") {
		// Fast path for strings
		return sha256.sync(val);
	}

	const hasher = new RSERWriterHasher();
	hasher.encodeValue(val);
	return hasher.digest();
}

function encodeValueToRSERBuffer(val: RSERValue, isStream: boolean): ArrayBuffer {
	const counter = new RSERWriterCounter();
	if (isStream) {
		counter.encodeStreamHeader();
	}
	counter.encodeValue(val);
	const payloadLength = counter.totalSize;
	counter.encodeMessageHeader(payloadLength);
	const messageLength = counter.totalSize;

	const buf = new RSERWriterMaterial(new ArrayBuffer(messageLength), counter);
	if (isStream) {
		buf.encodeStreamHeader();
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
				decorateErrorWithDiagnostics(
					err,
					{
						description: {
							message: markup`An error occured while decoding binary file ${createAbsoluteFilePath(
								String(readStream.path),
							)}`,
							category: DIAGNOSTIC_CATEGORIES.parse,
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
				decodeStream.append(getArrayBuffer(chunk));
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
