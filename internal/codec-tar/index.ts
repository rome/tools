/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import stream = require("stream");

type HeaderType =
	| "file"
	| "link"
	| "symlink"
	| "directory"
	| "block-device"
	| "character-device"
	| "fifo"
	| "contiguous-file";

type Header = {
	name: string;
	size: number;
	mode: number;
	mtime: Date;
	type: HeaderType;
	linkname: undefined | string;
	uid: number;
	gid: number;
	uname: undefined | string;
	gname: undefined | string;
	devmajor: number;
	devminor: number;
};

type PartialHeader = {
	name: string;
	mode?: number;
	mtime?: Date;
	type?: HeaderType;
	linkname?: string;
	uid?: number;
	gid?: number;
	uname?: string;
	gname?: string;
	devmajor?: number;
	devminor?: number;
};

const END_OF_TAR = Buffer.alloc(1_024);
const ZEROS = "0000000000000000000";
const SEVENS = "7777777777777777777";
const ZERO_OFFSET = "0".charCodeAt(0);
const USTAR = "ustar\x0000";
const MASK = 4_095;
const DMODE = 493;
const FMODE = 420;

function encodeOct(num: number, n: number): string {
	const oct = num.toString(8);
	if (oct.length > n) {
		return `${SEVENS.slice(0, n)} `;
	} else {
		return `${ZEROS.slice(0, n - oct.length)}${oct} `;
	}
}

function checksum(block: Buffer): number {
	let sum = 8 * 32;
	for (let i = 0; i < 148; i++) {
		sum += block[i];
	}
	for (let j = 156; j < 512; j++) {
		sum += block[j];
	}
	return sum;
}

function toTypeflag(type: HeaderType): number {
	switch (type) {
		case "file":
			return 0;
		case "link":
			return 1;
		case "symlink":
			return 2;
		case "character-device":
			return 3;
		case "block-device":
			return 4;
		case "directory":
			return 5;
		case "fifo":
			return 6;
		case "contiguous-file":
			return 7;
	}

	return 0;
}

function encodeHeader(header: Header): Buffer {
	const buf = Buffer.alloc(512);

	let name = header.name;
	let prefix = "";

	if (Buffer.byteLength(name) !== name.length) {
		throw new Error(
			"utf-8 filename is only supported in PAX, we only support USTAR",
		);
	}

	// If a filename is over 100 characters then split it up if possible (requires a directory)
	while (Buffer.byteLength(name) > 100) {
		const i = name.indexOf("/");
		if (i === -1) {
			throw new Error(
				"filename is too long for USTAR and it was in no directory",
			);
		}

		prefix += prefix ? `/${name.slice(0, i)}` : name.slice(0, i);
		name = name.slice(i + 1);
	}

	if (Buffer.byteLength(name) > 100) {
		throw new Error("filename is too long for USTAR");
	}

	if (Buffer.byteLength(prefix) > 155) {
		throw new Error("prefix is too long for USTAR");
	}

	if (header.linkname !== undefined && Buffer.byteLength(header.linkname) > 100) {
		throw new Error("linkname is too long for USTAR");
	}

	buf.write(name);
	buf.write(encodeOct(header.mode & MASK, 6), 100);
	buf.write(encodeOct(header.uid, 6), 108);
	buf.write(encodeOct(header.gid, 6), 116);
	buf.write(encodeOct(header.size, 11), 124);
	buf.write(encodeOct(header.mtime.getTime() / 1_000 | 0, 11), 136);

	buf[156] = ZERO_OFFSET + toTypeflag(header.type);

	if (header.linkname !== undefined) {
		buf.write(header.linkname, 157);
	}

	buf.write(USTAR, 257);
	if (header.uname !== undefined) {
		buf.write(header.uname, 265);
	}
	if (header.gname !== undefined) {
		buf.write(header.gname, 297);
	}
	buf.write(encodeOct(header.devmajor || 0, 6), 329);
	buf.write(encodeOct(header.devminor || 0, 6), 337);

	if (prefix !== "") {
		buf.write(prefix, 345);
	}

	buf.write(encodeOct(checksum(buf), 6), 148);

	return buf;
}

export class TarWriter {
	constructor(stream: stream.Writable) {
		this.finalized = false;
		this.stream = stream;
	}

	private stream: stream.Writable;
	private finalized: boolean;

	private static normalizeHeader(partial: PartialHeader, size: number): Header {
		let mode = partial.mode;
		if (mode === undefined) {
			if (partial.type === "directory") {
				mode = DMODE;
			} else {
				mode = FMODE;
			}
		}

		return {
			name: partial.name,
			size,
			mode,
			mtime: partial.mtime ?? new Date(),
			type: partial.type ?? "file",
			linkname: partial.linkname,
			uid: partial.uid ?? 0,
			gid: partial.gid ?? 0,
			uname: partial.uname,
			gname: partial.gname,
			devmajor: partial.devmajor ?? 0,
			devminor: partial.devminor ?? 0,
		};
	}

	private overflow(size: number) {
		size &= 511;
		if (size > 0) {
			this.stream.write(END_OF_TAR.slice(0, 512 - size));
		}
	}

	public append(rawHeader: PartialHeader, rawBuffer: string | Buffer) {
		if (this.finalized) {
			throw new Error("Already finalized file");
		}

		const buffer: Buffer =
			rawBuffer instanceof Buffer ? rawBuffer : Buffer.from(rawBuffer);
		const header = TarWriter.normalizeHeader(rawHeader, buffer.length);

		this.stream.write(encodeHeader(header));
		this.stream.write(buffer);
		this.overflow(header.size);
	}

	public finalize(): Promise<void> {
		this.finalized = true;

		return new Promise((resolve, reject) => {
			const {stream} = this;

			stream.on(
				"close",
				() => {
					resolve();
				},
			);

			stream.on(
				"error",
				(err) => {
					reject(err);
				},
			);

			stream.write(END_OF_TAR);
			stream.end();
		});
	}
}
