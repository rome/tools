/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  DEFAULT_CLIENT_FLAGS,
  DEFAULT_CLIENT_REQUEST_FLAGS,
} from '../common/types/client';
import {JSONFileReference} from '../common/types/files';
import {
  DiagnosticPointer,
  getDiagnosticsFromError,
  DiagnosticOrigin,
  PartialDiagnosticAdvice,
  createSingleDiagnosticError,
  DiagnosticsError,
} from '@romejs/diagnostics';
import {DiagnosticsPrinterFlags} from '@romejs/cli-diagnostics';
import {ProjectDefinition} from '@romejs/project';
import {ResolverOptions} from './fs/Resolver';
import {BundlerConfig} from '../common/types/bundler';
import MasterBridge, {
  MasterQueryRequest,
  MasterQueryResponse,
} from '../common/bridges/MasterBridge';
import Master, {
  MasterClient,
  MasterMarker,
  MasterUnfinishedMarker,
} from './Master';
import {Reporter} from '@romejs/cli-reporter';
import {Event} from '@romejs/events';
import {serializeCLIFlags, SerializeCLITarget} from '@romejs/cli-flags';
import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import {Program} from '@romejs/js-ast';
import {TransformStageName} from '@romejs/js-compiler';
import WorkerBridge, {
  PrefetchedModuleSignatures,
  WorkerAnalyzeDependencyResult,
  WorkerCompileResult,
  WorkerParseOptions,
  WorkerCompilerOptions,
  WorkerFormatResult,
  WorkerLintResult,
} from '../common/bridges/WorkerBridge';
import {ModuleSignature} from '@romejs/js-analysis';
import {
  AbsoluteFilePath,
  createAbsoluteFilePath,
  AbsoluteFilePathSet,
  createUnknownFilePath,
} from '@romejs/path';
import crypto = require('crypto');

import {createErrorFromStructure, getErrorStructure} from '@romejs/v8';
import {Dict, RequiredProps} from '@romejs/typescript-helpers';
import {number1, number0, coerce0} from '@romejs/ob1';
import {MemoryFSGlobOptions} from './fs/MemoryFileSystem';

type MasterRequestOptions = {
  client: MasterClient;
  query: MasterQueryRequest;
  master: Master;
  bridge: MasterBridge;
  reporter: Reporter;
};

let requestIdCounter = 0;

type NormalizedCommandFlags = {
  flags: undefined | Dict<unknown>;
  defaultFlags: Dict<unknown>;
};

export class MasterRequestInvalid extends DiagnosticsError {}

export default class MasterRequest {
  constructor(opts: MasterRequestOptions) {
    this.query = opts.query;
    this.master = opts.master;
    this.bridge = opts.bridge;
    this.reporter = opts.reporter;
    this.markerEvent = new Event({
      name: 'MasterRequest.marker',
      onError: this.master.onFatalErrorBound,
    });
    this.endEvent = new Event({
      name: 'MasterRequest.teardown',
      onError: this.master.onFatalErrorBound,
      serial: true,
    });
    this.client = opts.client;
    this.id = requestIdCounter++;

    this.normalizedCommandFlags = {
      flags: {},
      defaultFlags: {},
    };
  }

  client: MasterClient;
  query: MasterQueryRequest;
  master: Master;
  bridge: MasterBridge;
  reporter: Reporter;
  id: number;
  markerEvent: Event<MasterMarker, void>;
  endEvent: Event<MasterQueryResponse, void>;
  normalizedCommandFlags: NormalizedCommandFlags;

  setNormalizedCommandFlags(normalized: NormalizedCommandFlags) {
    this.normalizedCommandFlags = normalized;
  }

  teardown(response: MasterQueryResponse) {
    this.reporter.teardown();
    this.endEvent.send(response);
  }

  async assertClientCwdProject(): Promise<ProjectDefinition> {
    const pointer = this.getDiagnosticPointerForClientCwd();
    return this.master.projectManager.assertProject(
      this.client.flags.cwd,
      pointer,
    );
  }

