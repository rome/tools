/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import os = require('os');

type FilePathOptions<Super> = {
  filename?: string;
  ext?: string;
  parent?: Super;
};

type FilePathOrString = string | UnknownFilePath;

function toFilePath(pathOrString: FilePathOrString): UnknownFilePath {
  if (typeof pathOrString === 'string') {
    return createUnknownFilePath(pathOrString);
  } else {
    return pathOrString;
  }
}

export * from './collections';

export type UnknownFilePath = AbsoluteFilePath | RelativeFilePath | URLFilePath;

export type PathSegments = Array<string>;

class BaseFilePath<Super extends UnknownFilePath> {
  constructor(parsed: ParsedPath, opts: FilePathOptions<Super>) {
    this.segments = parsed.segments;
    this.absoluteTarget = parsed.absoluteTarget;
    this.absoluteType = parsed.absoluteType;

    // Memoized
    this.memoizedParent = opts.parent;
    this.memoizedFilename = opts.filename;
    this.memoizedExtension = opts.ext;
    this.memoizedChildren = new Map();
  }

  segments: PathSegments;

  memoizedFilename: undefined | string;
  memoizedExtension: undefined | string;
  memoizedParent: undefined | Super;

  // Memoize children when append() is called with strings
  memoizedChildren: Map<string, Super>;

  absoluteType: ParsedPathAbsoluteType;
  absoluteTarget: undefined | string;

  getParsed(): ParsedPath {
    return {
      segments: this.segments,
      absoluteTarget: this.absoluteTarget,
      absoluteType: this.absoluteType,
    };
  }

  // These methods ensure the correct return classes
  _assert(): Super {
    throw new Error('Unimplemented');
  }
  _fork(parsed: ParsedPath, opts: FilePathOptions<Super>): Super {
    throw new Error('Unimplemented');
  }

