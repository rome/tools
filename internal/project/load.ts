/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// In this file, all methods are synchronous. This is pretty gross since the rest of Rome is async everything.
// This is required so we can integrate the project config code in third-party integrations with sync architectures.
// Project configs are initialized very infrequently anyway so we can live with the extremely minor perf hit.
import {Consumer} from "@internal/consume";
import {
	PartialProjectConfig,
	ProjectConfig,
	ProjectConfigMeta,
	ProjectConfigMetaHard,
	ProjectConfigObjects,
	ProjectConfigTarget,
	createDefaultProjectConfig,
} from "./types";
import {parsePathPatternsFile} from "@internal/path-match";
import {
	arrayOfPatterns,
	arrayOfStrings,
	getParentConfigDependencies,
	mergeAbsoluteFilePathSets,
	mergeArrays,
} from "./utils";
import {ConsumeJSONResult, consumeJSONExtra} from "@internal/codec-json";
import {AbsoluteFilePath, AbsoluteFilePathSet} from "@internal/path";
import {exists, lstat, readDirectory, readFileText} from "@internal/fs";
import crypto = require("crypto");
import {parseSemverRange} from "@internal/codec-semver";
import {descriptions} from "@internal/diagnostics";
import {PROJECT_CONFIG_PACKAGE_JSON_FIELD} from "./constants";

const IGNORE_FILENAMES = [".gitignore", ".hgignore"];

type NormalizedPartial = {
	partial: PartialProjectConfig;
	meta: ProjectConfigMetaHard;
};

function categoryExists(consumer: Consumer): boolean {
	if (!consumer.exists()) {
		return false;
	}

	const value = consumer.asUnknown();
	if (typeof value === "boolean") {
		consumer.unexpected(descriptions.PROJECT_CONFIG.BOOLEAN_CATEGORY(value));
		return false;
	}

	return true;
}

export async function loadCompleteProjectConfig(
	projectDirectory: AbsoluteFilePath,
	configPath: AbsoluteFilePath,
): Promise<{
	meta: ProjectConfigMeta;
	config: ProjectConfig;
}> {
	// TODO use consumer.capture somehow here to aggregate errors
	const {partial, meta} = await loadPartialProjectConfig(
		projectDirectory,
		configPath,
	);
	const {consumer} = meta;

	// Produce a defaultConfig with some directory specific values
	const _defaultConfig: ProjectConfig = createDefaultProjectConfig();
	const defaultConfig: ProjectConfig = {
		..._defaultConfig,
		vcs: {
			..._defaultConfig.vcs,
			root: projectDirectory,
		},
	};

	const name = consumer.get("name").asString(
		`project-${projectDirectory.getBasename()}`,
	);

	const config: ProjectConfig = {
		...defaultConfig,
		name,
		root: partial.root === undefined ? defaultConfig.root : partial.root,
		...mergePartialConfig(defaultConfig, partial),
	};

	// Infer VCS ignore files as lint ignore rules
	for (const filename of IGNORE_FILENAMES) {
		const possiblePath = config.vcs.root.append(filename);
		meta.configDependencies.add(possiblePath);

		if (await exists(possiblePath)) {
			const file = await readFileText(possiblePath);

			consumer.handleThrownDiagnostics(() => {
				const patterns = parsePathPatternsFile({
					input: file,
					path: possiblePath,
				});

				// TODO: Maybe these are useful in other places?
				config.lint.ignore = [...config.lint.ignore, ...patterns];
			});
		}
	}

	return {
		config,
		meta,
	};
}

async function loadPartialProjectConfig(
	projectDirectory: AbsoluteFilePath,
	configPath: AbsoluteFilePath,
): Promise<NormalizedPartial> {
	const configFile = await readFileText(configPath);
	const res = consumeJSONExtra({
		path: configPath,
		input: configFile,
	});

	return normalizeProjectConfig(res, configPath, configFile, projectDirectory);
}