  createDiagnosticsPrinter(origin: DiagnosticOrigin): DiagnosticsPrinter {
    return new DiagnosticsPrinter({
      origins: [
        {
          category: 'master',
          message: `${this.query.commandName} command was dispatched`,
        },
        origin,
      ],
      reporter: this.reporter,
      cwd: this.client.flags.cwd,
      flags: this.getDiagnosticsPrinterFlags(),
      readFile: this.master.readDiagnosticsPrinterFile.bind(this.master),
    });
  }

  getDiagnosticsPrinterFlags(): DiagnosticsPrinterFlags {
    const {requestFlags} = this.query;
    return {
      grep: requestFlags.grep,
      inverseGrep: requestFlags.inverseGrep,
      focus: requestFlags.focus,
      showAllDiagnostics: requestFlags.showAllDiagnostics,
      verboseDiagnostics: requestFlags.verboseDiagnostics,
      maxDiagnostics: requestFlags.maxDiagnostics,
      fieri: requestFlags.fieri,
    };
  }

  expectArgumentLength(min: number, max: number = min) {
    const {args} = this.query;
    let message;

    let excessive = false;

    if (min === max) {
      if (args.length !== min) {
        message = `Expected exactly <number emphasis>${min}</number> arguments`;
      }
    } else {
      if (args.length < min) {
        message = `Expected at least <number emphasis>${min}</number> arguments`;
      }

      if (args.length > max) {
        excessive = true;
        message =
          `Expected no more than <number emphasis>${min}</number> arguments`;
      }
    }

    if (message !== undefined) {
      this.throwDiagnosticFlagError(excessive
        ? 'Too many arguments' : 'Missing arguments', {
        type: 'arg-range',
        from: min,
        to: max,
      }, [
        {
          type: 'log',
          category: 'info',
          message,
        },
      ]);
    }
  }

  throwDiagnosticFlagError(
    message: string,
    target: SerializeCLITarget = {type: 'none'},
    advice?: PartialDiagnosticAdvice,
  ) {
    const pointer = this.getDiagnosticPointerFromFlags(target);
    throw new MasterRequestInvalid(message, [
      {
        message,
        filename: 'argv',
        category: target.type === 'arg' || target.type === 'arg-range'
          ? 'args/invalid' : 'flags/invalid',
        ...pointer,
        advice,
      },
    ]);
  }

  getDiagnosticPointerForClientCwd(): DiagnosticPointer {
    const cwd = this.client.flags.cwd.join();
    return {
      sourceText: cwd,
      start: {
        index: number0,
        line: number1,
        column: number0,
      },
      end: {
        index: coerce0(cwd.length),
        line: number1,
        column: coerce0(cwd.length),
      },
      filename: 'cwd',
    };
  }

  getDiagnosticPointerFromFlags(target: SerializeCLITarget): DiagnosticPointer {
    const {query} = this;
    return serializeCLIFlags({
      prefix: `rome ${query.commandName}`,
      flags: ({
        ...this.client.flags,
        ...query.requestFlags,
        ...this.normalizedCommandFlags.flags,
      } as Dict<unknown>),
      args: query.args,
      defaultFlags: {
        ...DEFAULT_CLIENT_FLAGS,
        ...DEFAULT_CLIENT_REQUEST_FLAGS,
        ...this.normalizedCommandFlags.defaultFlags,
        clientName: this.client.flags.clientName,
      },
      shorthandFlags: new Set(),
    }, target);
  }

  getResolverOptionsFromFlags(): RequiredProps<ResolverOptions, 'origin'> {
    const {requestFlags} = this.query;
    return {
      origin: this.client.flags.cwd,
      platform: requestFlags.resolverPlatform,
      scale: requestFlags.resolverScale,
      mocks: requestFlags.resolverMocks,
    };
  }

  getBundlerConfigFromFlags(
    resolverOpts?: Partial<ResolverOptions>,
  ): BundlerConfig {
    return {
      inlineSourceMap: false,
      cwd: this.client.flags.cwd,
      resolver: {
        ...this.getResolverOptionsFromFlags(),
        ...resolverOpts,
      },
    };
  }

