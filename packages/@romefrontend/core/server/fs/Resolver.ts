/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Manifest, ManifestDefinition} from "@romefrontend/codec-js-manifest";
import Server from "../Server";
import {PLATFORM_ALIASES, Platform} from "../../common/types/platform";
import {
	ProjectDefinition,
	createDefaultProjectConfig,
} from "@romefrontend/project";
import {FileReference} from "../../common/types/files";
import resolverSuggest from "./resolverSuggest";
import {
	AbsoluteFilePath,
	RelativeFilePath,
	URLFilePath,
	UnknownFilePath,
	createFilePathFromSegments,
	createRelativeFilePath,
} from "@romefrontend/path";
import {DiagnosticAdvice, DiagnosticLocation} from "@romefrontend/diagnostics";
import {IMPLICIT_JS_EXTENSIONS} from "../../common/file-handlers/javascript";
import {writeFile} from "@romefrontend/fs";
import https = require("https");

import {MOCKS_FOLDER_NAME} from "@romefrontend/core/common/constants";
import {Consumer} from "@romefrontend/consume";

function request(
	url: string,
): Promise<
	| ResolverQueryResponseFetchError
	| {
			type: "DOWNLOADED";
			content: string;
		}
> {
	return new Promise((resolve) => {
		const req = https.get(
			url,
			(res) => {
				if (res.statusCode !== 200) {
					console.log("non-200 return");
					resolve({
						type: "FETCH_ERROR",
						source: undefined,
						advice: [
							{
								type: "log",
								category: "info",
								text: `<hyperlink target="${url}" /> returned a ${res.statusCode} status code`,
							},
						],
					});
					return;
				}

				let data = "";

				res.on(
					"data",
					(chunk) => {
						data += chunk;
					},
				);

				res.on(
					"end",
					() => {
						resolve({type: "DOWNLOADED", content: data});
					},
				);
			},
		);

		req.on(
			"error",
			(err) => {
				resolve({
					type: "FETCH_ERROR",
					source: undefined,
					advice: [
						{
							type: "log",
							category: "info",
							text: `<hyperlink target="${url}" /> resulted in the error "${err.message}"`,
						},
					],
				});
			},
		);
	});
}

const NODE_MODULES = "node_modules";

export type ResolverRemoteQuery = Omit<ResolverOptions, "origin"> & {
	origin: URLFilePath | AbsoluteFilePath;
	source: UnknownFilePath;
	// Allows a resolution to stop at a directory or package boundary
	requestedType?: "package" | "directory";
	// Treat the source as a path (without being explicitly relative), and then a module/package if it fails to resolve
	entry?: boolean;
	// Strict disables implicit extensions
	strict?: boolean;
};

export type ResolverLocalQuery = Omit<ResolverRemoteQuery, "origin"> & {
	origin: AbsoluteFilePath;
};

export type ResolverQuerySource =
	| undefined
	| {
			source?: string;
			location?: DiagnosticLocation;
		};

type ResolverQueryResponseFoundType =
	| "package"
	| "mock"
	| "virtual"
	| "implicitPlatform"
	| "implicitScale"
	| "implicitExtension"
	| "implicitIndex";

export type ResolverQueryResponseFound = {
	type: "FOUND";
	types: Array<ResolverQueryResponseFoundType>;
	path: AbsoluteFilePath;
	ref: FileReference;
};

export type ResolverQueryResponseMissing = {
	type: "MISSING";
	source: undefined | ResolverQuerySource;
	advice?: undefined;
};

export type ResolverQueryResponseUnsupported = {
	type: "UNSUPPORTED";
	source: undefined | ResolverQuerySource;
	advice: DiagnosticAdvice;
};

export type ResolverQueryResponseFetchError = {
	type: "FETCH_ERROR";
	source: undefined | ResolverQuerySource;
	advice: DiagnosticAdvice;
};

type FilenameVariant = {
	path: UnknownFilePath;
	types: Array<ResolverQueryResponseFoundType>;
};

const QUERY_RESPONSE_MISSING: ResolverQueryResponseMissing = {
	type: "MISSING",
	source: undefined,
};

export type ResolverQueryResponseNotFound =
	| ResolverQueryResponseMissing
	| ResolverQueryResponseFetchError
	| ResolverQueryResponseUnsupported;

export type ResolverQueryResponse =
	| ResolverQueryResponseFound
	| ResolverQueryResponseNotFound;

