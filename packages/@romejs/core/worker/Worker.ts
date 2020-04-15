/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ModuleSignature, TypeCheckProvider} from '@romejs/js-analysis';
import WorkerBridge, {
  WorkerProjects,
  PrefetchedModuleSignatures,
  WorkerPartialManifest,
  WorkerPartialManifests,
  WorkerParseOptions,
} from '../common/bridges/WorkerBridge';
import {Program, ConstSourceType, ConstProgramSyntax} from '@romejs/js-ast';
import Logger from '../common/utils/Logger';
import {parseJS} from '@romejs/js-parser';
import {Profiler} from '@romejs/v8';
import WorkerAPI from './WorkerAPI';
import {Reporter} from '@romejs/cli-reporter';
import setupGlobalErrorHandlers from '../common/utils/setupGlobalErrorHandlers';
import {UserConfig, loadUserConfig} from '../common/userConfig';
import {hydrateJSONProjectConfig} from '@romejs/project';
import {Diagnostics, DiagnosticsError} from '@romejs/diagnostics';
import {
  createUnknownFilePath,
  AbsoluteFilePath,
  AbsoluteFilePathMap,
  UnknownFilePathMap,
  createAbsoluteFilePath,
} from '@romejs/path';
import {lstat, writeFile, readFileText} from '@romejs/fs';
import {
  FileReference,
  convertTransportFileReference,
} from '../common/types/files';
import {getFileHandlerAssert} from '../common/fileHandlers';
import {TransformProjectDefinition} from '@romejs/js-compiler';

export type ParseResult = {
  ast: Program;
  project: TransformProjectDefinition;
  path: AbsoluteFilePath;
  lastAccessed: number;
  sourceText: string;
  generated: boolean;
};

type WorkerOptions = {
  globalErrorHandlers: boolean;
  bridge: WorkerBridge;
};

export default class Worker {
  constructor(opts: WorkerOptions) {
    this.bridge = opts.bridge;

    this.userConfig = loadUserConfig();
    this.partialManifests = new Map();
    this.projects = new Map();
    this.astCache = new AbsoluteFilePathMap();
    this.moduleSignatureCache = new UnknownFilePathMap();
    this.buffers = new AbsoluteFilePathMap();

    this.logger = new Logger('worker', () => opts.bridge.log.hasSubscribers(), {
      streams: [
        {
          type: 'all',
          format: 'none',
          columns: Reporter.DEFAULT_COLUMNS,
          unicode: true,
          write(chunk) {
            opts.bridge.log.send(chunk.toString());
          },
        },
      ],
    });

    //
    this.api = new WorkerAPI(this);

    if (opts.globalErrorHandlers) {
      setupGlobalErrorHandlers((err) => {
        // TODO
        err;
      });
    }
  }

  userConfig: UserConfig;

  bridge: WorkerBridge;
  api: WorkerAPI;
  logger: Logger;

  partialManifests: Map<number, WorkerPartialManifest>;
  projects: Map<number, TransformProjectDefinition>;
  astCache: AbsoluteFilePathMap<ParseResult>;
  moduleSignatureCache: UnknownFilePathMap<ModuleSignature>;
  buffers: AbsoluteFilePathMap<string>;

  getPartialManifest(id: number): WorkerPartialManifest {
    const manifest = this.partialManifests.get(id);
    if (manifest === undefined) {
      throw new Error(`Requested manifest ${id} but we don't have it`);
    }
    return manifest;
  }

  end() {
    // This will only actually be called when a Worker is created inside of the Master

    // Clear internal maps for memory, in case the Worker instance sticks around
    this.astCache.clear();
    this.projects.clear();
    this.moduleSignatureCache.clear();
  }

