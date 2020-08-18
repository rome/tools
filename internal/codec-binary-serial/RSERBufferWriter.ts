import {IntSize} from "./types";
import RSERBufferAssembler from "./RSERBufferAssembler";
import {utf8Encode} from "./utf8";

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

	public getWritableSize() {
		return this.buffer.byteLength - this.writeOffset;
	}

	private assertWritableSize(size: number) {
		const remaining = this.getWritableSize();

		if (remaining < size) {
			throw new Error(
				`Wanted to write ${size} bytes but only have ${remaining} remaining`,
			);
		}
	}

	public appendBytes(buf: Uint8Array) {
		const size = buf.byteLength;
		this.assertWritableSize(size);
		this.bytes.set(buf, this.writeOffset);
		this.writeOffset += size;
	}

	protected appendString(text: string, size: number) {
		utf8Encode(text, this.bytes, this.writeOffset, size);
		this.writeOffset += size;
	}

	protected writeCode(code: number) {
		this.writeByte(code);
	}

	protected writeByte(value: number) {
		this.assertWritableSize(1);
		this.view.setInt8(this.writeOffset, value);
		this.writeOffset++;
	}

	protected writeInt(value: bigint | number, size: IntSize) {
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

	protected writeFloat(value: number) {
		this.assertWritableSize(8);
		this.view.setFloat64(this.writeOffset, value);
		this.writeOffset += 8;
	}
}