export async function normalizeProjectConfig(
	res: ConsumeJSONResult,
	configPath: AbsoluteFilePath,
	configFile: string,
	projectDirectory: AbsoluteFilePath,
): Promise<NormalizedPartial> {
	let {consumer} = res;

	let configSourceSubKey;
	let name: undefined | string;
	const isInPackageJson = configPath.getBasename() === "package.json";
	if (isInPackageJson) {
		// Infer name from package.json
		name = consumer.get("name").asStringOrVoid();

		consumer = consumer.get(PROJECT_CONFIG_PACKAGE_JSON_FIELD);
		configSourceSubKey = PROJECT_CONFIG_PACKAGE_JSON_FIELD;
	}

	const hash = crypto.createHash("sha256").update(configFile).digest("hex");

	const config: PartialProjectConfig = {
		compiler: {},
		bundler: {},
		cache: {},
		lint: {},
		resolver: {},
		develop: {},
		typeCheck: {},
		tests: {},
		files: {},
		vcs: {},
		dependencies: {},
		targets: new Map(),
	};

	if (name !== undefined) {
		config.name = name;
	}

	const meta: ProjectConfigMetaHard = {
		projectDirectory,
		configPath,
		consumer,
		consumersChain: [consumer],
		configHashes: [hash],
		configSourceSubKey,
		configDependencies: getParentConfigDependencies(projectDirectory),
	};

	// We never use `name` here but it's used in `loadCompleteProjectConfig`
	consumer.markUsedProperty("name");

	if (consumer.has("version")) {
		const version = consumer.get("version");

		consumer.handleThrownDiagnostics(() => {
			config.version = parseSemverRange({
				path: consumer.filename,
				input: version.asString(),
				offsetPosition: version.getLocation("inner-value").start,
			});

			// TODO verify that config.version range satisfies current version
		});
	}

	if (consumer.has("root")) {
		config.root = consumer.get("root").asBoolean();
	}

	const cache = consumer.get("cache");
	if (categoryExists(cache)) {
		// TODO
	}

	const resolver = consumer.get("resolver");
	if (categoryExists(resolver)) {
		// TODO
	}

	const bundler = consumer.get("bundler");
	if (categoryExists(bundler)) {
		if (bundler.has("externals")) {
			config.bundler.externals = arrayOfStrings(bundler.get("externals"));
		}
	}

	const typeChecking = consumer.get("typeChecking");
	if (categoryExists(typeChecking)) {
		if (typeChecking.has("enabled")) {
			config.typeCheck.enabled = typeChecking.get("enabled").asBoolean();
		}

		if (typeChecking.has("libs")) {
			const libs = await normalizeTypeCheckingLibs(
				projectDirectory,
				typeChecking.get("libs"),
			);
			config.typeCheck.libs = libs.files;
			meta.configDependencies = new AbsoluteFilePathSet([
				...meta.configDependencies,
				...libs.directories,
				...libs.files,
			]);
		}
	}

	const dependencies = consumer.get("dependencies");
	if (categoryExists(dependencies)) {
		if (dependencies.has("enabled")) {
			config.dependencies.enabled = dependencies.get("dependencies").asBoolean();
		}
	}

	const lint = consumer.get("lint");
	if (categoryExists(lint)) {
		if (lint.has("ignore")) {
			config.lint.ignore = arrayOfPatterns(lint.get("ignore"));
		}

		if (lint.has("globals")) {
			config.lint.globals = arrayOfStrings(lint.get("globals"));
		}
	}

	const tests = consumer.get("tests");
	if (categoryExists(tests)) {
		if (tests.has("ignore")) {
			config.tests.ignore = arrayOfPatterns(tests.get("ignore"));
		}
	}

	const develop = consumer.get("develop");
	if (categoryExists(develop)) {
		if (develop.has("serveStatic")) {
			config.develop.serveStatic = develop.get("serveStatic").asBoolean();
		}
	}

	const files = consumer.get("files");
	if (categoryExists(files)) {
		if (files.has("vendorPath")) {
			config.files.vendorPath = projectDirectory.resolve(
				files.get("vendorPath").asString(),
			);
		}

		if (files.has("maxSize")) {
			config.files.maxSize = files.get("maxSize").asNumber();
		}

		if (files.has("assetExtensions")) {
			config.files.assetExtensions = files.get("assetExtensions").asMappedArray((
				item,
			) => item.asString());
		}
	}

	const vcs = consumer.get("vcs");
	if (categoryExists(vcs)) {
		if (vcs.has("root")) {
			config.vcs.root = projectDirectory.resolve(vcs.get("root").asString());
		}
	}

	const compiler = consumer.get("compiler");
	if (categoryExists(compiler)) {
		// TODO
	}

	const targets = consumer.get("targets");
	if (categoryExists(targets)) {
		for (const [name, object] of targets.asMap()) {
			const target: ProjectConfigTarget = {
				constraints: object.get("constraints").asImplicitMappedArray((item) =>
					item.asString()
				),
			};
			object.enforceUsedProperties("config target property");
			config.targets.set(name, target);
		}
	}

	// Need to get this before enforceUsedProperties so it will be flagged
	const _extends = consumer.get("extends");

	// Flag unknown properties
	consumer.enforceUsedProperties("config property");

	if (_extends.exists()) {
		return await extendProjectConfig(projectDirectory, _extends, config, meta);
	}

	return {
		partial: config,
		meta,
	};
}

