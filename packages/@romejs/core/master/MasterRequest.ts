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
  DiagnosticLocation,
  getDiagnosticsFromError,
  DiagnosticAdvice,
  DiagnosticsError,
  DiagnosticCategory,
  Diagnostic,
  DiagnosticsProcessor,
  Diagnostics,
  DiagnosticDescription,
  descriptions,
} from '@romejs/diagnostics';
import {
  DiagnosticsPrinterFlags,
  DiagnosticsPrinter,
} from '@romejs/cli-diagnostics';
import {
  ProjectDefinition,
  ProjectConfigCategoriesWithIgnoreAndEnabled,
} from '@romejs/project';
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
import {
  Event,
  EventSubscription,
  mergeEventSubscriptions,
} from '@romejs/events';
import {serializeCLIFlags, SerializeCLITarget} from '@romejs/cli-flags';
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
  WorkerLintOptions,
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
import {ob1Number1, ob1Number0, ob1Coerce0} from '@romejs/ob1';
import {MemoryFSGlobOptions} from './fs/MemoryFileSystem';
import {markup} from '@romejs/string-markup';
import {
  DiagnosticsProcessorOptions,
} from '@romejs/diagnostics/DiagnosticsProcessor';
import {JSONObject} from '@romejs/codec-json';
import {VCSClient} from '@romejs/vcs';

type MasterRequestOptions = {
  master: Master;
  client: MasterClient;
  query: MasterQueryRequest;
};

let requestIdCounter = 0;

type NormalizedCommandFlags = {
  flags: undefined | Dict<unknown>;
  defaultFlags: Dict<unknown>;
};

type ResolvedArg = {
  path: AbsoluteFilePath;
  location: DiagnosticLocation;
  project: ProjectDefinition;
};

type ResolvedArgs = Array<ResolvedArg>;

export type MasterRequestGetFilesOptions = Omit<MemoryFSGlobOptions,
  | 'getProjectIgnore'
  | 'getProjectEnabled'> & {
  ignoreArgumentMisses?: boolean;
  ignoreProjectIgnore?: boolean;
  disabledDiagnosticCategory?: DiagnosticCategory;
  advice?: DiagnosticAdvice;
  configCategory?: ProjectConfigCategoriesWithIgnoreAndEnabled;
  verb?: string;
  noun?: string;
  args?: Array<string>;
};

export type MasterRequestGetFilesResult = {
  projects: Set<ProjectDefinition>;
  paths: AbsoluteFilePathSet;
};

export class MasterRequestInvalid extends DiagnosticsError {
  constructor(message: string, diagnostics: Diagnostics, showHelp: boolean) {
    super(message, diagnostics);
    this.showHelp = showHelp;
  }

  showHelp: boolean;
}

function hash(val: JSONObject): string {
  return val === undefined || Object.keys(val).length === 0
    ? 'none'
    : crypto.createHash('sha256').update(JSON.stringify(val)).digest('hex');
}

export default class MasterRequest {
  constructor(opts: MasterRequestOptions) {
    this.query = opts.query;
    this.master = opts.master;
    this.bridge = opts.client.bridge;
    this.reporter = opts.client.reporter;
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
    this.markers = [];
    this.start = Date.now();

    this.normalizedCommandFlags = {
      flags: {},
      defaultFlags: {},
    };
  }

  start: number;
  client: MasterClient;
  query: MasterQueryRequest;
  master: Master;
  bridge: MasterBridge;
  reporter: Reporter;
  id: number;
  markerEvent: Event<MasterMarker, void>;
  endEvent: Event<undefined | MasterQueryResponse, void>;
  normalizedCommandFlags: NormalizedCommandFlags;
  markers: Array<MasterMarker>;

  async init() {
    if (this.query.requestFlags.collectMarkers) {
      this.markerEvent.subscribe((marker) => {
        this.markers.push(marker);
      });
    }

    await this.master.handleRequestStart(this);
  }

  teardown(
    res: undefined | MasterQueryResponse,
  ): undefined | MasterQueryResponse {
    // Output timing information
    if (this.query.requestFlags.timing) {
      const end = Date.now();
      this.reporter.info(`Request took <duration emphasis>${String(end -
        this.start)}</duration>`);
    }

    if (res !== undefined) {
      // If the query asked for no data then strip all diagnostics and data values
      if (this.query.noData) {
        if (res.type === 'SUCCESS') {
          res = {
            type: 'SUCCESS',
            hasData: res.data !== undefined,
            data: undefined,
            markers: [],
          };
        } else if (res.type === 'DIAGNOSTICS') {
          res = {
            type: 'DIAGNOSTICS',
            diagnostics: [],
          };
        } else if (res.type === 'INVALID_REQUEST') {
          res = {
            type: 'INVALID_REQUEST',
            diagnostics: [],
            showHelp: res.showHelp,
          };
        }
      }

      // Add on markers
      if (res.type === 'SUCCESS') {
        res = {
          ...res,
          markers: this.markers,
        };
      }
    }

    this.reporter.teardown();
    this.endEvent.send(res);
    this.master.handleRequestEnd(this);
    return res;
  }