function shouldReturnQueryResponse(res: ResolverQueryResponse): boolean {
	return res.type === "FOUND" || res.source !== undefined;
}

export function isPathLike(source: UnknownFilePath): boolean {
	return source.isAbsolute() || source.isExplicitRelative();
}

function appendTypeQueryResponse(
	res: ResolverQueryResponse,
	types: Array<ResolverQueryResponseFoundType>,
): ResolverQueryResponse {
	if (res.type === "FOUND") {
		return {
			...res,
			types: [...res.types, ...types],
		};
	} else {
		return res;
	}
}

export type ResolverOptions = {
	origin?: AbsoluteFilePath;
	mocks?: boolean;
	platform?: Platform;
	scale?: number;
};

type ExportAlias = {
	key: Consumer;
	value: RelativeFilePath;
};

function attachExportAliasIfUnresolved(
	res: ResolverQueryResponse,
	alias: ExportAlias,
) {
	if (res.type === "FOUND") {
		return res;
	}

	const location = alias.key.getDiagnosticLocation("value");

	return {
		...res,
		source: location === undefined
			? undefined
			: {
					location,
					source: alias.value.join(),
				},
	};
}

function getExportsAlias(
	{
		manifest,
		relative,
		platform,
	}: {
		manifest: Manifest;
		relative: UnknownFilePath;
		platform?: Platform;
	},
): undefined | ExportAlias {
	if (typeof manifest.exports === "boolean") {
		return undefined;
	}

	if (platform === undefined) {
		return undefined;
	}

	if (!relative.isRelative()) {
		return undefined;
	}

	const aliases = manifest.exports.get(relative.assertRelative());
	if (aliases === undefined) {
		return undefined;
	}

	const alias = aliases.get(platform);
	if (alias !== undefined) {
		return {
			key: alias.consumer,
			value: alias.relative,
		};
	}

	const def = aliases.get("default");
	if (def !== undefined) {
		return {
			key: def.consumer,
			value: def.relative,
		};
	}

	// TODO check for directory aliases
	return undefined;
}

function getPreferredMainKey(
	consumer: Consumer,
	manifest: Manifest,
	platform?: Platform,
): undefined | ExportAlias {
	const alias = getExportsAlias({
		manifest,
		relative: createRelativeFilePath("."),
		platform,
	});
	if (alias !== undefined) {
		return alias;
	}

	if (manifest.main !== undefined) {
		return {
			key: consumer.get("main"),
			value: createRelativeFilePath(manifest.main),
		};
	}

	return undefined;
}

export default class Resolver {
	constructor(server: Server) {
		this.server = server;
	}

	server: Server;

	init() {}

	async findProjectFromQuery(query: ResolverRemoteQuery) {
		// If we were passed an absolute path then we should find and add the project it belongs to
		if (query.source.isAbsolute()) {
			await this.server.projectManager.findProject(
				query.source.assertAbsolute(),
			);
		} else if (query.origin.isAbsolute()) {
			const origin = query.origin.assertAbsolute();
			await this.server.projectManager.findProject(origin);
			await this.server.projectManager.findProject(
				origin.append(query.source.assertRelative()),
			);
		}
	}

	async resolveEntryAssert(
		query: ResolverRemoteQuery,
		querySource?: ResolverQuerySource,
	): Promise<ResolverQueryResponseFound> {
		await this.findProjectFromQuery(query);
		return this.resolveAssert({...query, entry: true}, querySource);
	}

	// I found myself wanting only `ref.path` a lot so this is just a helper method
	async resolveEntryAssertPath(
		query: ResolverRemoteQuery,
		querySource?: ResolverQuerySource,
	): Promise<AbsoluteFilePath> {
		const res = await this.resolveEntryAssert(query, querySource);
		return res.path;
	}

	async resolveEntry(query: ResolverRemoteQuery): Promise<ResolverQueryResponse> {
		await this.findProjectFromQuery(query);
		return this.resolveRemote({...query, entry: true});
	}

	async resolveAssert(
		query: ResolverRemoteQuery,
		origQuerySource?: ResolverQuerySource,
	): Promise<ResolverQueryResponseFound> {
		const resolved = await this.resolveRemote(query);
		if (resolved.type === "FOUND") {
			return resolved;
		} else {
			throw resolverSuggest(this, query, resolved, origQuerySource);
		}
	}