  async getFilesFromArgs(
    globOpts:
      & MemoryFSGlobOptions
      & {
        advice?: PartialDiagnosticAdvice;
        configCategory?: string;
        verb?: string;
        noun?: string;
      } = {},
  ): Promise<AbsoluteFilePathSet> {
    const {master} = this;
    const {flags} = this.client;

    // Build up args, defaulting to the current project dir if none passed
    const rawArgs = [...this.query.args];
    const resolvedArgs: Array<{
      path: AbsoluteFilePath;
      pointer: DiagnosticPointer;
      project: ProjectDefinition;
    }> = [];
    if (rawArgs.length === 0) {
      const pointer = this.getDiagnosticPointerForClientCwd();
      const project = await this.assertClientCwdProject();
      resolvedArgs.push({
        path: project.folder,
        pointer,
        project,
      });
    } else {
      for (let i = 0;
      i < rawArgs.length;
      i++) {
        const arg = rawArgs[i];

        const pointer = this.getDiagnosticPointerFromFlags({
          type: 'arg',
          key: i,
        });

        const resolved = await this.master.resolver.resolveEntryAssert({
          origin: flags.cwd,
          source: createUnknownFilePath(arg),
          requestedType: 'folder',
        }, {
          pointer,
        });

        resolvedArgs.push({
          project: this.master.projectManager.assertProjectExisting(
            resolved.path,
          ),
          path: resolved.path,
          pointer,
        });
      }
    }

    // Build up files
    const paths: AbsoluteFilePathSet = new AbsoluteFilePathSet();
    for (const {path, pointer, project} of resolvedArgs) {
      const matches = master.memoryFs.glob(path, globOpts);

      if (matches.size > 0) {
        for (const path of matches) {
          paths.add(path);
        }
        continue;
      }

      let advice: PartialDiagnosticAdvice = globOpts.advice === undefined
        ? [] : [...globOpts.advice];

      // Hint if `path` failed `globOpts.test`
      if (globOpts.getProjectEnabled !== undefined) {
        const test = globOpts.getProjectEnabled(project);

        if (!test.enabled && test.source !== undefined) {
          const testSource = test.source;

          const explanationPrefix = globOpts.verb === undefined
            ? 'Files excluded' : `Files excluded from ${globOpts.verb}`;

          if (testSource.value === undefined) {
            let explanation =
            `${explanationPrefix} as it's not enabled for this project`;
            if (globOpts.configCategory !== undefined) {
              explanation +=
              `. Run <command>rome config enable-category ${globOpts.configCategory}</command> to enable it.`;
            }
            advice.push({
              type: 'log',
              category: 'info',
              message: explanation,
            });
          } else {
            advice.push({
              type: 'log',
              category: 'info',
              message: `${explanationPrefix} as it's explicitly disabled in this project config`,
            });

            const disabledPointer = testSource.value.getDiagnosticPointer(
              'value',
            );
            if (disabledPointer !== undefined) {
              advice.push({
                type: 'frame',
                ...disabledPointer,
              });
            }
          }
        }
      }

      // Hint if all files were ignored
      if (globOpts.getProjectIgnore !== undefined) {
        const ignore = globOpts.getProjectIgnore(project);

        const withoutIgnore = await this.getFilesFromArgs({
          ...globOpts,
          getProjectIgnore: undefined,
        });

        if (withoutIgnore.size > 0) {
          advice.push({
            type: 'log',
            category: 'info',
            message: 'The following files were ignored',
          });

          advice.push({
            type: 'list',
            list: Array.from(withoutIgnore, (path) =>
              `<filelink target="${path.join()}" />`
            ),
            truncate: true,
          });

          if (ignore.source !== undefined && ignore.source.value !== undefined) {
            const ignorePointer = ignore.source.value.getDiagnosticPointer(
              'value',
            );

            if (ignorePointer !== undefined) {
              advice.push({
                type: 'log',
                category: 'info',
                message: 'Ignore patterns were defined here',
              });

              advice.push({
                type: 'frame',
                ...ignorePointer,
              });
            }
          }
        }
      }

      throw createSingleDiagnosticError({
        ...pointer,
        category: 'args/fileNotFound',
        message: globOpts.noun === undefined
          ? 'No files found' : `No files to ${globOpts.noun} found`,
        advice,
      });
    }

    return paths;
  }

  normalizeCompileResult(res: WorkerCompileResult): WorkerCompileResult {
    const {projectManager} = this.master;

    // Turn all the cacheDependencies entries from 'absolute paths to UIDs
    return {
      ...res,
      cacheDependencies: res.cacheDependencies.map((filename) => {
        return (
          projectManager.getFileReference(createAbsoluteFilePath(filename)).uid
        );
      }),
    };
  }

