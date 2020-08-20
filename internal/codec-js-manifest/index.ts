/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from "@internal/consume";
import {SemverVersionNode, parseSemverVersion} from "@internal/codec-semver";
import {
	SPDXExpressionNode,
	parseSPDXLicense,
} from "@internal/codec-spdx-license";
import {normalizeDependencies, parseGitDependencyPattern} from "./dependencies";
import {
	MBoolean,
	MString,
	Manifest,
	ManifestExportConditions,
	ManifestExportNestedCondition,
	ManifestExportRelativeCondition,
	ManifestExports,
	ManifestMap,
	ManifestName,
	ManifestPerson,
	ManifestRepository,
} from "./types";
import {tryParseWithOptionalOffsetPosition} from "@internal/parser-core";
import {normalizeName} from "./name";
import {descriptions} from "@internal/diagnostics";
import {RelativeFilePathMap, createRelativeFilePath} from "@internal/path";
import {toCamelCase} from "@internal/string-utils";
import {PathPatterns, parsePathPattern} from "@internal/path-match";
import {normalizeCompatManifest} from "@internal/codec-js-manifest/compat";

export * from "./types";

export * from "./convert";

export {manifestNameToString} from "./name";

const TYPO_KEYS: Map<string, string> = new Map([
	["autohr", "author"],
	["autor", "author"],
	["contributers", "contributors"],
	["depends", "dependencies"],
	["hampage", "homepage"],
	["hompage", "homepage"],
	["prefereGlobal", "preferGlobal"],
	["publicationConfig", "publishConfig"],
	["repo", "repository"],
	["repostitory", "repository"],
	["script", "scripts"],
]);

function normalizeBoolean(consumer: Consumer, key: string): MBoolean {
	if (consumer.has(key)) {
		return consumer.get(key).asBoolean();
	} else {
		return undefined;
	}
}

function normalizeString(consumer: Consumer, key: string): MString {
	if (consumer.has(key)) {
		return consumer.get(key).asString();
	} else {
		return undefined;
	}
}

function normalizePathPatterns(consumer: Consumer, loose: boolean): PathPatterns {
	return normalizeStringArray(consumer, loose).map((str) =>
		parsePathPattern({
			input: str,
		})
	);
}

function normalizeStringArray(consumer: Consumer, loose: boolean): Array<string> {
	if (consumer.exists()) {
		// When we are loose and expect an array but got a string, consider it to be a single element
		if (loose) {
			const val = consumer.asUnknown();

			if (typeof val === "string") {
				return [consumer.asString()];
			}

			// npm for some reason sometimes populates bundleDependencies as false? Despite it being a misspelling?
			if (val === false) {
				return [];
			}
		}

		return consumer.asMappedArray((item) => item.asString());
	} else {
		return [];
	}
}

function normalizeStringMap(
	root: Consumer,
	key: string,
	loose: boolean,
): ManifestMap {
	const map: ManifestMap = new Map();

	if (!root.has(key)) {
		return map;
	}

	const consumer = root.get(key);

	// Some code uses arrays for this case... Maybe we can normalize them. A `engines` array becomes an object with '*' properties etc
	if (Array.isArray(consumer.asUnknown()) && loose) {
		return map;
	}

	for (const [name, value] of consumer.asMap()) {
		// In loose mode let's be really generous
		if (loose && typeof value.asUnknown() !== "string") {
			continue;
		}

		map.set(name, value.asString());
	}

	return map;
}

function normalizeBin(
	consumer: Consumer,
	name: MString,
	loose: boolean,
): ManifestMap {
	const map: ManifestMap = new Map();
	if (!consumer.has("bin")) {
		return map;
	}

	// Allow a `bin` string
	const obj = consumer.get("bin");
	if (typeof obj.asUnknown() === "string") {
		if (name === undefined) {
			obj.unexpected(descriptions.MANIFEST.STRING_BIN_WITHOUT_NAME);
		} else {
			map.set(name, obj.asString());
			return map;
		}
	}

	// Otherwise expect it to be an object
	return normalizeStringMap(consumer, "bin", loose);
}

function extractLicenseFromObjectConsumer(
	consumer: Consumer,
): [string, Consumer] {
	const prop = consumer.get("type");
	const value = prop.asString();
	return [value, prop];
}

// These are all licenses I found that are wrong, we should eventually remove this as we update those deps
const INVALID_IGNORE_LICENSES = [
	"UNLICENSED",
	"none",
	"Facebook Platform License",
	"BSD",
	"MIT/X11",
	"Public Domain",
	"MIT License",
	"BSD-like",
];

