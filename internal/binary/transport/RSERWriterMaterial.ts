import {IntSize, writeInt} from "./utils";
import RSERWriterBase from "./RSERWriterBase";
import {utf8Encode} from "../utf8";
import RSERBufferParser from "./RSERBufferParser";
import RSERWriterCounter from "./RSERWriterCounter";

export default class RSERWriterMaterial extends RSERWriterBase {
	constructor(buffer: ArrayBuffer, counter: RSERWriterCounter) {
		super();
		this.references = counter.references;
		this.totalSize = buffer.byteLength;
		this.writeOffset = 0;
		this.buffer = buffer;
		this.bytes = new Uint8Array(buffer);
		this.view = new DataView(buffer);
	}

	public totalSize: number;
	public writeOffset: number;
	public bytes: Uint8Array;
	public buffer: ArrayBuffer;
	public view: DataView;

	static allocate(size: number): RSERWriterMaterial {
		return new RSERWriterMaterial(
			new ArrayBuffer(size),
			new RSERWriterCounter(),
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

	public writeBytes(buf: Uint8Array) {
		if (buf.byteLength === 0) {
			return;
		}

		const offset = this.assertWritableSize(buf.byteLength);
		this.bytes.set(buf, offset);
	}

	protected writeString(text: string, size: number) {
		if (size === 0) {
			return;
		}

		const offset = this.assertWritableSize(size);
		utf8Encode(text, this.bytes, offset, size);
	}

	protected writeByte(value: number) {
		const offset = this.assertWritableSize(1);
		this.view.setInt8(offset, value);
	}

	protected writeInt(value: bigint | number, size: IntSize) {
		const offset = this.assertWritableSize(size);
		writeInt(value, size, offset, this.view);
	}

	protected writeFloat(value: number) {
		const offset = this.assertWritableSize(8);
		this.view.setFloat64(offset, value);
	}
}
