import {AbsoluteFilePath} from "@internal/path";
import {NodeSystemError} from "@internal/node";

export type MissingFileReturn<T> =
	| {
			missing: false;
			value: T;
		}
	| {
			missing: true;
			value: undefined;
		};

export class FileNotFound extends Error implements NodeSystemError {
	constructor(path: AbsoluteFilePath, suffixMessage?: string) {
		super(
			suffixMessage === undefined
				? `${path.join()} not found`
				: `${path.join()}: ${suffixMessage}`,
		);
		this.suffixMessage = suffixMessage;
		this.name = "FileNotFound";
		this._path = path;
	}

	public suffixMessage: undefined | string;
	public _path: AbsoluteFilePath;

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
			if (err instanceof FileNotFound && err._path.equal(path)) {
				return {missing: true, value: undefined};
			} else {
				throw err;
			}
		}
	}
}