	async resolveRemote(
		query: ResolverRemoteQuery,
	): Promise<ResolverQueryResponse> {
		const {origin, source} = query;

		if (source.isURL()) {
			const sourceURL = source.assertURL();
			const protocol = sourceURL.getProtocol();

			switch (protocol) {
				case "http":
				case "https": {
					let projectConfig = createDefaultProjectConfig();

					if (origin.isAbsolute()) {
						const project = this.server.projectManager.findProjectExisting(
							query.origin.assertAbsolute(),
						);
						if (project !== undefined) {
							projectConfig = project.config;
						}
					}

					const remotePath = projectConfig.files.vendorPath.append(
						source.join().replace(/[\/:]/g, "$").replace(/\$+/g, "$"),
					);

					if (!this.server.memoryFs.exists(remotePath)) {
						const result = await request(source.join());
						if (result.type === "DOWNLOADED") {
							await writeFile(remotePath, result.content);
						} else {
							return result;
						}
					}

					return {
						type: "FOUND",
						types: [],
						ref: this.server.projectManager.getURLFileReference(
							remotePath,
							sourceURL,
						),
						path: remotePath,
					};
				}

				default:
					return {
						type: "UNSUPPORTED",
						source: undefined,
						advice: [
							{
								type: "log",
								category: "info",
								text: `<emphasis>${protocol}</emphasis> is not a supported remote protocol`,
							},
						],
					};
			}
		}

		if (origin.isURL()) {
			if (source.isAbsolute() || source.isExplicitRelative()) {
				// Relative to the origin
				return this.resolveRemote({
					...query,
					source: origin.resolve(source),
				});
			} else {
				// TODO add support for import maps
				return {
					type: "MISSING",
					source: undefined,
				};
			}
		}

		return this.resolveLocal({
			...query,
			origin: query.origin.assertAbsolute(),
		});
	}

	resolveLocal(query: ResolverLocalQuery): ResolverQueryResponse {
		// Do some basic checks to determine if this is an absolute or relative path
		if (isPathLike(query.source)) {
			return this.resolvePath(query);
		}

		// Now resolve it as a module
		const resolved = this.resolveModule(query);

		// If we didn't resolve to a module, and we were asked to resolve relative, then do that
		if (resolved.type === "MISSING" && query.entry === true) {
			return this.resolvePath(query);
		}

		return resolved;
	}

	*getFilenameVariants(
		query: ResolverLocalQuery,
		path: UnknownFilePath,
	): Iterable<FilenameVariant> {
		const seen: Set<string> = new Set();
		for (const variant of this._getFilenameVariants(query, path, [])) {
			const filename = variant.path.join();
			if (seen.has(filename)) {
				continue;
			}

			seen.add(filename);
			yield variant;
		}
	}

	*_getFilenameVariants(
		query: ResolverLocalQuery,
		path: UnknownFilePath,
		callees: Array<ResolverQueryResponseFoundType>,
	): Iterable<FilenameVariant> {
		const {platform} = query;

		yield {path, types: callees};

		//
		const {handler} = this.server.projectManager.getHandlerWithProject(
			path.isAbsolute() ? path.assertAbsolute() : query.origin,
		);
		const usesUnknownExtension = !query.strict && handler === undefined;

		// Check with appended `platform`
		if (platform !== undefined && !callees.includes("implicitPlatform")) {
			yield* this._getFilenameVariants(
				query,
				path.addExtension(`.${platform}`),
				[...callees, "implicitPlatform"],
			);

			// Check if this platform has any subplatforms
			const platformAliases = PLATFORM_ALIASES[platform];
			if (platformAliases !== undefined) {
				for (const platform of platformAliases) {
					yield* this._getFilenameVariants(
						query,
						path.addExtension(`.${platform}`, true),
						[...callees, "implicitPlatform"],
					);
				}
			}
		}

		// Check with appended extensions
		if (usesUnknownExtension && !callees.includes("implicitExtension")) {
			for (const ext of IMPLICIT_JS_EXTENSIONS) {
				yield* this._getFilenameVariants(
					query,
					path.addExtension(`.${ext}`),
					[...callees, "implicitExtension"],
				);
			}
		}

		// Check with appended `scale`, other.filename
		if (
			handler !== undefined &&
			handler.canHaveScale === true &&
			!callees.includes("implicitScale")
		) {
			const scale = query.scale === undefined ? 3 : query.scale;
			for (let i = scale; i >= 1; i--) {
				yield* this._getFilenameVariants(
					query,
					path.changeBasename(
						`${path.getExtensionlessBasename()}@${String(i)}x${path.memoizedExtension}`,
					),
					[...callees, "implicitScale"],
				);
			}
		}
	}

