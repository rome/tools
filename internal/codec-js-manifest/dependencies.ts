/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from "@internal/consume";
import {
	SemverRangeNode,
	parseSemverRange,
	stringifySemver,
} from "@internal/codec-semver";
import {tryParseWithOptionalOffsetPosition} from "@internal/parser-core";
import {UnknownPath, createUnknownPath} from "@internal/path";
import {manifestNameToString, normalizeName} from "./name";
import {ob1Add} from "@internal/ob1";
import {descriptions} from "@internal/diagnostics";
import {ManifestName} from "./types";

export type DependencyPattern =
	| GistPattern
	| HostedGitPattern
	| HTTPTarballPattern
	| SemverPattern
	| GitPattern
	| FilePattern
	| TagPattern
	| NpmPattern
	| LinkPattern
	| WorkspacePattern;

export type ManifestDependencies = Map<string, DependencyPattern>;

type UrlWithHash = {
	url: string;
	hash: string | undefined;
};

export function stringifyDependencyPattern(pattern: DependencyPattern): string {
	switch (pattern.type) {
		case "hosted-git": {
			let str = `${pattern.host}:${pattern.user}/${pattern.repo}`;
			if (pattern.commitish !== undefined) {
				str += `#${pattern.commitish}`;
			}
			return str;
		}

		case "file":
			return `file:${pattern.path}`;

		case "gist":
			return `gist:${pattern.id}`;

		case "semver":
			return stringifySemver(pattern.range);

		case "tag":
			return pattern.tag;

		case "git":
		case "http-tarball":
			if (pattern.hash === undefined) {
				return pattern.url;
			} else {
				return `${pattern.url}#${pattern.hash}`;
			}

		case "npm": {
			let str = `${NPM_PREFIX}${pattern.name}`;
			if (pattern.range !== undefined) {
				str += `@${stringifySemver(pattern.range)}`;
			}
			return str;
		}

		case "link":
			return `${LINK_PREFIX}${pattern.path.join()}`;

		case "workspace":
			return `${WORKSPACE_PREFIX}${pattern.path}`;
	}
}

function explodeHashUrl(pattern: string, consumer: Consumer): UrlWithHash {
	const parts = pattern.split("#");

	if (parts.length > 2) {
		consumer.unexpected(descriptions.MANIFEST.TOO_MANY_HASH_PARTS);
	}

	return {
		hash: parts[1],
		url: parts[0],
	};
}

function removePrefix(prefix: string, value: string): string {
	if (value.startsWith(prefix)) {
		return value.slice(prefix.length);
	} else {
		return value;
	}
}

//# Gist

const GIST_PREFIX = "gist:";

type GistPattern = {
	type: "gist";
	id: string;
};

function parseGist(pattern: string): GistPattern {
	return {
		type: "gist",
		id: pattern.slice(GIST_PREFIX.length),
	};
}

//# Hosted Gist
export type HostedGitHost = "bitbucket" | "github" | "gitlab";

type IncompleteHostedGitPattern = {
	type: "hosted-git";
	host: HostedGitHost;
	user: string;
	repo: string;
	commitish: undefined | string;
};

type HostedGitPattern = IncompleteHostedGitPattern & {
	url: string;
};

