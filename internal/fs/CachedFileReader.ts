import { decodeUTF8, getArrayBuffer } from "@internal/binary";
import {AbsoluteFilePath, AbsoluteFilePathMap} from "@internal/path";

export default class CachedFileReader {
	constructor() {
		this.cached = new AbsoluteFilePathMap();
	}

	private cached: AbsoluteFilePathMap<string | ArrayBuffer | Promise<string | ArrayBuffer>>;

	public cache(path: AbsoluteFilePath, view: string | ArrayBuffer) {
		this.cached.set(path, view);
	}

	public async readFile(path: AbsoluteFilePath): Promise<ArrayBuffer> {
		const cached = this.cached.get(path);
		if (cached !== undefined) {
			return getArrayBuffer(await cached);
		}

		const promise = path.readFile();
		this.cached.set(path, promise);

		const buff = await promise;
		this.cache(path, buff);
		return buff;
	}

	public async readFileText(path: AbsoluteFilePath): Promise<string> {
		const cached = this.cached.get(path);
		if (cached !== undefined) {
			return decodeUTF8(await cached);
		}

		return this.readFile(path).then((buff) => decodeUTF8(buff));
	}
}
