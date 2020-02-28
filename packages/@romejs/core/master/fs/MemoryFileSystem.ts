/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Master from '../Master';
import {Manifest, ManifestDefinition} from '@romejs/codec-js-manifest';
import {PathPatterns} from '@romejs/path-match';
import {ProjectConfig, ROME_CONFIG_FILENAMES} from '@romejs/project';
import {
  DiagnosticsProcessor,
  getDiagnosticsFromError,
} from '@romejs/diagnostics';
import {Reporter} from '@romejs/cli-reporter';
import {createWatchmanClient} from '@romejs/codec-watchman';
import {Event} from '@romejs/events';
import {consumeJSON} from '@romejs/codec-json';
import {normalizeManifest} from '@romejs/codec-js-manifest';
import {humanizeNumber} from '@romejs/string-utils';
import {matchPathPatterns} from '@romejs/path-match';
import {WatchmanSubscriptionValue} from '@romejs/codec-watchman';
import {WorkerPartialManifest} from '../../common/bridges/WorkerBridge';
import {
  AbsoluteFilePath,
  AbsoluteFilePathMap,
  AbsoluteFilePathSet,
} from '@romejs/path';
import {lstat, readFileText, exists, readdir, watch} from '@romejs/fs';
import crypto = require('crypto');
import fs = require('fs');
import {
  getFileHandler,
  getFileHandlerExtensions,
} from '../../common/fileHandlers';
import {
  TEST_FOLDER_NAME,
  MOCKS_FOLDER_NAME,
} from '@romejs/core/common/constants';

const DEFAULT_DENYLIST = ['.hg', '.git'];

const PACKAGE_JSON = 'package.json';

// Whenever we're performing an operation on a set of files, always do these first as they may influence how the rest are processed
const PRIORITY_FILES = new Set(ROME_CONFIG_FILENAMES);

type DeclareManifestOpts = {
  diagnostics: DiagnosticsProcessor;
  dirname: AbsoluteFilePath;
  path: AbsoluteFilePath;
  hasteName: undefined | string;
  hastePath: AbsoluteFilePath;
};

type CrawlOptions = {
  diagnostics: DiagnosticsProcessor;
  crawl: boolean;
  onFoundDirectory?: (filename: AbsoluteFilePath) => void;
  tick?: (filename: AbsoluteFilePath) => void;
};

export type StatsType = 'unknown' | 'directory' | 'file';

export type Stats = {
  size: number;
  mtime: number;
  type: StatsType;
};

export type WatcherClose = () => void;

export type HasteCollisionCallback = (
  hasteName: string,
  existing: string,
  filename: string,
) => void;

async function createRegularWatcher(
  memoryFs: MemoryFileSystem,
  diagnostics: DiagnosticsProcessor,
  projectFolderPath: AbsoluteFilePath,
): Promise<WatcherClose> {
  const projectFolder = projectFolderPath.join();
  const {logger} = memoryFs.master;

  // Create activity spinners for all connected reporters
  const activity = memoryFs.master.connectedReporters.progress({
    initDelay: 1000,
  });
  activity.setTitle(`Adding project ${projectFolder}`);

  const watchers: AbsoluteFilePathMap<fs.FSWatcher> = new AbsoluteFilePathMap();

  try {
    function onFoundDirectory(folderPath: AbsoluteFilePath) {
      if (watchers.has(folderPath)) {
        return;
      }

      if (process.platform === 'linux') {
        // Node on Linux doesn't support recursive directory watching so we need an fs.watch for every directory...
      } else if (!folderPath.equal(projectFolderPath)) {
        // If we're on any other platform then only watch the root project folder
        return;
      }

      const watcher = watch(
        folderPath,
        {recursive: true, persistent: false},
        (eventType, filename) => {
          const path = folderPath.resolve(filename);

          memoryFs
            .stat(path)
            .then(newStats => {
              const diagnostics = memoryFs.master.createDisconnectedDiagnosticsProcessor(
                [
                  {
                    category: 'memory-fs',
                    message: 'Processing fs.watch changes',
                  },
                ],
              );

              if (newStats.type === 'file') {
                memoryFs.handleFileChange(path, newStats, {
                  diagnostics,
                  crawl: true,
                });
              } else if (newStats.type === 'directory') {
                memoryFs.addDirectory(path, newStats, {
                  crawl: true,
                  diagnostics,
                  onFoundDirectory,
                });
              }
            })
            .catch(err => {
              if (err.code === 'ENOENT') {
                memoryFs.handleDeletion(path);
              } else {
                throw err;
              }
            });
        },
      );
      watchers.set(folderPath, watcher);
    }

    // No need to call watch() on the projectFolder since it will call us

    // Perform an initial crawl
    const stats = await memoryFs.stat(projectFolderPath);
    await memoryFs.addDirectory(projectFolderPath, stats, {
      crawl: true,
      diagnostics,
      onFoundDirectory,
    });
    logger.info(
      `[MemoryFileSystem] Finished initial crawl for ${projectFolder} - added ${humanizeNumber(
        memoryFs.countFiles(projectFolderPath),
      )} files`,
    );
  } finally {
    activity.end();
  }

  return () => {
    for (const watcher of watchers.values()) {
      watcher.close();
    }
  };
}

