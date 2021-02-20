import {AnyFilePath, AnyPath, PathFormatOptions, PathSegments} from "../types";
import AbsoluteFilePath from "./AbsoluteFilePath";
import RelativePath from "./RelativePath";
import UIDPath from "./UIDPath";
import URLPath from "./URLPath";
import {ParsedPath, parsePathSegments, splitPathSegments} from "../parse";
import {enhanceNodeInspectClass} from "@internal/node";

export interface FilePathMemoBase {
	filename?: string;
	ext?: string;
}

export interface FilePathMemo<Super> extends FilePathMemoBase {
	parent?: Super;
	unique?: Super;
}

function createEmptyMemo<FilePath>(): FilePathMemo<FilePath> {
	return {
		filename: undefined,
		ext: undefined,
		parent: undefined,
		unique: undefined,
	};
}

function getExtension(basename: string): string {
	const match = basename.match(/\.(.*?)$/);
	if (match == null) {
		return "";
	} else {
		return match[0];
	}
}

export class BasePath<Super extends AnyPath = AnyPath> {
	constructor(parsed: ParsedPath, memo: FilePathMemo<Super> = createEmptyMemo()) {
		this.segments = parsed.segments;
		this.parsed = parsed;
		this.memo = memo;
		this.memoizedChildren = new Map();
		this[Symbol.toStringTag] = "BasePath";
	}

	public parsed: ParsedPath;
	protected segments: PathSegments;
	protected memo: FilePathMemo<Super>;
	public [Symbol.toStringTag]: string;

	// Memoize children when append() is called with strings
	private memoizedChildren: Map<string, Super>;

	protected _assert(): Super {
		throw new Error("Unimplemented");
	}

	protected _fork(parsed: ParsedPath, memo?: FilePathMemo<Super>): Super {
		parsed;
		memo;
		throw new Error("Unimplemented");
	}

	public addExtension(ext: string, clearExt: boolean = false): Super {
		const newBasename = clearExt
			? this.getExtensionlessBasename()
			: this.getBasename();
		const newExt = clearExt ? ext : this.memo.ext + ext;
		const segments = this.getParentSegments().concat(newBasename + ext);

		return this._fork(
			{
				...this.parsed,
				segments,
			},
			{
				...createEmptyMemo(),
				ext: newExt,
				parent: this.memo.parent,
			},
		);
	}

	public changeBasename(newBasename: string): Super {
		const segments = this.getParentSegments().concat(newBasename);
		return this._fork(
			{
				...this.parsed,
				segments,
			},
			{
				...createEmptyMemo(),
				parent: this.memo.parent,
			},
		);
	}

	public getBasename(): string {
		const {segments} = this;
		const offset = this.isExplicitDirectory() ? 2 : 1;
		return segments[segments.length - offset] || "";
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
		return !this.isRoot() && this.getParentSegments().length > 0;
	}

	public getParent(): Super {
		if (this.memo.parent !== undefined) {
			return this.memo.parent;
		}

		const segments = this.getParentSegments();
		if (segments.length === 0) {
			throw new Error("No parent segments");
		}

		const parent = this._fork({
			...this.parsed,
			//explicitDirectory: true,
			segments,
		});
		this.memo.parent = parent;
		return parent;
	}

	public getParentSegments(): PathSegments {
		// Should we throw an error?
		if (this.isRoot()) {
			return this.segments;
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

	public assertFilePath(): AnyFilePath {
		throw this._unexpected("Expected relative or absolute file path");
	}

	public assertURL(): URLPath {
		throw this._unexpected("Expected URL");
	}

	public isRoot(): boolean {
		if (this.segments.length <= 1) {
			return true;
		}

		if (this.segments.length === 2) {
			// Explicit directory reference
			return this.parsed.absoluteType === "windows-drive";
		}

		if (this.segments.length === 3) {
			return this.parsed.absoluteType === "windows-unc";
		}

		return false;
	}

	private isWindows(): boolean {
		const {absoluteType} = this.parsed;
		return absoluteType === "windows-drive" || absoluteType === "windows-unc";
	}

	public isFilePath(): this is AnyFilePath {
		return false;
	}

	public isURL(): this is URLPath {
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

	public isRelativeTo(other: AnyFilePath): boolean {
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

	public isImplicitRelative(): boolean {
		return !(this.isExplicitRelative() || this.isAbsolute() || this.isURL());
	}

	public isExplicitRelative(): boolean {
		return this.parsed.explicitRelative;
	}

	public isExplicitDirectory(): boolean {
		return this.parsed.explicitDirectory;
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
		return this.segments;
	}

	public hasSegment(name: string): boolean {
		return this.segments.includes(name);
	}

	public getUnique(): Super {
		const memoUnique = this.memo.unique;
		if (memoUnique !== undefined) {
			return memoUnique;
		}

		// If we don't satisfy the below conditions then we're already unique
		if (
			this.isUID() ||
			!(this.isRoot() || this.isExplicitRelative() || this.isExplicitDirectory())
		) {
			return this._assert();
		}

		// Treat all Windows drive paths as case insensitive
		// Convert all segments to lowercase. Bail if they were all lowercase.
		// TODO this causes issues with file maps/sets
		/*if (this.absoluteType === "windows-drive") {
			const hadSegments = segments !== undefined;
			if (segments === undefined) {
				segments = this.getRawSegments();
			}

			let didModify = false;
			segments = segments.map((part) => {
				const lower = part.toLowerCase();
				if (lower !== part) {
					didModify = true;
				}
				return lower;
			});
			if (!didModify && !hadSegments) {
				segments = undefined;
			}
		}*/

		const path = this._fork(parsePathSegments(this.segments, this.parsed.hint));
		this.memo.unique = path;
		return path;
	}

	// Support some bad string coercion. Such as serialization in CLI flags.
	public toString(): string {
		return this.join();
	}

	public join(): string {
		const memoJoined = this.memo.filename;
		if (memoJoined !== undefined) {
			return memoJoined;
		}

		const segments = [...this.segments];

		if (this.isExplicitDirectory()) {
			segments.push("");
		}

		if (this.isExplicitRelative() && segments[0] !== "..") {
			segments.unshift(".");
		}

		if (segments.length === 0) {
			segments.push(".");
		}

		let filename;
		if (this.isWindows()) {
			filename = segments.join("\\");
		} else {
			filename = segments.join("/");
		}
		this.memo.filename = filename;
		return filename;
	}

	public equal(other: undefined | BasePath<AnyPath>): boolean {
		if (other === undefined) {
			return false;
		}

		if (other === this) {
			return true;
		}

		// Fast path for memoized strings
		if (this.memo.filename !== undefined && other.memo.filename !== undefined && this.join() === other.join()) {
			return true;
		}

		const a = this.getUnique().getSegments();
		const b = other.getUnique().getSegments();

		// Quick check
		if (a.length !== b.length) {
			return false;
		}

		// Check validity of a
		for (let i = 0; i < a.length; i++) {
			if (a[i] !== b[i]) {
				return false;
			}
		}

		return true;
	}

	public format(opts?: PathFormatOptions): string {
		opts;
		return this.join();
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

		const parsed = parsePathSegments(
			[...this.getSegments(), ...segments],
			this.parsed.hint,
			this.parsed,
		);
		const child = this._fork(parsed);

		if (typeof item === "string") {
			this.memoizedChildren.set(item, child);
		}

		return child;
	}
}

enhanceNodeInspectClass(
	BasePath,
	(path) => {
		return `${path[Symbol.toStringTag]}<${path.format()}>`;
	},
);
