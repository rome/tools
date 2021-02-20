import {ParsedPath, parsePathSegments} from "../parse";
import {BasePath, FilePathMemo} from "./BasePath";
import {AnyFilePath, AnyPath, PathFormatOptions, PathSegments} from "../types";
import {createRelativePath} from "../factories";
import RelativePath from "./RelativePath";
import {createFilePath} from "../factories";
import {FSWatcher} from "@internal/fs";
import {AbsoluteFilePathSet} from "../collections";
import fs = require("fs");

// This file contains some wrappers around Node's fs module
// NOTE We don't bother using Node's built-in fs promise functions at all. They already contain a level of indirection to callbacks.

type DataCallback<Data> = (err: null | Error, data: Data) => void;
type VoidCallback = (err: null | Error) => void;

export default class AbsoluteFilePath extends BasePath<AbsoluteFilePath> {
	public [Symbol.toStringTag] = "AbsoluteFilePath";

	private chain: undefined | (AbsoluteFilePath[]);

	protected _assert(): AbsoluteFilePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPath,
		opts?: FilePathMemo<AbsoluteFilePath>,
	): AbsoluteFilePath {
		return new AbsoluteFilePath(parsed, opts);
	}

	public isFilePath(): this is AnyFilePath {
		return true;
	}

	public assertFilePath(): AnyFilePath {
		return this;
	}

	public isAbsolute(): this is AbsoluteFilePath {
		return true;
	}

	public assertAbsolute(): AbsoluteFilePath {
		return this;
	}

	public getChain(): AbsoluteFilePath[] {
		if (this.chain !== undefined) {
			return this.chain;
		}

		const paths: AbsoluteFilePath[] = [];
		this.chain = paths;

		// We use getParent here so we can reuse as much memoized information as possible
		let target: AbsoluteFilePath = this;
		while (true) {
			paths.push(target);
			if (target.isRoot()) {
				break;
			} else {
				target = target.getParent();
			}
		}

		return paths;
	}

	public resolve(other: string | AnyFilePath): AbsoluteFilePath;
	public resolve(other: AnyPath): Exclude<AnyPath, RelativePath>;
	public resolve(
		other: string | AnyPath,
	): AbsoluteFilePath | Exclude<AnyPath, RelativePath> {
		if (typeof other === "string") {
			other = createFilePath(other);
		}
		if (!other.isRelative()) {
			return other;
		}

		return new AbsoluteFilePath(
			parsePathSegments(
				[...this.getSegments(), ...other.getSegments()],
				"absolute",
				{
					explicitDirectory: other.isExplicitDirectory(),
				},
			),
		);
	}

	public toExplicitRelative(): AbsoluteFilePath {
		return this;
	}

	public relativeForce(otherRaw: AbsoluteFilePath | RelativePath): RelativePath {
		return this.relative(otherRaw).assertRelative();
	}

	public relative(
		otherRaw: AbsoluteFilePath | RelativePath,
	): AbsoluteFilePath | RelativePath {
		const other = this.resolve(otherRaw);

		if (other.equal(this)) {
			return createRelativePath(".");
		}

		const absolute = this.getSegments().slice();
		const relative = other.getSegments().slice();

		// Impossible to relativize two absolute paths with different roots
		if (absolute[0] !== relative[0]) {
			return other;
		}

		// Remove common starting segments
		while (absolute[0] === relative[0]) {
			absolute.shift();
			relative.shift();
		}

		let finalSegments: PathSegments = [];
		for (let i = 0; i < absolute.length; i++) {
			finalSegments.push("..");
		}
		finalSegments = finalSegments.concat(relative);

		return new RelativePath(parsePathSegments(finalSegments, "relative"));
	}

	public format({cwd, home}: PathFormatOptions = {}): string {
		const filename = this.join();
		const names: string[] = [];
		names.push(filename);

		// Get a path relative to HOME
		if (home !== undefined && this.isRelativeTo(home)) {
			// Path starts with the home directory, so let's trim it off
			const relativeToHome = home.relative(this._assert());

			// Add tilde and push it as a possible name
			// We construct this manually to get around the segment normalization which would explode ~
			names.push(
				new RelativePath({
					hint: "relative",
					segments: ["~", ...relativeToHome.getSegments()],
					absoluteType: "posix",
					absoluteTarget: undefined,
					explicitDirectory: this.parsed.explicitDirectory,
					explicitRelative: false,
				}).join(),
			);
		}

		// Get a path relative to the cwd
		if (cwd !== undefined) {
			names.push(cwd.relative(this).join());
		}

		// Get the shortest name
		const human = names.sort((a, b) => a.length - b.length)[0];
		if (human === "") {
			return "./";
		} else {
			return human;
		}
	}

	public watch(
		options:
			| {
					encoding?: BufferEncoding | null;
					persistent?: boolean;
					recursive?: boolean;
				}
			| undefined,
		listener?: (event: string, filename: null | string) => void,
	): FSWatcher {
		return fs.watch(this.join(), options, listener);
	}
	
	public readFile(): Promise<Buffer> {
		return this.promisifyData(
			(filename, callback) => fs.readFile(filename, callback),
		);
	}
	
	public async readFileText(): Promise<string> {
		return (await this.readFile()).toString();
	}
	
	// Return value is meant to be consumed via ParserOptions
	public async readFileTextMeta(): Promise<{
		path: AbsoluteFilePath;
		input: string;
	}> {
		return {
			input: (await this.readFile()).toString(),
			path: this,
		};
	}
	
	public writeFile(
		content: string | NodeJS.ArrayBufferView,
	): Promise<void> {
		return this.promisifyVoid(
			(filename, callback) => fs.writeFile(filename, content, callback),
		);
	}
	
	public copyFileTo(
		dest: AbsoluteFilePath,
	): Promise<void> {
		return this.promisifyVoid(
			(src, callback) => fs.copyFile(src, dest.join(), callback),
		);
	}
	
	public readDirectory(): Promise<AbsoluteFilePathSet> {
		return this.wrapReject(
			new Promise((resolve, reject) => {
				fs.readdir(
					this.join(),
					(err, files) => {
						if (err === null) {
							resolve(new AbsoluteFilePathSet(files.sort().map((basename) => {
								return this.append(basename);
							})));
						} else {
							reject(err);
						}
					},
				);
			}),
			1,
		);
	}
	
	public lstat(): Promise<fs.BigIntStats> {
		return this.promisifyData(
			(filename, callback) =>
				(fs.lstat as typeof fs.stat)(filename, {bigint: true}, callback)
			,
		);
	}

	// Wrapping await in parens is gross so offer this to make other code nicer
	public async notExists(): Promise<boolean> {
		return !(await this.exists());
	}
	
	public exists(): Promise<boolean> {
		return new Promise((resolve) => {
			fs.exists(
				this.join(),
				(exists) => {
					resolve(exists);
				},
			);
		});
	}
	
	public removeFile(): Promise<void> {
		return this.promisifyVoid(
			(filename, callback) =>
				fs.unlink(
					filename,
					(err) => {
						if (err != null && err.code !== "ENOENT") {
							callback(err);
						} else {
							callback(null);
						}
					},
				)
			,
		);
	}
	
	// We previously just use fs.rmdir with the `recursive: true` flag but it was added in Node 12.10 and we need to support 12.8.1
	// NB: There are probably race conditions, we could switch to openFile and openDirectory if it's a problem
	// https://github.com/rome/tools/issues/1001
	public async removeDirectory(): Promise<void> {
		if (await this.notExists()) {
			return;
		}
	
		// Delete all inner files
		for (const subpath of await this.readDirectory()) {
			const stats = await subpath.lstat();
			if (stats.isDirectory()) {
				await subpath.removeDirectory();
			} else {
				await subpath.removeFile();
			}
		}
	
		// Remove directory with all files deleted
		return this.promisifyVoid(
			(filename, callback) => fs.rmdir(filename, callback),
		);
	}
	
	public createDirectory(): Promise<void> {
		return this.promisifyVoid(
			(filename, callback) =>
				fs.mkdir(
					filename,
					{
						recursive: true,
					},
					callback,
				)
			,
		);
	}
	
	public openFile(
		flags: fs.OpenMode = "r",
		mode?: fs.Mode,
	): Promise<fs.promises.FileHandle> {
		return fs.promises.open(this.join(), flags, mode);
	}
	
	public openDirectory(
		opts: fs.OpenDirOptions = {},
	): Promise<fs.Dir> {
		return this.promisifyData(
			(filename, callback) => fs.opendir(filename, opts, callback),
		);
	}
	
	public createWriteStream(
		opts?: Parameters<typeof fs.createWriteStream>[1],
	): fs.WriteStream {
		return fs.createWriteStream(this.join(), opts);
	}
	
	public createReadStream(
		opts?: Parameters<typeof fs.createReadStream>[1],
	): fs.ReadStream {
		return fs.createReadStream(this.join(), opts);
	}
	
	// Super special sync methods that we should only use sparingly if there's absolutely no way to do them async
	
	public readFileTextSync(): string {
		return fs.readFileSync(this.join(), "utf8");
	}
	
	public lstatSync(): fs.Stats {
		return fs.lstatSync(this.join());
	}

	// Internal helpers
	private promisifyData<Data>(
		factory: (path: string, callback: DataCallback<Data>) => void,
	): Promise<Data> {
		return this.wrapReject(
			new Promise((resolve, reject) => {
				factory(
					this.join(),
					(err, data) => {
						if (err === null) {
							resolve(data);
						} else {
							reject(err);
						}
					},
				);
			}),
			2,
		);
	}

	private promisifyVoid(
		factory: (path: string, callback: VoidCallback) => void,
	): Promise<void> {
		return this.wrapReject(
			new Promise((resolve, reject) => {
				factory(
					this.join(),
					(err) => {
						if (err === null) {
							resolve();
						} else {
							reject(err);
						}
					},
				);
			}),
			2,
		);
	}

	private wrapReject<T>(promise: Promise<T>, addFrames: number): Promise<T> {
		addFrames;
		return promise;
	}
}