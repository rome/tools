import {AbsoluteFilePath, AbsoluteFilePathMap} from "@internal/path";
import {readFile} from "./index";

export default class CachedFileReader {
	constructor() {
		this.cached = new AbsoluteFilePathMap();
	}

	private cached: AbsoluteFilePathMap<Buffer | Promise<Buffer>>;

	cache(path: AbsoluteFilePath, buffer: Buffer) {
		this.cached.set(path, buffer);
	}

	async readFile(path: AbsoluteFilePath): Promise<Buffer> {
		const cached = this.cached.get(path);
		if (cached !== undefined) {
			return cached;
		}

		const promise = readFile(path);
		this.cached.set(path, promise);

		const buff = await promise;
		this.cache(path, buff);
		return buff;
	}

	readFileText(path: AbsoluteFilePath): Promise<string> {
		return this.readFile(path).then((buff) => buff.toString());
	}
}