async function normalizeTypeCheckingLibs(
	projectDirectory: AbsoluteFilePath,
	consumer: Consumer,
): Promise<{
	directories: Array<AbsoluteFilePath>;
	files: AbsoluteFilePathSet;
}> {
	const libFiles: AbsoluteFilePathSet = new AbsoluteFilePathSet();

	// Normalize library directories
	const directories: Array<AbsoluteFilePath> = arrayOfStrings(consumer).map((
		libDirectory,
	) => projectDirectory.resolve(libDirectory));

	// Crawl library directories and add their files
	for (const directory of directories) {
		const files = await readDirectory(directory);
		for (const file of files) {
			const stats = await lstat(file);
			if (stats.isFile()) {
				libFiles.add(file);
			} else if (stats.isDirectory()) {
				directories.push(file);
			}
		}
	}

	return {
		files: libFiles,
		directories,
	};
}

async function extendProjectConfig(
	projectDirectory: AbsoluteFilePath,
	extendsStrConsumer: Consumer,
	config: PartialProjectConfig,
	meta: ProjectConfigMetaHard,
): Promise<NormalizedPartial> {
	const extendsRelative = extendsStrConsumer.asString();

	if (extendsRelative === "parent") {
		// TODO maybe do some magic here?
	}

	const extendsPath = projectDirectory.resolve(extendsRelative);
	const {partial: extendsObj, meta: extendsMeta} = await loadPartialProjectConfig(
		extendsPath.getParent(),
		extendsPath,
	);

	// Check for recursive config
	if (extendsMeta.configDependencies.has(meta.configPath)) {
		throw extendsStrConsumer.unexpected(
			descriptions.PROJECT_CONFIG.RECURSIVE_CONFIG,
		);
	}

	const merged: PartialProjectConfig = mergePartialConfig(extendsObj, config);

	const lintIgnore = mergeArrays(extendsObj.lint.ignore, config.lint.ignore);
	if (lintIgnore !== undefined) {
		merged.lint.ignore = lintIgnore;
	}

	const lintGlobals = mergeArrays(extendsObj.lint.globals, config.lint.globals);
	if (lintGlobals !== undefined) {
		merged.lint.globals = lintGlobals;
	}

	const testingIgnore = mergeArrays(
		extendsObj.tests.ignore,
		config.tests.ignore,
	);
	if (testingIgnore !== undefined) {
		merged.tests.ignore = testingIgnore;
	}

	const typeCheckingLibs = mergeAbsoluteFilePathSets(
		extendsObj.typeCheck.libs,
		config.typeCheck.libs,
	);
	if (typeCheckingLibs !== undefined) {
		merged.typeCheck.libs = typeCheckingLibs;
	}

	const bundlerExternals = mergeArrays(
		extendsObj.bundler.externals,
		config.bundler.externals,
	);
	if (bundlerExternals !== undefined) {
		merged.bundler.externals = bundlerExternals;
	}

	return {
		partial: merged,
		meta: {
			...meta,
			consumersChain: [...meta.consumersChain, ...extendsMeta.consumersChain],
			configDependencies: new AbsoluteFilePathSet([
				...meta.configDependencies,
				...extendsMeta.configDependencies,
				extendsPath,
			]),
			configHashes: [...meta.configHashes, ...extendsMeta.configHashes],
		},
	};
}

type MergedPartialConfig<
	A extends PartialProjectConfig,
	B extends PartialProjectConfig
> = {[Key in keyof ProjectConfigObjects]: A[Key] & B[Key]};

function mergePartialConfig<
	A extends PartialProjectConfig,
	B extends PartialProjectConfig
>(a: A, b: B): MergedPartialConfig<A, B> {
	return {
		cache: {
			...a.cache,
			...b.cache,
		},
		compiler: {
			...a.compiler,
			...b.compiler,
		},
		lint: {
			...a.lint,
			...b.lint,
		},
		develop: {
			...a.develop,
			...b.develop,
		},
		bundler: {
			...a.bundler,
			...b.bundler,
		},
		dependencies: {
			...a.dependencies,
			...b.dependencies,
		},
		resolver: {
			...a.resolver,
			...b.resolver,
		},
		typeCheck: {
			...a.typeCheck,
			...b.typeCheck,
		},
		tests: {
			...a.tests,
			...b.tests,
		},
		files: {
			...a.files,
			...b.files,
		},
		vcs: {
			...a.vcs,
			...b.vcs,
		},
		targets: new Map([...a.targets.entries(), ...b.targets.entries()]),
	};
}