const GITHUB_SHORTHAND = /^[^:@%\/\s.\-][^:@%\/\s]*[\/][^:@\s\/%]+(?:#.*)?$/;

const HOSTED_GIT_PREFIXES: Array<HostedGitHost> = [
	"bitbucket",
	"github",
	"gitlab",
];

function parseHostedGit(
	host: HostedGitHost,
	pattern: string,
	consumer: Consumer,
): HostedGitPattern {
	// Extract and trim hash
	let commitish: undefined | string;
	if (pattern.includes("#")) {
		const hashIndex = pattern.indexOf("#");
		commitish = pattern.slice(hashIndex + 1);
		pattern = pattern.slice(0, hashIndex - 1);
	}

	const parts = pattern.split("/");
	if (parts.length > 2) {
		consumer.unexpected(descriptions.MANIFEST.TOO_MANY_HOSTED_GIT_PARTS);
	}

	let user = parts[0];
	if (user === undefined) {
		consumer.unexpected(descriptions.MANIFEST.MISSING_HOSTED_GIT_USER);
		user = "unknown";
	}

	let repo = parts[1];
	if (repo === undefined) {
		consumer.unexpected(descriptions.MANIFEST.MISSING_HOSTED_GIT_REPO);
		repo = "unknown";
	}

	const incomplete: IncompleteHostedGitPattern = {
		type: "hosted-git",
		host,
		user,
		repo,
		commitish,
	};

	return {
		...incomplete,
		url: getHostedGitURL(incomplete),
	};
}

export function getHostedGitURL(pattern: IncompleteHostedGitPattern): string {
	switch (pattern.host) {
		case "bitbucket":
			return "";

		case "gitlab":
			return "";

		case "github":
			return "";
	}
}

//# Regular Git
type GitPattern = UrlWithHash & {
	type: "git";
};

const GIT_PATTERN_MATCHERS = [
	/^git:/,
	/^git\+.+:/,
	/^ssh:/,
	/^https?:.+\.git$/,
	/^https?:.+\.git#.+/,
];

function parseGit(pattern: string, consumer: Consumer): GitPattern {
	return {
		type: "git",
		...explodeHashUrl(pattern, consumer),
	};
}

//# HTTP Tarball
type HTTPTarballPattern = UrlWithHash & {
	type: "http-tarball";
};

function parseHttpTarball(
	pattern: string,
	consumer: Consumer,
): HTTPTarballPattern {
	return {
		type: "http-tarball",
		...explodeHashUrl(pattern, consumer),
	};
}

//# Semver Range
type SemverPattern = {
	type: "semver";
	range: SemverRangeNode;
};

function parseSemver(
	pattern: string,
	consumer: Consumer,
	loose: boolean,
): SemverPattern {
	const ast = tryParseWithOptionalOffsetPosition(
		{
			loose,
			path: consumer.path,
			input: pattern,
		},
		{
			getOffsetPosition: () => consumer.getLocation("inner-value").start,
			parse: (opts) => parseSemverRange(opts),
		},
	);

	return {
		type: "semver",
		range: ast,
	};
}

//# File
const FILE_PREFIX_REGEX = /^\.{1,2}\//;

type FilePattern = {
	type: "file";
	path: string;
};

function parseFile(pattern: string): FilePattern {
	return {
		type: "file",
		path: removePrefix("file:", pattern),
	};
}

//# Tag

// This regex will likely need to be refined, not sure what the allowable characters of a tag are
const TAG_REGEX = /^[a-z]+$/g;

type TagPattern = {
	type: "tag";
	tag: string;
};

function parseTag(pattern: string): TagPattern {
	return {
		type: "tag",
		tag: pattern,
	};
}

//# Workspace
const WORKSPACE_PREFIX = "workspace:";

type WorkspacePattern = {
	type: "workspace";
	path: string;
};

function parseWorkspace(pattern: string): WorkspacePattern {
	return {
		type: "workspace",
		path: pattern.slice(WORKSPACE_PREFIX.length),
	};
}

//# Link
const LINK_PREFIX = "link:";

type LinkPattern = {
	type: "link";
	path: UnknownPath;
};

function parseLink(pattern: string): LinkPattern {
	return {
		type: "link",
		path: createUnknownPath(pattern.slice(LINK_PREFIX.length)),
	};
}

//# Explicit npm
const NPM_PREFIX = "npm:";

type NpmPattern = {
	type: "npm";
	name: ManifestName;
	range: undefined | SemverRangeNode;
};

function parseNpm(
	pattern: string,
	consumer: Consumer,
	loose: boolean,
): NpmPattern {
	// Prune prefix
	let offset = NPM_PREFIX.length;
	pattern = pattern.slice(NPM_PREFIX.length);

	if (pattern === "") {
		consumer.unexpected(descriptions.MANIFEST.EMPTY_NPM_PATTERN);
		return {
			type: "npm",
			name: {
				org: undefined,
				packageName: undefined,
			},
			range: undefined,
		};
	}

	// Split and verify count
	const parts = pattern.split("@");
	let nameRaw = "";
	let rangeRaw: undefined | string;

	// Org signifier
	if (parts[0] === "") {
		nameRaw += "@";
		parts.shift();
	}

	// Name - We know there'll be at least two due to the empty string conditional
	nameRaw = String(parts.shift());

	// Range
	rangeRaw = parts.shift();

	if (parts.length > 0) {
		consumer.unexpected(descriptions.MANIFEST.TOO_MANY_NPM_PARTS);
	}

	const name = normalizeName({
		name: nameRaw,
		loose,
		unexpected({description, at, start, end}) {
			consumer.unexpected(
				description,
				{
					at,
					loc: start === undefined
						? undefined
						: consumer.getLocationRange(
								ob1Add(start, offset),
								end === undefined ? undefined : ob1Add(end, offset),
								"inner-value",
							),
				},
			);
		},
	});

	// Increase offset passed name
	offset += nameRaw.length;
	offset++;

	let range: undefined | SemverRangeNode;
	if (rangeRaw !== undefined) {
		range = tryParseWithOptionalOffsetPosition(
			{
				loose,
				path: consumer.path,
				input: rangeRaw,
			},
			{
				getOffsetPosition: () => {
					const pos = consumer.getLocation("inner-value").start;
					return {
						...pos,
						column: ob1Add(pos.column, offset),
					};
				},
				parse: (opts) => parseSemverRange(opts),
			},
		);
	}

	return {
		type: "npm",
		name,
		range,
	};
}

//#
export function parseGitDependencyPattern(
	consumer: Consumer,
): undefined | GitPattern | HostedGitPattern {
	const pattern = consumer.asString();

	for (const host of HOSTED_GIT_PREFIXES) {
		const prefix = `${host}:`;
		if (pattern.startsWith(prefix)) {
			return parseHostedGit(host, removePrefix(prefix, pattern), consumer);
		}
	}

	for (const matcher of GIT_PATTERN_MATCHERS) {
		if (matcher.test(pattern)) {
			return parseGit(pattern, consumer);
		}
	}

	if (GITHUB_SHORTHAND.test(pattern)) {
		return parseHostedGit("github", pattern, consumer);
	}

	return undefined;
}

// Check if we received something that looks like a pattern that we don't support
const UNSUPPORTED_PATTERN = /^([a-z]+):/i;

export function parseDependencyPattern(
	consumer: Consumer,
	loose: boolean,
): DependencyPattern {
	const pattern = consumer.asString();

	const gitPattern = parseGitDependencyPattern(consumer);
	if (gitPattern !== undefined) {
		return gitPattern;
	}

	if (pattern.startsWith("http://") || pattern.startsWith("https://")) {
		return parseHttpTarball(pattern, consumer);
	}

	if (pattern.startsWith(WORKSPACE_PREFIX)) {
		return parseWorkspace(pattern);
	}

	if (pattern.startsWith(GIST_PREFIX)) {
		return parseGist(pattern);
	}

	if (pattern.startsWith(NPM_PREFIX)) {
		return parseNpm(pattern, consumer, loose);
	}

	if (pattern.startsWith(LINK_PREFIX)) {
		return parseLink(pattern);
	}

	if (
		FILE_PREFIX_REGEX.test(pattern) ||
		createUnknownPath(pattern).isAbsolute() ||
		pattern.startsWith("file:")
	) {
		return parseFile(pattern);
	}

	if (pattern.match(TAG_REGEX)) {
		return parseTag(pattern);
	}

	const unsupportedMatch = pattern.match(UNSUPPORTED_PATTERN);
	if (unsupportedMatch != null) {
		throw consumer.unexpected(
			descriptions.MANIFEST.UNSUPPORTED_DEPENDENCY_PATTERN_PREFIX(
				unsupportedMatch[1],
			),
		);
	}

	return parseSemver(pattern, consumer, loose);
}

export function normalizeDependencies(
	root: Consumer,
	key: string,
	loose: boolean,
): ManifestDependencies {
	const map: ManifestDependencies = new Map();

	if (!root.has(key)) {
		return map;
	}

	const consumer = root.get(key);

	// Some ridiculous code has the dependencies property as an empty array
	if (Array.isArray(consumer.asUnknown()) && loose) {
		return map;
	}

	for (const [rawName, value] of consumer.asMap()) {
		const nameObj = normalizeName({
			name: rawName,
			loose,
			unexpected: ({description, at}) => {
				value.unexpected(
					description,
					{
						at,
						target: "key",
					},
				);
			},
		});

		const name = manifestNameToString(nameObj);
		if (name !== undefined) {
			map.set(name, parseDependencyPattern(value, loose));
		}
	}

	return map;
}
