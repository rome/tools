// These methods are defined here as they share a lot of logic for matching paths
// They however belong to their associated class and should use those methods and not be imported from here
import {
	AbsoluteFilePath,
	AbsoluteFilePathSet,
	UnknownFilePath,
	createUnknownFilePath,
} from "@internal/path";
import {
	DiagnosticAdvice,
	DiagnosticCategory,
	DiagnosticLocation,
	Diagnostics,
	DiagnosticsError,
	descriptions,
} from "@internal/diagnostics";
import {
	ProjectConfigCategoriesWithIgnore,
	ProjectDefinition,
} from "@internal/project";
import {ResolverQueryResponseFound} from "./Resolver";
import {Server, ServerRequest} from "@internal/core";
import {
	EventSubscription,
	SubscriptionWrapperHelpers,
	mergeEventSubscriptions,
	wrapSubscriptionConsumer,
} from "@internal/events";
import {markup} from "@internal/markup";
import {
	PathPatterns,
	matchPathPatterns,
	parsePathPattern,
} from "@internal/path-match";

type GetFilesTryAlternateArg = (
	path: UnknownFilePath,
) => undefined | UnknownFilePath;

export interface GlobOptions {
	extensions?: Array<string>;
	overrideIgnore?: PathPatterns;
	configCategory?: ProjectConfigCategoriesWithIgnore;
	test?: (path: AbsoluteFilePath) => boolean;
}

export interface GetFilesOptions extends Omit<GlobOptions, "getProjectIgnore"> {
	tryAlternateArg?: GetFilesTryAlternateArg;
	ignoreArgumentMisses?: boolean;
	ignoreProjectIgnore?: boolean;
	disabledDiagnosticCategory?: DiagnosticCategory;
	advice?: DiagnosticAdvice;
	verb?: string;
	noun?: string;
	args?: Array<string>;
	globber?: Globber;
}
// One off resolve
interface ResolveFilesOptions extends GetFilesOptions {
	onResolvedDirectories?: (path: AbsoluteFilePath) => void;
}

type ResolvedArg = {
	path: AbsoluteFilePath;
	location: DiagnosticLocation;
	project: ProjectDefinition;
};

type ResolvedArgs = Array<ResolvedArg>;

function relativeToResolvedArgs(
	args: Iterable<AbsoluteFilePath>,
	path: AbsoluteFilePath,
): boolean {
	for (const arg of args) {
		if (arg.equal(path) || path.isRelativeTo(arg)) {
			return true;
		}
	}
	return false;
}

async function resolveFilesFromArgs(
	req: ServerRequest,
	{
		args: overrideArgs,
		tryAlternateArg,
		onResolvedDirectories: onResolvedArgument,
	}: ResolveFilesOptions,
): Promise<ResolvedArgs> {
	req.checkCancelled();

	const rawArgs = overrideArgs ?? req.query.args;
	const resolvedArgs: ResolvedArgs = [];
	const {cwd} = req.client.flags;

	// If args was explicitly provided then don't assume empty args is the project root
	if (rawArgs.length === 0 && overrideArgs === undefined) {
		if (onResolvedArgument !== undefined) {
			onResolvedArgument(req.client.flags.cwd);
		}

		const location = req.getDiagnosticLocationForClientCwd();
		const project = await req.assertClientCwdProject();
		resolvedArgs.push({
			path: project.directory,
			location,
			project,
		});
	} else {
		for (let i = 0; i < rawArgs.length; i++) {
			const arg = rawArgs[i];

			const location = req.getDiagnosticLocationFromFlags({
				type: "arg",
				key: i,
			});

			let source = createUnknownFilePath(arg);
			let resolved: undefined | ResolverQueryResponseFound;

			if (tryAlternateArg !== undefined) {
				const alternateSource = tryAlternateArg(source);
				if (alternateSource !== undefined) {
					const resolvedAlternate = await req.server.resolver.resolveEntry({
						origin: cwd,
						source: alternateSource,
						// Alow requests to stop at directories
						requestedType: "directory",
					});
					if (resolvedAlternate.type === "FOUND") {
						resolved = resolvedAlternate;
					}
				}
			}

			if (resolved === undefined) {
				resolved = await req.server.resolver.resolveEntryAssert(
					{
						origin: cwd,
						source,
						requestedType: "directory",
					},
					{
						location,
					},
				);
			}

			// Project will exist as it would have been instantiated by the above resolveEntry calls
			const project = req.server.projectManager.assertProjectExisting(
				resolved.path,
			);
			resolvedArgs.push({
				project,
				path: resolved.path,
				location,
			});
		}
	}

	return resolvedArgs;
}

