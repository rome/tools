/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Server from "../Server";
import {
	PROJECT_CONFIG_DIRECTORY,
	PROJECT_CONFIG_FILENAMES,
	PROJECT_CONFIG_PACKAGE_JSON_FIELD,
	PROJECT_CONFIG_SENSITIVE_DIRECTORIES,
	PROJECT_CONFIG_WARN_FILENAMES,
	ProjectConfig,
	ProjectConfigMeta,
	ProjectDefinition,
	assertHardMeta,
	createDefaultProjectConfig,
	createDefaultProjectConfigMeta,
	loadCompleteProjectConfig,
} from "@internal/project";
import {
	WorkerPartialManifests,
	WorkerProjects,
} from "../../common/bridges/WorkerBridge";
import {WorkerContainer} from "../WorkerManager";
import {
	DiagnosticLocation,
	DiagnosticsProcessor,
	createSingleDiagnosticError,
	descriptions,
} from "@internal/diagnostics";
import {
	ManifestDefinition,
	manifestNameToString,
} from "@internal/codec-js-manifest";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	AnyFilePath,
	URLPath,
	UnknownPathMap,
	createAbsoluteFilePath,
} from "@internal/path";
import {FileReference} from "../../common/types/files";
import {
	GetFileHandlerResult,
	getFileHandlerFromPath,
} from "../../common/file-handlers/index";
import {IMPLICIT_JS_EXTENSIONS} from "../../common/file-handlers/javascript";
import {createDirectory, readFileText} from "@internal/fs";
import {Consumer} from "@internal/consume";
import {consumeJSON} from "@internal/codec-json";
import {VCSClient, getVCSClient} from "@internal/vcs";
import {FilePathLocker} from "@internal/async/lockers";
import {FileNotFound} from "@internal/fs/FileNotFound";
import {markup} from "@internal/markup";
import {ReporterNamespace} from "@internal/cli-reporter";
import {ExtendedMap} from "@internal/collections";

function cleanUidParts(parts: Array<string>): string {
	let uid = "";

	let lastPart = "";
	for (const part of parts) {
		if (uid !== "") {
			uid += "/";
		}

		// Prune off any prefix shared with the last part
		let sharedPrefix = "";
		for (let i = 0; i < part.length && lastPart[i] === part[i]; i++) {
			sharedPrefix += part[i];
		}

		const partWithoutExtension = part.split(".")[0];
		if (sharedPrefix === partWithoutExtension) {
			uid += part;
		} else {
			uid += part.slice(sharedPrefix.length);
		}

		lastPart = part;
	}

	return uid;
}

// If a UID has a relative path that's just index.js, index.ts etc then omit it
function cleanRelativeUidPath(relative: AnyFilePath): undefined | string {
	return relative.join();

	const segments = relative.getSegments();

	// Quick deopt if there last segment is not index.
	if (!segments[segments.length - 1].startsWith("index.")) {
		return relative.join();
	}

	// Verify and pop off the last segment if it matches index.VALID_JS_EXTENSION
	const basename = relative.getBasename();
	for (const ext of IMPLICIT_JS_EXTENSIONS) {
		// Got a matching basename that we should omit
		if (basename === `index.${ext}`) {
			if (segments.length === 1) {
				// If there's a single segment then we don't want anything
				return undefined;
			} else {
				return relative.getParent().join();
			}
		}
	}

	// No matches, we hit the index. check above but not any of the valid extensions
	return relative.join();
}

export type ProjectConfigSource = {
	consumer: Consumer;
	value: undefined | Consumer;
};

export default class ProjectManager {
	constructor(server: Server) {
		this.server = server;
		this.logger = server.logger.namespace(markup`[ProjectManager]`);

		this.projectIdCounter = 0;
		this.projectConfigDependenciesToIds = new AbsoluteFilePathMap();
		this.projectLoadingLocks = new FilePathLocker();
		this.projectDirectoryToProject = new AbsoluteFilePathMap();
		this.projects = new ExtendedMap("projects");

		// We maintain these maps so we can reverse any uids, and protect against collisions
		this.uidToFilename = new Map();
		this.filenameToUid = new AbsoluteFilePathMap();
		this.remoteToLocalPath = new UnknownPathMap();
		this.localPathToRemote = new AbsoluteFilePathMap();
	}

