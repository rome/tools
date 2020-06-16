import {AbsoluteFilePath} from "@romejs/path";

export class FileNotFound extends Error {
	constructor(path: AbsoluteFilePath, message?: string) {
		super(message === undefined ? path.join() : `${path.join()}: ${message}`);
		this.name = "FileNotFound";
		this.path = path;
	}

	path: AbsoluteFilePath;

	static async maybeAllowMissing<T>(
		allow: undefined | boolean,
		path: AbsoluteFilePath,
		factory: () => T | Promise<T>,
	): Promise<undefined | T> {
		if (allow) {
			return FileNotFound.allowMissing(path, factory);
		} else {
			return factory();
		}
	}

	static async allowMissing<T>(
		path: AbsoluteFilePath,
		factory: () => T | Promise<T>,
	): Promise<undefined | T> {
		try {
			return await factory();
		} catch (err) {
			if (err instanceof FileNotFound && err.path.equal(path)) {
				return undefined;
			} else {
				throw err;
			}
		}
	}
}