  async init() {
    const bridge: WorkerBridge = this.bridge;

    bridge.endEvent.subscribe(() => {
      this.end();
    });

    let profiler: undefined | Profiler;
    bridge.profilingStart.subscribe(async (data) => {
      if (profiler !== undefined) {
        throw new Error('Expected no profiler to be running');
      }
      profiler = new Profiler();
      await profiler.startProfiling(data.samplingInterval);
    });

    bridge.profilingStop.subscribe(async () => {
      if (profiler === undefined) {
        throw new Error('Expected a profiler to be running');
      }
      const workerProfile = await profiler.stopProfiling();
      profiler = undefined;
      return workerProfile;
    });

    bridge.compileJS.subscribe((payload) => {
      return this.api.compileJS(
        convertTransportFileReference(payload.file),
        payload.stage,
        payload.options,
      );
    });

    bridge.parseJS.subscribe((payload) => {
      return this.api.parseJS(
        convertTransportFileReference(payload.file),
        payload.opts,
      );
    });

    bridge.lint.subscribe((payload) => {
      return this.api.lint(
        convertTransportFileReference(payload.file),
        payload.options,
      );
    });

    bridge.format.subscribe((payload) => {
      return this.api.format(
        convertTransportFileReference(payload.file),
        payload.options,
      );
    });

    bridge.analyzeDependencies.subscribe((payload) => {
      return this.api.analyzeDependencies(convertTransportFileReference(
        payload.file,
      ));
    });

    bridge.evict.subscribe((payload) => {
      this.evict(createAbsoluteFilePath(payload.filename));
      return undefined;
    });

    bridge.moduleSignatureJS.subscribe((payload) => {
      return this.api.moduleSignatureJS(convertTransportFileReference(
        payload.file,
      ));
    });

    bridge.updateProjects.subscribe((payload) => {
      return this.updateProjects(payload.projects);
    });

    bridge.updateManifests.subscribe((payload) => {
      return this.updateManifests(payload.manifests);
    });

    bridge.status.subscribe(() => {
      return {
        astCacheSize: this.astCache.size,
        pid: process.pid,
        memoryUsage: process.memoryUsage(),
        uptime: process.uptime(),
      };
    });

    bridge.updateBuffer.subscribe((payload) => {
      return this.updateBuffer(
        convertTransportFileReference(payload.file),
        payload.content,
      );
    });
  }

  async updateBuffer(ref: FileReference, content: string) {
    const path = ref.real;
    this.buffers.set(path, content);

    // Now outdated
    this.astCache.delete(path);
    this.moduleSignatureCache.delete(path);
  }

  async getTypeCheckProvider(
    projectId: number,
    prefetchedModuleSignatures: PrefetchedModuleSignatures = {},
  ): Promise<TypeCheckProvider> {
    const libs: Array<Program> = [];

    // TODO Figure out how to get the uids for the libraries, probably adding some additional stuff to ProjectConfig?

    /*
    const projectConfig = this.getProjectConfig(projectId);
    for (const filename of projectConfig.typeChecking.libs) {
      const {ast, err} = await this.parse(filename, uid, projectId);
      if (err) {
        throw err;
      } else {
        invariant(ast, 'expected ast');
        libs.push(ast);
      }
    }
    */
    const resolveGraph = async (
      key: string,
    ): Promise<undefined | ModuleSignature> => {
      const value = prefetchedModuleSignatures[key];
      if (value === undefined) {
        return undefined;
      }

      switch (value.type) {
        case 'RESOLVED': {
          this.moduleSignatureCache.set(createUnknownFilePath(
            value.graph.filename,
          ), value.graph);
          return value.graph;
        }

        case 'OWNED':
          return this.api.moduleSignatureJS(convertTransportFileReference(
            value.file,
          ));

        case 'POINTER':
          return resolveGraph(value.key);

        case 'USE_CACHED': {
          const cached = this.moduleSignatureCache.get(createUnknownFilePath(
            value.filename,
          ));
          if (cached === undefined) {
            throw new Error(
                `Master told us we have the export types for ${value.filename} cached but we dont!`,
              );
          }
          return cached;
        }
      }
    };

    return {
      getExportTypes: async (
        origin: string,
        relative: string,
      ): Promise<undefined | ModuleSignature> => {
        return resolveGraph(`${origin}:${relative}`);
      },
      libs,
    };
  }

  populateDiagnosticsMtime(diagnostics: Diagnostics): Diagnostics {
    return diagnostics;
  }

  async readFile(path: AbsoluteFilePath): Promise<string> {
    const buffer = this.buffers.get(path);
    if (buffer === undefined) {
      return await readFileText(path);
    } else {
      return buffer;
    }
  }