async function createWatchmanWatcher(
  memoryFs: MemoryFileSystem,
  diagnostics: DiagnosticsProcessor,
  projectFolderPath: AbsoluteFilePath,
  projectConfig: ProjectConfig,
): Promise<WatcherClose> {
  const projectFolder = projectFolderPath.join();
  const {connectedReporters} = memoryFs.master;

  const activity = connectedReporters.progress();
  activity.setTitle(`Adding project ${projectFolder} with watchman`);

  let timeout;

  function queueCallout() {
    timeout = setTimeout(
      memoryFs.master.wrapFatal(() => {
        connectedReporters.warn(
          'Watchman is taking a while to respond. Watchman may have just started and is still crawling the disk.',
        );

        // Show an even more aggressive message when watchman takes longer
        queueCallout();
      }),
      5000,
    );
  }

  // Show a message when watchman takes too long
  queueCallout();

  try {
    const client = await createWatchmanClient(Reporter.fromProcess());

    const event = await client.createSubscription(projectFolder, {
      fields: ['mtime', 'name', 'size', 'type', 'exists'],
      expression: [
        'anyof',
        ['type', 'd'],
        ['suffix', getFileHandlerExtensions(projectConfig)],
      ],
    });

    const initial: WatchmanSubscriptionValue = await event.wait();
    if (initial.is_fresh_instance !== true) {
      throw new Error('Expected this to be a fresh instance');
    }
    clearTimeout(timeout);

    const processChanges = async (
      data: WatchmanSubscriptionValue,
      diagnostics: DiagnosticsProcessor,
    ) => {
      if (data['state-enter'] || data['state-leave']) {
        return;
      }

      const dirs: Array<[AbsoluteFilePath, any]> = [];
      const files: Array<[AbsoluteFilePath, any]> = [];

      for (const file of data.files) {
        const path = projectFolderPath.append(file.name);

        if (file.exists === false) {
          memoryFs.handleDeletion(path);
          continue;
        }

        if (file.type === 'f') {
          const basename = path.getBasename();

          if (PRIORITY_FILES.has(basename)) {
            files.unshift([path, file]);
          } else {
            files.push([path, file]);
          }
        } else if (file.type === 'd') {
          dirs.push([path, file]);
        }
      }

      await Promise.all(
        dirs.map(async ([path, info]) => {
          await memoryFs.addDirectory(
            path,
            {
              size: info.size,
              mtime: info.mtime,
              type: 'directory',
            },
            {diagnostics, crawl: false},
          );
        }),
      );

      await Promise.all(
        files.map(async ([path, info]) => {
          const stats: Stats = {
            size: info.size,
            mtime: info.mtime,
            type: 'file',
          };

          if (memoryFs.files.has(path)) {
            await memoryFs.handleFileChange(path, stats, {
              diagnostics,
              crawl: false,
            });
          } else {
            await memoryFs.addFile(path, stats, {
              diagnostics,
              crawl: false,
            });
          }
        }),
      );
    };

    activity.setText(`Processing results`);
    await processChanges(initial, diagnostics);

    event.subscribe((data: WatchmanSubscriptionValue) => {
      processChanges(
        data,
        memoryFs.master.createDisconnectedDiagnosticsProcessor([
          {
            category: 'memory-fs',
            message: 'Processing watchman changes',
          },
        ]),
      );
    });

    activity.end();

    return () => {
      // TODO close
    };
  } catch (err) {
    activity.end();

    if (err.message.includes('RootResolveError')) {
      // Fallback to node processor
      memoryFs.master.connectedReporters.error(
        `Failed to use watchman: ${err.message}`,
      );
      return createRegularWatcher(memoryFs, diagnostics, projectFolderPath);
    } else {
      throw err;
    }
  } finally {
    clearTimeout(timeout);
  }
}