export type GetFilesFlushCallback = (
	paths: AbsoluteFilePathSet,
) => void | Promise<void>;

export const getFilesFromArgs = wrapSubscriptionConsumer(async function(
	helper: SubscriptionWrapperHelpers,
	req: ServerRequest,
	opts: GetFilesOptions = {},
	maybeFlushCallback?: GetFilesFlushCallback,
): Promise<AbsoluteFilePathSet> {
	const {server} = req;
	const globber = opts.globber ?? new Globber(opts, req.server);

	req.checkCancelled();

	let resolveOptions: ResolveFilesOptions = opts;

	// Files that have already been flushed and we want remove from the final result
	const flushed: AbsoluteFilePathSet = new AbsoluteFilePathSet();

	// If provided with a flush callback, watch for new files that would match and debounce them to the callback
	if (maybeFlushCallback !== undefined) {
		const flushCallback: GetFilesFlushCallback = maybeFlushCallback;
		const directories = new AbsoluteFilePathSet();

		resolveOptions = {
			onResolvedDirectories(path) {
				directories.add(path);
			},
		};

		let pendingFlush = new AbsoluteFilePathSet();
		let lastFlushed = Date.now();

		function queueFlush() {
			const now = Date.now();
			const timeSinceLastFlush = now - lastFlushed;

			// Flush every 100 files or 100ms
			if (timeSinceLastFlush < 100 && pendingFlush.size < 100) {
				return;
			} else {
				lastFlushed = now;
			}

			// Reset pending and register paths as flushed
			const paths = pendingFlush;
			pendingFlush = new AbsoluteFilePathSet();
			for (const path of paths) {
				flushed.add(path);
			}

			flushCallback(paths);
		}

		helper.add(
			server.memoryFs.newFileEvent.subscribe((path) => {
				if (relativeToResolvedArgs(directories, path)) {
					const matches = globber.glob(path);
					if (matches.size > 0) {
						// Queue flush
						for (const path of matches) {
							pendingFlush.add(path);
						}
						queueFlush();
					}
				}
			}),
		);
	}

	const resolvedArgs = await resolveFilesFromArgs(req, resolveOptions);

	// Resolved arguments that resulted in no files
	const noArgMatches: Set<ResolvedArg> = new Set();

	// Match files
	const paths: AbsoluteFilePathSet = new AbsoluteFilePathSet();
	for (const arg of resolvedArgs) {
		const matches = globber.glob(arg.path);

		if (matches.size === 0) {
			noArgMatches.add(arg);
		} else {
			for (const path of matches) {
				paths.add(path);
			}
		}
	}

	if (noArgMatches.size > 0 && !opts.ignoreArgumentMisses) {
		await globUnmatched(req, opts, {noArgMatches, foundPaths: paths});
	}

	// Remove flushed paths so we don't double emit them
	for (const path of flushed) {
		flushed.delete(path);
	}

	return paths;
});

