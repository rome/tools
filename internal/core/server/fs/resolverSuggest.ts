/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Resolver, {
	ResolverLocalQuery,
	ResolverQueryResponseNotFound,
	ResolverQuerySource,
	ResolverRemoteQuery,
	isPathLike,
} from "./Resolver";
import {
	DiagnosticAdvice,
	buildSuggestionAdvice,
	createSingleDiagnosticError,
	descriptions,
} from "@internal/diagnostics";
import {orderBySimilarity} from "@internal/string-utils";
import {AbsoluteFilePath, createUnknownPath} from "@internal/path";
import {PLATFORMS, Server} from "@internal/core";
import {StaticMarkups, markup} from "@internal/markup";
import {ExtendedMap} from "@internal/collections";

export default function resolverSuggest(
	{resolver, query, resolved, origQuerySource, server}: {
		resolver: Resolver;
		server: Server;
		query: ResolverRemoteQuery;
		resolved: ResolverQueryResponseNotFound;
		origQuerySource?: ResolverQuerySource;
	},
): Error {
	let errMsg = "";
	if (resolved.type === "UNSUPPORTED") {
		errMsg = "Unsupported path format";
	} else if (resolved.type === "MISSING") {
		errMsg = "Cannot find";
	} else if (resolved.type === "FETCH_ERROR") {
		errMsg = "Failed to fetch";
	}

	errMsg += ` "${query.source.join()}" from "${query.origin.join()}"`;

	// Use the querySource returned by the resolution which will be the one that actually triggered this error, otherwise use the query source provided to us
	const querySource =
		resolved.source === undefined ? origQuerySource : resolved.source;
	if (querySource === undefined || querySource.location === undefined) {
		// TODO do something about the `advice` on some `resolved` that may contain metadata?
		throw new Error(errMsg);
	}

	const {location} = querySource;

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
		const validPlatforms: StaticMarkups = [];
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
					markup`<emphasis>${PLATFORM}</emphasis> at <filelink emphasis target="${resolved.ref.uid}" />`,
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

		// Hint on any indirection
		if (
			origQuerySource !== undefined &&
			origQuerySource.location !== undefined &&
			resolved.source !== undefined
		) {
			advice.push({
				type: "log",
				category: "info",
				text: markup`Found while resolving <emphasis>${query.source}</emphasis> from <filelink emphasis target="${query.origin}" />`,
			});

			const origPointer = origQuerySource.location;

			advice.push({
				type: "frame",
				location: origPointer,
			});
		}

		// Suggestions based on similarity to paths and packages on disk
		if (!skipSimilaritySuggestions) {
			const suggestions = getSuggestions(server, resolver, localQuery);
			if (suggestions.size > 0) {
				const originDirectory = resolver.getOriginDirectory(localQuery);

				// Relative paths to absolute
				const relativeToAbsolute: ExtendedMap<string, string> = new ExtendedMap(
					"relativeToAbsolute",
				);

				const relativeSuggestions = Array.from(
					suggestions,
					([human, absolute]) => {
						if (human !== absolute) {
							relativeToAbsolute.set(human, absolute);
							return human;
						}

						let relativePath = originDirectory.relative(absolute);

						// If the user didn't use extensions, then neither should we
						if (!query.source.hasExtensions()) {
							// TODO only do this if it's an implicit extension
							relativePath = relativePath.changeBasename(
								relativePath.getExtensionlessBasename(),
							);
						}

						const relativeStr = relativePath.toExplicitRelative().join();
						relativeToAbsolute.set(relativeStr, absolute);
						return relativeStr;
					},
				);

				advice = [
					...advice,
					...buildSuggestionAdvice(
						query.source.join(),
						relativeSuggestions,
						{
							formatItem: (relative) => {
								const absolute = relativeToAbsolute.assert(relative);
								return markup`<filelink target="${absolute}">${relative}</filelink>`;
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

	// TODO check if this would have been successful if not for exports access control
	const source =
		querySource.source === undefined ? query.source.join() : querySource.source;

	if (resolved.advice !== undefined) {
		advice = advice.concat(resolved.advice);
	}

	throw createSingleDiagnosticError({
		location,
		description: {
			...descriptions.RESOLVER.NOT_FOUND(resolved.type, source, location),
			advice,
		},
	});
}

type Suggestions = Map<string, string>;

function getPathSuggestions(
	server: Server,
	resolver: Resolver,
	query: ResolverLocalQuery,
): Suggestions {
	const {source} = query;
	const originDirectory = resolver.getOriginDirectory(query);
	const suggestions: Suggestions = new Map();

	// Try normal resolved
	tryPathSuggestions({
		server,
		resolver,
		suggestions,
		path: originDirectory.resolve(source),
	});

	// Remove . and .. entries from beginning
	const sourceParts = [...source.getSegments()];
	while (sourceParts[0] === "." || sourceParts[0] === "..") {
		sourceParts.shift();
	}

	// Try parent directories of the origin

	for (const path of originDirectory.getChain()) {
		tryPathSuggestions({
			server,
			resolver,
			suggestions,
			path: path.append(...sourceParts),
		});
	}

	return suggestions;
}

const MIN_SIMILARITY = 0.8;

function tryPathSuggestions(
	{server, resolver, suggestions, path}: {
		server: Server;
		resolver: Resolver;
		suggestions: Suggestions;
		path: AbsoluteFilePath;
	},
) {
	const {memoryFs} = server;

	const segments = path.getSegments();
	const chain = path.getChain();

	// Get all segments that are unknown
	for (let i = chain.length - 1; i >= 0; i--) {
		const path = chain[i];

		if (memoryFs.exists(path)) {
			// If this is an absolute match then we should be a suggestion
			if (i === chain.length) {
				const filename = path.join();
				suggestions.set(filename, filename);
			}

			// Otherwise this segment exists and should have been dealt with previously in the loop
			break;
		}

		const parentPath = path.getParent();

		// Our basename isn't valid, but our parent exists
		if (!memoryFs.exists(path) && memoryFs.exists(parentPath)) {
			const entries = Array.from(
				memoryFs.readdir(parentPath),
				(path) => path.join(),
			);
			if (entries.length === 0) {
				continue;
			}

			const ratings = orderBySimilarity(
				path.getExtensionlessBasename(),
				entries.map((target) => {
					return createUnknownPath(target).getExtensionlessBasename();
				}),
				{
					minRating: MIN_SIMILARITY,
				},
			);

			for (const rating of ratings) {
				tryPathSuggestions({
					server,
					resolver,
					suggestions,
					path: createUnknownPath(rating.target).append(...segments.slice(1)).assertAbsolute(),
				});
			}
		}
	}
}

function getPackageSuggestions(
	server: Server,
	query: ResolverLocalQuery,
): Suggestions {
	const possibleGlobalPackages: ExtendedMap<string, string> = new ExtendedMap(
		"possibleGlobalPackages",
	);

	const mainProject = server.projectManager.findLoadedProject(query.origin);
	if (mainProject !== undefined) {
		const projects = server.projectManager.getHierarchyFromProject(mainProject);

		for (const project of projects) {
			for (const [name, value] of project.packages) {
				possibleGlobalPackages.set(name, value.directory.join());
			}
		}
	}

	// TODO Add node_modules
	const matches: Array<[string, string]> = orderBySimilarity(
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
	} else if (isPathLike(query.source)) {
		return getPathSuggestions(server, resolver, query);
	} else {
		return getPackageSuggestions(server, query);
	}
}