function normalizeLicense(
	consumer: Consumer,
	loose: boolean,
): undefined | SPDXExpressionNode {
	if (!consumer.has("license")) {
		return undefined;
	}

	let licenseProp = consumer.get("license");
	let licenseId;

	// Support some legacy ways of specifying licenses: https://docs.npmjs.com/files/package.json#license
	const raw = licenseProp.asUnknown();
	if (loose && Array.isArray(raw)) {
		const licenseIds = licenseProp.asMappedArray((consumer) =>
			extractLicenseFromObjectConsumer(consumer)[0]
		);
		licenseId = `(${licenseIds.join(" OR ")})`;
	} else if (loose && typeof raw === "object") {
		[licenseId, licenseProp] = extractLicenseFromObjectConsumer(licenseProp);
	} else {
		licenseId = licenseProp.asString();
	}

	// Allow referring to a custom license
	if (licenseId.startsWith("SEE LICENSE IN ")) {
		return undefined;
	}

	// Not valid licenses...
	if (INVALID_IGNORE_LICENSES.includes(licenseId)) {
		return undefined;
	}

	// Parse as a SPDX expression
	return tryParseWithOptionalOffsetPosition(
		{
			loose,
			path: consumer.path,
			input: licenseId,
		},
		{
			getOffsetPosition: () => licenseProp.getLocation("inner-value").start,
			parse: (opts) => parseSPDXLicense(opts),
		},
	);
}

function normalizeVersion(
	consumer: Consumer,
	loose: boolean,
): undefined | SemverVersionNode {
	if (!consumer.has("version")) {
		return undefined;
	}

	const prop = consumer.get("version");
	const rawVersion = prop.asString();

	// Used in some package.json templates
	if (rawVersion === "VERSION_STRING") {
		return undefined;
	}

	const ast = tryParseWithOptionalOffsetPosition(
		{
			path: consumer.path,
			input: rawVersion,
			// Some node_modules have bogus versions, like being prefixed with a v like:
			// https://github.com/itinance/react-native-fs/commit/6232d4e392d5b52cca0792fdfe5903b7fb6b1c5c#diff-b9cfc7f2cdf78a7f4b91a753d10865a2R3
			loose,
		},
		{
			getOffsetPosition: () => prop.getLocation("inner-value").start,
			parse: (opts) => parseSemverVersion(opts),
		},
	);
	return ast;
}

function normalizePerson(consumer: Consumer, loose: boolean): ManifestPerson {
	if (typeof consumer.asUnknown() === "string") {
		// Parse the string. Format: name (url) <email>
		const str = consumer.asString();

		const nameMatch = str.match(/^([^(<]+)/);
		let name: string | undefined;
		if (nameMatch) {
			name = nameMatch[0].trim();
		}

		const person: ManifestPerson = {
			name,
			url: undefined,
			email: undefined,
			twitter: undefined,
			github: undefined,
		};

		const emailMatch = str.match(/<([^>]+)>/);
		if (emailMatch) {
			person.email = emailMatch[1];
		}

		const urlMatch = str.match(/\(([^)]+)\)/);
		if (urlMatch) {
			person.url = urlMatch[1];
		}

		return person;
	} else {
		// Validate as an object
		let url: string | undefined = consumer.get("url").asStringOrVoid();

		// Some packages use "web" or "website" instead of "url"
		if (loose) {
			if (url === undefined) {
				url = consumer.get("web").asStringOrVoid();
			}

			if (url === undefined) {
				url = consumer.get("website").asStringOrVoid();
			}
		}

		let github = consumer.get("github").asStringOrVoid();

		if (loose && github === undefined) {
			// Some rando packages use this
			github =
				consumer.get("githubUsername").asStringOrVoid() ||
				consumer.get("github-username").asStringOrVoid();
		}

		const person: ManifestPerson = {
			name: consumer.get("name").asString(loose ? "" : undefined),
			email: consumer.get("email").asStringOrVoid(),
			twitter: consumer.get("twitter").asStringOrVoid(),
			github,
			url,
		};
		if (!loose) {
			consumer.enforceUsedProperties();
		}
		return person;
	}
}

function normalizePeople(
	consumer: Consumer,
	loose: boolean,
): undefined | Array<ManifestPerson> {
	if (!consumer.exists()) {
		return;
	}

	// Some packages have a single maintainer object instead of an array
	if (loose && consumer.isObject()) {
		return [normalizePerson(consumer, loose)];
	}

	// If it's not an array then just leave it. Some people put a URL here.
	if (loose && !Array.isArray(consumer.asUnknown())) {
		return;
	}

	const people: Array<ManifestPerson> = [];

	for (const item of consumer.asIterable()) {
		people.push(normalizePerson(item, loose));
	}

	return people;
}