	private server: Server;
	private logger: ReporterNamespace;

	private uidToFilename: Map<string, AbsoluteFilePath>;
	private filenameToUid: AbsoluteFilePathMap<string>;

	private remoteToLocalPath: UnknownPathMap<AbsoluteFilePath>;
	private localPathToRemote: AbsoluteFilePathMap<URLPath>;

	// Lock to prevent race conditions that result in the same project being loaded multiple times at once
	private projectLoadingLocks: FilePathLocker;

	private projects: ExtendedMap<number, ProjectDefinition>;
	private projectDirectoryToProject: AbsoluteFilePathMap<ProjectDefinition>;
	private projectConfigDependenciesToIds: AbsoluteFilePathMap<Set<number>>;
	private projectIdCounter: number;

	public async init() {
		await this.injectVirtualModules();

		this.server.memoryFs.deletedFileEvent.subscribe((paths) => {
			this.handleDeleted(paths);
		});

		const vendorProjectConfig: ProjectConfig = {
			...createDefaultProjectConfig(),
			name: "rome-internal-remote",
		};
		const defaultVendorPath = vendorProjectConfig.files.vendorPath;
		// TODO find a way to do th
		await createDirectory(defaultVendorPath);
		await this.declareProject({
			projectDirectory: defaultVendorPath,
			meta: createDefaultProjectConfigMeta(),
			config: vendorProjectConfig,
		});
		await this.server.memoryFs.watch(defaultVendorPath);
	}

	// Add a default project for virtual modules
	// This will automatically be sent to workers
	private async injectVirtualModules() {
		const projectDirectory = this.server.virtualModules.getMockDirectory();

		const projectConfig: ProjectConfig = {
			...createDefaultProjectConfig(),
			name: "rome-virtual-modules",
		};

		await this.declareProject({
			projectDirectory,
			meta: createDefaultProjectConfigMeta(),
			config: projectConfig,
		});
	}

	private handleDeleted(paths: Array<AbsoluteFilePath>) {
		for (const path of paths) {
			const filename = path.join();

			this.projectConfigDependenciesToIds.delete(path);

			// Remove uids
			const uid = this.filenameToUid.get(path);
			this.filenameToUid.delete(path);
			if (uid !== undefined) {
				this.uidToFilename.delete(filename);
			}
		}
	}

	public getRemoteFromLocalPath(path: AbsoluteFilePath): undefined | URLPath {
		return this.localPathToRemote.get(path);
	}

	public getFilePathFromUid(uid: string): undefined | AbsoluteFilePath {
		return this.uidToFilename.get(uid);
	}

	public getFilePathFromUidOrAbsolute(
		uid: undefined | string,
	): undefined | AbsoluteFilePath {
		if (uid === undefined) {
			return undefined;
		}

		const uidToPath = this.getFilePathFromUid(uid);
		if (uidToPath !== undefined) {
			return uidToPath;
		}

		const path = createAbsoluteFilePath(uid);
		if (path.isAbsolute()) {
			return path.assertAbsolute();
		}

		return undefined;
	}

	public normalizeFilenamesToFilePaths(
		filenames: Iterable<undefined | string>,
	): {
		absolutes: AbsoluteFilePathSet;
		others: Set<undefined | string>;
	} {
		const others: Set<undefined | string> = new Set();
		const absolutes = new AbsoluteFilePathSet();

		for (const filename of filenames) {
			if (filename === undefined) {
				others.add(undefined);
				continue;
			}

			const absolute = this.getFilePathFromUidOrAbsolute(filename);
			if (absolute === undefined) {
				// Relative path
				others.add(filename);
			} else {
				absolutes.add(absolute);
			}
		}

		return {absolutes, others};
	}

