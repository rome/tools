import {Consumer} from "@internal/consume";
import {SemverVersionNode, parseSemverVersion} from "@internal/codec-semver";
import {
	SPDXLicenseParseResult,
	SPDXLicenseParserOptions,
	parseSPDXLicense,
} from "@internal/codec-spdx-license";
import {normalizeDependencies} from "./dependencies";
import {
	MString,
	Manifest,
	ManifestDependencies,
	ManifestEnvironment,
	ManifestFiles,
	ManifestMetadata,
	ManifestPeople,
	ManifestStringMap,
	ManifestURLs,
} from "../types";
import {tryParseWithOptionalOffsetPosition} from "@internal/parser-core";
import {manifestNameToString} from "./name";
import {descriptions} from "@internal/diagnostics";
import {AbsoluteFilePath} from "@internal/path";
import {toCamelCase} from "@internal/string-utils";
import {PathPattern, parsePathPattern} from "@internal/path-match";
import {normalizeCompatManifest} from "@internal/codec-js-manifest/normalize/compat";
import {CompilerProject} from "@internal/compiler";
import {
	normalizeBugsField,
	normalizeExportsField,
	normalizeNameField,
	normalizePeopleField,
	normalizePersonField,
	normalizeRepoField,
} from "./fields";

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

function normalizePathPatterns(
	consumer: Consumer,
	loose: boolean,
): PathPattern[] {
	return normalizeStringArray(consumer, loose).map((str) =>
		parsePathPattern({
			input: str,
		})
	);
}

function normalizeStringArray(consumer: Consumer, loose: boolean): string[] {
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
): ManifestStringMap {
	const map: ManifestStringMap = new Map();

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
): ManifestStringMap {
	const map: ManifestStringMap = new Map();
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

function normalizeLicense(
	consumer: Consumer,
	{name, version}: {
		name: undefined | string;
		version: undefined | SemverVersionNode;
	},
	{projects, path, loose}: NormalizeContext,
): undefined | SPDXLicenseParseResult {
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

	const opts: SPDXLicenseParserOptions = {
		loose,
		path,
		input: licenseId,
	};

	if (name !== undefined && version !== undefined) {
		opts.exceptions = {
			packageName: name,
			packageVersion: version,
			projects,
		};
	}

	return tryParseWithOptionalOffsetPosition(
		opts,
		{
			getOffsetPosition: () => licenseProp.getLocation("inner-value").start,
			parse: (opts) => parseSPDXLicense(opts),
		},
	);
}

function normalizeVersion(
	{consumer, path, loose}: NormalizeContext,
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
			path,
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

function normalizeManifestMetadata(
	context: NormalizeContext,
): {
	metadata: ManifestMetadata;
	parsedLicense: undefined | SPDXLicenseParseResult;
} {
	const {consumer, loose} = context;
	const name = normalizeNameField(consumer, loose);
	const version = normalizeVersion(context);

	if (loose) {
		normalizeCompatManifest(consumer, name, version);
	}

	const strName = name === undefined ? undefined : manifestNameToString(name);

	const parsedLicense = normalizeLicense(
		consumer,
		{name: strName, version},
		context,
	);

	return {
		parsedLicense,
		metadata: {
			name,
			version,
			private: consumer.get("private").asBoolean(false),
			description: consumer.get("description").asStringOrVoid(),
			license: parsedLicense?.license,
			keywords: normalizeStringArray(consumer.get("keywords"), loose),
		},
	};
}

function normalizeManifestDependencies(
	{consumer, loose}: NormalizeContext,
): ManifestDependencies {
	return {
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
	};
}

function normalizeManifestFiles(
	{consumer, loose}: NormalizeContext,
	packageName: MString,
): ManifestFiles {
	return {
		type: consumer.get("type").asStringSetOrVoid(["module", "commonjs"]),
		bin: normalizeBin(consumer, packageName, loose),
		scripts: normalizeStringMap(consumer, "scripts", loose),
		files: normalizePathPatterns(consumer.get("files"), loose),
		main: consumer.get("main").asRelativePathOrVoid(),
		exports: normalizeExportsField(consumer.get("exports")),
	};
}

function normalizeManifestURLs(
	{consumer, loose}: NormalizeContext,
): ManifestURLs {
	return {
		homepage: consumer.get("homepage").asURLPathOrVoid(),
		repository: normalizeRepoField(consumer.get("repository"), loose),
		bugs: normalizeBugsField(consumer.get("bugs"), loose),
	};
}

function normalizeManifestEnvironment(
	{consumer, loose}: NormalizeContext,
): ManifestEnvironment {
	return {
		engines: normalizeStringMap(consumer, "engines", loose),
		cpu: normalizeStringArray(consumer.get("cpu"), loose),
		os: normalizeStringArray(consumer.get("os"), loose),
	};
}

function normalizeManifestPeople(
	{consumer, loose}: NormalizeContext,
): ManifestPeople {
	// Loose: If `author` is an array then implicitly convert it to `maintainers`
	const authorProp = consumer.get("author");
	const contributorProp = consumer.get("contributors");
	let maintainersProp = consumer.get("maintainers");

	let author;
	if (!authorProp.isEmpty()) {
		if (loose && Array.isArray(authorProp.getValue())) {
			maintainersProp = authorProp;
		} else {
			author = normalizePersonField(authorProp, loose);
		}
	}

	const maintainers = normalizePeopleField(maintainersProp, loose);
	const contributors = normalizePeopleField(contributorProp, loose);

	return {maintainers, contributors, author};
}

type NormalizeContext = {
	path: AbsoluteFilePath;
	loose: boolean;
	consumer: Consumer;
	projects: CompilerProject[];
};

export async function normalizeManifest(
	path: AbsoluteFilePath,
	consumer: Consumer,
	projects: CompilerProject[],
): Promise<Manifest> {
	const loose = path.hasSegment("node_modules");

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

	const context: NormalizeContext = {
		path,
		loose,
		consumer,
		projects,
	};

	const {metadata, parsedLicense} = normalizeManifestMetadata(context);

	return {
		...metadata,
		...normalizeManifestDependencies(context),
		...normalizeManifestFiles(context, metadata.name.packageName),
		...normalizeManifestPeople(context),
		...normalizeManifestURLs(context),
		...normalizeManifestEnvironment(context),

		raw: consumer.asJSONObject(),
		diagnostics: {
			license: parsedLicense?.diagnostics,
		},
	};
}
