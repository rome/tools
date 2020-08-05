/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import os = require("os");

type FilePathOptions<Super> = {
	filename?: string;
	ext?: string;
	parent?: Super;
};

type FilePathOrString = string | UnknownFilePath;

function toFilePath(pathOrString: FilePathOrString): UnknownFilePath {
	if (typeof pathOrString === "string") {
		return createUnknownFilePath(pathOrString);
	} else {
		return pathOrString;
	}
}

export * from "./collections";

export type UnknownFilePath = AbsoluteFilePath | RelativeFilePath | URLFilePath;

export type PathSegments = Array<string>;

class BaseFilePath<Super extends UnknownFilePath> {
	constructor(parsed: ParsedPath, opts: FilePathOptions<Super>) {
		if (parsed.segments.length === 0) {
			throw new Error("Cannot construct a FilePath with zero segments");
		}

		this.segments = parsed.segments;
		this.absoluteTarget = parsed.absoluteTarget;
		this.absoluteType = parsed.absoluteType;

		// Memoized
		this.memoizedUnique = undefined;
		this.memoizedParent = opts.parent;
		this.memoizedFilename = opts.filename;
		this.memoizedExtension = opts.ext;
		this.memoizedChildren = new Map();
	}

	protected type: string = "unknown";
	protected segments: PathSegments;

	private memoizedUnique: undefined | Super;
	private memoizedFilename: undefined | string;
	private memoizedExtension: undefined | string;
	private memoizedParent: undefined | Super;

	// Memoize children when append() is called with strings
	private memoizedChildren: Map<string, Super>;

	private absoluteType: ParsedPathAbsoluteType;
	protected absoluteTarget: undefined | string;

	// Actually meant to be CUSTOM_PRETTY_FORMAT from "@internal/pretty-format" but it causes a module cycle
	public [Symbol.for("custom-pretty-format")](): string {
		return `Path<${this.type}> ${this.join()}`;
	}

	private getParsed(): ParsedPath {
		return {
			segments: this.segments,
			absoluteTarget: this.absoluteTarget,
			absoluteType: this.absoluteType,
		};
	}

	// These methods ensure the correct return classes
	protected _assert(): Super {
		throw new Error("Unimplemented");
	}

	protected _fork(parsed: ParsedPath, opts: FilePathOptions<Super>): Super {
		throw new Error("Unimplemented");
	}

	public addExtension(ext: string, clearExt: boolean = false): Super {
		const newBasename = clearExt
			? this.getExtensionlessBasename()
			: this.getBasename();
		const newExt = clearExt ? ext : this.memoizedExtension + ext;
		const segments = this.getParentSegments(false).concat(newBasename + ext);

		return this._fork(
			{
				...this.getParsed(),
				segments,
			},
			{
				ext: newExt,
				parent: this.memoizedParent,
			},
		);
	}