	private setUid(path: AbsoluteFilePath, uid: string) {
		const filename = path.join();

		// Verify we didn't already generate this uid for another file
		const collided = this.uidToFilename.get(uid);
		if (collided !== undefined && !collided.equal(path)) {
			throw new Error(
				`UID collision between ${filename} and ${collided}: ${uid}`,
			);
		}

		this.uidToFilename.set(uid, path);
		this.filenameToUid.set(path, uid);
	}

	public getUid(path: AbsoluteFilePath, allowMissing: boolean = false): string {
		// We maintain a map of file paths to UIDs
		// We clear the UID when a path is deleted.
		// If getUid is called on a file that doesn't exist then we'll populate it and it will exist forever.
		if (!this.server.memoryFs.exists(path) && !allowMissing) {
			throw new FileNotFound(path);
		}

		// Allow passing in a UID
		const filename = path.join();
		if (this.uidToFilename.has(filename)) {
			return filename;
		}

		// Check if we've already calculated and saved a UID
		const existing = this.filenameToUid.get(path);
		if (existing !== undefined) {
			return existing;
		}

		const project = this.assertProjectExisting(path);

		// Format of uids will be <PROJECT_NAME>/<PACKAGE_NAME>/<RELATIVE>
		const parts: Array<string> = [];

		let root = project.directory;

		// Push on parent package names
		let targetPackagePath = path;
		while (true) {
			const pkg = this.server.memoryFs.getOwnedManifest(targetPackagePath);
			if (pkg === undefined || pkg.directory.equal(project.directory)) {
				break;
			} else {
				const name = manifestNameToString(pkg.manifest.name);
				if (name !== undefined) {
					parts.unshift(name);

					if (targetPackagePath === path) {
						root = pkg.directory;
					}
				}
				targetPackagePath = pkg.directory.getParent();
			}
		}

		parts.unshift(project.config.name);

		const relative = cleanRelativeUidPath(root.relative(path));
		if (relative !== undefined) {
			parts.push(relative);
		}

		const uid = cleanUidParts(parts);
		if (this.server.memoryFs.exists(path) || !allowMissing) {
			this.setUid(path, uid);
		}
		return uid;
	}

	public getFileReference(path: AbsoluteFilePath): FileReference {
		const project = this.assertProjectExisting(path);
		const uid = this.getUid(path);
		const pkg = this.server.memoryFs.getOwnedManifest(path);
		return {
			uid,
			project: project.id,
			real: path,
			manifest: pkg === undefined ? undefined : pkg.id,
			relative: project.directory.relative(path).assertRelative(),
			remote: this.localPathToRemote.has(path),
		};
	}

	public getURLFileReference(
		local: AbsoluteFilePath,
		url: URLPath,
	): FileReference {
		if (!this.remoteToLocalPath.has(url)) {
			this.remoteToLocalPath.set(url, local);
			this.localPathToRemote.set(local, url);
		}

		return this.getFileReference(local);
	}

	public async maybeEvictProjects(
		paths: Array<AbsoluteFilePath>,
	): Promise<boolean> {
		// Check if this filename is a rome config dependency
		let projectIds: Set<number> = new Set();
		for (const path of paths) {
			const pathProjectIds = this.projectConfigDependenciesToIds.get(path);
			if (pathProjectIds !== undefined) {
				projectIds = new Set([...projectIds, ...pathProjectIds]);
			}
		}
		if (projectIds.size === 0) {
			return false;
		}

		const projectsToEvict: Set<ProjectDefinition> = new Set();

		function getAllProjects(project: ProjectDefinition) {
			let children: Array<ProjectDefinition> = [];
			for (const child of project.children) {
				children = children.concat(getAllProjects(child));
			}
			return [project, ...children];
		}

		for (const evictProjectId of projectIds) {
			// Fetch the project
			const project = this.projects.assert(evictProjectId);

			// Add all parent projects
			let topProject = project;
			while (topProject.parent !== undefined) {
				topProject = topProject.parent;
			}
			for (const project of getAllProjects(topProject)) {
				projectsToEvict.add(project);
			}
		}

		// Evict
		for (const project of projectsToEvict) {
			await this.evictProject(project, true);
		}

		return true;
	}

