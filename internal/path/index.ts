/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import os = require("os");

interface AgnosticFilePathMemo {
	filename: undefined | string;
	ext: undefined | string;
}

interface FilePathMemo<Super> extends AgnosticFilePathMemo {
	parent: undefined | Super;
	unique: undefined | Super;
}

function createEmptyMemo<FilePath>(): FilePathMemo<FilePath> {
	return {
		filename: undefined,
		ext: undefined,
		parent: undefined,
		unique: undefined,
	};
}

type FilePathOrString = string | AnyFilePath;

function toFilePath(
	pathOrString: FilePathOrString,
	hint: PathTypeHint,
): AnyFilePath {
	if (typeof pathOrString === "string") {
		return createUnknownPath(pathOrString, hint);
	} else {
		return pathOrString;
	}
}

export * from "./collections";

export type AnyFilePath =
	| UnknownPath
	| AbsoluteFilePath
	| RelativeFilePath
	| URLPath;

export type PathSegments = Array<string>;

export abstract class BasePath<Super extends AnyFilePath = AnyFilePath> {
	constructor(parsed: ParsedPath, memo: FilePathMemo<Super>) {
		this.segments = parsed.segments;
		this.parsed = parsed;
		this.memo = memo;
		this.memoizedChildren = new Map();
	}

	public parsed: ParsedPath;
	protected segments: PathSegments;
	protected memo: FilePathMemo<Super>;

	// Memoize children when append() is called with strings
	private memoizedChildren: Map<string, Super>;

	// Actually meant to be CUSTOM_PRETTY_FORMAT from "@internal/pretty-format" but it causes a module cycle
	public [Symbol.for("custom-pretty-format")](): string {
		return `${this.constructor.name}: ${this.join()}`;
	}

	protected abstract _assert(): Super
	protected abstract _fork(parsed: ParsedPath, memo: FilePathMemo<Super>): Super

	private getPortableMemo(): AgnosticFilePathMemo & {
		parent: undefined;
		unique: undefined;
	} {
		return {
			parent: undefined,
			unique: undefined,
			filename: this.memo.filename,
			ext: this.memo.ext,
		};
	}

