/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Resolver, {
	ResolverLocalQuery,
	ResolverQueryResponseNotFound,
	ResolverRemoteQuery,
} from "./Resolver";
import {
	DiagnosticAdvice,
	buildSuggestionAdvice,
	createSingleDiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {orderBySimilarity} from "@internal/string-utils";
import {AbsoluteFilePath, AbsoluteFilePathSet, FilePath} from "@internal/path";
import {PLATFORMS, Server} from "@internal/core";
import {StaticMarkup, markup} from "@internal/markup";
import {ExtendedMap} from "@internal/collections";

export default function resolverSuggest(
	{resolver, rootQuery, resolved, server}: {
		resolver: Resolver;
		server: Server;
		rootQuery: ResolverRemoteQuery;
		resolved: ResolverQueryResponseNotFound;
	},
): Error {
	const query = resolved.query ?? rootQuery;

	let errMsg = "";
	if (resolved.type === "UNSUPPORTED") {
		errMsg = "Unsupported path format";
	} else if (resolved.type === "MISSING") {
		errMsg = "Cannot find";
	} else if (resolved.type === "FETCH_ERROR") {
		errMsg = "Failed to fetch";
	}

	errMsg += ` "${query.source.join()}" from "${query.origin.join()}"`;

	// Cannot produce a diagnostic location without a query location
	// NB: Actually we can, just point it at query.origin but that doesn't feel right?
	const {location} = query;
	if (location === undefined) {
		// TODO do something about the `advice` on some `resolved` that may contain metadata?
		throw new Error(errMsg);
	}

	let advice: DiagnosticAdvice = [];

	if (query.origin.isAbsolute()) {
		const localQuery: ResolverLocalQuery = {
			...query,
			origin: query.origin.assertAbsolute(),
		};

		// Provide advice in strict-mode if a non-strict version existed
		if (query.strict) {
			const nonStrictResolved = resolver.resolveLocal({
				...localQuery,
				strict: false,
			});

			if (nonStrictResolved.type === "FOUND") {
				if (nonStrictResolved.types.includes("implicitIndex")) {
					advice.push({
						type: "log",
						category: "info",
						text: markup`This successfully resolves as an implicit index file. Trying adding <emphasis>/index${nonStrictResolved.path.getExtensions()}</emphasis> to the end of the import source`,
					});
				} else if (nonStrictResolved.types.includes("implicitExtension")) {
					advice.push({
						type: "log",
						category: "info",
						text: markup`This successfully resolves as an implicit extension. Try adding the extension <emphasis>${nonStrictResolved.path.getExtensions()}</emphasis>`,
					});
				}
			}
		}

		// We may set this to `true` for stuff like forgetting a platform
		let skipSimilaritySuggestions = false;

		// Try other platforms
		const validPlatforms: StaticMarkup[] = [];
		for (const PLATFORM of PLATFORMS) {
			if (PLATFORM === query.platform) {
				continue;
			}

			const resolved = resolver.resolveLocal({
				...localQuery,
				platform: PLATFORM,
			});

			if (resolved.type === "FOUND") {
				validPlatforms.push(
					markup`<emphasis>${PLATFORM}</emphasis> at <emphasis>${resolved.ref.uid}</emphasis>`,
				);
			}
		}
		if (validPlatforms.length > 0) {
			if (query.platform === undefined) {
				advice.push({
					type: "log",
					category: "info",
					text: markup`No platform was specified but we found modules for the following platforms`,
				});
			} else {
				advice.push({
					type: "log",
					category: "info",
					text: markup`No module found for the platform <emphasis>${query.platform}</emphasis> but we found these others`,
				});
			}

			skipSimilaritySuggestions = true;

			advice.push({
				type: "list",
				list: validPlatforms,
			});
		}

		// Suggestions based on similarity to paths and packages on disk
		if (!skipSimilaritySuggestions) {
			const suggestions = getSuggestions(server, resolver, localQuery);
			if (suggestions.size > 0) {
				const originDirectory = resolver.getOriginDirectory(localQuery);

				// Some suggestions may not be absolute paths
				const humanToPath: ExtendedMap<string, FilePath> = new ExtendedMap(
					"humanToPath",
				);

				const humanSuggestions = Array.from(
					suggestions,
					([human, absolute]) => {
						if (human === absolute.join()) {
							let relativePath = originDirectory.relative(absolute);

							// If the user didn't use extensions, then neither should we
							if (!query.source.hasAnyExtensions()) {
								// TODO only do this if it's an implicit extension
								relativePath = relativePath.changeBasename(
									relativePath.getExtensionlessBasename(),
								);
							}

							if (relativePath.isRelative()) {
								if (query.source.isExplicitRelative()) {
									relativePath = relativePath.toExplicitRelative();
								}

								const human = relativePath.join();
								humanToPath.set(human, absolute);
								return human;
							}
						}

						humanToPath.set(human, absolute);
						return human;
					},
				);

				advice = [
					...advice,
					...buildSuggestionAdvice(
						query.source.join(),
						humanSuggestions,
						{
							formatItem: (relative) => {
								const absolute = humanToPath.assert(relative);
								return markup`<filelink target="${absolute.join()}">${relative}</filelink>`;
							},
						},
					),
				];
			}
		}

		// Hint if this was an entry resolve and the cwd wasn't a project
		if (
			query.entry === true &&
			server.projectManager.findLoadedProject(localQuery.origin) === undefined
		) {
			advice.push({
				type: "log",
				category: "warn",
				text: markup`You aren't in a Rome project`,
			});
		}
	}

	// Hint on any indirection
	if (query !== rootQuery) {
		if (rootQuery.location === undefined) {
			advice.push({
				type: "log",
				category: "info",
				text: markup`Found while resolving <emphasis>${rootQuery.source}</emphasis> from <emphasis>${rootQuery.origin}</emphasis>`,
			});
		} else {
			advice.push({
				type: "log",
				category: "info",
				text: markup`Found while resolving <emphasis>${rootQuery.source}</emphasis> from <emphasis>${rootQuery.location.path}</emphasis>`,
			});

			advice.push({
				type: "frame",
				location: rootQuery.location,
			});
		}
	}

	// TODO check if this would have been successful if not for exports access control

	if (resolved.advice !== undefined) {
		advice = advice.concat(resolved.advice);
	}

	throw createSingleDiagnosticsError({
		location,
		description: descriptions.RESOLVER.NOT_FOUND(
			resolved.type,
			query.origin,
			query.source,
			advice,
		),
	});
}

type Suggestions = Map<string, AbsoluteFilePath>;

function getPathSuggestions(
	server: Server,
	resolver: Resolver,
	query: ResolverLocalQuery,
): Suggestions {
	const suggestions: Suggestions = new Map();
	const {source} = query;
	if (!source.isRelative()) {
		return suggestions;
	}

	const originDirectory = resolver.getOriginDirectory(query);

	// Remove .. segments from beginning
	const sourceParts = [...source.getSegments()];
	while (sourceParts[0] === "..") {
		sourceParts.shift();
	}

	const seen: AbsoluteFilePathSet = new AbsoluteFilePathSet();

	// Try parent directories of the origin
	for (const path of originDirectory.getChain()) {
		tryPathSuggestions({
			server,
			resolver,
			suggestions,
			seen,
			path: path.append(...sourceParts),
		});
	}

	return suggestions;
}

const MIN_SIMILARITY = 0.8;

function tryPathSuggestions(
	{server, resolver, suggestions, seen, path}: {
		server: Server;
		resolver: Resolver;
		suggestions: Suggestions;
		path: AbsoluteFilePath;
		seen: AbsoluteFilePathSet;
	},
) {
	if (seen.has(path)) {
		return;
	} else {
		seen.add(path);
	}

	const {memoryFs} = server;
	if (memoryFs.exists(path)) {
		// Found an absolute match
		suggestions.set(path.join(), path);
		return;
	}

	const search: [AbsoluteFilePath, string[]][] = [];

	// Build up directories we need to search and the segments to append after
	let target = path;
	let segments: string[] = [];
	while (true) {
		if (target.hasParent()) {
			search.push([target, segments]);
			segments = [...segments, target.getBasename()];
			target = target.getParent();
		} else {
			break;
		}
	}

	// Traverse up directory chain from the root
	for (const [path, segments] of search.reverse()) {
		const parentPath = path.getParent();

		// If our parent exists, but we do not, then we are the first step of the problem
		if (!memoryFs.exists(path) && memoryFs.exists(parentPath)) {
			const basenames: string[] = [];
			const basenameToExtensions: Map<string, string[]> = new Map();
			for (const path of memoryFs.readdir(parentPath)) {
				const basename = path.getExtensionlessBasename();
				basenames.push(basename);

				let withExts = basenameToExtensions.get(basename);
				if (withExts === undefined) {
					withExts = [];
					basenameToExtensions.set(basename, withExts);
				}
				withExts.push(path.getBasename());
			}
			if (basenames.length === 0) {
				continue;
			}

			// Try to find similar files in this directory that match our basename
			const ratings = orderBySimilarity(
				path.getExtensionlessBasename(),
				basenames,
				{
					minRating: MIN_SIMILARITY,
				},
			);

			for (const rating of ratings) {
				const basenames = basenameToExtensions.get(rating.target)!;
				for (const basename of basenames) {
					tryPathSuggestions({
						server,
						resolver,
						suggestions,
						path: parentPath.append(basename, ...segments),
						seen,
					});
				}
			}
		}
	}
}

function getPackageSuggestions(
	server: Server,
	query: ResolverLocalQuery,
): Suggestions {
	const possibleGlobalPackages: ExtendedMap<string, AbsoluteFilePath> = new ExtendedMap(
		"possibleGlobalPackages",
	);

	const mainProject = server.projectManager.findLoadedProject(query.origin);
	if (mainProject !== undefined) {
		const projects = server.projectManager.getHierarchyFromProject(mainProject);

		for (const project of projects) {
			for (const [name, value] of project.packages) {
				possibleGlobalPackages.set(name, value.directory);
			}
		}
	}

	// TODO Add node_modules
	const matches: [string, AbsoluteFilePath][] = orderBySimilarity(
		query.source.join(),
		Array.from(possibleGlobalPackages.keys()),
		{minRating: MIN_SIMILARITY},
	).map((item) => {
		const name = item.target;
		const absolute = possibleGlobalPackages.assert(name);
		return [name, absolute];
	});
	return new Map(matches);
}

function getSuggestions(
	server: Server,
	resolver: Resolver,
	query: ResolverLocalQuery,
): Suggestions {
	if (query.entry === true) {
		return new Map([
			...getPathSuggestions(server, resolver, query),
			...getPackageSuggestions(server, query),
		]);
	} else if (query.source.isFilePath()) {
		return getPathSuggestions(server, resolver, query);
	} else {
		return getPackageSuggestions(server, query);
	}
}
