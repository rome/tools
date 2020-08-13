import {IntSize} from "./types";
import RSERBufferAssembler from "./RSERBufferAssembler";

const textEncoder = new TextEncoder();

export default class RSERBufferWriter extends RSERBufferAssembler {
	constructor(buffer: ArrayBuffer) {
		super();
		this.totalSize = buffer.byteLength;
		this.writeOffset = 0;
		this.buffer = buffer;
		this.array = new Uint8Array(buffer);
		this.view = new DataView(buffer);
	}

	static allocate(size: number): RSERBufferWriter {
		// NB: Are there any downsides to this always be a SharedArrayBuffer?
		return new RSERBufferWriter(new SharedArrayBuffer(size));
	}

	array: Uint8Array;
	buffer: ArrayBuffer;
	view: DataView;
	writeOffset: number;

	getWritableSize() {
		return this.buffer.byteLength - this.writeOffset;
	}

	assertWritableSize(size: number) {
		const remaining = this.getWritableSize();

		if (remaining < size) {
			throw new Error(
				`Wanted to write ${size} bytes but only have ${remaining} remaining`,
			);
		}
	}

	appendArray(buf: Uint8Array) {
		const size = buf.byteLength;
		this.assertWritableSize(size);
		this.array.set(buf, this.writeOffset);
		this.writeOffset += size;
	}

	appendString(text: string) {
		this.appendArray(textEncoder.encode(text));
	}

	writeCode(code: number) {
		this.writeByte(code);
	}

	writeByte(value: number) {
		this.assertWritableSize(1);
		this.view.setInt8(this.writeOffset, value);
		this.writeOffset++;
	}

	writeInt(value: bigint | number, size: IntSize) {
		this.assertWritableSize(size);

		if (typeof value === "bigint") {
			if (size === 8) {
				this.view.setBigInt64(this.writeOffset, value);
				this.writeOffset += size;
				return;
			} else {
				throw new Error(`Expected size 8 for bigint but got ${size}`);
			}
		}

		switch (size) {
			case 1: {
				this.view.setInt8(this.writeOffset, value);
				break;
			}

			case 2: {
				this.view.setInt16(this.writeOffset, value);
				break;
			}

			case 4: {
				this.view.setInt32(this.writeOffset, value);
				break;
			}

			default:
				throw new Error(`Unsupported integer size ${size}`);
		}

		this.writeOffset += size;
	}

	writeFloat(value: number) {
		this.assertWritableSize(8);
		this.view.setFloat64(this.writeOffset, value);
		this.writeOffset += 8;
	}
}