	finishResolverQueryResponse(
		path: AbsoluteFilePath,
		types?: Array<ResolverQueryResponseFoundType>,
	): ResolverQueryResponse {
		return {
			type: "FOUND",
			types: types === undefined ? [] : types,
			ref: this.server.projectManager.getFileReference(path),
			path,
		};
	}

	getOriginDirectory(query: ResolverLocalQuery): AbsoluteFilePath {
		const {memoryFs} = this.server;
		const {origin} = query;

		if (memoryFs.isFile(origin)) {
			return origin.getParent();
		} else {
			return origin;
		}
	}

	resolvePath(
		query: ResolverLocalQuery,
		checkVariants: boolean = true,
		types?: Array<ResolverQueryResponseFoundType>,
	): ResolverQueryResponse {
		const {memoryFs} = this.server;

		// Resolve the path heiarchy
		const originDirectory = this.getOriginDirectory(query);
		const resolvedOrigin = originDirectory.resolve(query.source);

		// Check if this is an absolute filename
		if (memoryFs.isFile(resolvedOrigin)) {
			// If we're querying a package then we should never return a file
			if (query.requestedType === "package") {
				return QUERY_RESPONSE_MISSING;
			}

			return this.finishResolverQueryResponse(resolvedOrigin, types);
		}

		// Check variants
		if (checkVariants) {
			for (const variant of this.getFilenameVariants(query, resolvedOrigin)) {
				if (variant.path.equal(resolvedOrigin)) {
					continue;
				}

				const resolved = this.resolvePath(
					{...query, source: variant.path},
					false,
					variant.types,
				);

				if (shouldReturnQueryResponse(resolved)) {
					return appendTypeQueryResponse(resolved, variant.types);
				}
			}
		}

		// check if this is a directory
		if (memoryFs.isDirectory(resolvedOrigin)) {
			if (query.requestedType === "directory") {
				return this.finishResolverQueryResponse(resolvedOrigin, types);
			}

			// If this has a package.json then follow the `main` field
			const manifestDef = memoryFs.getManifestDefinition(resolvedOrigin);
			if (manifestDef !== undefined) {
				// If we're resolving a package then don't follow this
				if (query.requestedType === "package") {
					return this.finishResolverQueryResponse(resolvedOrigin, types);
				}

				const main = getPreferredMainKey(
					manifestDef.consumer,
					manifestDef.manifest,
					query.platform,
				);
				if (main !== undefined) {
					const resolved = this.resolvePath(
						{
							...query,
							origin: resolvedOrigin,
							source: main.value,
						},
						true,
						["package"],
					);

					return attachExportAliasIfUnresolved(resolved, main);
				}
			}

			if (!query.strict) {
				// Check if it has an index.* file
				for (const ext of IMPLICIT_JS_EXTENSIONS) {
					const indexResolved = this.resolvePath(
						{
							...query,
							source: resolvedOrigin.append(`index.${ext}`),
						},
						true,
						["implicitIndex"],
					);

					if (shouldReturnQueryResponse(indexResolved)) {
						return indexResolved;
					}
				}
			}
		}

		return QUERY_RESPONSE_MISSING;
	}

	resolvePackageDirectory(
		query: ResolverLocalQuery,
		moduleName: string,
	): undefined | ManifestDefinition {
		// Find the project
		const project = this.server.projectManager.findProjectExisting(query.origin);
		if (project === undefined) {
			return undefined;
		}

		// Find the package
		const projects = this.server.projectManager.getHierarchyFromProject(project);

		for (const project of projects) {
			const pkg = project.packages.get(moduleName);
			if (pkg !== undefined) {
				return pkg;
			}
		}

		return undefined;
	}

	resolvePackage(
		query: ResolverLocalQuery,
		moduleName: string,
		moduleNameParts: Array<string>,
	): ResolverQueryResponse {
		const manifestDef = this.resolvePackageDirectory(query, moduleName);
		return this.resolveManifest(query, manifestDef, moduleNameParts);
	}