export default class MemoryFileSystem {
  constructor(master: Master) {
    this.master = master;

    this.watchPromises = new Map();
    this.directoryListings = new AbsoluteFilePathMap();
    this.directories = new AbsoluteFilePathMap();
    this.files = new AbsoluteFilePathMap();
    this.manifests = new AbsoluteFilePathMap();
    this.watchers = new Map();
    this.manifestCounter = 0;

    this.changedFileEvent = new Event({
      name: 'MemoryFileSystem.changedFile',
      onError: master.onFatalErrorBound,
    });
    this.deletedFileEvent = new Event({
      name: 'MemoryFileSystem.deletedFile',
      onError: master.onFatalErrorBound,
    });
  }

  manifestCounter: number;
  master: Master;
  directoryListings: AbsoluteFilePathMap<AbsoluteFilePathMap<AbsoluteFilePath>>;
  directories: AbsoluteFilePathMap<Stats>;
  files: AbsoluteFilePathMap<Stats>;
  manifests: AbsoluteFilePathMap<ManifestDefinition>;

  watchers: Map<
    string,
    {
      path: AbsoluteFilePath;
      close: WatcherClose;
    }
  >;

  watchPromises: Map<
    string,
    {
      promise: Promise<WatcherClose>;
      path: AbsoluteFilePath;
    }
  >;

  changedFileEvent: Event<
    {path: AbsoluteFilePath; oldStats: undefined | Stats; newStats: Stats},
    void
  >;
  deletedFileEvent: Event<AbsoluteFilePath, void>;

  init() {}

  unwatch(dirPath: AbsoluteFilePath) {
    const dir = dirPath.join();
    const watcher = this.watchers.get(dir);
    if (watcher === undefined) {
      return;
    }

    this.watchers.delete(dir);
    watcher.close();

    // Go through and clear all files and directories from our internal maps
    // NOTE: We deliberately do not call 'deletedFileEvent' as the code that
    // calls us will already be cleaning up
    let queue: Array<AbsoluteFilePath> = [dirPath];
    while (queue.length > 0) {
      const path = queue.pop();
      if (path === undefined) {
        throw new Error('Unknown path');
      }

      this.directories.delete(path);
      this.manifests.delete(path);
      this.files.delete(path);

      const listing = this.directoryListings.get(path);
      if (listing !== undefined) {
        this.directoryListings.delete(path);
        queue = queue.concat(Array.from(listing.values()));
      }
    }
  }

  unwatchAll() {
    for (const {close} of this.watchers.values()) {
      close();
    }
  }

  readdir(path: AbsoluteFilePath): Iterable<AbsoluteFilePath> {
    const listing = this.directoryListings.get(path);
    if (listing === undefined) {
      return [];
    } else {
      return listing.values();
    }
  }

  isDirectory(filename: AbsoluteFilePath): boolean {
    return this.directories.has(filename);
  }

  isFile(filename: AbsoluteFilePath): boolean {
    return this.files.has(filename);
  }

  getFiles(): Array<Stats> {
    return Array.from(this.files.values());
  }

  getManifestDefinition(
    dirname: AbsoluteFilePath,
  ): undefined | ManifestDefinition {
    return this.manifests.get(dirname);
  }

  getManifest(dirname: AbsoluteFilePath): undefined | Manifest {
    const def = this.getManifestDefinition(dirname);
    if (def === undefined) {
      return undefined;
    } else {
      return def.manifest;
    }
  }

  getOwnedManifest(path: AbsoluteFilePath): undefined | ManifestDefinition {
    for (const dir of path.getChain()) {
      const def = this.master.memoryFs.getManifestDefinition(dir);
      if (def !== undefined) {
        return def;
      }
    }
  }