async function globUnmatched(
	req: ServerRequest,
	opts: GetFilesOptions,
	{foundPaths, noArgMatches}: {
		foundPaths: AbsoluteFilePathSet;
		noArgMatches: Set<ResolvedArg>;
	},
) {
	const {server} = req;
	const {configCategory, ignoreProjectIgnore} = opts;
	const diagnostics: Diagnostics = [];

	for (const {path, project, location} of noArgMatches) {
		let category: DiagnosticCategory = "args/fileNotFound";

		let advice: DiagnosticAdvice = [...(opts.advice || [])];

		// Hint if all files were ignored
		if (configCategory !== undefined && !ignoreProjectIgnore) {
			const withoutIgnore = await getFilesFromArgs(
				req,
				{
					...opts,
					ignoreProjectIgnore: true,
				},
			);

			// Remove paths that we already successfully found
			for (const path of foundPaths) {
				withoutIgnore.delete(path);
			}

			if (withoutIgnore.size > 0) {
				advice.push({
					type: "log",
					category: "info",
					text: markup`The following files were ignored`,
				});

				advice.push({
					type: "list",
					list: Array.from(withoutIgnore, (path) => markup`${path}`),
					truncate: true,
				});

				const ignoreSource = server.projectManager.findProjectConfigConsumer(
					project,
					(consumer) =>
						consumer.has(configCategory) &&
						consumer.get(configCategory).get("ignore")
					,
				);

				if (ignoreSource.value !== undefined) {
					const ignorePointer = ignoreSource.value.getDiagnosticLocation(
						"value",
					);

					advice.push({
						type: "log",
						category: "info",
						text: markup`Ignore patterns were defined here`,
					});

					advice.push({
						type: "frame",
						location: ignorePointer,
					});
				}
			}
		}

		diagnostics.push({
			location: {
				...location,
				marker: markup`${path}`,
			},
			description: {
				...descriptions.FLAGS.NO_FILES_FOUND(opts.noun),
				category,
				advice,
			},
		});
	}

	throw new DiagnosticsError(
		"ServerRequest.getFilesFromArgs: Some arguments did not resolve to any files",
		diagnostics,
	);
}

// Watching

export type WatchFilesEvent = {
	paths: AbsoluteFilePathSet;
	initial: boolean;
	chunk: boolean;
};

export interface WatchFilesOptions extends GetFilesOptions {
	parallel?: boolean;
}

export type WatchFilesCallback = (opts: WatchFilesEvent) => Promise<void>;

export async function watchFilesFromArgs(
	req: ServerRequest,
	opts: WatchFilesOptions,
	callback: WatchFilesCallback,
): Promise<EventSubscription> {
	req.checkCancelled();

	// Everything needs to be relative to this
	const globber = new Globber(opts, req.server);

	let timeout: undefined | NodeJS.Timeout;
	let pendingPaths = new AbsoluteFilePathSet();

	const resolvedArgPaths = new AbsoluteFilePathSet();
	let initial = true;
	let chunk = true;

	let runningCallback = false;
	let flushing: undefined | Promise<void>;

	async function flush() {
		await flushing;

		timeout = undefined;

		if (!initial && pendingPaths.size === 0) {
			return;
		}

		flushing = _flush();
		await flushing;
		flushing = undefined;
	}

	async function _flush() {
		const paths = pendingPaths;
		pendingPaths = new AbsoluteFilePathSet();

		runningCallback = true;
		await callback({paths, initial, chunk});
		runningCallback = false;

		// Flush again if there were paths emitting while running
		if (pendingPaths.size > 0) {
			await flush();
		}
	}

	function debounce(paths: AbsoluteFilePathSet) {
		for (const path of paths) {
			pendingPaths.add(path);
		}

		if (!runningCallback && timeout === undefined) {
			timeout = setTimeout(() => flush(), 100);
		}
	}

	function maybeDebounce(path: AbsoluteFilePath) {
		if (relativeToResolvedArgs(resolvedArgPaths, path)) {
			debounce(globber.glob(path));
		}
	}

	const pendingRefreshPaths = new AbsoluteFilePathSet();

	const refreshFileEvent = req.server.refreshFileEvent.subscribe((
		path: AbsoluteFilePath,
	) => {
		if (initial) {
			pendingRefreshPaths.add(path);
		} else {
			maybeDebounce(path);
		}
	});

	const endSubscription = req.endEvent.subscribe(() => {
		sub.unsubscribe();
	});

	const sub = mergeEventSubscriptions([
		refreshFileEvent,
		{
			async unsubscribe() {
				if (timeout !== undefined) {
					clearTimeout(timeout);

					// Finish flushing if necessary
					await flushing;
				}
			},
		},
	]);

	// Get initial paths
	const initialPaths = await getFilesFromArgs(
		req,
		{
			...opts,
			globber,
		},
		//debounce,
	);
	for (const path of initialPaths) {
		pendingPaths.add(path);
	}
	chunk = false;
	await flush();
	initial = false;

	// Resolve initial arguments so we can check watched files
	const resolvedArgs = await resolveFilesFromArgs(req, opts);
	for (const {path} of resolvedArgs) {
		resolvedArgPaths.add(path);
	}

	// Flush any files that were changed while we were waiting on the initial paths
	if (pendingRefreshPaths.size > 0) {
		for (const path of pendingRefreshPaths) {
			maybeDebounce(path);
		}
		await flush();
	}

	return mergeEventSubscriptions([endSubscription, sub]);
}