  setNormalizedCommandFlags(normalized: NormalizedCommandFlags) {
    this.normalizedCommandFlags = normalized;
  }

  async assertClientCwdProject(): Promise<ProjectDefinition> {
    const location = this.getDiagnosticPointerForClientCwd();
    return this.master.projectManager.assertProject(
      this.client.flags.cwd,
      location,
    );
  }

  async getVCSClient(): Promise<VCSClient> {
    return this.master.projectManager.getVCSClient(
      await this.assertClientCwdProject(),
    );
  }

  async maybeGetVCSClient(): Promise<undefined | VCSClient> {
    return this.master.projectManager.maybeGetVCSClient(
      await this.assertClientCwdProject(),
    );
  }

  async assertCleanVSC() {
    if (this.query.requestFlags.allowDirty) {
      return;
    }

    const vsc = await this.maybeGetVCSClient();
    if (vsc === undefined) {
      return;
    }

    const files = await vsc.getUncommittedFiles();
    if (files.length > 0) {
      this.throwDiagnosticFlagError({
        description: descriptions.FLAGS.DIRTY_VSC(files),
        target: {type: 'command'},
        showHelp: false,
      });
    }
  }

  createDiagnosticsProcessor(
    opts: DiagnosticsProcessorOptions = {},
  ): DiagnosticsProcessor {
    return new DiagnosticsProcessor({
      markupOptions: this.reporter.markupOptions,
      ...opts,
    });
  }

