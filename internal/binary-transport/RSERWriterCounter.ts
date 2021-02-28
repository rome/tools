import RSERWriterBase from "./RSERWriterBase";
import {IntSize} from "./utils";

export default class RSERWriterCounter extends RSERWriterBase {
	constructor() {
		super();
		this.totalSize = 0;
	}

	public totalSize: number;

	protected writeByte() {
		this.totalSize += 1;
	}

	protected writeInt(value: bigint | number, size: IntSize) {
		this.totalSize += size;
	}

	protected writeFloat() {
		this.totalSize += 8;
	}

	protected writeString(buf: string, size: number) {
		this.totalSize += size;
	}

	public writeBytes(buf: Uint8Array) {
		this.totalSize += buf.byteLength;
	}
}