	public changeBasename(newBasename: string): Super {
		const segments = this.getParentSegments(false).concat(newBasename);
		return this._fork(
			{
				...this.getParsed(),
				segments,
			},
			{
				parent: this.memoizedParent,
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
		return this.getParentSegments().length > 0;
	}

	public getParent(): Super {
		if (this.memoizedParent !== undefined) {
			return this.memoizedParent;
		}

		const segments = this.getParentSegments();
		if (segments.length === 0) {
			throw new Error("No parent segments");
		}

		const parent = this._fork(
			{
				...this.getParsed(),
				segments,
			},
			{},
		);
		this.memoizedParent = parent;
		return parent;
	}

	public getParentSegments(explicit: boolean = true): PathSegments {
		// Should we throw an error?
		if (this.isRoot()) {
			return this.segments;
		}

		const segments = this.getSegments().slice(0, -1);

		// Always make this an explicit directory
		if (explicit && segments.length > 0 && segments[0] !== "") {
			segments.push("");
		}

		return segments;
	}

	public preferExplicitRelative(): Super | RelativeFilePath {
		if (this.isRelative()) {
			return this.toExplicitRelative();
		} else {
			return this._assert();
		}
	}

	public toExplicitRelative(): RelativeFilePath {
		const relative = this.assertRelative();
		if (relative.isExplicitRelative()) {
			return relative;
		} else {
			return createRelativeFilePath(".").append(relative);
		}
	}

	public assertRelative(): RelativeFilePath {
		if (this.isAbsolute()) {
			throw new Error(`Expected relative file path but got: ${this.join()}`);
		} else {
			return new RelativeFilePath(
				this.getParsed(),
				{
					ext: this.memoizedExtension,
					filename: this.memoizedFilename,
				},
			);
		}
	}

	public assertAbsolute(): AbsoluteFilePath {
		if (this.isAbsolute()) {
			return new AbsoluteFilePath(
				this.getParsed(),
				{
					ext: this.memoizedExtension,
					filename: this.memoizedFilename,
				},
			);
		} else {
			throw new Error(`Expected absolute file path but got: ${this.join()}`);
		}
	}

	public assertURL(): URLFilePath {
		if (this.isURL()) {
			return new URLFilePath(
				this.getParsed(),
				{
					ext: this.memoizedExtension,
					filename: this.memoizedFilename,
				},
			);
		} else {
			throw new Error(`Expected URL file path but got: ${this.join()}`);
		}
	}

	public isRoot(): boolean {
		if (this.segments.length === 1) {
			return true;
		}

		if (this.segments.length === 2) {
			// Explicit directory reference
			return this.segments[1] === "";
		}

		if (this.segments.length === 3) {
			return this.absoluteType === "windows-unc";
		}

		return false;
	}

	public isWindows(): boolean {
		return (
			this.absoluteType === "windows-drive" ||
			this.absoluteType === "windows-unc"
		);
	}

	public isPosix(): boolean {
		return !this.isWindows();
	}

	public isURL(): boolean {
		return this.absoluteType === "url";
	}

	public isAbsolute(): boolean {
		return this.absoluteTarget !== undefined && this.absoluteType !== "url";
	}

	public isRelative(): boolean {
		return !this.isAbsolute();
	}

	public isRelativeTo(otherRaw: FilePathOrString): boolean {
		const other = toFilePath(otherRaw);
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
		const [firstSeg] = this.segments;
		return !this.isURL() && (firstSeg === "." || firstSeg === "..");
	}

	public isExplicitDirectory(): boolean {
		const {segments} = this;
		return segments[segments.length - 1] === "";
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
		if (this.memoizedExtension === undefined) {
			const ext = getExtension(this.getBasename());
			this.memoizedExtension = ext;
			return ext;
		} else {
			return this.memoizedExtension;
		}
	}

	public hasExtensions() {
		return this.getExtensions() !== "";
	}

	public getSegments(): PathSegments {
		let {segments} = this;

		if (!this.isRoot()) {
			if (this.isExplicitDirectory()) {
				segments = segments.slice(0, -1);
			}

			if (segments[0] === ".") {
				segments = segments.slice(1);
			}
		}

		return segments;
	}

	public getRawSegments(): PathSegments {
		return this.segments;
	}

	public getUnique(): Super {
		if (this.memoizedUnique !== undefined) {
			return this.memoizedUnique;
		}

		let segments: undefined | PathSegments;

		if (!this.isRoot()) {
			if (this.isExplicitDirectory()) {
				segments = this.getSegments();

				if (this.isExplicitRelative()) {
					segments = segments.slice(1);
				}
			} else if (this.isExplicitRelative()) {
				segments = this.getRawSegments().slice(1);
			}
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

		let path: Super;
		if (segments === undefined) {
			// Cache ourselves as it could have been expensive determining that we are already unique
			path = this._assert();
		} else {
			path = this._fork(parsePathSegments(segments), {});
		}
		this.memoizedUnique = path;
		return path;
	}

	// Support some bad string coercion. Such as serialization in CLI flags.
	public toString(): string {
		return this.join();
	}

	public join(): string {
		if (this.memoizedFilename !== undefined) {
			return this.memoizedFilename;
		}

		const {segments} = this;

		let filename;
		if (this.isWindows()) {
			filename = segments.join("\\");
		} else {
			filename = segments.join("/");
		}
		this.memoizedFilename = filename;
		return filename;
	}

	public equal(other: UnknownFilePath): boolean {
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
						segments: ["~", ...relativeToHome.getSegments()],
						absoluteType: "posix",
						absoluteTarget: undefined,
					},
					{},
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

	public append(...items: Array<FilePathOrString>): Super {
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

		const parsed = parsePathSegments([
			...this.getSegments(),
			...toFilePath(item).getSegments(),
		]);
		const child = this._fork(parsed, {});

		if (typeof item === "string") {
			this.memoizedChildren.set(item, child);
		}

		return child;
	}
}

export class RelativeFilePath extends BaseFilePath<RelativeFilePath> {
	// TypeScript is structurally typed whereas here we would prefer nominal typing
	// We use this as a hack.
	protected type: "relative" = "relative";

	protected _assert(): RelativeFilePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPath,
		opts: FilePathOptions<RelativeFilePath>,
	): RelativeFilePath {
		return new RelativeFilePath(parsed, opts);
	}

	public assertRelative(): RelativeFilePath {
		return this;
	}
}

export class AbsoluteFilePath extends BaseFilePath<AbsoluteFilePath> {
	protected type: "absolute" = "absolute";

	private chain: undefined | Array<AbsoluteFilePath>;

	protected _assert(): AbsoluteFilePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPath,
		opts: FilePathOptions<AbsoluteFilePath>,
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

	public resolveMaybeUrl(
		otherRaw: FilePathOrString,
	): URLFilePath | AbsoluteFilePath {
		const other = toFilePath(otherRaw);
		if (other.isURL()) {
			return other.assertURL();
		} else {
			return this.resolve(other);
		}
	}

	public resolve(otherRaw: FilePathOrString): AbsoluteFilePath {
		const other = toFilePath(otherRaw);
		if (other.isAbsolute()) {
			return other.assertAbsolute();
		}

		return new AbsoluteFilePath(
			parsePathSegments([...this.getSegments(), ...other.getSegments()]),
			{},
		);
	}

	public relative(otherRaw: FilePathOrString): UnknownFilePath {
		const other = this.resolve(toFilePath(otherRaw));

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

		return createUnknownFilePathFromSegments(parsePathSegments(finalSegments));
	}
}

export class URLFilePath extends BaseFilePath<URLFilePath> {
	protected type: "url" = "url";

	protected _assert(): URLFilePath {
		return this;
	}

	protected _fork(
		parsed: ParsedPath,
		opts: FilePathOptions<URLFilePath>,
	): URLFilePath {
		return new URLFilePath(parsed, opts);
	}

	public assertURL(): URLFilePath {
		return this;
	}

	public isURL(): boolean {
		return true;
	}

	public getDomain(): string {
		return this.segments[2];
	}

	public getProtocol(): string {
		const {absoluteTarget} = this;
		if (absoluteTarget === undefined) {
			throw new Error("Expected a URLFilePath to always have an absoluteTarget");
		}
		return absoluteTarget;
	}

	public resolve(path: UnknownFilePath): URLFilePath {
		if (path.isURL()) {
			return path.assertURL();
		} else if (path.isAbsolute()) {
			// Get the segments that include the protocol and domain
			const domainSegments = this.getSegments().slice(0, 3);
			const finalSegments = [...domainSegments, ...path.getSegments()];
			return new URLFilePath(parsePathSegments(finalSegments), {});
		} else {
			return this.append(path);
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
	absoluteType: ParsedPathAbsoluteType;
	absoluteTarget: undefined | string;
	segments: PathSegments;
};

function parsePathSegments(segments: PathSegments): ParsedPath {
	if (segments.length === 0) {
		throw new Error("Cannot construct a FilePath with zero segments");
	}

	let absoluteType: ParsedPathAbsoluteType = "posix";
	let absoluteTarget: undefined | string;
	let firstSeg = segments[0];

	// Detect URL
	if (
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
				);

			default: {
				const absoluteSegments = segments.slice(0, 3);
				return {
					segments: normalizeSegments(
						segments,
						absoluteSegments.length,
						absoluteSegments,
					),
					absoluteType: "url",
					absoluteTarget,
				};
			}
		}
	}

	// Explode home directory
	if (firstSeg === "~") {
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
	} else if (isWindowsDrive(firstSeg)) {
		const drive = firstSeg.toUpperCase();
		absoluteSegments.push(drive);
		absoluteType = "windows-drive";
		absoluteTarget = `drive:${drive}`;
		segmentOffset++;
	}

	const pathSegments = normalizeSegments(
		segments,
		segmentOffset,
		absoluteSegments,
	);
	return {
		segments: pathSegments,
		absoluteType,
		absoluteTarget,
	};
}

function normalizeSegments(
	segments: Array<string>,
	offset: number,
	absoluteSegments: Array<string>,
): Array<string> {
	const relativeSegments: PathSegments = [];
	for (let i = offset; i < segments.length; i++) {
		let seg = segments[i];

		// Only allow a dot part in the first position, otherwise it's a noop
		if (
			seg === "." &&
			(segments[1] === ".." || i > 0 || absoluteSegments.length > 0)
		) {
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
		finalSegments.push("");
	}

	return finalSegments;
}

function createUnknownFilePathFromSegments(parsed: ParsedPath): UnknownFilePath {
	const path = new BaseFilePath(parsed, {});

	if (path.isAbsolute()) {
		return path.assertAbsolute();
	} else if (path.isURL()) {
		return path.assertURL();
	} else {
		return path.assertRelative();
	}
}

type CreationArg = UnknownFilePath | string;

export function createFilePathFromSegments(
	segments: Array<string>,
): UnknownFilePath {
	const parsed = parsePathSegments(segments);
	return createUnknownFilePathFromSegments(parsed);
}

export function toJoinedFilePath(filename: CreationArg): string {
	if (typeof filename === "string") {
		return filename;
	} else {
		return createUnknownFilePath(filename).join();
	}
}

export function createRelativeFilePath(filename: CreationArg): RelativeFilePath {
	return createUnknownFilePath(filename).assertRelative();
}

export function createURLFilePath(filename: CreationArg): URLFilePath {
	return createUnknownFilePath(filename).assertURL();
}

export function createAbsoluteFilePath(filename: CreationArg): AbsoluteFilePath {
	return createUnknownFilePath(filename).assertAbsolute();
}

export function createUnknownFilePath(filename: CreationArg): UnknownFilePath {
	// Allows using the create methods above to be used in places where strings are more ergonomic (eg. in third-party code)
	if (filename instanceof BaseFilePath) {
		return filename;
	}

	// Might be better to do a manual loop to detect escaped slashes or some other weirdness
	const segments = filename.split(/[\\\/]/g);
	const parsed = parsePathSegments(segments);
	return createUnknownFilePathFromSegments(parsed);
}

// These are some utility methods so you can pass in `undefined | string`
export function maybeCreateURLFilePath(
	filename: undefined | CreationArg,
): undefined | URLFilePath {
	if (filename !== undefined) {
		return createURLFilePath(filename);
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

export function maybeCreateUnknownFilePath(
	filename: undefined | CreationArg,
): undefined | UnknownFilePath {
	if (filename !== undefined) {
		return createUnknownFilePath(filename);
	} else {
		return undefined;
	}
}