  getPartialManifest(def: ManifestDefinition): WorkerPartialManifest {
    return {
      path: def.path.join(),
      type: def.manifest.type,
    };
  }

  addFileToDirectoryListing(path: AbsoluteFilePath) {
    const dirname = path.getParent();
    let listing = this.directoryListings.get(dirname);
    if (listing === undefined) {
      listing = new AbsoluteFilePathMap();
      this.directoryListings.set(dirname, listing);
    }
    listing.set(path, path);
  }

  handleDeletion(path: AbsoluteFilePath) {
    // If a folder then evict all children
    const folderInfo = this.directories.get(path);
    if (folderInfo !== undefined) {
      this.directories.delete(path);

      const listing = this.directoryListings.get(path);
      if (listing !== undefined) {
        this.directoryListings.delete(path);
        for (const path of listing.values()) {
          this.handleDeletion(path);
        }
      }
    }

    // Remove from 'all possible caches
    this.files.delete(path);

    // Remove from 'haste maps
    this.handleDeletedHaste(path);

    // If this is a manifest filename then clear it from 'any possible package and our internal module map
    const basename = path.getBasename();
    if (basename === 'package.json') {
      this.handleDeletedManifest(path);
    }

    // Remove from 'parent directory listing
    const dirname = path.getParent();
    const parentListing = this.directoryListings.get(dirname);
    if (parentListing !== undefined) {
      parentListing.delete(path);
    }

    this.deletedFileEvent.send(path);
  }

  handleDeletedHaste(path: AbsoluteFilePath) {
    const hasteName = this.getHasteName(path);
    if (hasteName === undefined) {
      return undefined;
    }

    const projects = this.master.projectManager.getHierarchyFromFilename(path);
    for (const {hasteMap} of projects) {
      const existing = hasteMap.get(hasteName);
      if (existing !== undefined && existing.equal(path)) {
        hasteMap.delete(hasteName);
      }
    }
  }

  handleDeletedManifest(path: AbsoluteFilePath) {
    const folder = path.getParent();
    const def = this.manifests.get(folder);
    if (def !== undefined) {
      this.manifests.delete(folder);
    }
  }

  async handleFileChange(
    path: AbsoluteFilePath,
    stats: Stats,
    opts: CrawlOptions,
  ): Promise<boolean> {
    const oldStats: undefined | Stats = this.getFileStats(path);
    const changed = await this.addFile(path, stats, opts);
    if (changed) {
      const newStats: Stats = this.getFileStatsAssert(path);
      this.changedFileEvent.send({path, oldStats, newStats});
    }
    return changed;
  }

  async watch(
    projectFolderPath: AbsoluteFilePath,
    projectConfig: ProjectConfig,
  ): Promise<void> {
    const {logger} = this.master;
    const projectFolder = projectFolderPath.join();

    // Defer if we're already currently initializing this project
    const cached = this.watchPromises.get(projectFolder);
    if (cached !== undefined) {
      await cached;
      return undefined;
    }

    // Check if we're already watching this folder
    if (this.watchers.has(projectFolder)) {
      return undefined;
    }

    // Check if we're already watching a parent directory
    for (const {path} of this.watchers.values()) {
      if (projectFolderPath.isRelativeTo(path)) {
        logger.info(
          `[MemoryFileSystem] Skipped crawl for ${projectFolder} because we're already watching the parent directory ${path.join()}`,
        );
        return undefined;
      }
    }

    // Defer if we're initializing a parent folder
    for (const {promise, path} of this.watchPromises.values()) {
      if (projectFolderPath.isRelativeTo(path)) {
        await promise;
        return undefined;
      }
    }

    // Wait if we're initializing descendents
    for (const {path, promise} of this.watchPromises.values()) {
      if (path.isRelativeTo(projectFolderPath)) {
        await promise;
      }
    }

    // New watch target
    logger.info(
      `[MemoryFileSystem] Adding new project folder ${projectFolder}`,
    );

    // Remove watchers that are descedents of this folder as this watcher will handle them
    for (const [loc, {close, path}] of this.watchers) {
      if (path.isRelativeTo(projectFolderPath)) {
        this.watchers.delete(loc);
        close();
      }
    }

    const diagnostics = new DiagnosticsProcessor({
      origins: [
        {
          category: 'memory-fs',
          message: 'Crawling project folder',
        },
      ],
    });

    let promise;
    if (projectConfig.files.watchman) {
      logger.info(`[MemoryFileSystem] Watching ${projectFolder} with watchman`);
      promise = createWatchmanWatcher(
        this,
        diagnostics,
        projectFolderPath,
        projectConfig,
      );
    } else {
      logger.info(`[MemoryFileSystem] Watching ${projectFolder} with fs.watch`);
      promise = createRegularWatcher(this, diagnostics, projectFolderPath);
    }
    this.watchPromises.set(projectFolder, {
      path: projectFolderPath,
      promise,
    });

    const watcherClose = await promise;
    this.watchers.set(projectFolder, {
      path: projectFolderPath,
      close: watcherClose,
    });
    this.watchPromises.delete(projectFolder);

    diagnostics.maybeThrowDiagnosticsError();
  }