  createDiagnosticsPrinter(
    processor: DiagnosticsProcessor = this.createDiagnosticsProcessor(),
  ): DiagnosticsPrinter {
    processor.unshiftOrigin({
      category: 'master',
      message: `${this.query.commandName} command was dispatched`,
    });

    return new DiagnosticsPrinter({
      processor,
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
        if (min === 0) {
          message = `Expected no arguments`;
        } else {
            message =
            `Expected exactly <number emphasis>${min}</number> arguments`;
        }
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
      this.throwDiagnosticFlagError({
        target: {
          type: 'arg-range',
          from: min,
          to: max,
        },
        description: descriptions.FLAGS.INCORRECT_ARG_COUNT(excessive, message),
      });
    }
  }

  throwDiagnosticFlagError({
    description,
    target = {type: 'none'},
    showHelp = true,
  }: {
    description: RequiredProps<Partial<DiagnosticDescription>, 'message'>;
    target?: SerializeCLITarget;
    showHelp?: boolean;
  }) {
    const location = this.getDiagnosticPointerFromFlags(target);

    let {category} = description;
    if (category === undefined) {
      category = target.type === 'arg' || target.type === 'arg-range'
        ? 'args/invalid'
        : 'flags/invalid';
    }

    const diag: Diagnostic = {
      description: {
        ...description,
        category,
      },
      location,
    };

    throw new MasterRequestInvalid(description.message.value, [diag], showHelp);
  }

  getDiagnosticPointerForClientCwd(): DiagnosticLocation {
    const cwd = this.client.flags.cwd.join();
    return {
      sourceText: cwd,
      start: {
        index: ob1Number0,
        line: ob1Number1,
        column: ob1Number0,
      },
      end: {
        index: ob1Coerce0(cwd.length),
        line: ob1Number1,
        column: ob1Coerce0(cwd.length),
      },
      filename: 'cwd',
    };
  }

  getDefaultFlags(): Dict<unknown> {
    return {
      ...DEFAULT_CLIENT_FLAGS,
      ...DEFAULT_CLIENT_REQUEST_FLAGS,
      ...this.normalizedCommandFlags.defaultFlags,
      clientName: this.client.flags.clientName,
    };
  }

  getFlags(): Dict<unknown> {
    return ({
      ...this.client.flags,
      ...this.query.requestFlags,
      ...this.normalizedCommandFlags.flags,
    } as Dict<unknown>);
  }

  getDiagnosticPointerFromFlags(target: SerializeCLITarget): DiagnosticLocation {
    const {query} = this;
    return serializeCLIFlags({
      programName: 'rome',
      commandName: query.commandName,
      flags: this.getFlags(),
      args: query.args,
      defaultFlags: this.getDefaultFlags(),
      incorrectCaseFlags: new Set(),
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

  async resolveFilesFromArgs(overrideArgs?: Array<string>): Promise<{
    projects: Set<ProjectDefinition>;
    resolvedArgs: ResolvedArgs;
  }> {
    const projects: Set<ProjectDefinition> = new Set();
    const rawArgs = overrideArgs === undefined ? this.query.args : overrideArgs;
    const resolvedArgs: ResolvedArgs = [];

    // If args was explicitly provided then don't assume empty args is the project root
    if (rawArgs.length === 0 && overrideArgs === undefined) {
      const location = this.getDiagnosticPointerForClientCwd();
      const project = await this.assertClientCwdProject();
      resolvedArgs.push({
        path: project.folder,
        location,
        project,
      });
      projects.add(project);
    } else {
      for (let i = 0; i < rawArgs.length; i++) {
        const arg = rawArgs[i];

        const location = this.getDiagnosticPointerFromFlags({
          type: 'arg',
          key: i,
        });

        const resolved = await this.master.resolver.resolveEntryAssert({
          origin: this.client.flags.cwd,
          source: createUnknownFilePath(arg),
          requestedType: 'folder',
        }, {
          location,
        });

        const project = this.master.projectManager.assertProjectExisting(
          resolved.path,
        );
        projects.add(project);

        resolvedArgs.push({
          project,
          path: resolved.path,
          location,
        });
      }
    }

    return {
      resolvedArgs,
      projects,
    };
  }

  async watchFilesFromArgs(
    opts: MasterRequestGetFilesOptions,
    callback: (
      result: MasterRequestGetFilesResult,
      initial: boolean,
    ) => Promise<void>,
  ): Promise<
    EventSubscription
  > {
    // Everything needs to be relative to this
    const {resolvedArgs} = await this.resolveFilesFromArgs();

    const initial = await this.getFilesFromArgs(opts);
    await callback(initial, true);

    let pendingEvictPaths: AbsoluteFilePathSet = new AbsoluteFilePathSet();
    let pendingEvictProjects: Set<ProjectDefinition> = new Set();
    let timeout: undefined | NodeJS.Timeout;
    let changesWhileRunningCallback = false;
    let runningCallback = false;

    async function flush() {
      if (pendingEvictPaths.size === 0) {
        return;
      }

      timeout = undefined;

      const result: MasterRequestGetFilesResult = {
        paths: pendingEvictPaths,
        projects: pendingEvictProjects,
      };
      pendingEvictPaths = new AbsoluteFilePathSet();
      pendingEvictProjects = new Set();

      runningCallback = true;
      await callback(result, false);
      runningCallback = false;

      if (changesWhileRunningCallback) {
        changesWhileRunningCallback = false;
        flush();
      }
    }

    const onChange = (path: AbsoluteFilePath) => {
      let matches = false;
      for (const arg of resolvedArgs) {
        if (arg.path.equal(path) || path.isRelativeTo(arg.path)) {
          matches = true;
          break;
        }
      }
      if (!matches) {
        return;
      }

      const project = this.master.projectManager.findProjectExisting(path);
      if (project !== undefined) {
        pendingEvictProjects.add(project);
      }

      pendingEvictPaths.add(path);

      // Buffer up evicted paths
      if (runningCallback) {
        changesWhileRunningCallback = true;
      } else if (timeout === undefined) {
        timeout = setTimeout(flush, 100);
      }
    };

    // Subscribe to evictions and file changes. This can cause double emits but we dedupe them with AbsoluteFilePathSet. An updated buffer dispatches a fileChangeEvent but NOT an evictEvent. An evictEvent is dispatched for all files in a project when the project config is changed but does NOT dispatch evictEvent.
    const evictSubscription = this.master.fileAllocator.evictEvent.subscribe(
      onChange,
    );
    const fileChangeEvent = this.master.fileChangeEvent.subscribe(onChange);

    return mergeEventSubscriptions([evictSubscription, fileChangeEvent]);
  }

  async getFilesFromArgs(
    opts: MasterRequestGetFilesOptions = {},
  ): Promise<MasterRequestGetFilesResult> {
    const {master} = this;
    const {configCategory, ignoreProjectIgnore} = opts;
    const {projects, resolvedArgs} = await this.resolveFilesFromArgs(opts.args);

    const extendedGlobOpts: MemoryFSGlobOptions = {...opts};

    if (configCategory !== undefined) {
        extendedGlobOpts.getProjectEnabled =
        (project) => project.config[configCategory].enabled;

      extendedGlobOpts.getProjectIgnore = (project) => ignoreProjectIgnore
        ? []
        : project.config[configCategory].ignore;
    }

    // Resolved arguments that resulted in no files
    const noArgMatches: Set<ResolvedArg> = new Set();

    // Match files
    const paths: AbsoluteFilePathSet = new AbsoluteFilePathSet();
    for (const arg of resolvedArgs) {
      const matches = master.memoryFs.glob(arg.path, extendedGlobOpts);

      if (matches.size === 0) {
        if (!opts.ignoreArgumentMisses) {
          noArgMatches.add(arg);
        }
      } else {
        for (const path of matches) {
          paths.add(path);
        }
      }
    }

    if (noArgMatches.size > 0) {
      const diagnostics: Diagnostics = [];

      for (const {path, project, location} of noArgMatches) {
        let category: DiagnosticCategory = 'args/fileNotFound';

        let advice: DiagnosticAdvice = opts.advice === undefined
          ? []
          : [...opts.advice];

        // Hint if `path` failed `globOpts.test`
        if (configCategory !== undefined &&
            !project.config[configCategory].enabled) {
          const enabledSource = master.projectManager.findProjectConfigConsumer(
            project,
            (consumer) => consumer.has(configCategory) && consumer.get(
              configCategory,
            ).get('enabled'),
          );

          const explanationPrefix = opts.verb === undefined
            ? 'Files excluded'
            : `Files excluded from ${opts.verb}`;

          if (opts.disabledDiagnosticCategory !== undefined) {
            category = opts.disabledDiagnosticCategory;
          }

          if (enabledSource.value === undefined) {
            let explanation = `${explanationPrefix} as it's not enabled for this project`;
            if (configCategory !== undefined) {
                explanation +=
                `. Run <command>rome config enable-category ${configCategory}</command> to enable it.`;
            }
            advice.push({
              type: 'log',
              category: 'info',
              message: explanation,
            });
          } else {
            advice.push(
              {
                type: 'log',
                category: 'info',
                message: `${explanationPrefix} as it's explicitly disabled in this project config`,
              },
            );

            const disabledPointer = enabledSource.value.getDiagnosticLocation(
              'value',
            );
            advice.push({
              type: 'frame',
              location: disabledPointer,
            });
          }
        }

        // Hint if all files were ignored
        if (configCategory !== undefined && !ignoreProjectIgnore) {
          const {paths: withoutIgnore} = await this.getFilesFromArgs({
            ...opts,
            ignoreProjectIgnore: true,
          });

          // Remove paths that we already successfully found
          for (const path of paths) {
            withoutIgnore.delete(path);
          }

          if (withoutIgnore.size > 0) {
            advice.push({
              type: 'log',
              category: 'info',
              message: 'The following files were ignored',
            });

            advice.push({
              type: 'list',
              list: Array.from(
                withoutIgnore,
                (path) => `<filelink target="${path.join()}" />`,
              ),
              truncate: true,
            });

            const ignoreSource = master.projectManager.findProjectConfigConsumer(
              project,
              (consumer) => consumer.has(configCategory) && consumer.get(
                configCategory,
              ).get('ignore'),
            );

            if (ignoreSource.value !== undefined) {
              const ignorePointer = ignoreSource.value.getDiagnosticLocation(
                'value',
              );

              advice.push({
                type: 'log',
                category: 'info',
                message: 'Ignore patterns were defined here',
              });

              advice.push({
                type: 'frame',
                location: ignorePointer,
              });
            }
          }
        }

        diagnostics.push({
          location: {
            ...location,
            marker: `<filelink target="${path.join()}" />`,
          },
          description: {
            ...descriptions.FLAGS.NO_FILES_FOUND(opts.noun),
            category,
            advice,
          },
        });
      }

      throw new DiagnosticsError(
          'MasterRequest.getFilesFromArgs: Some arguments did not resolve to any files',
          diagnostics,
        );
    }

    return {paths, projects};
  }

  normalizeCompileResult(res: WorkerCompileResult): WorkerCompileResult {
    const {projectManager} = this.master;

    // Turn all the cacheDependencies entries from 'absolute paths to UIDs
    return {
      ...res,
      cacheDependencies: res.cacheDependencies.map(
        (filename) => {
          return projectManager.getFileReference(
              createAbsoluteFilePath(filename),
            ).uid;
        },
      ),
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

        throw createErrorFromStructure(
            {
              ...info,
              advice: [
                ...info.advice,
                {
                  type: 'log',
                  category: 'info',
                  message: markup`Error occurred while requesting ${method} for <filelink emphasis target="${ref.uid}" />`,
                },
              ],
            },
          );
      } else {
        // We don't want to tamper with these
        throw err;
      }
    }
  }

  async requestWorkerUpdateBuffer(
    path: AbsoluteFilePath,
    content: string,
  ): Promise<void> {
    await this.wrapRequestDiagnostic(
      'updateBuffer',
      path,
      (bridge, file) => bridge.updateBuffer.call({file, content}),
    );
    this.master.fileChangeEvent.send(path);
  }

  async requestWorkerParse(
    path: AbsoluteFilePath,
    opts: WorkerParseOptions,
  ): Promise<Program> {
    return this.wrapRequestDiagnostic(
      'parse',
      path,
      (bridge, file) => bridge.parseJS.call({file, options: opts}),
    );
  }

  async requestWorkerLint(
    path: AbsoluteFilePath,
    optionsWithoutModSigs: Omit<WorkerLintOptions, 'prefetchedModuleSignatures'>,
  ): Promise<
    WorkerLintResult
  > {
    const {cache} = this.master;
    const cacheEntry = await cache.get(path);

    const cacheKey = hash(optionsWithoutModSigs);
    const cached = cacheEntry.lint[cacheKey];
    if (cached !== undefined) {
      return cached;
    }

    const prefetchedModuleSignatures = await this.maybePrefetchModuleSignatures(
      path,
    );

    const options: WorkerLintOptions = {
      ...optionsWithoutModSigs,
      prefetchedModuleSignatures,
    };

    const res = await this.wrapRequestDiagnostic(
      'lint',
      path,
      (bridge, file) => bridge.lint.call({file, options, parseOptions: {}}),
    );

    await cache.update(path, (cacheEntry) => ({
      lint: {
        ...cacheEntry.lint,
        [cacheKey]: res,
      },
    }));

    return res;
  }

  async requestWorkerFormat(
    path: AbsoluteFilePath,
    parseOptions: WorkerParseOptions,
  ): Promise<undefined | WorkerFormatResult> {
    return await this.wrapRequestDiagnostic(
      'format',
      path,
      (bridge, file) => bridge.format.call({file, parseOptions}),
    );
  }

  async requestWorkerCompile(
    path: AbsoluteFilePath,
    stage: TransformStageName,
    options: WorkerCompilerOptions,
    parseOptions: WorkerParseOptions,
  ): Promise<WorkerCompileResult> {
    const {cache} = this.master;

    // Create a cache key comprised of the stage and hash of the options
    const cacheKey = `${stage}:${hash(options)}`;

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

      return bridge.compileJS.call({file, stage, options, parseOptions});
    });

    const res = this.normalizeCompileResult({
      ...compileRes,
      cached: false,
    });

    // There's a race condition here between the file being opened and then rewritten
    await cache.update(path, (cacheEntry) => ({
      compile: {
        ...cacheEntry.compile,
        [cacheKey]: {
          ...res,
          cached: true,
        },
      },
    }));

    return res;
  }

  async requestWorkerAnalyzeDependencies(
    path: AbsoluteFilePath,
    parseOptions: WorkerParseOptions,
  ): Promise<WorkerAnalyzeDependencyResult> {
    const {cache} = this.master;

    const cacheEntry = await cache.get(path);
    if (cacheEntry.analyzeDependencies !== undefined) {
      return cacheEntry.analyzeDependencies;
    }

    const res = await this.wrapRequestDiagnostic('analyzeDependencies', path, (
      bridge,
      file,
    ) => bridge.analyzeDependencies.call({file, parseOptions}));
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
    path: AbsoluteFilePath,
    parseOptions: WorkerParseOptions,
  ): Promise<ModuleSignature> {
    const {cache} = this.master;

    const cacheEntry = await cache.get(path);
    if (cacheEntry.moduleSignature !== undefined) {
      return cacheEntry.moduleSignature;
    }

    const res = await this.wrapRequestDiagnostic('moduleSignature', path, (
      bridge,
      file,
    ) => bridge.moduleSignatureJS.call({file, parseOptions}));
    await cache.update(path, {
      moduleSignature: res,
    });
    return res;
  }

  async maybePrefetchModuleSignatures(
    path: AbsoluteFilePath,
  ): Promise<PrefetchedModuleSignatures> {
    const {projectManager} = this.master;

    const prefetchedModuleSignatures: PrefetchedModuleSignatures = {};
    const project = await projectManager.assertProject(path);
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