	public async evictProject(project: ProjectDefinition, reload: boolean) {
		await this.server.memoryFs.processingLock.wrap(async () => {
			const evictProjectId = project.id;

			// Remove the config locs from our internal map that belong to this project
			for (const [configLoc, projectIds] of this.projectConfigDependenciesToIds) {
				if (projectIds.has(evictProjectId)) {
					projectIds.delete(evictProjectId);
				}

				if (projectIds.size === 0) {
					this.projectConfigDependenciesToIds.delete(configLoc);
				}
			}

			// Notify all workers that it should delete the project
			for (const {bridge} of this.server.workerManager.getWorkers()) {
				// Evict project
				bridge.updateProjects.send({
					projects: [
						{
							id: evictProjectId,
							directory: project.directory,
							config: undefined,
						},
					],
				});

				// Evict packages
				bridge.updateManifests.send({
					manifests: Array.from(
						project.manifests.values(),
						(def) => ({
							id: def.id,
							manifest: undefined,
						}),
					),
				});
			}

			// Delete the project from 'our internal map
			this.projects.delete(evictProjectId);
			this.projectDirectoryToProject.delete(project.directory);

			// Tell the MemoryFileSystem to close the watcher so new file events are not emitted
			this.server.memoryFs.close(project.directory);

			// Evict all files that belong to this project and delete their project mapping
			const ownedFiles: Array<AbsoluteFilePath> = Array.from(
				this.server.memoryFs.glob(project.directory),
			);
			this.handleDeleted(ownedFiles);
			await Promise.all(
				ownedFiles.map((path) =>
					this.server.fileAllocator.evict(
						path,
						markup`project dependency change`,
					)
				),
			);

			// Tell the MemoryFileSystem to clear it's maps
			this.server.memoryFs.unwatch(project.directory);

			this.logger.info(
				markup`Evicted project <emphasis>${project.directory}</emphasis>`,
			);

			if (reload) {
				this.logger.info(
					markup`Reloading evicted project <emphasis>${project.directory}</emphasis>`,
				);
				await this.findProject(project.directory);
			}
		});
	}

	public getProjects(): Array<ProjectDefinition> {
		return Array.from(this.projects.values());
	}

	private addDependencyToProjectId(
		path: AbsoluteFilePath,
		projectId: number,
	): void {
		const ids = this.projectConfigDependenciesToIds.get(path);

		if (ids === undefined) {
			this.projectConfigDependenciesToIds.set(path, new Set([projectId]));
		} else {
			ids.add(projectId);
		}
	}

	public findProjectConfigConsumer(
		def: ProjectDefinition,
		test: (consumer: Consumer) => undefined | false | Consumer,
	): ProjectConfigSource {
		const meta = assertHardMeta(def.meta);

		for (const consumer of meta.consumersChain) {
			const value = test(consumer);
			if (value !== undefined && value !== false && value.exists()) {
				return {value, consumer: meta.consumer};
			}
		}

		return {value: undefined, consumer: meta.consumer};
	}

	public async getVCSClient(project: ProjectDefinition): Promise<VCSClient> {
		const client = await this.maybeGetVCSClient(project);

		if (client === undefined) {
			const {
				value: rootConfigConsumer,
				consumer,
			} = this.findProjectConfigConsumer(
				project,
				(consumer) => consumer.has("vsc") && consumer.get("vsc").get("root"),
			);

			const rootConfigLocation: undefined | DiagnosticLocation =
				rootConfigConsumer === undefined
					? undefined
					: rootConfigConsumer.getDiagnosticLocation();

			const location: DiagnosticLocation =
				rootConfigLocation === undefined
					? consumer.getDiagnosticLocation()
					: rootConfigLocation;

			throw createSingleDiagnosticError({
				description: descriptions.PROJECT_MANAGER.NO_VCS(rootConfigLocation),
				location,
			});
		} else {
			return client;
		}
	}

