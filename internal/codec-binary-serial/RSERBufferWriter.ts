import {IntSize} from "./types";
import RSERBufferAssembler from "./RSERBufferAssembler";
import {utf8Encode} from "./utf8";
import {RSERBufferParser} from ".";

export default class RSERBufferWriter extends RSERBufferAssembler {
	constructor(buffer: ArrayBuffer, assembler: RSERBufferAssembler) {
		super();
		this.references = assembler.references;
		this.totalSize = buffer.byteLength;
		this.writeOffset = 0;
		this.buffer = buffer;
		this.bytes = new Uint8Array(buffer);
		this.view = new DataView(buffer);
	}

	public writeOffset: number;
	public bytes: Uint8Array;
	public buffer: ArrayBuffer;
	public view: DataView;

	static allocate(size: number): RSERBufferWriter {
		return new RSERBufferWriter(
			new ArrayBuffer(size),
			new RSERBufferAssembler(),
		);
	}

	protected onReferenceCreate() {}

	public toParser(): RSERBufferParser {
		return new RSERBufferParser(this.view);
	}

	public getWritableSize() {
		return this.buffer.byteLength - this.writeOffset;
	}

	private assertWritableSize(size: number): number {
		const remaining = this.getWritableSize();

		if (remaining < size) {
			throw new Error(
				`Wanted to write ${size} bytes but only have ${remaining} remaining`,
			);
		} else {
			const currWriteOffset = this.writeOffset;
			this.writeOffset += size;
			return currWriteOffset;
		}
	}

	public appendBytes(buf: Uint8Array) {
		if (buf.byteLength === 0) {
			return;
		}

		const offset = this.assertWritableSize(buf.byteLength);
		this.bytes.set(buf, offset);
	}

	protected appendString(text: string, size: number) {
		if (size === 0) {
			return;
		}

		const offset = this.assertWritableSize(size);
		utf8Encode(text, this.bytes, offset, size);
	}

	protected writeCode(code: number) {
		this.writeByte(code);
	}

	protected writeByte(value: number) {
		const offset = this.assertWritableSize(1);
		this.view.setInt8(offset, value);
	}

	protected writeInt(value: bigint | number, size: IntSize) {
		const offset = this.assertWritableSize(size);

		if (typeof value === "bigint") {
			if (size === 8) {
				this.view.setBigInt64(offset, value);
				return;
			} else {
				throw new Error(`Expected size 8 for bigint but got ${size}`);
			}
		}

		switch (size) {
			case 1: {
				this.view.setInt8(offset, value);
				break;
			}

			case 2: {
				this.view.setInt16(offset, value);
				break;
			}

			case 4: {
				this.view.setInt32(offset, value);
				break;
			}

			default:
				throw new Error(`Unsupported integer size ${size}`);
		}
	}

	protected writeFloat(value: number) {
		const offset = this.assertWritableSize(8);
		this.view.setFloat64(offset, value);
	}
}