  startMarker(
    opts: Omit<MasterUnfinishedMarker, 'start'>,
  ): MasterUnfinishedMarker {
    this.master.logger.info('Started marker %s', opts.label);
    return {
      ...opts,
      start: Date.now(),
    };
  }

  endMarker(startMarker: MasterUnfinishedMarker): MasterMarker {
    const endMarker: MasterMarker = {
      ...startMarker,
      end: Date.now(),
    };
    this.master.logger.info('Finished marker %s', startMarker.label);
    this.markerEvent.send(endMarker);
    return endMarker;
  }

  async wrapRequestDiagnostic<T>(
    method: string,
    path: AbsoluteFilePath,
    factory: (bridge: WorkerBridge, ref: JSONFileReference) => Promise<T>,
  ): Promise<T> {
    const {master} = this;
    const owner = await master.fileAllocator.getOrAssignOwner(path);
    const ref = master.projectManager.getTransportFileReference(path);

    const marker = this.startMarker({
      label: `${method}: ${ref.uid}`,
      facet: method,
      rowId: `worker ${owner.id}`,
    });

    try {
      const res: T = await factory(owner.bridge, ref);
      this.endMarker(marker);
      return res;
    } catch (err) {
      let diagnostics = getDiagnosticsFromError(err);

      if (diagnostics === undefined) {
        const info = getErrorStructure(err);

        throw createErrorFromStructure({
          ...info,
          advice: [
            ...info.advice,
            {
              type: 'log',
              category: 'info',
              message: `Error occurred while requesting ${method} for <filelink emphasis target="${ref.uid}" />`,
            },
          ],
        });
      } else {
        // We don't want to tamper with these
        throw err;
      }
    }
  }

  async requestWorkerParse(
    filename: AbsoluteFilePath,
    opts: WorkerParseOptions,
  ): Promise<Program> {
    return this.wrapRequestDiagnostic('parse', filename, (bridge, file) =>
      bridge.parseJS.call({file, opts})
    );
  }

  async requestWorkerLint(
    filename: AbsoluteFilePath,
    fix: boolean,
  ): Promise<WorkerLintResult> {
    const {cache} = this.master;
    const cacheEntry = await cache.get(filename);
    if (cacheEntry.lint !== undefined) {
      return cacheEntry.lint;
    }

    const prefetchedModuleSignatures = await this.maybePrefetchModuleSignatures(
      filename,
    );

    const res = await this.wrapRequestDiagnostic(
      'lint',
      filename,
      (bridge, file) => bridge.lint.call({file, fix, prefetchedModuleSignatures}),
    );

    await cache.update(filename, {
      lint: res,
    });

    return res;
  }

  async requestWorkerFormat(
    path: AbsoluteFilePath,
  ): Promise<undefined | WorkerFormatResult> {
    return await this.wrapRequestDiagnostic('format', path, (bridge, file) =>
      bridge.format.call({file})
    );
  }

  async requestWorkerCompile(
    path: AbsoluteFilePath,
    stage: TransformStageName,
    options?: WorkerCompilerOptions,
  ): Promise<WorkerCompileResult> {
    const {cache} = this.master;

    // Create a cache key comprised of the stage and hash of the options
    const optionsHash = options === undefined ? 'none' : crypto.createHash(
      'sha256',
    ).update(JSON.stringify(options)).digest('hex');
    const cacheKey = `${stage}:${optionsHash}`;

    // Check cache for this stage and options
    const cacheEntry = await cache.get(path);
    const cached = cacheEntry.compile[cacheKey];
    if (cached !== undefined) {
      // TODO check cacheDependencies
      return cached;
    }

    const compileRes = await this.wrapRequestDiagnostic('compile', path, (
      bridge,
      file,
    ) => {
      // We allow options to be passed in as undefined so we can compute an easy cache key
      if (options === undefined) {
        options = {};
      }

      return bridge.compileJS.call({file, stage, options});
    });

    const res = this.normalizeCompileResult({
      ...compileRes,
      cached: false,
    });

    // There's a race condition here between the file being opened and then rewritten
    await cache.update(path, (cacheEntry) =>
      ({
        compile: {
          ...cacheEntry.compile,
          [cacheKey]: {
            ...res,
            cached: true,
          },
        },
      })
    );

    return res;
  }

