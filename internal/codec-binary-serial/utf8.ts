/**
 * Code in this file taken from https://github.com/msgpack/msgpack-javascript and licensed under:
 *
 * Copyright 2019 The MessagePack Community.
 *
 * Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby
 * granted, provided that the above copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
 * INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
 * AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */

const sharedTextEncoder = new TextEncoder();
const sharedTextDecoder = new TextDecoder();

export const TEXT_ENCODER_THRESHOLD = 200;
export const TEXT_DECODER_THRESHOLD = 200;

const CHUNK_SIZE = 0x1000;

export function utf8Count(str: string): number {
	const strLength = str.length;

	let byteLength = 0;
	let pos = 0;
	while (pos < strLength) {
		let value = str.charCodeAt(pos++);

		if ((value & 0xffffff80) === 0) {
			// 1-byte
			byteLength++;
			continue;
		} else if ((value & 0xfffff800) === 0) {
			// 2-bytes
			byteLength += 2;
		} else {
			// handle surrogate pair
			if (value >= 0xd800 && value <= 0xdbff) {
				// high surrogate
				if (pos < strLength) {
					const extra = str.charCodeAt(pos);
					if ((extra & 0xfc00) === 0xdc00) {
						++pos;
						value = ((value & 0x3ff) << 10) + (extra & 0x3ff) + 0x10000;
					}
				}
			}

			if ((value & 0xffff0000) === 0) {
				// 3-byte
				byteLength += 3;
			} else {
				// 4-byte
				byteLength += 4;
			}
		}
	}
	return byteLength;
}

export function utf8Encode(
	str: string,
	output: Uint8Array,
	outputOffset: number,
	byteLength: number,
): void {
	if (byteLength > TEXT_ENCODER_THRESHOLD) {
		if (sharedTextEncoder.encodeInto === undefined) {
			output.set(sharedTextEncoder.encode(str), outputOffset);
		} else {
			sharedTextEncoder.encodeInto(str, output.subarray(outputOffset));
		}
		return;
	}

	const strLength = str.length;
	let offset = outputOffset;
	let pos = 0;
	while (pos < strLength) {
		let value = str.charCodeAt(pos++);

		if ((value & 0xffffff80) === 0) {
			// 1-byte
			output[offset++] = value;
			continue;
		} else if ((value & 0xfffff800) === 0) {
			// 2-bytes
			output[offset++] = value >> 6 & 0x1f | 0xc0;
		} else {
			// handle surrogate pair
			if (value >= 0xd800 && value <= 0xdbff) {
				// high surrogate
				if (pos < strLength) {
					const extra = str.charCodeAt(pos);
					if ((extra & 0xfc00) === 0xdc00) {
						++pos;
						value = ((value & 0x3ff) << 10) + (extra & 0x3ff) + 0x10000;
					}
				}
			}

			if ((value & 0xffff0000) === 0) {
				// 3-byte
				output[offset++] = value >> 12 & 0xf | 0xe0;
				output[offset++] = value >> 6 & 0x3f | 0x80;
			} else {
				// 4-byte
				output[offset++] = value >> 18 & 0x7 | 0xf0;
				output[offset++] = value >> 12 & 0x3f | 0x80;
				output[offset++] = value >> 6 & 0x3f | 0x80;
			}
		}

		output[offset++] = value & 0x3f | 0x80;
	}
}

export function utf8Decode(
	bytes: Uint8Array,
	inputOffset: number,
	byteLength: number,
): string {
	if (byteLength > TEXT_DECODER_THRESHOLD) {
		const stringBytes = bytes.subarray(inputOffset, inputOffset + byteLength);
		return sharedTextDecoder.decode(stringBytes);
	}

	let offset = inputOffset;
	const end = offset + byteLength;

	const units: Array<number> = [];
	let result = "";
	while (offset < end) {
		const byte1 = bytes[offset++];
		if ((byte1 & 0x80) === 0) {
			// 1 byte
			units.push(byte1);
		} else if ((byte1 & 0xe0) === 0xc0) {
			// 2 bytes
			const byte2 = bytes[offset++] & 0x3f;
			units.push((byte1 & 0x1f) << 6 | byte2);
		} else if ((byte1 & 0xf0) === 0xe0) {
			// 3 bytes
			const byte2 = bytes[offset++] & 0x3f;
			const byte3 = bytes[offset++] & 0x3f;
			units.push((byte1 & 0x1f) << 12 | byte2 << 6 | byte3);
		} else if ((byte1 & 0xf8) === 0xf0) {
			// 4 bytes
			const byte2 = bytes[offset++] & 0x3f;
			const byte3 = bytes[offset++] & 0x3f;
			const byte4 = bytes[offset++] & 0x3f;
			let unit = (byte1 & 0x7) << 0x12 | byte2 << 0xc | byte3 << 0x6 | byte4;
			if (unit > 0xffff) {
				unit -= 0x10000;
				units.push(unit >>> 10 & 0x3ff | 0xd800);
				unit = 0xdc00 | unit & 0x3ff;
			}
			units.push(unit);
		} else {
			units.push(byte1);
		}

		if (units.length >= CHUNK_SIZE) {
			result += String.fromCharCode(...units);
			units.length = 0;
		}
	}

	if (units.length > 0) {
		result += String.fromCharCode(...units);
	}

	return result;
}