	public async maybeGetVCSClient(
		project: ProjectDefinition,
	): Promise<undefined | VCSClient> {
		return await getVCSClient(project.config.vcs.root);
	}

	public addDiskProject(
		opts: {
			projectDirectory: AbsoluteFilePath;
			configPath: AbsoluteFilePath;
		},
	): Promise<void> {
		const {projectDirectory, configPath} = opts;

		return this.projectLoadingLocks.wrapLock(
			projectDirectory,
			async () => {
				if (this.hasLoadedProjectDirectory(projectDirectory)) {
					// Already defined
					return;
				}

				const {config, meta} = await loadCompleteProjectConfig(
					projectDirectory,
					configPath,
				);

				await this.declareProject({
					projectDirectory: opts.projectDirectory,
					meta,
					config,
				});
			},
		);
	}

	private async declareProject(
		{
			projectDirectory,
			meta,
			config,
		}: {
			projectDirectory: AbsoluteFilePath;
			meta: ProjectConfigMeta;
			config: ProjectConfig;
		},
	): Promise<void> {
		// Make sure there's no project with the same `name` as us
		for (const project of this.getProjects()) {
			if (project.config.name === config.name) {
				// TODO
				throw new Error(
					`Conflicting project name ${config.name}. ${projectDirectory.join()} and ${project.directory.join()}`,
				);
			}
		}

		// Declare the project
		const parentProject = this.findLoadedProject(projectDirectory.getParent());
		const project: ProjectDefinition = {
			config,
			meta,
			directory: projectDirectory,
			id: this.projectIdCounter++,
			packages: new Map(),
			manifests: new Map(),
			parent: parentProject,
			children: new Set(),
			initialized: false,
		};

		this.logger.info(
			markup`Declared project <emphasis>#${project.id}</emphasis> from <emphasis>${projectDirectory}</emphasis>`,
		);

		this.projects.set(project.id, project);
		this.projectDirectoryToProject.set(projectDirectory, project);

		if (parentProject !== undefined) {
			parentProject.children.add(project);
		}

		// Add all project config dependencies so changes invalidate the whole project
		if (meta.configPath !== undefined) {
			this.addDependencyToProjectId(meta.configPath, project.id);
		}
		for (const loc of meta.configDependencies) {
			this.addDependencyToProjectId(loc, project.id);
		}

		// Notify other pieces of our creation
		await this.server.workerManager.onNewProject(project);
	}

	public declareManifest(
		project: ProjectDefinition,
		isProjectPackage: boolean,
		def: ManifestDefinition,
		diagnostics: DiagnosticsProcessor,
	) {
		const name = manifestNameToString(def.manifest.name);

		const type = isProjectPackage ? "project package manifest" : "manifest";
		this.logger.info(
			markup`Declaring ${type} <emphasis>${name}</emphasis> in project <emphasis>#${project.id}</emphasis> in <emphasis>${def.directory}</emphasis>`,
		);

		// Declare this package in all projects
		const projects = this.getHierarchyFromProject(project);

		// Check for collisions
		if (isProjectPackage && name !== undefined) {
			for (const project of projects) {
				// If there is no package then there's nothing to collide
				const existingPackage = project.packages.get(name);
				if (existingPackage === undefined) {
					continue;
				}

				diagnostics.addDiagnostic({
					description: descriptions.PROJECT_MANAGER.DUPLICATE_PACKAGE(
						name,
						existingPackage.path.join(),
					),
					location: def.consumer.get("name").getDiagnosticLocation(
						"inner-value",
					),
				});
				return;
			}
		}

		// Set as a package
		for (const project of projects) {
			this.addDependencyToProjectId(def.path, project.id);
			project.manifests.set(def.id, def);

			if (isProjectPackage && name !== undefined) {
				project.packages.set(name, def);
			}
		}
	}

