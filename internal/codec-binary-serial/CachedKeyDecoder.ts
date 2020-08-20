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

import {utf8Decode} from "./utf8";

interface KeyCacheRecord {
	readonly bytes: Uint8Array;
	readonly value: string;
}

const MAX_KEY_LENGTH = 16;
const MAX_LENGTH_PER_KEY = 16;

export class CachedKeyDecoder {
	constructor() {
		// Avoid `new Array(N)` to create a non-sparse array for performance.
		this.caches = [];
		for (let i = 0; i < MAX_KEY_LENGTH; i++) {
			this.caches.push([]);
		}
	}

	private caches: Array<Array<KeyCacheRecord>>;

	public canBeCached(byteLength: number): boolean {
		return byteLength > 0 && byteLength <= MAX_KEY_LENGTH;
	}

	private get(
		bytes: Uint8Array,
		inputOffset: number,
		byteLength: number,
	): string | undefined {
		const records = this.caches[byteLength - 1];
		const recordsLength = records.length;

		FIND_CHUNK: for (let i = 0; i < recordsLength; i++) {
			const record = records[i];
			const recordBytes = record.bytes;

			for (let j = 0; j < byteLength; j++) {
				if (recordBytes[j] !== bytes[inputOffset + j]) {
					continue FIND_CHUNK;
				}
			}
			return record.value;
		}

		return undefined;
	}

	private store(bytes: Uint8Array, value: string) {
		const records = this.caches[bytes.length - 1];
		const record: KeyCacheRecord = {bytes, value};

		if (records.length >= MAX_LENGTH_PER_KEY) {
			// `records` are full!
			// Set `record` to a randomized position.
			records[Math.random() * records.length | 0] = record;
		} else {
			records.push(record);
		}
	}

	public decode(
		bytes: Uint8Array,
		inputOffset: number,
		byteLength: number,
	): string {
		const cachedValue = this.get(bytes, inputOffset, byteLength);
		if (cachedValue !== undefined) {
			return cachedValue;
		}

		const value = utf8Decode(bytes, inputOffset, byteLength);
		const slicedCopyOfBytes = bytes.slice(inputOffset, inputOffset + byteLength);
		this.store(slicedCopyOfBytes, value);
		return value;
	}
}