function normalizeRepo(
	consumer: Consumer,
	loose: boolean,
): undefined | ManifestRepository {
	if (!consumer.exists()) {
		return;
	}

	if (typeof consumer.asUnknown() === "string") {
		let url = consumer.asString();

		// If this is a hosted git shorthand then explode it
		const parsed = parseGitDependencyPattern(consumer);
		if (parsed?.type === "hosted-git") {
			url = parsed.url;
		}

		return {
			type: "git",
			url,
			directory: undefined,
		};
	} else {
		let url: string;
		let type: string;

		if (loose) {
			// A lot of packages omit the "type"
			type = consumer.get("type").asString("git");

			// thanks i hate it
			consumer.markUsedProperty("web");
			consumer.markUsedProperty("git");
			consumer.markUsedProperty("dist");

			// Some gross packages use "repository" instead of "url"
			let looseUrl = consumer.get("url").asStringOrVoid();

			if (looseUrl === undefined) {
				looseUrl = consumer.get("repository").asStringOrVoid();
			}

			if (looseUrl === undefined) {
				consumer.unexpected(descriptions.MANIFEST.MISSING_REPO_URL);
				url = "";
			} else {
				url = looseUrl;
			}
		} else {
			url = consumer.get("url").asString();
			type = consumer.get("type").asString();
		}

		const repo: Manifest["repository"] = {
			type,
			url,
			directory: consumer.get("directory").asStringOrVoid(),
		};
		if (!loose) {
			consumer.enforceUsedProperties();
		}
		return repo;
	}
}

function normalizeExports(consumer: Consumer): boolean | ManifestExports {
	const unknown = consumer.asUnknown();

	// "exports": false
	if (typeof unknown === "boolean") {
		return consumer.asBoolean();
	}

	if (!consumer.exists()) {
		return true;
	}

	const exports: ManifestExports = new RelativeFilePathMap();

	// "exports": "./index.js"
	if (typeof unknown === "string") {
		exports.set(
			createRelativeFilePath("."),
			new Map([["default", createRelativeExportCondition(consumer)]]),
		);
		return exports;
	}

	const dotConditions: ManifestExportConditions = new Map();

	for (const [relative, value] of consumer.asMap()) {
		// If it's not a relative path then it's a platform for the root
		if (relative[0] !== ".") {
			if (exports.size > 0) {
				value.unexpected(descriptions.MANIFEST.MIXED_EXPORTS_PATHS);
			}

			dotConditions.set(relative, createRelativeExportCondition(value));
			continue;
		}

		if (dotConditions.size > 0) {
			value.unexpected(descriptions.MANIFEST.MIXED_EXPORTS_PATHS);
		}

		const conditions = normalizeExportsConditions(value);
		exports.set(value.getKey().asExplicitRelativeFilePath(), conditions);
	}

	if (dotConditions.size > 0) {
		exports.set(createRelativeFilePath("."), dotConditions);
	}

	return exports;
}

function createRelativeExportCondition(
	value: Consumer,
): ManifestExportRelativeCondition {
	return {
		type: "relative",
		consumer: value,
		relative: value.asExplicitRelativeFilePath(),
	};
}

function normalizeExportsConditions(value: Consumer): ManifestExportConditions {
	const conditions: ManifestExportConditions = new Map();
	const unknown = value.asUnknown();

	if (typeof unknown === "string") {
		conditions.set("default", createRelativeExportCondition(value));
	} else if (Array.isArray(unknown)) {
		// Find the first item that passes validation
		for (const elem of value.asIterable()) {
			const {consumer, diagnostics} = elem.capture();
			const result = normalizeExportsConditions(consumer);
			if (diagnostics.length === 0) {
				return result;
			}
		}
	} else {
		for (const [type, prop] of value.asMap()) {
			if (prop.isObject()) {
				const condition: ManifestExportNestedCondition = {
					type: "nested",
					consumer: prop,
					conditions: new Map(),
				};

				for (const [name, subprop] of prop.asMap()) {
					condition.conditions.set(name, createRelativeExportCondition(subprop));
				}

				conditions.set(type, condition);
			} else {
				conditions.set(type, createRelativeExportCondition(prop));
			}
		}
	}

	return conditions;
}

function normalizeBugs(
	consumer: Consumer,
	loose: boolean,
): undefined | Manifest["bugs"] {
	if (!consumer.exists()) {
		return;
	}

	if (typeof consumer.asUnknown() === "string") {
		return {
			email: undefined,
			url: consumer.asString(),
		};
	} else {
		let email = consumer.get("email").asStringOrVoid();

		// Some use a `mail` property
		if (loose && email === undefined) {
			email = consumer.get("mail").asStringOrVoid();
		}

		// TODO remove this
		consumer.markUsedProperty("type");

		const bugs: Manifest["bugs"] = {
			email,
			url: consumer.get("url").asStringOrVoid(),
		};
		if (!loose) {
			consumer.enforceUsedProperties();
		}
		return bugs;
	}
}