  async stat(path: AbsoluteFilePath): Promise<Stats> {
    const stats = await lstat(path);

    let type: StatsType = 'unknown';
    if (stats.isDirectory()) {
      type = 'directory';
    } else if (stats.isFile()) {
      type = 'file';
    }

    return {
      type,
      size: stats.size,
      mtime: stats.mtimeMs,
    };
  }

  getMtime(filename: AbsoluteFilePath) {
    const stats = this.getFileStats(filename);
    if (stats === undefined) {
      throw new Error(
        `File ${filename.join()} not in database, cannot get mtime`,
      );
    } else {
      return stats.mtime;
    }
  }

  getFileStats(filename: AbsoluteFilePath): undefined | Stats {
    return this.files.get(filename);
  }

  getFileStatsAssert(filename: AbsoluteFilePath): Stats {
    const stats = this.getFileStats(filename);
    if (stats === undefined) {
      throw new Error(`Expected file stats for ${filename}`);
    }
    return stats;
  }

  isIgnored(path: AbsoluteFilePath, type: 'directory' | 'file'): boolean {
    const project = this.master.projectManager.findProjectExisting(path);
    if (project === undefined) {
      return false;
    }

    // If we're a file and don't have an extension handler so there's no reason for us to care about it
    if (type === 'file' && getFileHandler(path, project.config) === undefined) {
      return true;
    }

    // Ensure we aren't in any of the default denylists
    const basename = path.getBasename();
    if (DEFAULT_DENYLIST.includes(basename)) {
      return true;
    }

    return false;
  }

  isInsideProject(path: AbsoluteFilePath): boolean {
    return path.getSegments().includes('node_modules') === false;
  }

  isInsideHaste(path: AbsoluteFilePath): boolean {
    const parts = path.getSegments();

    if (!this.isInsideProject(path)) {
      return false;
    }

    // Don't consider files in mocks
    const project = this.master.projectManager.findProjectExisting(path);
    if (project !== undefined) {
      if (
        parts.includes(TEST_FOLDER_NAME) ||
        parts.includes(MOCKS_FOLDER_NAME)
      ) {
        return false;
      }
    }

    // Check if we're inside a haste package, child files of a haste package shouldn't be added to the haste map
    for (const dir of path.getChain()) {
      const packagePath = dir.append(PACKAGE_JSON);
      if (path.equal(packagePath)) {
        // isInsideHaste will be called after we declare a haste package, all it's subfiles wont be inside the haste map but we should still be
        continue;
      }

      const manifest = this.getManifest(packagePath);
      if (manifest !== undefined && manifest.raw.haste_commonjs === true) {
        return false;
      }
    }

    return true;
  }

  getHasteName(path: AbsoluteFilePath): undefined | string {
    const filename = path.join();

    let {handler, ext} = this.master.projectManager.getHandlerWithProject(path);
    if (handler === undefined || handler.hasteMode === undefined) {
      return undefined;
    }

    const basename = path.getBasename();

    if (handler.hasteMode === 'ext') {
      ext = '.' + ext; // we also want to remove the dot suffix from the haste name

      if (!filename.endsWith(ext)) {
        throw new Error(
          `Expected ${filename} to end with ${ext} as it was returned as the extension name`,
        );
      }

      return basename.slice(0, -ext.length);
    } else if (handler.hasteMode === 'noext') {
      return basename;
    }

    return undefined;
  }

