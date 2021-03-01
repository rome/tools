import {
	FilePath,
	ParsedPath,
	Path,
	PathFormatOptions,
	PathSegments,
	ReadablePath,
} from "./types";
import AbsoluteFilePath from "./classes/AbsoluteFilePath";
import RelativePath from "./classes/RelativePath";
import UIDPath from "./classes/UIDPath";
import URLPath from "./classes/URLPath";
import DataURIPath from "./classes/DataURIPath";
import {normalizeRelativeSegments, splitPathSegments} from "./parse";
import {enhanceNodeInspectClass} from "@internal/node";
import {equalArray} from "@internal/typescript-helpers";
import stream = require("stream");

export type FilePathMemo<Super> = {
	ext?: string;
	parent?: Super;
};

function getExtension(basename: string): string {
	const match = basename.match(/\.(.*?)$/);
	if (match == null) {
		return "";
	} else {
		return match[0];
	}
}

export abstract class BasePath<
	SuperParsed extends ParsedPath = ParsedPath,
	Super extends Path = Path
> {
	constructor(parsed: SuperParsed, memo: FilePathMemo<Super> = {}) {
		this.relativeSegments = parsed.relativeSegments;
		this.parsed = parsed;
		this.memo = memo;
		this.memoizedJoin = undefined;
		this.memoizedFormatOptions = undefined;
		this.memoizedFormat = undefined;
		this.memoizedChildren = new Map();
		this.memoizedUnique = undefined;
	}

	public parsed: SuperParsed;
	public [Symbol.toStringTag]: string;

	protected relativeSegments: PathSegments;
	protected memo: FilePathMemo<Super>;

	// Memoize children when append() is called with strings
	private memoizedChildren: Map<string, Super>;

	private memoizedJoin: undefined | string;
	private memoizedUnique: undefined | Super;

	// Allow caching a single formatted value for an options object
	private memoizedFormat: undefined | string;
	private memoizedFormatOptions:
		| undefined
		| [string, undefined | AbsoluteFilePath, undefined | AbsoluteFilePath];

	protected abstract _assert(): Super;
	protected abstract _join(): string;
	protected abstract _equalAbsolute(parsed: ParsedPath): boolean;
	protected abstract _fork(
		parsed: SuperParsed,
		memo?: FilePathMemo<Super>,
	): Super;
	protected abstract _getUnique(): Super;
	protected abstract _format(opts?: PathFormatOptions): string;

	protected _forkAppend(segments: PathSegments): Super {
		if (segments.length === 0) {
			return this._assert();
		}

		return this._fork({
			...this.parsed,
			...normalizeRelativeSegments([...this.getSegments(), ...segments]),
		});
	}

	protected getDisplaySegments(): PathSegments {
		let segments = [...this.relativeSegments];
		if (this.isExplicitDirectory()) {
			segments.push("");
		}
		return segments;
	}

	public changeExtension(ext: string): Super {
		const newBasename = this.getExtensionlessBasename();
		const relativeSegments = this.getParentSegments().concat(newBasename + ext);

		return this._fork(
			{
				...this.parsed,
				relativeSegments,
			},
			{
				ext,
				parent: this.memo.parent,
			},
		);
	}

	public addExtension(ext: string): Super {
		const newBasename = this.getBasename();
		const newExt = this.memo.ext + ext;
		const relativeSegments = this.getParentSegments().concat(newBasename + ext);

		return this._fork(
			{
				...this.parsed,
				relativeSegments,
			},
			{
				ext: newExt,
				parent: this.memo.parent,
			},
		);
	}

	public changeBasename(newBasename: string): Super {
		const relativeSegments = this.getParentSegments().concat(newBasename);
		return this._fork(
			{
				...this.parsed,
				relativeSegments,
			},
			{
				parent: this.memo.parent,
			},
		);
	}

	public getBasename(): string {
		const {relativeSegments: segments} = this;
		return segments[segments.length - 1] || "";
	}

	public getExtensionlessBasename(): string {
		const basename = this.getBasename();
		const ext = this.getExtensions();

		if (ext === "") {
			return basename;
		} else {
			return basename.slice(0, -ext.length);
		}
	}

	public hasParent() {
		return !this.isRoot();
	}

	public getParent(): Super {
		if (this.memo.parent !== undefined) {
			return this.memo.parent;
		}

		const parent = this._fork({
			...this.parsed,
			explicitDirectory: true,
			relativeSegments: this.getParentSegments(),
		});
		this.memo.parent = parent;
		return parent;
	}

	public getParentSegments(): PathSegments {
		if (this.isRoot()) {
			throw this._unexpected("Already at root and thus have no parent");
		}

		return this.getSegments().slice(0, -1);
	}

	private _unexpected(message: string) {
		throw new Error(`${this[Symbol.toStringTag]}<${this.join()}>: ${message}`);
	}

	public assertRelative(): RelativePath {
		throw this._unexpected("Expected relative path");
	}

	public assertUID(): UIDPath {
		throw this._unexpected("Expected UID path");
	}

	public assertAbsolute(): AbsoluteFilePath {
		throw this._unexpected("Expected absolute file path");
	}

	public assertReadable(): ReadablePath {
		throw this._unexpected("Expected absolute or data URL path");
	}

	public assertFilePath(): FilePath {
		throw this._unexpected("Expected relative or absolute file path");
	}

	public assertURL(): URLPath {
		throw this._unexpected("Expected URL");
	}

	public assertDataURI(): DataURIPath {
		throw this._unexpected("Expected Data URL");
	}

	public isRoot(): boolean {
		return this.relativeSegments.length === 0;
	}

	public isReadable(): this is ReadablePath {
		return false;
	}

	public isFilePath(): this is FilePath {
		return false;
	}

	public isURL(): this is URLPath {
		return false;
	}

	public isDataURI(): this is DataURIPath {
		return false;
	}

	public isUID(): this is UIDPath {
		return false;
	}

	public isAbsolute(): this is AbsoluteFilePath {
		return false;
	}

	public isRelative(): this is RelativePath {
		return false;
	}

	public isExplicitRelative(): boolean {
		return false;
	}

	public isImplicitRelative(): boolean {
		return false;
	}

	public hasEndExtension(ext: string): boolean {
		return this.getExtensions().endsWith(`.${ext}`);
	}

	public hasExtension(ext: string): boolean {
		return (
			this.hasEndExtension(ext) || this.getExtensions().includes(`.${ext}.`)
		);
	}

	public getExtensions(): string {
		const memoExt = this.memo.ext;
		if (memoExt === undefined) {
			const ext = getExtension(this.getBasename());
			this.memo.ext = ext;
			return ext;
		} else {
			return memoExt;
		}
	}

	public getDotlessExtensions(): string {
		return this.getExtensions().slice(1);
	}

	public hasAnyExtensions() {
		return this.getExtensions() !== "";
	}

	public getSegments(): PathSegments {
		return this.relativeSegments;
	}

	public hasSegment(name: string): boolean {
		return this.relativeSegments.includes(name);
	}

	public getUnique(): Super {
		const memoUnique = this.memoizedUnique;
		if (memoUnique !== undefined) {
			return memoUnique;
		}

		let path: Super = this._assert();

		if (this.parsed.explicitDirectory) {
			path = this._fork({
				...this.parsed,
				explicitDirectory: false,
			}).getUnique() as Super;
		} else {
			path = this._getUnique();
		}

		this.memoizedUnique = path;
		return path;
	}

	public isRelativeTo(other: FilePath): boolean {
		const otherSegments = other.getSegments();
		const ourSegments = this.getSegments();

		// We can't be relative to a path with more segments than us
		if (otherSegments.length > ourSegments.length) {
			return false;
		}

		// Check that we start with the same segments as the other
		for (let i = 0; i < otherSegments.length; i++) {
			if (otherSegments[i] !== ourSegments[i]) {
				return false;
			}
		}

		return true;
	}

	public isExplicitDirectory(): boolean {
		return this.parsed.explicitDirectory;
	}

	// Support some bad string coercion. Such as serialization in CLI flags.
	public toString(): string {
		return this.join();
	}

	public format(opts?: PathFormatOptions): string {
		if (opts === undefined) {
			if (this.memoizedFormat !== undefined) {
				return this.memoizedFormat;
			}
		} else {
			const {memoizedFormatOptions} = this;
			if (
				memoizedFormatOptions !== undefined &&
				memoizedFormatOptions[1] === opts.cwd &&
				memoizedFormatOptions[2] === opts.home
			) {
				return memoizedFormatOptions[0];
			}
		}

		const formatted = this._format(opts);
		if (opts === undefined) {
			this.memoizedFormat = formatted;
		} else {
			this.memoizedFormatOptions = [formatted, opts.cwd, opts.home];
		}
		return formatted;
	}

	public join(): string {
		const memoJoined = this.memoizedJoin;
		if (memoJoined !== undefined) {
			return memoJoined;
		}

		const joined = this._join();
		this.memoizedJoin = joined;
		return joined;
	}

	public equalAbsolute(other: undefined | BasePath): boolean {
		if (other === undefined) {
			return false;
		}

		if (other === this) {
			return true;
		}

		return this._equalAbsolute(other.parsed);
	}

	public equal(other: undefined | BasePath): boolean {
		if (other === undefined) {
			return false;
		}

		if (other === this) {
			return true;
		}

		// Fast path for memoized strings
		if (
			this.memoizedJoin !== undefined &&
			other.memoizedJoin === this.memoizedJoin
		) {
			return true;
		}

		if (!this.equalAbsolute(other)) {
			return false;
		}

		return equalArray(this.getSegments(), other.getSegments());
	}

	public append(...items: Array<RelativePath | string>): Super {
		if (items.length === 0) {
			return this._assert();
		}

		if (items.length === 1) {
			return this._append(items[0]);
		}

		let target: Super = this._assert();
		for (const item of items) {
			// @ts-ignore
			target = target._append(item);
		}
		return target;
	}

	private _append(item: string | RelativePath): Super {
		if (typeof item === "string") {
			const cached = this.memoizedChildren.get(item);
			if (cached !== undefined) {
				return cached;
			}
		}

		let segments: PathSegments;
		if (typeof item === "string") {
			segments = splitPathSegments(item);
		} else {
			segments = item.getSegments();
		}

		const child = this._forkAppend(segments);

		if (typeof item === "string") {
			this.memoizedChildren.set(item, child);
		}

		return child;
	}
}

enhanceNodeInspectClass(
	// @ts-ignore: We know this is an abstract class but it's ok...
	BasePath,
	(path) => {
		return `${path[Symbol.toStringTag]}<${path.format()}>`;
	},
);

export abstract class ReadableBasePath<
	Parsed extends ParsedPath,
	Super extends Path = Path
> extends BasePath<Parsed, Super> {
	public abstract readFile(): Promise<ArrayBuffer>;
	public abstract readFileText(): Promise<string>;
	public abstract createReadStream(): stream.Readable;
	public abstract exists(): Promise<boolean>;

	// Return value is meant to be consumed via ParserOptions
	public async readFileTextMeta(): Promise<{
		path: ReadablePath;
		input: string;
	}> {
		return {
			input: (await this.readFile()).toString(),
			path: this.assertReadable(),
		};
	}
}
