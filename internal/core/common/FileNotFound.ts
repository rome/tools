import {AbsoluteFilePath} from "@internal/path";

export type MissingFileReturn<T> =
	| {
			missing: false;
			value: T;
		}
	| {
			missing: true;
			value: undefined;
		};

export class FileNotFound extends Error {
	constructor(path: AbsoluteFilePath, message?: string) {
		super(
			message === undefined
				? `${path.join()} not found`
				: `${path.join()}: ${message}`,
		);
		this.name = "FileNotFound";
		this.path = path;
	}

	public path: AbsoluteFilePath;

	public static async maybeAllowMissing<T>(
		allow: undefined | boolean,
		path: AbsoluteFilePath,
		factory: () => T | Promise<T>,
	): Promise<MissingFileReturn<T>> {
		if (allow) {
			return FileNotFound.allowMissing(path, factory);
		} else {
			return {
				value: await factory(),
				missing: false,
			};
		}
	}

	public static async allowMissing<T>(
		path: AbsoluteFilePath,
		factory: () => T | Promise<T>,
	): Promise<MissingFileReturn<T>> {
		try {
			return {
				value: await factory(),
				missing: false,
			};
		} catch (err) {
			if (err instanceof FileNotFound && err.path.equal(path)) {
				return {missing: true, value: undefined};
			} else {
				throw err;
			}
		}
	}
}
