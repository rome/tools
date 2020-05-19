/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from "@romejs/consume";
import {
	SemverRangeNode,
	parseSemverRange,
	stringifySemver,
} from "@romejs/codec-semver";
import {tryParseWithOptionalOffsetPosition} from "@romejs/parser-core";
import {UnknownFilePath, createUnknownFilePath} from "@romejs/path";
import {normalizeName} from "./name";
import {ob1Add} from "@romejs/ob1";
import {descriptions} from "@romejs/diagnostics";
import {ManifestName} from "./types";

export type DependencyPattern =
	| HostedGitPattern
	| HTTPTarballPattern
	| SemverPattern
	| GitPattern
	| FilePattern
	| TagPattern
	| NpmPattern
	| LinkPattern;

export type ManifestDependencies = Map<ManifestName, DependencyPattern>;

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

//# HOSTED GIT
export type HostedGitHost = "bitbucket" | "github" | "gist" | "gitlab";

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
	"gist",
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
		case "gist":
			return "";

		case "github":
			return "";
	}
}

//# REGULAR GIT
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

//# TARBALL
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

//# SEMVER
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

//# FILE
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

//# TAG

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

//# LINK
const LINK_PREFIX = "link:";

type LinkPattern = {
	type: "link";
	path: UnknownFilePath;
};

function parseLink(pattern: string): LinkPattern {
	return {
		type: "link",
		path: createUnknownFilePath(pattern.slice(LINK_PREFIX.length)),
	};
}

//# NPM
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

	if (pattern.startsWith(NPM_PREFIX)) {
		return parseNpm(pattern, consumer, loose);
	}

	if (pattern.startsWith(LINK_PREFIX)) {
		return parseLink(pattern);
	}

	if (
		FILE_PREFIX_REGEX.test(pattern) ||
		createUnknownFilePath(pattern).isAbsolute() ||
		pattern.startsWith("file:")
	) {
		return parseFile(pattern);
	}

	if (pattern.match(TAG_REGEX)) {
		return parseTag(pattern);
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
		const name = normalizeName({
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

		map.set(name, parseDependencyPattern(value, loose));
	}

	return map;
}
