import {ParsedPath, ParsedPathDataURI} from "../types";
import {ReadableBasePath, FilePathMemo} from "../bases";
import { decodeUTF8, encodeBase64, getArrayBuffer } from "@internal/binary";
import stream = require("stream");

export default class DataURIPath extends ReadableBasePath<ParsedPathDataURI, DataURIPath> {
	protected frozen = true;
	public [Symbol.toStringTag] = "DataURIPath";

	protected _assert(): DataURIPath {
		return this;
	}

	protected _getUnique(): DataURIPath {
		return this;
	}

	protected _fork(parsed: ParsedPathDataURI, opts: FilePathMemo<DataURIPath>): DataURIPath {
		throw new Error("Cannot fork a DataURIPath");
	}

	protected _equalAbsolute(other: ParsedPath): boolean {
		const {parsed} = this;

		if (other.type !== "data") {
			return false;
		}

		if (parsed.mime !== other.mime) {
			return false;
		}

		if (typeof parsed.data === "string") {
			if (other.data !== parsed.data) {
				return false;
			}
		} else {
			if (typeof other.data === "string") {
				return false;
			}

			if (parsed.data.byteLength !== other.data.byteLength) {
				return false;
			}

			const a = new Int8Array(parsed.data);
			const b = new Int8Array(other.data);
			for (let i = 0; i < a.byteLength; i++) {
				if (a[i] !== b[i]) {
					return false;
				}
			}
		}

		return true;
	}

	protected _join(): string {
		const {mime, data} = this.parsed;
		let joined = `data:${mime}`;

		if (typeof data === "string") {
			joined += `,${data}`;
		} else {
			joined += ";base64,";
			joined += encodeBase64(data);
		}

		return joined;
	}

	protected _format(): string {
		const joined = this._join();
		if (joined.length > 100) {
			// Truncate as we could have a huge file
			return `${joined.slice(0, 100)} ... ${joined.length - 100} more characters`;
		} else {
			return joined;
		}
	}

	public isDataURI(): this is DataURIPath {
		return true;
	}

	public assertDataURI(): DataURIPath {
		return this;
	}

	public assertReadable(): DataURIPath {
		return this;
	}

	public isReadable(): this is DataURIPath {
		return true;
	}

	public async exists(): Promise<boolean> {
		return true;
	}

	public async readFile(): Promise<ArrayBuffer> {
		return getArrayBuffer(this.parsed.data);
	}

	public async readFileText(): Promise<string> {
		return decodeUTF8(this.parsed.data);
	}

	public createReadStream(): stream.Readable {
		const {data} = this.parsed;
		const readable = new stream.Readable({
			read: () => {
				if (typeof data === "string") {
					readable.push(data);
				} else {
					readable.push(Buffer.from(data));
				}
				readable.push(null);
			},
		});
		return readable;
	}
}

DataURIPath.prototype[Symbol.toStringTag] = "DataURIPath";