	resolveManifest(
		query: ResolverLocalQuery,
		manifestDef: undefined | ManifestDefinition,
		moduleNameParts: Array<string>,
	): ResolverQueryResponse {
		if (manifestDef === undefined) {
			return QUERY_RESPONSE_MISSING;
		}

		if (moduleNameParts.length > 0) {
			// Submodules of this package are private
			if (manifestDef.manifest.exports === false) {
				return QUERY_RESPONSE_MISSING;
			}

			// Check if we're allowed to touch this submodule
			if (manifestDef.manifest.exports !== true) {
				const alias = getExportsAlias({
					manifest: manifestDef.manifest,
					relative: createFilePathFromSegments(moduleNameParts),
					platform: query.platform,
				});

				if (alias === undefined) {
					// No submodule found
					return QUERY_RESPONSE_MISSING;
				}

				// Alias found!
				const resolved = this.resolvePath(
					{
						...query,
						source: manifestDef.directory.append(alias.value),
					},
					true,
					["package"],
				);
				return attachExportAliasIfUnresolved(resolved, alias);
			}
		}

		// All exports are enabled or we are importing the root
		return this.resolvePath(
			{
				...query,
				source: manifestDef.directory.appendList(...moduleNameParts),
			},
			true,
			["package"],
		);
	}

	resolveMock(
		query: ResolverLocalQuery,
		project: ProjectDefinition | undefined,
		parentDirectories: Array<AbsoluteFilePath>,
	): ResolverQueryResponse {
		if (project === undefined) {
			return QUERY_RESPONSE_MISSING;
		}

		const moduleName = query.source.assertRelative();

		for (const dir of parentDirectories) {
			const mocksDir = dir.append(MOCKS_FOLDER_NAME);

			// No use resolving against a directory that doesn't exist
			if (!this.server.memoryFs.exists(mocksDir)) {
				continue;
			}

			const resolved = this.resolveLocal({
				...query,
				source: mocksDir.append(moduleName),
			});

			if (shouldReturnQueryResponse(resolved)) {
				return appendTypeQueryResponse(resolved, ["mock"]);
			}
		}

		return QUERY_RESPONSE_MISSING;
	}

	// Given a reference to a module, extract the module name and any trailing relative paths
	splitModuleName(path: UnknownFilePath): [string, Array<string>] {
		// fetch the first part of the path as that's the module name
		// possible values of `moduleNameFull` could be `react` or `react/lib/whatever`
		const [moduleName, ...moduleNameParts] = path.getSegments();

		// For scoped modules in the form of `@romefrontend/bar`, make sure we keep the `/bar` on the module name
		if (moduleName[0] === "@" && moduleNameParts.length > 0) {
			return [`${moduleName}/${moduleNameParts.shift()}`, moduleNameParts];
		}

		return [moduleName, moduleNameParts];
	}

	resolveModule(query: ResolverLocalQuery): ResolverQueryResponse {
		const {origin, source} = query;

		// Get project for the origin
		const project = this.server.projectManager.findProjectExisting(origin);

		// Get all the parent directories for when we crawl up
		const parentDirectories = this.getOriginDirectory(query).getChain();

		// If mocks are enabled for this query then check all parent mocks directory
		if (query.mocks === true) {
			const mockResolved = this.resolveMock(query, project, parentDirectories);
			if (shouldReturnQueryResponse(mockResolved)) {
				return mockResolved;
			}
		}

		// Extract the module name and it's relative file parts
		const [moduleName, moduleNameParts] = this.splitModuleName(source);

		// Resolve a virtual module
		const virtualResolved = this.server.virtualModules.resolvePossibleVirtualModuleName(
			moduleName,
		);
		if (virtualResolved !== undefined) {
			return this.resolvePath(
				{
					...query,
					source: virtualResolved.appendList(...moduleNameParts),
				},
				true,
				["virtual"],
			);
		}

		// Check if it matches any of our project packages
		const packageResolved = this.resolvePackage(
			query,
			moduleName,
			moduleNameParts,
		);
		if (shouldReturnQueryResponse(packageResolved)) {
			return packageResolved;
		}

		// Check all parent directories for node_modules
		for (const dir of parentDirectories) {
			const modulePath = dir.append(NODE_MODULES).append(moduleName);
			const manifestDef = this.server.memoryFs.getManifestDefinition(modulePath);
			if (manifestDef !== undefined) {
				return this.resolveManifest(query, manifestDef, moduleNameParts);
			}
		}

		return QUERY_RESPONSE_MISSING;
	}
}
