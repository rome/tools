/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	Manifest,
	ManifestDefinition,
	ManifestExportCondition,
} from "@internal/codec-js-manifest";
import Server from "../Server";
import {PLATFORM_ALIASES, Platform} from "../../common/types/platform";
import {
	ProjectConfig,
	ProjectDefinition,
	createDefaultProjectConfig,
} from "@internal/project";
import {FileReference} from "@internal/core";
import resolverSuggest from "./resolverSuggest";
import {
	AbsoluteFilePath,
	Path,
	RelativePath,
	URLPath,
	createRelativePath,
	createRelativePathFromSegments,
} from "@internal/path";
import {DiagnosticAdvice, DiagnosticLocation} from "@internal/diagnostics";
import {IMPLICIT_JS_EXTENSIONS} from "../../common/file-handlers/javascript";
import {MOCKS_DIRECTORY_NAME} from "@internal/core/common/constants";
import {Consumer} from "@internal/consume";
import {markup} from "@internal/markup";
import https = require("https");
import {
	buildPathFromAliasPattern,
	matchAliasPattern,
} from "@internal/project/aliases";

function request(
	url: string,
	query: ResolverRemoteQuery,
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
					resolve({
						type: "FETCH_ERROR",
						query,
						advice: [
							{
								type: "log",
								category: "info",
								text: markup`<hyperlink target="${url}" /> returned a ${String(
									res.statusCode,
								)} HTTP status code`,
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
					query,
					advice: [
						{
							type: "log",
							category: "info",
							text: markup`<hyperlink target="${url}" /> resulted in the error "${err.message}"`,
						},
					],
				});
			},
		);
	});
}

const NODE_MODULES = "node_modules";

export interface ResolverRemoteQuery extends Omit<ResolverOptions, "origin"> {
	origin: URLPath | AbsoluteFilePath;
	source: Path;
	location?: DiagnosticLocation;
	// Allows a resolution to stop at a directory or package boundary
	requestedType?: "package" | "directory";
	// Treat the source as a path (without being explicitly relative), and then a module/package if it fails to resolve
	entry?: boolean;
	// Strict disables implicit extensions
	strict?: boolean;
}

export interface ResolverLocalQuery extends Omit<ResolverRemoteQuery, "origin"> {
	origin: AbsoluteFilePath;
}
export interface ResolverEntryQuery extends ResolverRemoteQuery {
	allowPartial?: boolean;
}

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
	types: ResolverQueryResponseFoundType[];
	path: AbsoluteFilePath;
	ref: FileReference;
};

export type ResolverQueryResponseMissing = {
	type: "MISSING";
	// If `query` is `undefined` then it indicates we can try alternative locations
	// This is never a ResolverRemoteQuery as we never have misses
	query: undefined | ResolverLocalQuery;
	advice?: undefined;
};

export type ResolverQueryResponseUnsupported = {
	type: "UNSUPPORTED";
	query: ResolverRemoteQuery;
	advice: DiagnosticAdvice[];
};

export type ResolverQueryResponseFetchError = {
	type: "FETCH_ERROR";
	query: ResolverRemoteQuery;
	advice: DiagnosticAdvice[];
};

type FilenameVariant = {
	path: Path;
	types: ResolverQueryResponseFoundType[];
};

export type ResolverQueryResponseNotFound =
	| ResolverQueryResponseMissing
	| ResolverQueryResponseFetchError
	| ResolverQueryResponseUnsupported;

export type ResolverQueryResponse =
	| ResolverQueryResponseFound
	| ResolverQueryResponseNotFound;

const QUERY_RESPONSE_MISSING: ResolverQueryResponseMissing = {
	type: "MISSING",
	query: undefined,
};

function shouldReturnQueryResponse(res: ResolverQueryResponse): boolean {
	return res.type === "FOUND" || res.query !== undefined;
}

function appendTypeQueryResponse(
	res: ResolverQueryResponse,
	types: ResolverQueryResponseFoundType[],
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
	value: RelativePath;
};

