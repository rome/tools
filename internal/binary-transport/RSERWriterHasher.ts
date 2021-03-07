import RSERWriterBase from "./RSERWriterBase";
import {IntSize, writeInt} from "./utils";
import crypto = require("crypto");

export default class RSERWriterHasher extends RSERWriterBase {
	constructor(algorithm: string = "sha256") {
		super();
		this.hash = crypto.createHash(algorithm);
	}

	private hash: crypto.Hash;

	protected encodeSize() {
		// Size ints are not needed for hashing as we just care about the values
	}

	protected encodeStringValue(value: string) {
		// Skip length counting
		this.writeString(value);
	}

	public digest(): string {
		return this.hash.digest("hex");
	}

	protected writeByte(value: number) {
		this.hash.update(new Uint8Array([value]));
	}

	protected writeInt(value: bigint | number, size: IntSize) {
		const buf = new ArrayBuffer(size);
		const view = new DataView(buf);
		writeInt(value, size, 0, view);
		this.hash.update(view);
	}

	protected writeFloat(value: number) {
		const buf = new ArrayBuffer(8);
		const view = new DataView(buf);
		view.setFloat64(0, value);
		this.hash.update(view);
	}

	protected writeString(buf: string) {
		this.hash.update(buf, "utf8");
	}

	public writeBytes(buf: Uint8Array) {
		this.hash.update(buf);
	}
}