function normalizeRootName(consumer: Consumer, loose: boolean): ManifestName {
	if (!consumer.has("name")) {
		return {
			packageName: undefined,
			org: undefined,
		};
	}

	const prop = consumer.get("name");

	return normalizeName({
		name: prop.asString(),
		loose,
		unexpected: ({description, at, start, end}) => {
			prop.unexpected(
				description,
				{
					at,
					loc: start === undefined
						? undefined
						: prop.getLocationRange(start, end, "inner-value"),
				},
			);
		},
	});
}

const DEPENDENCIES_KEYS = ["", "dev", "peer", "optional"];

const INCORRECT_DEPENDENCIES_SUFFIXES = [
	"depdenencies",
	"dependancies",
	"dependecies",
];

function checkDependencyKeyTypo(key: string, prop: Consumer) {
	for (const depPrefixKey of DEPENDENCIES_KEYS) {
		// Ignore if the key is a valid dependency key
		const depKey =
			depPrefixKey === "" ? "dependencies" : `${depPrefixKey}Dependencies`;
		if (key === depKey) {
			return;
		}

		// Check for casing issues
		const lowerKey = key.toLowerCase();
		if (lowerKey === depKey) {
			prop.unexpected(descriptions.MANIFEST.INCORRECT_CAMEL_CASING(key, depKey));
		}

		// Check for common suffix misspellings
		for (const suffix of INCORRECT_DEPENDENCIES_SUFFIXES) {
			if (lowerKey === `${depPrefixKey}${suffix}`) {
				prop.unexpected(descriptions.MANIFEST.TYPO(key, depKey));
			}
		}

		// Check for kebab casing
		if (toCamelCase(depKey) === lowerKey) {
			prop.unexpected(descriptions.MANIFEST.INCORRECT_CAMEL_CASING(key, depKey));
		}
	}
}

export async function normalizeManifest(consumer: Consumer): Promise<Manifest> {
	const loose =
		consumer.path !== undefined &&
		consumer.path.getSegments().includes("node_modules");

	// Check for typos. Ignore them in loose mode.
	if (!loose) {
		for (const [key, prop] of consumer.asMap()) {
			// Check for typos for dependencies
			checkDependencyKeyTypo(key, prop);

			// Check for other typos
			const correctKey = TYPO_KEYS.get(key);
			if (correctKey !== undefined) {
				prop.unexpected(descriptions.MANIFEST.TYPO(key, correctKey));
			}
		}
	}

	const name = normalizeRootName(consumer, loose);
	const version = normalizeVersion(consumer, loose);

	if (loose) {
		normalizeCompatManifest(consumer, name, version);
	}

	return {
		name,
		version,
		private: normalizeBoolean(consumer, "private") === true,
		description: normalizeString(consumer, "description"),
		license: normalizeLicense(consumer, loose),
		type: consumer.get("type").asStringSetOrVoid(["module", "commonjs"]),
		bin: normalizeBin(consumer, name.packageName, loose),
		scripts: normalizeStringMap(consumer, "scripts", loose),
		homepage: normalizeString(consumer, "homepage"),
		repository: normalizeRepo(consumer.get("repository"), loose),
		bugs: normalizeBugs(consumer.get("bugs"), loose),
		engines: normalizeStringMap(consumer, "engines", loose),
		files: normalizePathPatterns(consumer.get("files"), loose),
		keywords: normalizeStringArray(consumer.get("keywords"), loose),
		cpu: normalizeStringArray(consumer.get("cpu"), loose),
		os: normalizeStringArray(consumer.get("os"), loose),
		main: normalizeString(consumer, "main"),
		exports: normalizeExports(consumer.get("exports")),
		// Dependency fields
		dependencies: normalizeDependencies(consumer, "dependencies", loose),
		devDependencies: normalizeDependencies(consumer, "devDependencies", loose),
		optionalDependencies: normalizeDependencies(
			consumer,
			"optionalDependencies",
			loose,
		),
		peerDependencies: normalizeDependencies(consumer, "peerDependencies", loose),
		bundledDependencies: [
			...normalizeStringArray(consumer.get("bundledDependencies"), loose),
			// Common misspelling. We error on the existence of this for strict manifests already.
			...normalizeStringArray(consumer.get("bundleDependencies"), loose),
		],
		// People fields
		author: consumer.has("author") && !consumer.get("author").isEmpty()
			? normalizePerson(consumer.get("author"), loose)
			: undefined,
		contributors: normalizePeople(consumer.get("contributors"), loose),
		maintainers: normalizePeople(consumer.get("maintainers"), loose),
		raw: consumer.asJSONObject(),
	};
}
