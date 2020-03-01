/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Master from '../Master';
import {
  ProjectDefinition,
  ProjectConfig,
  DEFAULT_PROJECT_CONFIG,
  ProjectConfigMeta,
  DEFAULT_PROJECT_CONFIG_META,
  loadCompleteProjectConfig,
  serializeJSONProjectConfig,
  assertHardMeta,
  ROME_CONFIG_WARN_FILENAMES,
  ROME_CONFIG_FILENAMES,
  ROME_CONFIG_PACKAGE_JSON_FIELD,
  ROME_CONFIG_FOLDER,
} from '@romejs/project';
import {
  WorkerProjects,
  WorkerPartialManifests,
} from '../../common/bridges/WorkerBridge';
import {WorkerContainer} from '../WorkerManager';
import {
  DiagnosticsProcessor,
  DiagnosticPointer,
  DiagnosticsError,
} from '@romejs/diagnostics';
import {matchPathPatterns} from '@romejs/path-match';
import {ManifestDefinition} from '@romejs/codec-js-manifest';
import {
  AbsoluteFilePath,
  UnknownFilePath,
  URLFilePath,
  AbsoluteFilePathMap,
  UnknownFilePathMap,
} from '@romejs/path';
import {JSONFileReference, FileReference} from '../../common/types/files';
import {
  IMPLICIT_JS_EXTENSIONS,
  GetFileHandlerResult,
  getFileHandler,
} from '../../common/fileHandlers';
import {createDirectory, readFileText} from '@romejs/fs';
import {Consumer} from '@romejs/consume';
import {consumeJSON} from '@romejs/codec-json';

function cleanUidParts(parts: Array<string>): string {
  let uid = '';

  let lastPart = '';
  for (const part of parts) {
    if (uid !== '') {
      uid += '/';
    }

    // Prune off any prefix shared with the last part
    let sharedPrefix = '';
    for (let i = 0; i < part.length && lastPart[i] === part[i]; i++) {
      sharedPrefix += part[i];
    }

    uid += part.slice(sharedPrefix.length);

    lastPart = part;
  }

  return uid;
}