  // This is a wrapper around _declareManifest as it can produce diagnostics
  async declareManifest(
    opts: DeclareManifestOpts,
  ): Promise<undefined | string> {
    try {
      return await this._declareManifest(opts);
    } catch (err) {
      const diagnostics = getDiagnosticsFromError(err);

      if (diagnostics === undefined) {
        throw err;
      } else {
        opts.diagnostics.addDiagnostics(diagnostics);
      }
      return undefined;
    }
  }

  async _declareManifest({
    path,
    hasteName,
    diagnostics,
  }: DeclareManifestOpts): Promise<undefined | string> {
    // Fetch the manifest
    const manifestRaw = await readFileText(path);
    const hash = crypto
      .createHash('sha256')
      .update(manifestRaw)
      .digest('hex');

    const consumer = consumeJSON({
      path: path,
      input: manifestRaw,
      consumeCategory: 'manifest',
    });

    const {
      manifest,
      diagnostics: normalizedDiagnostics,
    } = await normalizeManifest(path, consumer);

    // If manifest is undefined then we failed to validate and have diagnostics
    if (normalizedDiagnostics.length > 0) {
      diagnostics.addDiagnostics(normalizedDiagnostics);
      return;
    }

    const folder = path.getParent();
    const manifestId = this.manifestCounter++;
    const def: ManifestDefinition = {
      id: manifestId,
      path: path,
      folder,
      consumer,
      manifest,
      hash,
    };

    this.manifests.set(folder, def);

    // Set haste name and haste location to the directory itself
    if (manifest.name !== undefined) {
      hasteName = manifest.name;
    }

    // If we aren't in node_modules then this is a project package
    const isProjectPackage = this.isInsideProject(path);
    const {projectManager} = this.master;
    const project = projectManager.findProjectExisting(path);
    if (project !== undefined) {
      projectManager.declareManifest(
        project,
        isProjectPackage,
        def,
        diagnostics,
      );
    }

    // Tell all workers of our discovery
    for (const worker of this.master.workerManager.getWorkers()) {
      worker.bridge.updateManifests.call({
        manifests: [{id: def.id, manifest: this.getPartialManifest(def)}],
      });
    }

    return hasteName;
  }

  glob(
    cwd: AbsoluteFilePath,
    opts: {
      extensions?: Array<string>;
      ignore?: PathPatterns;
    } = {},
  ): AbsoluteFilePathSet {
    const {extensions, ignore} = opts;

    const paths: AbsoluteFilePathSet = new AbsoluteFilePathSet();
    const ignoreParsed: PathPatterns = ignore === undefined ? [] : ignore;

    let crawl: Array<AbsoluteFilePath> = [cwd];

    while (crawl.length > 0) {
      const path = crawl.pop();
      if (path === undefined) {
        throw new Error('crawl.length already validated');
      }

      const matched = matchPathPatterns(path, ignoreParsed, cwd);

      // Don't even recurse into explicit matches
      if (matched === 'EXPLICIT_MATCH') {
        continue;
      }

      // Add if a matching file
      if (this.files.has(path) && matched === 'NO_MATCH') {
        // Remove the prefixed dot
        const ext = path.getExtensions().slice(1);
        if (extensions === undefined || extensions.includes(ext)) {
          paths.add(path);
        }
        continue;
      }

      // Crawl if we're a folder
      // NOTE: We still continue crawling on implicit matches
      const listing = this.directoryListings.get(path);
      if (listing !== undefined) {
        crawl = crawl.concat(Array.from(listing.values()));
        continue;
      }

      // TODO maybe throw? not a file or folder, doesn't exist!
    }

    return paths;
  }

  getAllFilesInFolder(folder: AbsoluteFilePath): Array<AbsoluteFilePath> {
    let files: Array<AbsoluteFilePath> = [];

    const listing = this.directoryListings.get(folder);
    if (listing !== undefined) {
      for (const file of listing.keys()) {
        if (this.files.has(file)) {
          files.push(file);
        } else {
          files = files.concat(this.getAllFilesInFolder(file));
        }
      }
    }

    return files;
  }

