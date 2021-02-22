import {AnyFilePath, AnyPath, PathFormatOptions, PathSegments} from "../types";
import AbsoluteFilePath from "./AbsoluteFilePath";
import RelativePath from "./RelativePath";
import UIDPath from "./UIDPath";
import URLPath from "./URLPath";
import {AnyParsedPath, splitPathSegments, normalizeSegments} from "../parse";
import {enhanceNodeInspectClass} from "@internal/node";
import { equalArray } from "@internal/typescript-helpers";

export interface FilePathMemoBase {
	joined?: string;
	ext?: string;
}

export type FilePathMemo<Super> = FilePathMemoBase & {
	parent?: Super;
	unique?: Super;
};

function getExtension(basename: string): string {
	const match = basename.match(/\.(.*?)$/);
	if (match == null) {
		return "";
	} else {
		return match[0];
	}
}

export abstract class BasePath<ParsedPath extends AnyParsedPath, Super extends AnyPath = AnyPath> {
	constructor(parsed: ParsedPath, memo: FilePathMemo<Super> = {}) {
		this.relativeSegments = parsed.relativeSegments;
		this.parsed = parsed;
		this.memo = memo;
		this.memoizedChildren = new Map();
		this[Symbol.toStringTag] = "BasePath";
	}

	public parsed: ParsedPath;
	protected relativeSegments: PathSegments;
	protected memo: FilePathMemo<Super>;
	public [Symbol.toStringTag]: string;

	// Memoize children when append() is called with strings
	private memoizedChildren: Map<string, Super>;

	protected abstract _assert(): Super;
	protected abstract _join(relative: Array<string>): string;
	protected abstract _equalAbsolute(parsed: AnyParsedPath): boolean;
	protected abstract _fork(parsed: ParsedPath, memo?: FilePathMemo<Super>): Super;
	protected abstract _getUnique(): Super;

	protected _forkAppend(segments: PathSegments): Super {
		return this._fork({
			...this.parsed,
			...normalizeSegments([...this.getSegments(), ...segments]),
		});
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

	public getParent(): Super {
		if (this.memo.parent !== undefined) {
			return this.memo.parent;
		}

		const relativeSegments = this.getParentSegments();
		if (relativeSegments.length === 0 && this.isRoot()) {
			throw new Error("This path is at the root and cannot go higher");
		}

		const parent = this._fork({
			...this.parsed,
			explicitDirectory: true,
			relativeSegments,
		});
		this.memo.parent = parent;
		return parent;
	}

	public getParentSegments(): PathSegments {
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
		return this.relativeSegments.length === 0;
	}

	public hasParent() {
		return this.relativeSegments.length > 1;
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

	public isExplicitRelative(): boolean {
		return false;
	}

	public isImplicitRelative(): boolean {
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
		return this.relativeSegments;
	}

	public hasSegment(name: string): boolean {
		return this.relativeSegments.includes(name);
	}

	public getUnique(): Super {
		const memoUnique = this.memo.unique;
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

		this.memo.unique = path;
		return path;
	}

	// Support some bad string coercion. Such as serialization in CLI flags.
	public toString(): string {
		return this.join();
	}

	public join(): string {
		const memoJoined = this.memo.joined;
		if (memoJoined !== undefined) {
			return memoJoined;
		}

		const segments = [...this.relativeSegments];

		if (this.isExplicitDirectory()) {
			segments.push("");
		}

		const joined = this._join(segments);
		this.memo.joined = joined;
		return joined;
	}

	public equalAbsolute(other: undefined | BasePath<AnyParsedPath, AnyPath>): boolean {
		if (other === undefined) {
			return false;
		}
		
		if (other === this) {
			return true;
		}

		return this._equalAbsolute(other.parsed);
	}

	public equal(other: undefined | BasePath<AnyParsedPath, AnyPath>): boolean {
		if (other === undefined) {
			return false;
		}

		if (other === this) {
			return true;
		}

		// Fast path for memoized strings
		if (this.memo.joined !== undefined && other.memo.joined !== undefined && this.join() === other.join()) {
			return true;
		}

		if (!this.equalAbsolute(other)) {
			return false;
		}

		return equalArray(this.getSegments(), other.getSegments());
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
