/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Server from "../Server";
import {
	ProjectConfig,
	ProjectConfigMeta,
	ProjectDefinition,
	ROME_CONFIG_FILENAMES,
	ROME_CONFIG_PACKAGE_JSON_FIELD,
	ROME_CONFIG_WARN_FILENAMES,
	assertHardMeta,
	createDefaultProjectConfig,
	createDefaultProjectConfigMeta,
	loadCompleteProjectConfig,
	serializeJSONProjectConfig,
} from "@romefrontend/project";
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
} from "@romefrontend/diagnostics";
import {
	ManifestDefinition,
	manifestNameToString,
} from "@romefrontend/codec-js-manifest";
import {
	AbsoluteFilePath,
	AbsoluteFilePathMap,
	AbsoluteFilePathSet,
	URLFilePath,
	UnknownFilePath,
	UnknownFilePathMap,
	createAbsoluteFilePath,
} from "@romefrontend/path";
import {FileReference, JSONFileReference} from "../../common/types/files";
import {
	GetFileHandlerResult,
	getFileHandlerFromPath,
} from "../../common/file-handlers/index";
import {IMPLICIT_JS_EXTENSIONS} from "../../common/file-handlers/javascript";
import {createDirectory, readFileText} from "@romefrontend/fs";
import {Consumer} from "@romefrontend/consume";
import {consumeJSON} from "@romefrontend/codec-json";
import {VCSClient, getVCSClient} from "@romefrontend/vcs";
import {FilePathLocker} from "@romefrontend/core/common/utils/lockers";
import {FileNotFound} from "@romefrontend/core/common/FileNotFound";

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
function cleanRelativeUidPath(relative: UnknownFilePath): undefined | string {
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

		this.projectIdCounter = 0;
		this.projectConfigDependenciesToIds = new AbsoluteFilePathMap();
		this.projectLoadingLocks = new FilePathLocker();
		this.projectDirectoryToProject = new AbsoluteFilePathMap();
		this.projects = new Map();

		// We maintain these maps so we can reverse any uids, and protect against collisions
		this.uidToFilename = new Map();
		this.filenameToUid = new AbsoluteFilePathMap();
		this.remoteToLocalPath = new UnknownFilePathMap();
		this.localPathToRemote = new AbsoluteFilePathMap();
	}

	server: Server;

	uidToFilename: Map<string, AbsoluteFilePath>;
	filenameToUid: AbsoluteFilePathMap<string>;

	remoteToLocalPath: UnknownFilePathMap<AbsoluteFilePath>;
	localPathToRemote: AbsoluteFilePathMap<URLFilePath>;

	// Lock to prevent race conditions that result in the same project being loaded multiple times at once
	projectLoadingLocks: FilePathLocker;

	projects: Map<number, ProjectDefinition>;
	projectDirectoryToProject: AbsoluteFilePathMap<ProjectDefinition>;
	projectConfigDependenciesToIds: AbsoluteFilePathMap<Set<number>>;
	projectIdCounter: number;

	async init() {
		this.injectVirtualModules();

		this.server.memoryFs.deletedFileEvent.subscribe((path) => {
			this.handleDeleted(path);
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
	async injectVirtualModules() {
		const projectDirectory = this.server.virtualModules.nullAbsolute;

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

	handleDeleted(path: AbsoluteFilePath) {
		const filename = path.join();

		this.projectConfigDependenciesToIds.delete(path);

		// Remove uids
		const uid = this.filenameToUid.get(path);
		this.filenameToUid.delete(path);
		if (uid !== undefined) {
			this.uidToFilename.delete(filename);
		}
	}

	getRemoteFromLocalPath(path: AbsoluteFilePath): undefined | URLFilePath {
		return this.localPathToRemote.get(path);
	}

	getFilePathFromUid(uid: string): undefined | AbsoluteFilePath {
		return this.uidToFilename.get(uid);
	}

	getFilePathFromUidOrAbsolute(
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

	normalizeFilenamesToFilePaths(
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

	setUid(path: AbsoluteFilePath, uid: string) {
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

	getUid(path: AbsoluteFilePath, allowMissing: boolean = false): string {
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
		if (!allowMissing) {
			this.setUid(path, uid);
		}
		return uid;
	}

	getFileReference(path: AbsoluteFilePath): FileReference {
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

	getURLFileReference(local: AbsoluteFilePath, url: URLFilePath): FileReference {
		if (!this.remoteToLocalPath.has(url)) {
			this.remoteToLocalPath.set(url, local);
			this.localPathToRemote.set(local, url);
		}

		return this.getFileReference(local);
	}

	getTransportFileReference(path: AbsoluteFilePath): JSONFileReference {
		const ref = this.getFileReference(path);
		return {
			...ref,
			relative: ref.relative.join(),
			real: ref.real.join(),
		};
	}

	async maybeEvictPossibleConfig(path: AbsoluteFilePath): Promise<boolean> {
		// TODO not sure if this case handles new manifests?
		// check if this filename is a rome config dependency
		const projectIds = this.projectConfigDependenciesToIds.get(path);
		if (projectIds === undefined) {
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
			const project = this.projects.get(evictProjectId);
			if (project === undefined) {
				throw new Error(
					`Expected project of id ${evictProjectId} since it was declared in projectConfigLocsToId`,
				);
			}

			// Add all parent projects
			let topProject = project;
			while (topProject.parent !== undefined) {
				topProject = topProject.parent;
			}
			for (const project of getAllProjects(topProject)) {
				projectsToEvict.add(project);
			}
		}

		for (const project of projectsToEvict) {
			await this.evictProject(project);
		}

		return true;
	}

	async evictProject(project: ProjectDefinition) {
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
						directory: project.directory.join(),
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

		// Evict all files that belong to this project and delete their project mapping
		const ownedFiles: Array<AbsoluteFilePath> = [];
		for (const path of this.server.memoryFs.glob(project.directory)) {
			this.handleDeleted(path);
			ownedFiles.push(path);
		}
		await Promise.all(
			ownedFiles.map((path) => this.server.fileAllocator.evict(path)),
		);

		// Tell the MemoryFileSystem to stop watching and clear it's maps
		this.server.memoryFs.unwatch(project.directory);
	}

	getProjects(): Array<ProjectDefinition> {
		return Array.from(this.projects.values());
	}

	addDependencyToProjectId(path: AbsoluteFilePath, projectId: number): void {
		const ids = this.projectConfigDependenciesToIds.get(path);

		if (ids === undefined) {
			this.projectConfigDependenciesToIds.set(path, new Set([projectId]));
		} else {
			ids.add(projectId);
		}
	}

	findProjectConfigConsumer(
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

	async getVCSClient(project: ProjectDefinition): Promise<VCSClient> {
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

	async maybeGetVCSClient(
		project: ProjectDefinition,
	): Promise<undefined | VCSClient> {
		return await getVCSClient(project.config.vcs.root);
	}

	addDiskProject(
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

	async declareProject(
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
				throw new Error(
					`Conflicting project name ${config.name}. ${projectDirectory.join()} and ${project.directory.join()}`,
				);
			}
		}

		// Declare the project
		const parentProject = this.findProjectExisting(projectDirectory.getParent());
		const project: ProjectDefinition = {
			config,
			meta,
			directory: projectDirectory,
			id: this.projectIdCounter++,
			packages: new Map(),
			manifests: new Map(),
			parent: parentProject,
			children: new Set(),
		};

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
		this.server.workerManager.onNewProject(project);
	}

	declareManifest(
		project: ProjectDefinition,
		isProjectPackage: boolean,
		def: ManifestDefinition,
		diagnostics: DiagnosticsProcessor,
	) {
		const name = manifestNameToString(def.manifest.name);

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

	async notifyWorkersOfProjects(
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
				config: serializeJSONProjectConfig(project.config),
				id: project.id,
				directory: project.directory.join(),
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

	async assertProject(
		path: AbsoluteFilePath,
		location?: DiagnosticLocation,
	): Promise<ProjectDefinition> {
		const project =
			this.findProjectExisting(path) || (await this.findProject(path));
		if (project) {
			return project;
		}

		if (location === undefined) {
			throw new Error(
				`Couldn't find a project. Checked ${ROME_CONFIG_FILENAMES.join(" or ")} for ${path.join()}`,
			);
		} else {
			throw createSingleDiagnosticError({
				location,
				description: descriptions.PROJECT_MANAGER.NOT_FOUND,
			});
		}
	}

	hasLoadedProjectDirectory(path: AbsoluteFilePath): boolean {
		return this.projectDirectoryToProject.has(path);
	}

	// Convenience method to get the project config and pass it to the file handler class
	getHandlerWithProject(path: AbsoluteFilePath): GetFileHandlerResult {
		const project = this.findProjectExisting(path);
		if (project === undefined) {
			return {ext: "", handler: undefined};
		} else {
			return getFileHandlerFromPath(path, project.config);
		}
	}

	getHierarchyFromFilename(path: AbsoluteFilePath): Array<ProjectDefinition> {
		const project = this.findProjectExisting(path);
		if (project === undefined) {
			return [];
		} else {
			return this.getHierarchyFromProject(project);
		}
	}

	getHierarchyFromProject(project: ProjectDefinition): Array<ProjectDefinition> {
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

	assertProjectExisting(path: AbsoluteFilePath): ProjectDefinition {
		const project = this.findProjectExisting(path);
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

	findProjectExisting(path: AbsoluteFilePath): undefined | ProjectDefinition {
		for (const dir of path.getChain()) {
			const project = this.projectDirectoryToProject.get(dir);
			if (project !== undefined) {
				return project;
			}
		}

		return undefined;
	}

	// Attempt to find a project on the real disk and seed it into the memory file system
	async findProject(
		cwd: AbsoluteFilePath,
	): Promise<undefined | ProjectDefinition> {
		// Check if we have an existing project
		const syncProject = this.findProjectExisting(cwd);
		if (syncProject !== undefined) {
			return syncProject;
		}

		const parentDirectories = cwd.getChain();

		// If not then let's access the file system and try to find one
		for (const dir of parentDirectories.slice().reverse()) {
			// Check for dedicated project configs
			for (const configFilename of ROME_CONFIG_FILENAMES) {
				// Check in root
				const configPath = dir.append(configFilename);

				const hasProject = await this.server.memoryFs.existsHard(configPath);
				if (hasProject) {
					await this.server.memoryFs.watch(dir);
					return this.assertProjectExisting(cwd);
				}
			}

			// Check for package.json
			const packagePath = dir.append("package.json");
			if (await this.server.memoryFs.existsHard(packagePath)) {
				const input = await readFileText(packagePath);
				const json = await consumeJSON({input, path: packagePath});
				if (json.has(ROME_CONFIG_PACKAGE_JSON_FIELD)) {
					await this.server.memoryFs.watch(dir);
					return this.assertProjectExisting(cwd);
				}
			}
		}

		// If we didn't find a project config then check for incorrect config filenames
		for (const dir of parentDirectories) {
			for (const basename of ROME_CONFIG_WARN_FILENAMES) {
				const path = dir.append(basename);

				if (await this.server.memoryFs.existsHard(path)) {
					this.warnIncorrectConfigFile(
						path,
						DiagnosticsProcessor.createImmediateThrower([
							{
								category: "project-manager",
								message: "Find project",
							},
						]),
					);
				}
			}
		}

		return undefined;
	}

	checkConfigFile(path: AbsoluteFilePath, diagnostics: DiagnosticsProcessor) {
		if (ROME_CONFIG_WARN_FILENAMES.includes(path.getBasename())) {
			this.warnIncorrectConfigFile(path, diagnostics);
		}
	}

	warnIncorrectConfigFile(
		path: AbsoluteFilePath,
		diagnostics: DiagnosticsProcessor,
	) {
		diagnostics.addDiagnostic({
			description: descriptions.PROJECT_MANAGER.INCORRECT_CONFIG_FILENAME(
				ROME_CONFIG_FILENAMES,
			),
			location: {
				filename: path.join(),
			},
		});
	}
}