  async requestWorkerAnalyzeDependencies(
    path: AbsoluteFilePath,
  ): Promise<WorkerAnalyzeDependencyResult> {
    const {cache} = this.master;

    const cacheEntry = await cache.get(path);
    if (cacheEntry.analyzeDependencies !== undefined) {
      return cacheEntry.analyzeDependencies;
    }

    const res = await this.wrapRequestDiagnostic('analyzeDependencies', path, (
      bridge,
      file,
    ) => bridge.analyzeDependencies.call({file}));
    await cache.update(path, {
      analyzeDependencies: {
        ...res,
        cached: true,
      },
    });

    return {
      ...res,
      cached: false,
    };
  }

  async requestWorkerModuleSignature(
    filename: AbsoluteFilePath,
  ): Promise<ModuleSignature> {
    const {cache} = this.master;

    const cacheEntry = await cache.get(filename);
    if (cacheEntry.moduleSignature !== undefined) {
      return cacheEntry.moduleSignature;
    }

    const res = await this.wrapRequestDiagnostic('moduleSignature', filename, (
      bridge,
      file,
    ) => bridge.moduleSignatureJS.call({file}));
    await cache.update(filename, {
      moduleSignature: res,
    });
    return res;
  }

  async maybePrefetchModuleSignatures(
    filename: AbsoluteFilePath,
  ): Promise<PrefetchedModuleSignatures> {
    const {projectManager} = this.master;

    const prefetchedModuleSignatures: PrefetchedModuleSignatures = {};
    const project = await projectManager.assertProject(filename);
    if (project.config.typeCheck.enabled === false) {
      return prefetchedModuleSignatures;
    }

    // get the owner of this file

    /*const rootOwner = await fileAllocator.getOrAssignOwner(filename);
    const rootOwnerId = workerManager.getIdFromBridge(rootOwner);

    // absolute filenames to redupe export graphs
    const absoluteFilenameToGraphKey: Map<string, string> = new Map();

    // TODO exclude graphs that aren't a part of the root graph
    for (const dep of await dependencyGraph.getTransitiveDependencies(
      filename,
    )) {
      const key = `${dep.origin}:${dep.relative}`;
      const absolute = dep.absoluteMocked;

      // TODO check if we have this graph by another key and point to it if necessary
      const existingEntryKey = absoluteFilenameToGraphKey.get(absolute);
      if (existingEntryKey !== undefined) {
        invariant(existingEntryKey !== key, 'duplicate transitive dependency key %s', key);
        prefetchedModuleSignatures[key] = {
          type: 'POINTER',
          key: existingEntryKey,
        };
        continue;
      }

      // set the key so we point to the value instead of reproducing the whole graph
      absoluteFilenameToGraphKey.set(absolute, key);

      // fetch the owner so we can leave out graphs owned by the worker
      const owner = await fileAllocator.getOrAssignOwner(absolute);
      if (owner === rootOwner) {
        const project = await projectManager.assertProject(absolute);
        prefetchedModuleSignatures[key] = {
          type: 'OWNED',
          filename: absolute,
          projectId: project.id,
        };
        continue;
      }

      // get mtime so we can use it for a cache
      const mtime = this.master.memoryFs.getMtime(absolute);

      // check if this worker has it cached
      // TODO figure out some way to evict this on file deletion
      const cacheKey = `moduleSignature:${absolute}`;
      const cachedMtime = workerManager.getValueFromWorkerCache(
        rootOwnerId,
        cacheKey,
      );
      if (cachedMtime === mtime) {
        prefetchedModuleSignatures[key] = {
          type: 'USE_CACHED',
          filename: absolute,
        };
        continue;
      } else {
        workerManager.setWorkerCacheValue(rootOwnerId, cacheKey, mtime);
      }

      // calculate the graph
      const graph = await this.moduleSignature(absolute);
      prefetchedModuleSignatures[key] = {
        type: 'RESOLVED',
        graph,
      };
    }*/
    return prefetchedModuleSignatures;
  }
}