	public async notifyWorkersOfProjects(
		workers: Array<WorkerContainer>,
		projects?: Array<ProjectDefinition>,
	): Promise<void> {
		if (projects === undefined) {
			projects = Array.from(this.projects.values());
		}

		const manifestsSerial: WorkerPartialManifests = [];
		const projectsSerial: WorkerProjects = [];
		for (const project of projects) {
			projectsSerial.push({
				config: project.config,
				id: project.id,
				directory: project.directory,
			});

			for (const def of project.manifests.values()) {
				manifestsSerial.push({
					id: def.id,
					manifest: this.server.memoryFs.getPartialManifest(def),
				});
			}
		}

		const promises = [];

		for (const worker of workers) {
			promises.push(
				worker.bridge.updateProjects.call({projects: projectsSerial}),
			);
			promises.push(
				worker.bridge.updateManifests.call({
					manifests: manifestsSerial,
				}),
			);
		}

		await Promise.all(promises);
	}

	public async assertProject(
		path: AbsoluteFilePath,
		location?: DiagnosticLocation,
	): Promise<ProjectDefinition> {
		const project =
			this.findLoadedProject(path) || (await this.findProject(path));
		if (project) {
			return project;
		}

		if (location === undefined) {
			throw new Error(
				`Couldn't find a project. Checked ${PROJECT_CONFIG_FILENAMES.join(
					" or ",
				)} for ${path.join()}`,
			);
		} else {
			throw createSingleDiagnosticError({
				location,
				description: descriptions.PROJECT_MANAGER.NOT_FOUND,
			});
		}
	}

	private hasLoadedProjectDirectory(path: AbsoluteFilePath): boolean {
		return this.projectDirectoryToProject.has(path);
	}

	// Convenience method to get the project config and pass it to the file handler class
	public getHandlerWithProject(path: AbsoluteFilePath): GetFileHandlerResult {
		const project = this.findLoadedProject(path);
		if (project === undefined) {
			return {ext: "", handler: undefined};
		} else {
			return getFileHandlerFromPath(path, project.config);
		}
	}

	public getHierarchyFromProject(
		project: ProjectDefinition,
	): Array<ProjectDefinition> {
		const projects: Array<ProjectDefinition> = [];

		let currProject: undefined | ProjectDefinition = project;
		while (currProject !== undefined) {
			projects.push(currProject);

			// root projects shouldn't be considered to have any parents
			if (currProject.config.root) {
				break;
			}

			currProject = project.parent;
		}

		return projects;
	}

	public assertProjectExisting(path: AbsoluteFilePath): ProjectDefinition {
		const project = this.findLoadedProject(path);
		if (project === undefined) {
			throw new Error(
				`Expected existing project for ${path.join()} only have ${Array.from(
					this.projectDirectoryToProject.keys(),
					(directory) => directory.join(),
				).join(", ")}`,
			);
		}
		return project;
	}

	public getProjectFromPath(
		path: AbsoluteFilePath,
	): undefined | ProjectDefinition {
		return this.projectDirectoryToProject.get(path);
	}

	public findLoadedProject(
		path: AbsoluteFilePath,
	): undefined | ProjectDefinition {
		for (const dir of path.getChain()) {
			const project = this.projectDirectoryToProject.get(dir);
			if (project !== undefined) {
				return project;
			}
		}

		return undefined;
	}