// Memory file system glob

const GLOB_IGNORE: PathPatterns = [
	parsePathPattern({input: "node_modules"}),
	parsePathPattern({input: ".git"}),
	parsePathPattern({input: ".hg"}),
];

function concatGlobIgnore(patterns: PathPatterns): PathPatterns {
	// If there are any negate patterns then it'll never include GLOB_IGNORE
	for (const pattern of patterns) {
		if (pattern.type === "PathPattern" && pattern.negate) {
			return patterns;
		}
	}

	return [...GLOB_IGNORE, ...patterns];
}

export class Globber {
	constructor(opts: GlobOptions, server: Server) {
		this.opts = opts;
		this.server = server;
		this.ignoresByProject = new Map();
	}

	ignoresByProject: Map<ProjectDefinition, PathPatterns>;
	server: Server;
	opts: GlobOptions;

	getIgnore(path: AbsoluteFilePath): PathPatterns {
		const {configCategory, overrideIgnore} = this.opts;
		const project = this.server.projectManager.findLoadedProject(path);

		let ignore: PathPatterns = overrideIgnore ?? [];
		if (configCategory === undefined || project === undefined) {
			return ignore;
		}

		const projectIgnore = this.ignoresByProject.get(project);
		if (projectIgnore === undefined) {
			ignore = concatGlobIgnore([
				...ignore,
				...project.config[configCategory].ignore,
			]);
			this.ignoresByProject.set(project, ignore);
			return ignore;
		} else {
			return projectIgnore;
		}
	}

	glob(cwd: AbsoluteFilePath): AbsoluteFilePathSet {
		const {extensions, test} = this.opts;
		const {server} = this;
		const {memoryFs} = server;

		const matches: AbsoluteFilePathSet = new AbsoluteFilePathSet();

		let queue: Array<AbsoluteFilePath> = [cwd];
		while (queue.length > 0) {
			const path = queue.pop()!;

			const ignore = this.getIgnore(path);
			const ignoreMatched = matchPathPatterns(path, ignore, cwd);

			// Don't even recurse into explicit matches
			if (ignoreMatched === "EXPLICIT_MATCH") {
				continue;
			}

			// Add if a matching file
			if (memoryFs.files.has(path) && ignoreMatched === "NO_MATCH") {
				if (test !== undefined && !test(path)) {
					continue;
				}

				// Check extensions
				if (extensions !== undefined) {
					let matchedExt = false;
					for (const ext of extensions) {
						matchedExt = path.hasEndExtension(ext);
						if (matchedExt) {
							break;
						}
					}
					if (!matchedExt) {
						continue;
					}
				}

				matches.add(path);
				continue;
			}

			// Crawl if we're a directory
			// NOTE: We still continue crawling on implicit matches
			const listing = memoryFs.directoryListings.get(path);
			if (listing !== undefined) {
				queue = queue.concat(Array.from(listing.values()));
			}

			// TODO maybe throw? not a file or directory, doesn't exist!
		}

		return matches;
	}
}