  addExtension(ext: string, clearExt: boolean = false): Super {
    const newBasename = clearExt
      ? this.getExtensionlessBasename()
      : this.getBasename();
    const newExt = clearExt ? ext : this.memoizedExtension + ext;
    const segments = this.getParentSegments().concat(newBasename + ext);

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

  changeBasename(newBasename: string): Super {
    const segments = this.getParentSegments().concat(newBasename);
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

  getBasename(): string {
    const {segments} = this;
    return segments[segments.length - 1];
  }

  getExtensionlessBasename(): string {
    const basename = this.getBasename();
    const ext = this.getExtensions();

    if (ext === '') {
      return basename;
    } else {
      return basename.slice(0, -ext.length);
    }
  }

  getParent(): Super {
    if (this.memoizedParent !== undefined) {
      return this.memoizedParent;
    }

    const parent = this._fork(
      {
        ...this.getParsed(),
        segments: this.getParentSegments(),
      },
      {},
    );
    this.memoizedParent = parent;
    return parent;
  }

  getParentSegments(): PathSegments {
    return this.segments.slice(0, -1);
  }

  toExplicitRelative(): RelativeFilePath {
    const relative = this.assertRelative();
    if (relative.isExplicitRelative()) {
      return relative;
    } else {
      return createRelativeFilePath('.').append(relative);
    }
  }

  assertRelative(): RelativeFilePath {
    if (this.isAbsolute()) {
      throw new Error(`Expected relative file path but got: ${this.join()}`);
    } else {
      return new RelativeFilePath(this.getParsed(), {
        ext: this.memoizedExtension,
        filename: this.memoizedFilename,
      });
    }
  }

  assertAbsolute(): AbsoluteFilePath {
    if (this.isAbsolute()) {
      return new AbsoluteFilePath(this.getParsed(), {
        ext: this.memoizedExtension,
        filename: this.memoizedFilename,
      });
    } else {
      throw new Error(`Expected absolute file path but got: ${this.join()}`);
    }
  }

  assertURL(): URLFilePath {
    if (this.isURL()) {
      return new URLFilePath(this.getParsed(), {
        ext: this.memoizedExtension,
        filename: this.memoizedFilename,
      });
    } else {
      throw new Error(`Expected URL file path but got: ${this.join()}`);
    }
  }

  isRoot(): boolean {
    if (this.segments.length === 1) {
      return true;
    }

    if (this.segments.length === 3) {
      return this.absoluteType === 'windows-unc';
    }

    return false;
  }

  isWindows(): boolean {
    return (
      this.absoluteType === 'windows-drive' ||
      this.absoluteType === 'windows-unc'
    );
  }

  isPosix(): boolean {
    return !this.isWindows();
  }

  isURL(): boolean {
    return this.absoluteType === 'url';
  }

  isAbsolute(): boolean {
    return this.absoluteTarget !== undefined && this.absoluteType !== 'url';
  }

  isRelative(): boolean {
    return !this.isAbsolute();
  }

  isRelativeTo(otherRaw: FilePathOrString): boolean {
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

  isImplicitRelative(): boolean {
    return !this.isExplicitRelative() && !this.isAbsolute() && !this.isURL();
  }

  isExplicitRelative(): boolean {
    const [firstSeg] = this.segments;
    return !this.isURL() && (firstSeg === '.' || firstSeg === '..');
  }

  getExtensions(): string {
    if (this.memoizedExtension === undefined) {
      const ext = getExtension(this.getBasename());
      this.memoizedExtension = ext;
      return ext;
    } else {
      return this.memoizedExtension;
    }
  }

  hasExtensions() {
    return this.getExtensions() !== '';
  }

  getSegments(): PathSegments {
    return this.segments;
  }

  // Support some bad string coercion. Such as serialization in CLI flags.
  toString(): string {
    return this.join();
  }

  join(): string {
    if (this.memoizedFilename !== undefined) {
      return this.memoizedFilename;
    }

    const {segments} = this;

    let filename;
    if (this.isWindows()) {
      filename = segments.join('\\');
    } else {
      filename = segments.join('/');
    }
    this.memoizedFilename = filename;
    return filename;
  }

  // This does some weird optimizations to avoid materializing complete filenames
  // Might not be relevant... TODO benchmark this or something lol
  equal(other: UnknownFilePath): boolean {
    // Quick check if we've materalized the filename on both instances
    if (
      this.memoizedFilename !== undefined &&
      other.memoizedFilename !== undefined
    ) {
      return this.memoizedFilename === other.memoizedFilename;
    }

    const a = this.getSegments();
    const b = other.getSegments();

    // Quick check
    if (a.length !== b.length) {
      return false;
    }

    for (let i = 0; i < a.length; i++) {
      if (a[i] !== b[i]) {
        return false;
      }
    }

    return true;
  }

  format(cwd?: AbsoluteFilePath): string {
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
            segments: ['~', ...relativeToHome.getSegments()],
            absoluteType: 'posix',
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
    if (human === '') {
      return './';
    } else {
      return human;
    }
  }

  append(raw: FilePathOrString | Array<FilePathOrString>): Super {
    // Check if we have a memoized instance
    if (typeof raw === 'string') {
      const cached = this.memoizedChildren.get(raw);
      if (cached !== undefined) {
        return cached;
      }
    }

    const items: Array<FilePathOrString> = Array.isArray(raw) ? raw : [raw];

    let segments: PathSegments = this.getSegments();

    for (const item of items) {
      segments = segments.concat(toFilePath(item).getSegments());
    }

    const parsed = parsePathSegments(segments);
    const child = this._fork(parsed, {});

    // Set memoized child if possible
    if (typeof raw === 'string') {
      this.memoizedChildren.set(raw, child);
    }

    return child;
  }
}

export class RelativeFilePath extends BaseFilePath<RelativeFilePath> {
  // TypeScript is structurally typed whereas here we would prefer nominal typing
  // We use this as a hack.
  type: 'relative' = 'relative';

  _assert(): RelativeFilePath {
    return this;
  }

  _fork(
    parsed: ParsedPath,
    opts: FilePathOptions<RelativeFilePath>,
  ): RelativeFilePath {
    return new RelativeFilePath(parsed, opts);
  }

  assertRelative(): RelativeFilePath {
    return this;
  }
}

export class AbsoluteFilePath extends BaseFilePath<AbsoluteFilePath> {
  type: 'absolute' = 'absolute';

  chain: undefined | Array<AbsoluteFilePath>;

  _assert(): AbsoluteFilePath {
    return this;
  }

  _fork(
    parsed: ParsedPath,
    opts: FilePathOptions<AbsoluteFilePath>,
  ): AbsoluteFilePath {
    return new AbsoluteFilePath(parsed, opts);
  }

  assertAbsolute(): AbsoluteFilePath {
    return this;
  }

  getChain(): Array<AbsoluteFilePath> {
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

  resolveMaybeUrl(otherRaw: FilePathOrString): URLFilePath | AbsoluteFilePath {
    const other = toFilePath(otherRaw);
    if (other.isURL()) {
      return other.assertURL();
    } else {
      return this.resolve(other);
    }
  }

  resolve(otherRaw: FilePathOrString): AbsoluteFilePath {
    const other = toFilePath(otherRaw);
    if (other.isAbsolute()) {
      return other.assertAbsolute();
    }

    return new AbsoluteFilePath(
      parsePathSegments([...this.getSegments(), ...other.getSegments()]),
      {},
    );
  }

  relative(otherRaw: FilePathOrString): UnknownFilePath {
    const other = this.resolve(toFilePath(otherRaw));

    if (other.equal(this)) {
      return createRelativeFilePath('.');
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
      finalSegments.push('..');
    }
    finalSegments = finalSegments.concat(relative);

    return createUnknownFilePathFromSegments(parsePathSegments(finalSegments));
  }
}

export class URLFilePath extends BaseFilePath<URLFilePath> {
  type: 'url' = 'url';

  _assert(): URLFilePath {
    return this;
  }

  _fork(parsed: ParsedPath, opts: FilePathOptions<URLFilePath>): URLFilePath {
    return new URLFilePath(parsed, opts);
  }

  assertURL(): URLFilePath {
    return this;
  }

  isURL(): boolean {
    return true;
  }

  getDomain(): string {
    return this.segments[2];
  }

  getProtocol(): string {
    const {absoluteTarget} = this;
    if (absoluteTarget === undefined) {
      throw new Error(
        'Expected a URLFilePath to always have an absoluteTarget',
      );
    }
    return absoluteTarget;
  }

  resolve(path: UnknownFilePath): URLFilePath {
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
    return '';
  } else {
    return match[0];
  }
}

function isWindowsDrive(first: string): boolean {
  return first.length === 2 && first[1] === ':' && /[A-Z]/i.test(first[0]);
}

type ParsedPathAbsoluteType = 'windows-drive' | 'windows-unc' | 'posix' | 'url';

type ParsedPath = {
  absoluteType: ParsedPathAbsoluteType;
  absoluteTarget: undefined | string;
  segments: PathSegments;
};

function parsePathSegments(segments: PathSegments): ParsedPath {
  if (segments.length === 0) {
    throw new Error('Cannot construct a FilePath with zero segments');
  }

  let absoluteType: ParsedPathAbsoluteType = 'posix';
  let absoluteTarget: undefined | string;
  let firstSeg = segments[0];

  // Detect URL
  if (
    !isWindowsDrive(firstSeg) &&
    firstSeg[firstSeg.length - 1] === ':' &&
    segments[1] === ''
  ) {
    absoluteTarget = firstSeg.slice(0, -1);

    switch (absoluteTarget) {
      case 'file':
        return parsePathSegments(segments.slice(2));

      default:
        const absoluteSegments = segments.slice(0, 3);
        return {
          segments: normalizeSegments(
            segments,
            absoluteSegments.length,
            absoluteSegments,
          ),
          absoluteType: 'url',
          absoluteTarget,
        };
    }
  }

  // Explode home directory
  if (firstSeg === '~') {
    segments = [...HOME_PATH.getSegments()];
    firstSeg = segments[0];
  }

  let segmentOffset = 0;

  // We first extract the "absolute" portion of a path, this includes any Windows drive letters, UNC hostnames etc
  const absoluteSegments: PathSegments = [];
  if (firstSeg === '') {
    // POSIX path
    absoluteSegments.push('');
    absoluteTarget = 'posix';
    segmentOffset++;

    // Windows UNC
    if (segments[1] === '' && segments.length >= 3 && segments[2] !== '') {
      const name = segments[2];
      segmentOffset += 2;
      absoluteSegments.push('');
      absoluteSegments.push(name);
      absoluteType = 'windows-unc';
      absoluteTarget = `unc:${name}`;
    }
  } else if (isWindowsDrive(firstSeg)) {
    const drive = firstSeg.toUpperCase();
    absoluteSegments.push(drive);
    absoluteType = 'windows-drive';
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
  const pathSegments: PathSegments = [];
  for (let i = offset; i < segments.length; i++) {
    let seg = segments[i];

    // Only allow a dot part in the first position, otherwise it's a noop
    if (
      seg === '.' &&
      (segments[1] === '..' || i > 0 || absoluteSegments.length > 0)
    ) {
      continue;
    }

    // Ignore empty segments, important scenarios have already been handled above
    if (seg === '') {
      continue;
    }

    // Remove the previous segment, as long as it's not also ..
    if (
      seg === '..' &&
      pathSegments.length > 0 &&
      pathSegments[pathSegments.length - 1] !== '..'
    ) {
      pathSegments.pop();
      continue;
    }

    pathSegments.push(seg);
  }

  return [...absoluteSegments, ...pathSegments];
}

function createUnknownFilePathFromSegments(
  parsed: ParsedPath,
): UnknownFilePath {
  const path = new BaseFilePath(parsed, {});

  if (path.isAbsolute()) {
    return path.assertAbsolute();
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
  if (typeof filename === 'string') {
    return filename;
  } else {
    return createUnknownFilePath(filename).join();
  }
}

export function createRelativeFilePath(
  filename: CreationArg,
): RelativeFilePath {
  return createUnknownFilePath(filename).assertRelative();
}

export function createURLFilePath(filename: CreationArg): URLFilePath {
  return createUnknownFilePath(filename).assertURL();
}

export function createAbsoluteFilePath(
  filename: CreationArg,
): AbsoluteFilePath {
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
  }
}

export function maybeCreateRelativeFilePath(
  filename: undefined | CreationArg,
): undefined | RelativeFilePath {
  if (filename !== undefined) {
    return createRelativeFilePath(filename);
  }
}

export function maybeCreateAbsoluteFilePath(
  filename: undefined | CreationArg,
): undefined | AbsoluteFilePath {
  if (filename !== undefined) {
    return createAbsoluteFilePath(filename);
  }
}

export function maybeCreateUnknownFilePath(
  filename: undefined | CreationArg,
): undefined | UnknownFilePath {
  if (filename !== undefined) {
    return createUnknownFilePath(filename);
  }
}