	// Attempt to find a project on the real disk and seed it into the memory file system
	public async findProject(
		cwd: AbsoluteFilePath,
	): Promise<undefined | ProjectDefinition> {
		await this.server.memoryFs.processingLock.wait();

		// Check if we have an existing project
		const syncProject = this.findLoadedProject(cwd);
		if (syncProject !== undefined) {
			return syncProject;
		}

		const processor = DiagnosticsProcessor.createImmediateThrower([
			{
				category: "project-manager",
				message: "Find project",
			},
		]);

		const parentDirectories = cwd.getChain();

		// If not then let's access the file system and try to find one
		for (const dir of parentDirectories.slice().reverse()) {
			// Check for dedicated project configs
			for (const configFilename of PROJECT_CONFIG_FILENAMES) {
				// Check in root
				const configPath = dir.append(PROJECT_CONFIG_DIRECTORY, configFilename);

				const hasProject = await this.server.memoryFs.existsHard(configPath);
				if (hasProject) {
					if (this.isLoadingBannedProjectPath(dir, configPath, processor)) {
						// Would have emitted a diagnostic
						return;
					}

					await this.server.memoryFs.watch(dir);
					return this.assertProjectExisting(cwd);
				}
			}

			// Check for package.json
			const packagePath = dir.append("package.json");
			if (await this.server.memoryFs.existsHard(packagePath)) {
				const input = await readFileText(packagePath);
				const json = await consumeJSON({input, path: packagePath});
				if (json.has(PROJECT_CONFIG_PACKAGE_JSON_FIELD)) {
					if (this.isLoadingBannedProjectPath(dir, packagePath, processor)) {
						// Would have emitted a diagnostic
						return;
					}

					await this.server.memoryFs.watch(dir);
					return this.assertProjectExisting(cwd);
				}
			}
		}

		// If we didn't find a project config then
		for (const dir of parentDirectories) {
			// Check for typo config filenames
			for (const basename of PROJECT_CONFIG_WARN_FILENAMES) {
				const path = dir.append(basename);

				if (await this.server.memoryFs.existsHard(path)) {
					this.checkPathForIncorrectConfig(path, processor);
				}
			}

			// Check for configs outside of a .config directory
			for (const configFilename of PROJECT_CONFIG_FILENAMES) {
				const path = dir.append(configFilename);

				if (await this.server.memoryFs.existsHard(path)) {
					this.checkPathForIncorrectConfig(path, processor);
				}
			}
		}

		this.logger.info(markup`Found no project for <emphasis>${cwd}</emphasis>`);

		return undefined;
	}

	// Refuse to load project path or root as valid project directories
	public isBannedProjectPath(projectFolder: AbsoluteFilePath): boolean {
		return (
			projectFolder.isRoot() ||
			PROJECT_CONFIG_SENSITIVE_DIRECTORIES.has(projectFolder)
		);
	}

	// Create a diagnostic if the project folder is sensitive
	private isLoadingBannedProjectPath(
		projectFolder: AbsoluteFilePath,
		configPath: AbsoluteFilePath,
		diagnostics: DiagnosticsProcessor,
	): boolean {
		if (this.isBannedProjectPath(projectFolder)) {
			diagnostics.addDiagnostic({
				description: descriptions.PROJECT_MANAGER.LOADING_SENSITIVE(
					projectFolder,
				),
				location: {
					filename: configPath.join(),
				},
			});
			return true;
		} else {
			return false;
		}
	}

	public checkPathForIncorrectConfig(
		path: AbsoluteFilePath,
		diagnostics: DiagnosticsProcessor,
	) {
		if (PROJECT_CONFIG_WARN_FILENAMES.includes(path.getBasename())) {
			diagnostics.addDiagnostic({
				description: descriptions.PROJECT_MANAGER.TYPO_CONFIG_FILENAME(
					path.getBasename(),
					PROJECT_CONFIG_FILENAMES,
				),
				location: {
					filename: path.join(),
				},
			});
		}

		if (
			PROJECT_CONFIG_FILENAMES.includes(path.getBasename()) &&
			path.getParent().getBasename() !== PROJECT_CONFIG_DIRECTORY
		) {
			diagnostics.addDiagnostic({
				description: descriptions.PROJECT_MANAGER.MISPLACED_CONFIG(
					path.getBasename(),
				),
				location: {
					filename: path.join(),
				},
			});
		}
	}
}