// If a UID has a relative path that's just index.js, index.ts etc then omit it
function cleanRelativeUidPath(relative: UnknownFilePath): undefined | string {
  return relative.join();

  const segments = relative.getSegments();

  // Quick deopt if there last segment is not index.
  if (!segments[segments.length - 1].startsWith('index.')) {
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

export default class ProjectManager {
  constructor(master: Master) {
    this.master = master;

    this.isAddingProject = false;
    this.pendingAddProjects = [];

    this.projectIdCounter = 0;
    this.projectFolderToId = new AbsoluteFilePathMap();
    this.projectConfigDependenciesToIds = new AbsoluteFilePathMap();
    this.fileToProject = new AbsoluteFilePathMap();
    this.projects = new Map();

    // We maintain these maps so we can reverse any uids, and protect against collisions
    this.uidToFilename = new Map();
    this.filenameToUid = new AbsoluteFilePathMap();
    this.remoteToLocalPath = new UnknownFilePathMap();
    this.localPathToRemote = new AbsoluteFilePathMap();
  }

  master: Master;

  isAddingProject: boolean;
  pendingAddProjects: Array<{
    projectFolder: AbsoluteFilePath;
    configPath: AbsoluteFilePath;
    resolve: (project: ProjectDefinition) => void;
  }>;

  uidToFilename: Map<string, AbsoluteFilePath>;
  filenameToUid: AbsoluteFilePathMap<string>;

  remoteToLocalPath: UnknownFilePathMap<AbsoluteFilePath>;
  localPathToRemote: AbsoluteFilePathMap<URLFilePath>;

  projects: Map<number, ProjectDefinition>;
  projectConfigDependenciesToIds: AbsoluteFilePathMap<Set<number>>;
  projectIdCounter: number;
  fileToProject: AbsoluteFilePathMap<{
    projectId: number;
    path: AbsoluteFilePath;
  }>;

  projectFolderToId: AbsoluteFilePathMap<number>;

  async init() {
    this.master.memoryFs.deletedFileEvent.subscribe(path => {
      this.handleDeleted(path);
    });

    const vendorProjectConfig: ProjectConfig = {
      ...DEFAULT_PROJECT_CONFIG,
      name: 'rome-internal-remote',
    };
    const defaultVendorPath = vendorProjectConfig.files.vendorPath;
    await createDirectory(defaultVendorPath, {recursive: true});
    await this.addProjectWithConfig({
      projectFolder: defaultVendorPath,
      meta: DEFAULT_PROJECT_CONFIG_META,
      config: vendorProjectConfig,
    });
    await this.master.memoryFs.watch(defaultVendorPath, vendorProjectConfig);
  }

  handleDeleted(path: AbsoluteFilePath) {
    const filename = path.join();

    this.projectConfigDependenciesToIds.delete(path);
    this.fileToProject.delete(path);

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

  getUid(path: AbsoluteFilePath): string {
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

    // For haste projects, use the haste name as the uid. If the user has multiple projects
    // with colliding uids then that's fine, it will just cause more cache misses as we compare
    // mtime, project config hash etc stored in the cache.
    if (this.isHasteDeclared(path, project)) {
      const hasteName = this.master.memoryFs.getHasteName(path);
      if (hasteName === undefined) {
        throw new Error(
          'isHasteDeclared returned true so this should always return a valid name',
        );
      }
      this.setUid(path, hasteName);
      return hasteName;
    }

    // Format of uids will be <PROJECT_NAME>/<PACKAGE_NAME>/<RELATIVE>
    const parts: Array<string> = [];

    let root = project.folder;

    // Push on parent package names
    let targetPackagePath = path;
    while (true) {
      const pkg = this.master.memoryFs.getOwnedManifest(targetPackagePath);
      if (pkg === undefined || pkg.folder.equal(project.folder)) {
        break;
      } else {
        const name = pkg.manifest.name;
        if (name !== undefined) {
          parts.unshift(name);

          if (targetPackagePath === path) {
            root = pkg.folder;
          }
        }
        targetPackagePath = pkg.folder.getParent();
      }
    }

    parts.unshift(project.config.name);

    const relative = cleanRelativeUidPath(root.relative(path));
    if (relative !== undefined) {
      parts.push(relative);
    }

    const uid = cleanUidParts(parts);
    this.setUid(path, uid);
    return uid;
  }

  getFileReference(path: AbsoluteFilePath): FileReference {
    const project = this.assertProjectExisting(path);
    const uid = this.getUid(path);
    const pkg = this.master.memoryFs.getOwnedManifest(path);
    return {
      uid,
      project: project.id,
      real: path,
      manifest: pkg === undefined ? undefined : pkg.id,
      remote: this.localPathToRemote.has(path),
    };
  }

  getURLFileReference(
    local: AbsoluteFilePath,
    url: URLFilePath,
  ): FileReference {
    if (!this.remoteToLocalPath.has(url)) {
      this.remoteToLocalPath.set(url, local);
      this.localPathToRemote.set(local, url);
    }

    return this.getFileReference(local);
  }

  getTransportFileReference(path: AbsoluteFilePath): JSONFileReference {
    return {
      ...this.getFileReference(path),
      real: path.join(),
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

    // Remove the config locs from 'our internal map that belong to this project
    for (const [configLoc, projectIds] of this.projectConfigDependenciesToIds) {
      if (projectIds.has(evictProjectId)) {
        projectIds.delete(evictProjectId);
      }

      if (projectIds.size === 0) {
        this.projectConfigDependenciesToIds.delete(configLoc);
      }
    }

    // Notify all workers that it should delete the project
    for (const {bridge} of this.master.workerManager.getWorkers()) {
      // Evict project
      bridge.updateProjects.send({
        projects: [
          {
            id: evictProjectId,
            folder: project.folder.join(),
            config: undefined,
          },
        ],
      });

      // Evict packages
      bridge.updateManifests.send({
        manifests: Array.from(project.manifests.values(), def => ({
          id: def.id,
          manifest: undefined,
        })),
      });
    }

    // Delete the project from 'our internal map
    this.projects.delete(evictProjectId);
    this.projectFolderToId.delete(project.folder);

    // Evict all files that belong to this project and delete their project mapping
    const ownedFiles: Array<AbsoluteFilePath> = [];
    for (const {projectId, path} of this.fileToProject.values()) {
      if (evictProjectId === projectId) {
        this.handleDeleted(path);
        ownedFiles.push(path);
      }
    }
    await Promise.all(
      ownedFiles.map(path => this.master.fileAllocator.evict(path)),
    );

    // Tell the MemoryFileSystem to stop watching and clear it's maps
    this.master.memoryFs.unwatch(project.folder);
  }

  getProjects(): Array<ProjectDefinition> {
    return Array.from(this.projects.values());
  }

  async queueAddProject(
    projectFolder: AbsoluteFilePath,
    configPath: AbsoluteFilePath,
  ): Promise<ProjectDefinition> {
    // Check if we've already loaded this project
    const maybeProject = this.findProjectExisting(projectFolder);
    if (maybeProject !== undefined) {
      return maybeProject;
    }

    // If we're currently adding a project then add it to the queue
    if (this.isAddingProject) {
      return new Promise(resolve => {
        this.pendingAddProjects.push({projectFolder, configPath, resolve});
      });
    }

    // First time loading this project
    this.isAddingProject = true;

    // fetch this project
    const mainProject = await this.addProject(projectFolder, configPath);
    const resolvedProjectsByDir: Map<string, ProjectDefinition> = new Map();
    resolvedProjectsByDir.set(projectFolder.join(), mainProject);

    // Resolve all pending projects that were added while we were adding the current project
    const resolvedProjects: Array<{
      project: ProjectDefinition;
      resolve: (project: ProjectDefinition) => void;
    }> = [];
    for (const {projectFolder, configPath, resolve} of this
      .pendingAddProjects) {
      // Check if the project has already been resolved
      const existing = resolvedProjectsByDir.get(projectFolder.join());
      if (existing !== undefined) {
        resolvedProjects.push({project: existing, resolve});
      } else {
        // It hasn't been resolved yet so let's add it
        const project = await this.addProject(projectFolder, configPath);
        resolvedProjects.push({project, resolve});
      }
    }

    // Resolve all promises
    for (const {project, resolve} of resolvedProjects) {
      resolve(project);
    }

    // Cleanup
    this.pendingAddProjects = [];
    this.isAddingProject = false;

    return mainProject;
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
    test: (consumer: Consumer) => undefined | Consumer,
  ): {
    consumer: Consumer;
    value: undefined | Consumer;
  } {
    const meta = assertHardMeta(def.meta);

    for (const consumer of meta.consumersChain) {
      const value = test(consumer);
      if (value !== undefined && value.exists()) {
        return {value, consumer: meta.consumer};
      }
    }

    return {value: undefined, consumer: meta.consumer};
  }

  async addProject(
    projectFolder: AbsoluteFilePath,
    configPath: AbsoluteFilePath,
  ): Promise<ProjectDefinition> {
    const {config, meta} = loadCompleteProjectConfig(projectFolder, configPath);

    return this.addProjectWithConfig({
      projectFolder,
      meta,
      config,
    });
  }

  async addProjectWithConfig({
    projectFolder,
    meta,
    config,
  }: {
    projectFolder: AbsoluteFilePath;
    meta: ProjectConfigMeta;
    config: ProjectConfig;
  }): Promise<ProjectDefinition> {
    // Make sure there's no project with the same `name` as us
    for (const project of this.projects.values()) {
      if (project.config.name === config.name) {
        throw new Error(
          `Conflicting project names. ${projectFolder} and ${project.folder}`,
        );
      }
    }

    // Declare the project
    const parentProject = this.findProjectExisting(projectFolder.getParent());
    const project: ProjectDefinition = {
      config,
      meta,
      folder: projectFolder,
      id: this.projectIdCounter++,
      packages: new Map(),
      manifests: new Map(),
      hasteMap: new Map(),
      parent: parentProject,
      children: new Set(),
    };

    this.projects.set(project.id, project);
    this.fileToProject.set(projectFolder, {
      path: projectFolder,
      projectId: project.id,
    });
    this.projectFolderToId.set(projectFolder, project.id);

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
    this.master.workerManager.onNewProject(project);

    // Start watching and crawl this project folder
    await this.master.memoryFs.watch(projectFolder, config);

    return project;
  }

  declareManifest(
    project: ProjectDefinition,
    isProjectPackage: boolean,
    def: ManifestDefinition,
    diagnostics: DiagnosticsProcessor,
  ) {
    const {name} = def.manifest;

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
          category: 'projectManager',
          filename: def.path.join(),
          message: `Duplicate package name <emphasis>${name}</emphasis>`,
          advice: [
            {
              type: 'log',
              category: 'info',
              message: `Defined already by <filelink target="${existingPackage.path}" />`,
            },
          ],
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

  isHasteIgnored(path: AbsoluteFilePath, config: ProjectConfig): boolean {
    return matchPathPatterns(path, config.haste.ignore) !== 'NO_MATCH';
  }

  isHasteDeclared(path: AbsoluteFilePath, project: ProjectDefinition): boolean {
    if (project.config.haste.enabled === false) {
      return false;
    }

    const hasteName = this.master.memoryFs.getHasteName(path);
    if (hasteName === undefined) {
      return false;
    }

    const existing = project.hasteMap.get(hasteName);
    if (existing === undefined) {
      return false;
    }

    return existing.equal(path);
  }

  declareHaste(
    path: AbsoluteFilePath,
    hasteName: string,
    hastePath: AbsoluteFilePath,
    diagnostics: DiagnosticsProcessor,
  ) {
    for (const project of this.getHierarchyFromFilename(path)) {
      const {hasteMap, config} = project;

      if (config.haste.enabled === false) {
        break;
      }

      if (this.isHasteIgnored(path, config)) {
        continue;
      }

      let existing = hasteMap.get(hasteName);
      if (existing !== undefined && !existing.equal(hastePath)) {
        const existingResolved = this.master.resolver.resolvePath({
          // `existing` will be absolute so the origin doesn't really matter
          origin: existing,
          source: existing,
        });

        // If both resolve to the same location then this isn't a collision and we should just ignore it
        if (
          existingResolved.type === 'FOUND' &&
          hastePath.equal(existingResolved.ref.real)
        ) {
          continue;
        }

        diagnostics.addDiagnostic({
          category: 'projectManager',
          filename: hastePath.join(),
          message: `Found a haste collision for <emphasis>${hasteName}</emphasis>`,
          advice: [
            {
              type: 'log',
              category: 'info',
              message: `Defined already by <filelink target="${existing}" />`,
            },
          ],
        });
        continue;
      }

      hasteMap.set(hasteName, hastePath);
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
        folder: project.folder.join(),
      });

      for (const def of project.manifests.values()) {
        manifestsSerial.push({
          id: def.id,
          manifest: this.master.memoryFs.getPartialManifest(def),
        });
      }
    }

    const promises = [];

    for (const worker of workers) {
      promises.push(
        worker.bridge.updateProjects.call({projects: projectsSerial}),
      );
      promises.push(
        worker.bridge.updateManifests.call({manifests: manifestsSerial}),
      );
    }

    await Promise.all(promises);
  }

  async assertProject(
    path: AbsoluteFilePath,
    pointer?: DiagnosticPointer,
  ): Promise<ProjectDefinition> {
    // We won't recurse up and check a parent project if we've already visited it
    const syncProject = this.findProjectExisting(path);
    const project = syncProject || (await this.findProject(path));

    if (project) {
      // Continue searching for projects up the directory
      // We don't do this for root projects since it would be a waste, but there's no implications other than some unnecessary work if we did
      if (project.config.root === false && syncProject === undefined) {
        await this.findProject(project.folder.getParent());
      }

      return project;
    }

    if (pointer === undefined) {
      throw new Error(
        `Couldn't find a project. Checked ${ROME_CONFIG_FILENAMES.join(
          ' or ',
        )} for ${path.join()}`,
      );
    }

    throw new DiagnosticsError(`No project found for ${path.join()}`, [
      {
        ...pointer,
        category: 'project',
        message: `Couldn't find a project`,
        advice: [
          {
            type: 'log',
            category: 'info',
            message:
              'Run <command>rome init</command> in this folder to initialize a project',
          },
        ],
      },
    ]);
  }

  // Convenience method to get the project config and pass it to the file handler class
  getHandlerWithProject(path: AbsoluteFilePath): GetFileHandlerResult {
    const project = this.findProjectExisting(path);
    if (project === undefined) {
      return {ext: '', handler: undefined};
    } else {
      return getFileHandler(path, project.config);
    }
  }

  getHierarchyFromFilename(
    filename: AbsoluteFilePath,
  ): Array<ProjectDefinition> {
    const project = this.findProjectExisting(filename);
    if (project === undefined) {
      return [];
    } else {
      return this.getHierarchyFromProject(project);
    }
  }

  getHierarchyFromProject(
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

  assertProjectExisting(path: AbsoluteFilePath): ProjectDefinition {
    const project = this.findProjectExisting(path);
    if (project === undefined) {
      throw new Error(`Expected existing project for ${path.join()}`);
    }
    return project;
  }

  findProjectExisting(cwd: AbsoluteFilePath): undefined | ProjectDefinition {
    const tried: Array<AbsoluteFilePath> = [];

    for (const dir of cwd.getChain()) {
      const cached = this.fileToProject.get(dir);
      if (cached === undefined) {
        tried.push(dir);
      } else {
        for (const dir of tried) {
          this.fileToProject.set(dir, cached);
        }

        const project = this.projects.get(cached.projectId);
        if (project === undefined) {
          throw new Error(
            'Expected project from project id found in fileToProject',
          );
        }
        return project;
      }
    }

    return undefined;
  }

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
    for (const dir of parentDirectories) {
      // Check for dedicated project configs
      for (const configFilename of ROME_CONFIG_FILENAMES) {
        // Check in root
        const configPath = dir.append(configFilename);
        const hasProject = await this.master.memoryFs.existsHard(configPath);
        if (hasProject) {
          return this.queueAddProject(dir, configPath);
        }

        // Check a .config folder
        const configPathNested = dir
          .append(ROME_CONFIG_FOLDER)
          .append(configFilename);
        const hasProjectNested = await this.master.memoryFs.existsHard(
          configPathNested,
        );
        if (hasProjectNested) {
          return this.queueAddProject(dir, configPathNested);
        }
      }

      // Check for package.json
      const packagePath = dir.append('package.json');
      if (await this.master.memoryFs.existsHard(packagePath)) {
        const input = await readFileText(packagePath);
        const json = await consumeJSON({input, path: packagePath});
        if (json.has(ROME_CONFIG_PACKAGE_JSON_FIELD)) {
          return this.queueAddProject(dir, packagePath);
        }
      }
    }

    // If we didn't find a project config then check for incorrect config filenames
    for (const dir of parentDirectories) {
      for (const basename of ROME_CONFIG_WARN_FILENAMES) {
        const path = dir.append(basename);

        if (await this.master.memoryFs.existsHard(path)) {
          this.warnIncorrectConfigFile(
            path,
            DiagnosticsProcessor.createImmediateThrower([
              {
                category: 'project-manager',
                message: 'Find project',
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
      category: 'projectManager',
      filename: path.join(),
      message: `Invalid rome config filename, <emphasis>${ROME_CONFIG_FILENAMES.join(
        ' or ',
      )}</emphasis> are the only valid filename`,
    });
  }
}