  countFiles(folder: AbsoluteFilePath): number {
    let count: number = 0;

    const listing = this.directoryListings.get(folder);
    if (listing !== undefined) {
      for (const file of listing.keys()) {
        count++;
        count += this.countFiles(file);
      }
    }

    return count;
  }

  hasStatsChanged(filename: AbsoluteFilePath, newStats: Stats): boolean {
    const oldStats = this.directories.get(filename) || this.files.get(filename);
    return oldStats === undefined || newStats.mtime !== oldStats.mtime;
  }

  async addDirectory(
    folderPath: AbsoluteFilePath,
    stats: Stats,
    opts: CrawlOptions,
  ): Promise<boolean> {
    if (!this.hasStatsChanged(folderPath, stats)) {
      return false;
    }

    // Check if this folder has been ignored
    if (this.isIgnored(folderPath, 'directory')) {
      return false;
    }

    if (opts.tick !== undefined) {
      opts.tick(folderPath);
    }

    this.addFileToDirectoryListing(folderPath);
    this.directories.set(folderPath, stats);

    if (opts.onFoundDirectory !== undefined) {
      opts.onFoundDirectory(folderPath);
    }

    if (opts.crawl) {
      // Crawl the folder
      const files = await readdir(folderPath);

      // Declare the file
      const declareItem = async (path: AbsoluteFilePath) => {
        const stats = await this.stat(path);
        if (stats.type === 'file') {
          await this.addFile(path, stats, opts);
        } else if (stats.type === 'directory') {
          await this.addDirectory(path, stats, opts);
        }
      };

      // Give priority to package.json as we base some haste heuristics on it's entry
      for (const file of files) {
        if (PRIORITY_FILES.has(file.getBasename())) {
          files.delete(file);
          await declareItem(file);
        }
      }

      // Add the rest of the items
      await Promise.all(Array.from(files, declareItem));
    }

    return true;
  }

  exists(path: AbsoluteFilePath): undefined | boolean {
    // if we have this in our cache then the file exists
    if (this.files.has(path) || this.directories.has(path)) {
      return true;
    }

    // If we're still performing an initial crawl of any path higher in the tree then we don't know if it exists yet
    for (const {path: projectFolder} of this.watchPromises.values()) {
      if (path.isRelativeTo(projectFolder)) {
        return undefined;
      }
    }

    // if we're watching the parent folder then we'd have it in our cache if it existed
    const parent = path.getParent();
    if (this.directories.has(parent)) {
      return false;
    }

    return undefined;
  }

  async existsHard(path: AbsoluteFilePath): Promise<boolean> {
    const resolvedExistence: undefined | boolean = this.exists(path);
    if (resolvedExistence === undefined) {
      return exists(path);
    } else {
      return resolvedExistence;
    }
  }

  async addFile(
    path: AbsoluteFilePath,
    stats: Stats,
    opts: CrawlOptions,
  ): Promise<boolean> {
    if (!this.hasStatsChanged(path, stats)) {
      return false;
    }

    // Check if this file has been ignored
    if (this.isIgnored(path, 'file')) {
      return false;
    }

    if (opts.tick !== undefined) {
      opts.tick(path);
    }

    this.files.set(path, stats);
    this.addFileToDirectoryListing(path);

    let hastePath = path;
    let hasteName = this.getHasteName(path);

    const basename = path.getBasename();
    const dirname = path.getParent();

    // Warn about potentially incorrect Rome config filenames
    const {projectManager} = this.master;
    projectManager.checkConfigFile(path, opts.diagnostics);

    // Add project if this is a config
    if (ROME_CONFIG_FILENAMES.includes(basename)) {
      await projectManager.queueAddProject(dirname, path);
    }

    // If this is a package.json then declare this module and setup the correct haste variables
    if (basename === 'package.json') {
      hasteName = await this.declareManifest({
        diagnostics: opts.diagnostics,
        dirname,
        path,
        hasteName,
        hastePath,
      });
      hastePath = dirname;
    }

    // Add to haste map
    if (hasteName !== undefined && this.isInsideHaste(path)) {
      projectManager.declareHaste(path, hasteName, hastePath, opts.diagnostics);
    }

    return true;
  }
}