	public toUnknown(): UnknownPath {
		return new UnknownPath(this.parsed, this.getPortableMemo());
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

		const parent = this._fork(
			{
				...this.parsed,
				//explicitDirectory: true,
				segments,
			},
			createEmptyMemo(),
		);
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

	public preferExplicitRelative(): Super | RelativeFilePath {
		if (this.isRelative()) {
			return this.toExplicitRelative();
		} else {
			return this._assert();
		}
	}

	public toExplicitRelative(): RelativeFilePath {
		if (this.isExplicitRelative()) {
			return this.assertRelative();
		} else {
			return createRelativeFilePath(".").append(...this.getSegments());
		}
	}

	public assertRelative(): RelativeFilePath {
		if (this.isAbsolute()) {
			throw new Error(
				`Expected relative file path but got: ${JSON.stringify(this.join())}`,
			);
		} else {
			return new RelativeFilePath(this.parsed, this.getPortableMemo());
		}
	}

	public assertAbsolute(): AbsoluteFilePath {
		if (this.isAbsolute()) {
			return new AbsoluteFilePath(this.parsed, this.getPortableMemo());
		} else {
			throw new Error(
				`Expected absolute file path but got: ${JSON.stringify(this.join())}`,
			);
		}
	}

	public assertURL(): URLPath {
		if (this.isURL()) {
			return new URLPath(this.parsed, this.getPortableMemo());
		} else {
			throw new Error(
				`Expected URL file path but got: ${JSON.stringify(this.join())}`,
			);
		}
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

	public isWindows(): boolean {
		const {absoluteType} = this.parsed;
		return absoluteType === "windows-drive" || absoluteType === "windows-unc";
	}

	public isPosix(): boolean {
		return !this.isWindows();
	}

	public isURL(): boolean {
		return this.parsed.absoluteType === "url";
	}

	public isAbsolute(): boolean {
		return (
			this.parsed.absoluteTarget !== undefined &&
			this.parsed.absoluteType !== "url"
		);
	}

	public isRelative(): boolean {
		return !this.isAbsolute();
	}

	public isRelativeTo(otherRaw: FilePathOrString): boolean {
		const other = toFilePath(otherRaw, "absolute");
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
		return !this.isExplicitRelative() && !this.isAbsolute() && !this.isURL();
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

	public hasExtensions() {
		return this.getExtensions() !== "";
	}

	public getSegments(): PathSegments {
		return this.segments;
	}

	public getUnique(): Super {
		const memoUnique = this.memo.unique;
		if (memoUnique !== undefined) {
			return memoUnique;
		}

		// If we don't satisfy the below conditions then we're already unique
		if (
			!this.isRoot() &&
			!this.isExplicitRelative() &&
			!this.isExplicitDirectory()
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

		const path = this._fork(
			parsePathSegments(this.segments, this.parsed.hint),
			createEmptyMemo(),
		);
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

	public equal(other: AnyFilePath): boolean {
		// @ts-ignore
		if (other === this) {
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

	public format(cwd?: AbsoluteFilePath): string {
		const filename = this.join();
		const names: Array<string> = [];
		names.push(filename);

		// Get a path relative to HOME
		if (this.isRelativeTo(HOME_PATH)) {
			// Path starts with the home directory, so let's trim it off
			const relativeToHome = HOME_PATH.relative(this._assert());

			// Add tilde and push it as a possible name
			// We construct this manually to get around the segment normalization which would explode ~
			names.push(
				new RelativeFilePath(
					{
						hint: "relative",
						segments: ["~", ...relativeToHome.getSegments()],
						absoluteType: "posix",
						absoluteTarget: undefined,
						explicitDirectory: this.parsed.explicitDirectory,
						explicitRelative: false,
					},
					createEmptyMemo(),
				).join(),
			);
		}

		// Get a path relative to the cwd
		if (cwd !== undefined) {
			names.push(cwd.relative(filename).join());
		}

		// Get the shortest name
		const human = names.sort((a, b) => a.length - b.length)[0];
		if (human === "") {
			return "./";
		} else {
			return human;
		}
	}

	public append(...items: Array<RelativeFilePath | string>): Super {
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

	private _append(item: FilePathOrString): Super {
		if (typeof item === "string") {
			const cached = this.memoizedChildren.get(item);
			if (cached !== undefined) {
				return cached;
			}
		}

		const parsed = parsePathSegments(
			[...this.getSegments(), ...toFilePath(item, "relative").getSegments()],
			this.parsed.hint,
			this.parsed,
		);
		const child = this._fork(parsed, createEmptyMemo());

		if (typeof item === "string") {
			this.memoizedChildren.set(item, child);
		}

		return child;
	}
}

export class UnknownPath extends BasePath<UnknownPath> {
	protected _fork(
		parsed: ParsedPath,
		opts: FilePathMemo<UnknownPath>,
	): UnknownPath {
		return new UnknownPath(parsed, opts);
	}

	protected _assert(): UnknownPath {
		return this;
	}
}

export class RelativeFilePath extends BasePath<RelativeFilePath> {
	// TypeScript is structurally typed whereas here we would prefer nominal typing
	// We use this as a hack.
	protected type: "relative" = "relative";

	protected _assert(): RelativeFilePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPath,
		opts: FilePathMemo<RelativeFilePath>,
	): RelativeFilePath {
		return new RelativeFilePath(parsed, opts);
	}

	public assertRelative(): RelativeFilePath {
		return this;
	}
}

export class AbsoluteFilePath extends BasePath<AbsoluteFilePath> {
	protected type: "absolute" = "absolute";

	private chain: undefined | Array<AbsoluteFilePath>;

	protected _assert(): AbsoluteFilePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPath,
		opts: FilePathMemo<AbsoluteFilePath>,
	): AbsoluteFilePath {
		return new AbsoluteFilePath(parsed, opts);
	}

	public assertAbsolute(): AbsoluteFilePath {
		return this;
	}

	public getChain(): Array<AbsoluteFilePath> {
		if (this.chain !== undefined) {
			return this.chain;
		}

		const paths: Array<AbsoluteFilePath> = [];
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

	public resolveMaybeUrl(otherRaw: FilePathOrString): URLPath | AbsoluteFilePath {
		const other = toFilePath(otherRaw, "url");
		if (other.isURL()) {
			return other.assertURL();
		} else {
			return this.resolve(other);
		}
	}

	public resolve(otherRaw: FilePathOrString): AbsoluteFilePath {
		const other = toFilePath(otherRaw, "auto");
		if (other.isAbsolute()) {
			return other.assertAbsolute();
		}

		return new AbsoluteFilePath(
			parsePathSegments(
				[...this.getSegments(), ...other.getSegments()],
				"absolute",
				{
					explicitDirectory: other.isExplicitDirectory(),
				},
			),
			createEmptyMemo(),
		);
	}

	public relative(otherRaw: FilePathOrString): AnyFilePath {
		const other = this.resolve(toFilePath(otherRaw, "relative"));

		if (other.equal(this)) {
			return createRelativeFilePath(".");
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

		return new UnknownPath(
			parsePathSegments(finalSegments, "relative"),
			createEmptyMemo(),
		);
	}
}

export class URLPath extends BasePath<URLPath> {
	protected type: "url" = "url";

	protected _assert(): URLPath {
		return this;
	}

	protected _fork(parsed: ParsedPath, opts: FilePathMemo<URLPath>): URLPath {
		return new URLPath(parsed, opts);
	}

	public assertURL(): URLPath {
		return this;
	}

	public isURL(): boolean {
		return true;
	}

	public getDomain(): string {
		return this.segments[2];
	}

	public getProtocol(): string {
		const {absoluteTarget} = this.parsed;
		if (absoluteTarget === undefined) {
			throw new Error("Expected a URLPath to always have an absoluteTarget");
		}
		return absoluteTarget;
	}

	public resolve(path: AnyFilePath): URLPath {
		if (path.isURL()) {
			return path.assertURL();
		} else if (path.isAbsolute()) {
			// Get the segments that include the protocol and domain
			const domainSegments = this.getSegments().slice(0, 3);
			const finalSegments = [...domainSegments, ...path.getSegments()];
			return new URLPath(
				parsePathSegments(finalSegments, "auto"),
				createEmptyMemo(),
			);
		} else {
			return this.append(path.assertRelative());
		}
	}
}

export const HOME_PATH = createAbsoluteFilePath(os.userInfo().homedir);
export const TEMP_PATH = createAbsoluteFilePath(os.tmpdir());
export const CWD_PATH = createAbsoluteFilePath(process.cwd());

function getExtension(basename: string): string {
	const match = basename.match(/\.(.*?)$/);
	if (match == null) {
		return "";
	} else {
		return match[0];
	}
}

function isWindowsDrive(first: string): boolean {
	return first.length === 2 && first[1] === ":" && /[A-Z]/i.test(first[0]);
}

type ParsedPathAbsoluteType = "windows-drive" | "windows-unc" | "posix" | "url";

type ParsedPath = {
	hint: PathTypeHint;
	absoluteType: ParsedPathAbsoluteType;
	absoluteTarget: undefined | string;
	segments: PathSegments;
	explicitRelative: boolean;
	explicitDirectory: boolean;
};

type PathTypeHint = "absolute" | "relative" | "url" | "auto";

function parsePathSegments(
	segments: PathSegments,
	hint: PathTypeHint,
	overrides: Pick<Partial<ParsedPath>, "explicitRelative" | "explicitDirectory"> = {

	},
): ParsedPath {
	let absoluteType: ParsedPathAbsoluteType = "posix";
	let absoluteTarget: undefined | string;
	let firstSeg = (segments[0] as undefined | string);

	// Detect URL
	if (
		firstSeg !== undefined &&
		!isWindowsDrive(firstSeg) &&
		firstSeg[firstSeg.length - 1] === ":" &&
		segments[1] === ""
	) {
		absoluteTarget = firstSeg.slice(0, -1);

		switch (absoluteTarget) {
			case "file":
				// Automatically normalize a file scheme into an absolute path
				return parsePathSegments(
					segments.slice(2).map((segment) => decodeURIComponent(segment)),
					"absolute",
				);

			default: {
				const absoluteSegments = segments.slice(0, 3);
				return {
					hint: "absolute",
					absoluteType: "url",
					absoluteTarget,
					...normalizeSegments(
						segments,
						absoluteSegments.length,
						absoluteSegments,
					),
				};
			}
		}
	}

	// Explode home directory
	if ((hint === "absolute" || hint === "auto") && firstSeg === "~") {
		segments = [...HOME_PATH.getSegments()];
		firstSeg = segments[0];
	}

	let segmentOffset = 0;

	// We first extract the "absolute" portion of a path, this includes any Windows drive letters, UNC hostnames etc
	const absoluteSegments: PathSegments = [];
	if (firstSeg === "") {
		// POSIX path
		absoluteSegments.push("");
		absoluteTarget = "posix";
		segmentOffset++;

		// Windows UNC
		if (segments[1] === "" && segments.length >= 3 && segments[2] !== "") {
			const name = segments[2];
			segmentOffset += 2;
			absoluteSegments.push("");
			absoluteSegments.push(name);
			absoluteType = "windows-unc";
			absoluteTarget = `unc:${name}`;
		}
	} else if (firstSeg !== undefined && isWindowsDrive(firstSeg)) {
		const drive = firstSeg.toUpperCase();
		absoluteSegments.push(drive);
		absoluteType = "windows-drive";
		absoluteTarget = `drive:${drive}`;
		segmentOffset++;
	}

	const {
		explicitDirectory,
		explicitRelative,
		segments: pathSegments,
	} = normalizeSegments(segments, segmentOffset, absoluteSegments);

	return {
		explicitDirectory: overrides.explicitDirectory || explicitDirectory,
		explicitRelative: overrides.explicitRelative || explicitRelative,
		segments: pathSegments,
		absoluteType,
		absoluteTarget,
		hint,
	};
}

function normalizeSegments(
	segments: Array<string>,
	offset: number,
	absoluteSegments: Array<string>,
): {
	explicitDirectory: boolean;
	explicitRelative: boolean;
	segments: Array<string>;
} {
	let explicitDirectory = false;
	let explicitRelative = false;

	const relativeSegments: PathSegments = [];
	for (let i = offset; i < segments.length; i++) {
		let seg = segments[i];

		// Ignore dots, we check for explicit relative below
		if (seg === ".") {
			continue;
		}

		// Ignore empty segments
		if (seg === "") {
			continue;
		}

		// Remove the previous segment, as long as it's not also ..
		if (
			seg === ".." &&
			relativeSegments.length > 0 &&
			relativeSegments[relativeSegments.length - 1] !== ".."
		) {
			relativeSegments.pop();
			continue;
		}

		relativeSegments.push(seg);
	}

	const finalSegments = [...absoluteSegments, ...relativeSegments];

	// Retain explicit directory
	if (
		segments[segments.length - 1] === "" &&
		finalSegments[finalSegments.length - 1] !== "" &&
		relativeSegments.length !== 0
	) {
		explicitDirectory = true;
	}

	explicitRelative =
		absoluteSegments.length === 0 &&
		(segments[0] === "." || segments[0] === "..");

	return {
		explicitDirectory,
		explicitRelative,
		segments: finalSegments,
	};
}

type CreationArg = AnyFilePath | string;

export function createFilePathFromSegments(
	segments: Array<string>,
	hint: PathTypeHint,
): UnknownPath {
	const parsed = parsePathSegments(segments, hint);
	return new UnknownPath(parsed, createEmptyMemo());
}

export function createRelativeFilePath(filename: CreationArg): RelativeFilePath {
	return createUnknownPath(filename, "relative").assertRelative();
}

export function createURLPath(filename: CreationArg): URLPath {
	return createUnknownPath(filename, "auto").assertURL();
}

export function createAbsoluteFilePath(filename: CreationArg): AbsoluteFilePath {
	return createUnknownPath(filename, "absolute").assertAbsolute();
}

export function createUnknownPath(
	filename: CreationArg,
	hint: PathTypeHint = "auto",
): UnknownPath {
	// Allows using the create methods above to be used in places where strings are more ergonomic (eg. in third-party code)
	if (filename instanceof BasePath) {
		return filename.toUnknown();
	}

	// Might be better to do a manual loop to detect escaped slashes or some other weirdness
	const segments = filename.split(/[\\\/]/g);
	const parsed = parsePathSegments(segments, hint);
	return new UnknownPath(parsed, createEmptyMemo());
}

// These are some utility methods so you can pass in `undefined | string`
export function maybeCreateURLPath(
	filename: undefined | CreationArg,
): undefined | URLPath {
	if (filename !== undefined) {
		return createURLPath(filename);
	} else {
		return undefined;
	}
}

export function maybeCreateRelativeFilePath(
	filename: undefined | CreationArg,
): undefined | RelativeFilePath {
	if (filename !== undefined) {
		return createRelativeFilePath(filename);
	} else {
		return undefined;
	}
}

export function maybeCreateAbsoluteFilePath(
	filename: undefined | CreationArg,
): undefined | AbsoluteFilePath {
	if (filename !== undefined) {
		return createAbsoluteFilePath(filename);
	} else {
		return undefined;
	}
}

export function maybeCreateUnknownPath(
	filename: undefined | CreationArg,
): undefined | UnknownPath {
	if (filename !== undefined) {
		return createUnknownPath(filename, "auto");
	} else {
		return undefined;
	}
}