  async parseJS(
    ref: FileReference,
    opts: WorkerParseOptions = {},
  ): Promise<ParseResult> {
    const path = createAbsoluteFilePath(ref.real);

    const {project: projectId, uid} = ref;
    const project = this.getProject(projectId);

    // Fetch and validate extension handler
    const {handler} = getFileHandlerAssert(ref.real, project.config);
    if (handler.toJavaScript === undefined) {
      throw new Error(`We don't know how to convert the file ${path} to js`);
    }

    // Get syntax
    let syntax: Array<ConstProgramSyntax> = [];
    if (opts.syntax !== undefined) {
      syntax = opts.syntax;
    } else if (handler.syntax !== undefined) {
      syntax = handler.syntax;
    }

    // Get source type
    let sourceType: undefined | ConstSourceType;
    if (opts.sourceType !== undefined) {
      sourceType = opts.sourceType;
    } else if (handler.sourceType !== undefined) {
      sourceType = handler.sourceType;
    } else {
      sourceType = 'script';

      if (ref.manifest !== undefined) {
        const manifest = this.getPartialManifest(ref.manifest);
        if (manifest.type === 'module') {
          sourceType = 'module';
        }
      }
    }

    if (project.config.bundler.mode === 'legacy') {
      sourceType = 'module';
    }

    const cacheEnabled = opts.cache !== false;

    if (cacheEnabled) {
      // Update the lastAccessed of the ast cache and return it, it will be evicted on

      // any file change
      const cachedResult: undefined | ParseResult = this.astCache.get(path);
      if (cachedResult !== undefined) {
        let useCached = true;

        if (cachedResult.ast.sourceType !== sourceType) {
          useCached = false;
        }

        if (useCached) {
          this.astCache.set(path, {
            ...cachedResult,
            lastAccessed: Date.now(),
          });
          return cachedResult;
        }
      }
    }

    this.logger.info(`Parsing:`, path);

    const stat = await lstat(path);

    const {sourceText, generated} = await handler.toJavaScript({
      file: ref,
      worker: this,
      project,
    });

    let manifestPath: undefined | string;
    if (ref.manifest !== undefined) {
      manifestPath = this.getPartialManifest(ref.manifest).path;
    }

    const ast = parseJS({
      input: sourceText,
      mtime: stat.mtimeMs,
      manifestPath,
      path: createUnknownFilePath(uid),
      sourceType,
      syntax,
    });

    // If the AST is corrupt then we don't under any circumstance allow it
    if (ast.corrupt) {
      throw new DiagnosticsError('Corrupt AST', ast.diagnostics);
    }

    // Sometimes we may want to allow the "fixed" AST
    const allowDiagnostics = opts.allowDiagnostics === true;
    if (!allowDiagnostics && ast.diagnostics.length > 0) {
      throw new DiagnosticsError(
        "AST diagnostics aren't allowed",
        ast.diagnostics,
      );
    }

    const res: ParseResult = {
      ast,
      lastAccessed: Date.now(),
      sourceText,
      project,
      path,
      generated,
    };

    if (cacheEnabled) {
      this.astCache.set(path, res);
    }

    return res;
  }

  getProject(id: number): TransformProjectDefinition {
    const config = this.projects.get(id);
    if (config === undefined) {
      throw new Error(
        `Unknown project ${id}, known projects are ${this.projects.keys()}`,
      );
    }
    return config;
  }

  async writeFile(path: AbsoluteFilePath, content: string): Promise<void> {
    // Write the file out
    await writeFile(path, content);

    // We just wrote the file but the server watcher hasn't had time to notify us
    this.evict(path);
  }

  evict(path: AbsoluteFilePath) {
    this.astCache.delete(path);
    this.moduleSignatureCache.delete(path);
  }

  updateManifests(manifests: WorkerPartialManifests) {
    for (const {id, manifest} of manifests) {
      if (manifest === undefined) {
        this.partialManifests.delete(id);
      } else {
        this.partialManifests.set(id, manifest);
      }
    }
  }

  updateProjects(projects: WorkerProjects) {
    for (const {config, folder, id} of projects) {
      if (config === undefined) {
        this.projects.delete(id);
      } else {
        this.projects.set(id, {
          folder: createAbsoluteFilePath(folder),
          config: hydrateJSONProjectConfig(config),
        });
      }
    }
  }
}
