import {normalizeRelativeSegments, parseRelativePathSegments} from "../parse";
import {FilePathMemo, ReadableBasePath} from "../bases";
import {
	FilePath,
	ParsedPath,
	ParsedPathAbsolute,
	Path,
	PathFormatOptions,
	PathSegments,
} from "../types";
import RelativePath from "./RelativePath";
import {createFilePath} from "../factories";
import {FSWatcher} from "@internal/fs";
import {AbsoluteFilePathSet, MixedPathMap} from "../collections";
import fs = require("fs");

export default class AbsoluteFilePath
	extends ReadableBasePath<ParsedPathAbsolute, AbsoluteFilePath> {
	constructor(
		parsed: ParsedPathAbsolute,
		memo: FilePathMemo<AbsoluteFilePath> = {},
	) {
		super(parsed, memo);
		this.memoizedRelative = undefined;
	}

	public [Symbol.toStringTag] = "AbsoluteFilePath";

	// We do not always initialize it to save on a bunch of allocations if relative() isn't used
	private memoizedRelative:
		| undefined
		| MixedPathMap<AbsoluteFilePath | RelativePath>;

	protected _assert(): AbsoluteFilePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPathAbsolute,
		opts?: FilePathMemo<AbsoluteFilePath>,
	): AbsoluteFilePath {
		return new AbsoluteFilePath(parsed, opts);
	}

	protected _getUnique(): AbsoluteFilePath {
		return this;
	}

	protected _equalAbsolute(other: ParsedPath): boolean {
		const {parsed} = this;

		switch (parsed.type) {
			case "absolute-windows-drive":
				return (
					other.type === "absolute-windows-drive" &&
					other.letter === parsed.letter
				);

			case "absolute-windows-unc":
				return (
					other.type === "absolute-windows-unc" &&
					other.servername === parsed.servername
				);

			case "absolute-unix":
				return other.type === "absolute-unix";

			default:
				return false;
		}
	}

	protected _join() {
		const relative = this.getDisplaySegments();
		const {parsed} = this;

		switch (parsed.type) {
			case "absolute-windows-drive":
				return [`${parsed.letter}:`, ...relative].join("\\");

			case "absolute-windows-unc":
				return [`\\\\${parsed.servername}`, ...relative].join("\\");

			case "absolute-unix":
				return `/${relative.join("/")}`;
		}
	}

	protected _format({cwd, home}: PathFormatOptions = {}): string {
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
					type: "relative",
					explicitRelative: false,
					explicitDirectory: this.parsed.explicitDirectory,
					relativeSegments: ["~", ...relativeToHome.getSegments()],
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

	public isFilePath(): this is FilePath {
		return true;
	}

	public assertFilePath(): FilePath {
		return this;
	}

	public isAbsolute(): this is AbsoluteFilePath {
		return true;
	}

	public assertAbsolute(): AbsoluteFilePath {
		return this;
	}

	public assertReadable(): AbsoluteFilePath {
		return this;
	}

	public isReadable(): this is AbsoluteFilePath {
		return true;
	}

	public *getChain(reverse: boolean = false): Iterable<AbsoluteFilePath> {
		if (!reverse) {
			yield this;
		}

		if (!this.isRoot()) {
			yield* this.getParent().getChain(reverse);
		}

		if (reverse) {
			yield this;
		}
	}

	public resolve(other: string | FilePath): AbsoluteFilePath;
	public resolve(other: Path): Exclude<Path, RelativePath>;
	public resolve(
		other: string | Path,
	): AbsoluteFilePath | Exclude<Path, RelativePath>;
	public resolve(
		other: string | Path,
	): AbsoluteFilePath | Exclude<Path, RelativePath> {
		if (typeof other === "string") {
			other = createFilePath(other);
		}
		if (!other.isRelative()) {
			return other;
		}

		return new AbsoluteFilePath({
			...this.parsed,
			...normalizeRelativeSegments([
				...this.relativeSegments,
				...other.getSegments(),
			]),
			explicitDirectory: other.isExplicitDirectory(),
		});
	}

	public relativeForce(otherRaw: AbsoluteFilePath | RelativePath): RelativePath {
		return this.relative(otherRaw).assertRelative();
	}

	public relative(
		otherRaw: AbsoluteFilePath | RelativePath,
	): AbsoluteFilePath | RelativePath {
		if (this.memoizedRelative !== undefined) {
			const memoized = this.memoizedRelative.get(otherRaw);
			if (memoized !== undefined) {
				return memoized;
			}
		}

		const other = this.resolve(otherRaw);

		if (other.equal(this)) {
			return new RelativePath({
				type: "relative",
				explicitDirectory: false,
				explicitRelative: true,
				relativeSegments: [],
			});
		}

		// Impossible to relativize two absolute paths with different absolute targets
		if (!this.equalAbsolute(other)) {
			return other;
		}

		const absolute = this.getSegments().slice();
		const relative = other.getSegments().slice();

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

		const path = new RelativePath(parseRelativePathSegments(finalSegments));

		// Store in memoize map
		if (this.memoizedRelative === undefined) {
			this.memoizedRelative = new MixedPathMap();
		}
		this.memoizedRelative.set(otherRaw, path);

		return path;
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

	public async readFile(): Promise<ArrayBuffer> {
		const data = await fs.promises.readFile(this.join());
		return data.buffer;
	}

	public async readFileText(): Promise<string> {
		return fs.promises.readFile(this.join(), "utf8");
	}

	public async writeFile(
		content: string | ArrayBuffer | fs.ReadStream,
	): Promise<void> {
		if (content instanceof fs.ReadStream) {
			return new Promise((resolve, reject) => {
				const writeStream = this.createWriteStream();
				content.pipe(writeStream);

				writeStream.on(
					"error",
					(err) => {
						reject(err);
					},
				);

				writeStream.on(
					"close",
					() => {
						resolve();
					},
				);
			});
		} else {
			let buff;
			if (content instanceof ArrayBuffer) {
				buff = new Uint8Array(content);
			} else {
				buff = content;
			}
			await fs.promises.writeFile(this.join(), buff);
		}
	}

	public copyFileTo(dest: AbsoluteFilePath): Promise<void> {
		return fs.promises.copyFile(this.join(), dest.join());
	}

	public async readDirectory(): Promise<AbsoluteFilePathSet> {
		const files = await fs.promises.readdir(this.join());
		return new AbsoluteFilePathSet(
			files.sort().map((basename) => {
				return this.append(basename);
			}),
		);
	}

	public lstat(): Promise<fs.BigIntStats> {
		// @ts-ignore: Incomplete stdlib type definition
		return fs.promises.lstat(this.join(), {bigint: true});
	}

	// Wrapping await in parens is gross so offer this to make other code nicer
	public async notExists(): Promise<boolean> {
		return !(await this.exists());
	}

	public async exists(): Promise<boolean> {
		try {
			await fs.promises.access(this.join());
			return true;
		} catch (err) {
			return false;
		}
	}

	public async removeFile(): Promise<void> {
		try {
			await fs.promises.unlink(this.join());
		} catch (err) {
			if (err.code !== "ENOENT") {
				throw err;
			}
		}
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
		await fs.promises.rmdir(this.join());
	}

	public async createDirectory(): Promise<void> {
		await fs.promises.mkdir(
			this.join(),
			{
				recursive: true,
			},
		);
	}

	public openFile(
		flags: fs.OpenMode = "r",
		mode?: fs.Mode,
	): Promise<fs.promises.FileHandle> {
		return fs.promises.open(this.join(), flags, mode);
	}

	public openDirectory(opts: fs.OpenDirOptions = {}): Promise<fs.Dir> {
		return fs.promises.opendir(this.join(), opts);
	}

	public createWriteStream(): fs.WriteStream {
		return fs.createWriteStream(this.join());
	}

	public createReadStream(): fs.ReadStream {
		return fs.createReadStream(this.join());
	}

	// Super special sync methods that we should only use sparingly if there's absolutely no way to do them async

	public readFileTextSync(): string {
		return fs.readFileSync(this.join(), "utf8");
	}

	public lstatSync(): fs.Stats {
		return fs.lstatSync(this.join());
	}
}

AbsoluteFilePath.prototype[Symbol.toStringTag] = "AbsoluteFilePath";