function attachExportAliasIfUnresolved(
	res: ResolverQueryResponse,
	alias: ExportAlias,
	query: ResolverLocalQuery,
): ResolverQueryResponse {
	if (res.type === "FOUND") {
		return res;
	}

	const location = alias.key.getDiagnosticLocation("value");

	return {
		...res,
		query: {
			...query,
			location,
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
		relative: RelativePath;
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

	const alias = resolveExportCondition(aliases.get(platform));
	if (alias !== undefined) {
		return alias;
	}

	const def = resolveExportCondition(aliases.get("default"));
	if (def !== undefined) {
		return def;
	}

	// TODO check for directory aliases
	return undefined;
}

function resolveExportCondition(
	entry: undefined | ManifestExportCondition,
): undefined | ExportAlias {
	if (entry === undefined) {
		return undefined;
	} else if (entry.type === "relative") {
		return {
			key: entry.consumer,
			value: entry.relative,
		};
	} else {
		return resolveExportCondition(entry.conditions.get(""));
	}
}

function getPreferredMainKey(
	consumer: Consumer,
	manifest: Manifest,
	platform?: Platform,
): undefined | ExportAlias {
	const alias = getExportsAlias({
		manifest,
		relative: createRelativePath("."),
		platform,
	});
	if (alias !== undefined) {
		return alias;
	}

	if (manifest.main !== undefined) {
		return {
			key: consumer.get("main"),
			value: manifest.main,
		};
	}

	return undefined;
}

export default class Resolver {
	constructor(server: Server) {
		this.server = server;
	}

	private server: Server;

	public init() {}

	private async findProjectFromQuery(query: ResolverRemoteQuery) {
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

	public async resolveEntryAssert(
		query: ResolverEntryQuery,
	): Promise<ResolverQueryResponseFound> {
		const attempt = await this.maybeResolveEntryWithoutFileSystem(query);
		if (attempt !== undefined) {
			return attempt;
		}

		await this.findProjectFromQuery(query);
		return this.resolveAssert({...query, entry: true});
	}

	// I found myself wanting only `ref.path` a lot so this is just a helper method
	public async resolveEntryAssertPath(
		query: ResolverEntryQuery,
	): Promise<AbsoluteFilePath> {
		const res = await this.resolveEntryAssert(query);
		return res.path;
	}

	public async resolveEntry(
		query: ResolverEntryQuery,
	): Promise<ResolverQueryResponse> {
		const attempt = await this.maybeResolveEntryWithoutFileSystem(query);
		if (attempt !== undefined) {
			return attempt;
		}

		await this.findProjectFromQuery(query);
		return this.resolveRemote({...query, entry: true});
	}

	private async maybeResolveEntryWithoutFileSystem(
		query: ResolverEntryQuery,
	): Promise<undefined | ResolverQueryResponseFound> {
		if (query.allowPartial === false) {
			return undefined;
		}

		const {projectManager} = this.server;
		let absolute: AbsoluteFilePath;

		if (query.source.isAbsolute()) {
			absolute = query.source.assertAbsolute();
		} else if (query.origin.isAbsolute()) {
			absolute = query.origin.assertAbsolute().append(
				query.source.assertRelative(),
			);
		} else {
			// URL or something?
			return undefined;
		}

		// If we have loaded projects then there's no point doing our dirty checks
		if (projectManager.findLoadedProject(absolute) !== undefined) {
			return undefined;
		}

		// Found an exact match
		if ((await absolute.exists()) && (await absolute.lstat()).isFile()) {
			const project = await projectManager.findProject(absolute, true);
			if (project === undefined) {
				return undefined;
			}

			return this.finishResolverQueryResponse(absolute);
		}

		return undefined;
	}

	public async resolveAssert(
		query: ResolverRemoteQuery,
	): Promise<ResolverQueryResponseFound> {
		const resolved = await this.resolveRemote(query);
		if (resolved.type === "FOUND") {
			return resolved;
		} else {
			throw resolverSuggest({
				resolver: this,
				server: this.server,
				rootQuery: query,
				resolved,
			});
		}
	}

	private async resolveRemote(
		query: ResolverRemoteQuery,
	): Promise<ResolverQueryResponse> {
		const {origin, source} = query;

		if (source.isURL()) {
			const sourceURL = source.assertURL();
			const protocol = sourceURL.getProtocol();

			switch (protocol) {
				case "http:":
				case "https:": {
					const projectConfig = this.getProjectConfig(query);
					const remotePath = projectConfig.files.vendorPath.append(
						source.join().replace(/[\/:]/g, "$").replace(/\$+/g, "$"),
					);

					if (!this.server.memoryFs.exists(remotePath)) {
						const result = await request(source.join(), query);
						if (result.type === "DOWNLOADED") {
							await remotePath.writeFile(result.content);
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
						query,
						advice: [
							{
								type: "log",
								category: "info",
								text: markup`<emphasis>${protocol}</emphasis> is not a supported remote protocol`,
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
					query: undefined,
				};
			}
		}

		if (source.isDataURI()) {
			// TODO
		}

		if (origin.isDataURI()) {
			return QUERY_RESPONSE_MISSING;
		}

		return this.resolveLocal({
			...query,
			origin: query.origin.assertAbsolute(),
		});
	}

	public resolveLocal(query: ResolverLocalQuery): ResolverQueryResponse {
		// Unambiguously a file path
		if (query.source.isAbsolute() || query.source.isExplicitRelative()) {
			return this.resolvePath(query);
		}

		// Try aliased paths first if they exist
		const projectConfig = this.getProjectConfig(query);
		for (const aliasedPath of this.getAliasedPaths(query, projectConfig.aliases)) {
			const resolved = this.resolvePath({
				...query,
				source: aliasedPath,
			});
			if (resolved.type === "FOUND") {
				return resolved;
			}
		}

		// Now resolve it as a module
		const resolved = this.resolveModule(query);

		// If we didn't resolve to a module, and we were asked to resolve relative, then do that
		if (resolved.type === "MISSING" && query.entry === true) {
			return this.resolvePath(query);
		}

		return resolved;
	}

	private getProjectConfig(
		query: ResolverLocalQuery | ResolverRemoteQuery,
	): ProjectConfig {
		let projectConfig = createDefaultProjectConfig();
		if (query.origin.isAbsolute()) {
			const project = this.server.projectManager.findLoadedProject(
				query.origin.assertAbsolute(),
			);
			if (project !== undefined) {
				projectConfig = project.config;
			}
		}
		return projectConfig;
	}

	private *getAliasedPaths(
		query: ResolverLocalQuery,
		aliasesConfig: ProjectConfig["aliases"],
	): Iterable<AbsoluteFilePath> {
		const queryPath = query.source.toString();
		for (const [alias, targets] of aliasesConfig.paths) {
			const matchedPath = matchAliasPattern(queryPath, alias);
			if (matchedPath === undefined) {
				continue;
			}

			for (const target of targets) {
				const newPath = buildPathFromAliasPattern(matchedPath, target);
				yield aliasesConfig.base.resolve(newPath);
			}
		}
	}

	private *getFilenameVariants(
		query: ResolverLocalQuery,
		path: Path,
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

	private *_getFilenameVariants(
		query: ResolverLocalQuery,
		path: Path,
		callees: ResolverQueryResponseFoundType[],
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
						path.changeExtension(`.${platform}`),
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
		if (handler?.canHaveScale === true && !callees.includes("implicitScale")) {
			const scale = query.scale ?? 3;
			for (let i = scale; i >= 1; i--) {
				yield* this._getFilenameVariants(
					query,
					path.changeBasename(
						`${path.getExtensionlessBasename()}@${String(i)}x${path.getExtensions()}`,
					),
					[...callees, "implicitScale"],
				);
			}
		}
	}

	private finishResolverQueryResponse(
		path: AbsoluteFilePath,
		types: ResolverQueryResponseFoundType[] = [],
	): ResolverQueryResponseFound {
		return {
			type: "FOUND",
			types,
			ref: this.server.projectManager.getFileReference(path),
			path: this.server.memoryFs.coalescePath(path),
		};
	}

	public getOriginDirectory(query: ResolverLocalQuery): AbsoluteFilePath {
		const {memoryFs} = this.server;
		const {origin} = query;

		if (memoryFs.isFile(origin)) {
			return origin.getParent();
		} else {
			return origin;
		}
	}

	private resolvePath(
		query: ResolverLocalQuery,
		checkVariants: boolean = true,
		types?: ResolverQueryResponseFoundType[],
	): ResolverQueryResponse {
		const {memoryFs} = this.server;

		// Resolve the path heiarchy
		const originDirectory = this.getOriginDirectory(query);
		const resolvedOrigin = originDirectory.resolve(query.source);
		if (!resolvedOrigin.isFilePath()) {
			return QUERY_RESPONSE_MISSING;
		}

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
					const mainQuery: ResolverLocalQuery = {
						...query,
						origin: resolvedOrigin,
						source: main.value,
					};

					const resolved = this.resolvePath(mainQuery, true, ["package"]);

					return attachExportAliasIfUnresolved(resolved, main, mainQuery);
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

	private resolvePackageDirectory(
		query: ResolverLocalQuery,
		moduleName: string,
	): undefined | ManifestDefinition {
		// Find the project
		const project = this.server.projectManager.findLoadedProject(query.origin);
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

	private resolvePackage(
		query: ResolverLocalQuery,
		moduleName: string,
		moduleNameParts: string[],
	): ResolverQueryResponse {
		const manifestDef = this.resolvePackageDirectory(query, moduleName);
		return this.resolveManifest(query, manifestDef, moduleNameParts);
	}

	private resolveManifest(
		query: ResolverLocalQuery,
		manifestDef: undefined | ManifestDefinition,
		moduleNameParts: string[],
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
					relative: createRelativePathFromSegments(moduleNameParts),
					platform: query.platform,
				});

				if (alias === undefined) {
					// No submodule found
					return QUERY_RESPONSE_MISSING;
				}

				// Alias found!
				const mainQuery = {
					...query,
					source: manifestDef.directory.append(alias.value),
				};
				const resolved = this.resolvePath(mainQuery, true, ["package"]);
				return attachExportAliasIfUnresolved(resolved, alias, mainQuery);
			}
		}

		// All exports are enabled or we are importing the root
		return this.resolvePath(
			{
				...query,
				source: manifestDef.directory.append(...moduleNameParts),
			},
			true,
			["package"],
		);
	}

	private resolveMock(
		query: ResolverLocalQuery,
		project: ProjectDefinition | undefined,
		parentDirectories: Iterable<AbsoluteFilePath>,
	): ResolverQueryResponse {
		if (project === undefined) {
			return QUERY_RESPONSE_MISSING;
		}

		const moduleName = query.source.assertRelative();

		for (const dir of parentDirectories) {
			const mocksDir = dir.append(MOCKS_DIRECTORY_NAME);

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
	private splitModuleName(path: Path): [string, string[]] {
		// fetch the first part of the path as that's the module name
		// possible values of `moduleNameFull` could be `react` or `react/lib/whatever`
		const [moduleName, ...moduleNameParts] = path.getSegments();

		// For scoped modules in the form of `@internal/bar`, make sure we keep the `/bar` on the module name
		if (moduleName[0] === "@" && moduleNameParts.length > 0) {
			return [`${moduleName}/${moduleNameParts.shift()}`, moduleNameParts];
		}

		return [moduleName, moduleNameParts];
	}

	private resolveModule(query: ResolverLocalQuery): ResolverQueryResponse {
		const {origin, source} = query;

		// Get project for the origin
		const project = this.server.projectManager.findLoadedProject(origin);

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
					source: virtualResolved.append(...moduleNameParts),
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
