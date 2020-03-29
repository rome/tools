/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest, Master} from '@romejs/core';
import {LINTABLE_EXTENSIONS} from '@romejs/core/common/fileHandlers';
import {
  DiagnosticLocation,
  descriptions,
  Diagnostics,
  DiagnosticsProcessor,
  DiagnosticSuppressions,
} from '@romejs/diagnostics';
import {FileReference} from '@romejs/core/common/types/files';
import {EventSubscription} from '@romejs/events';
import {MasterRequestGetFilesOptions} from '../MasterRequest';
import {AbsoluteFilePathSet, AbsoluteFilePathMap} from '@romejs/path';
import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import DependencyGraph from '../dependencies/DependencyGraph';
import {ReporterProgressOptions, ReporterProgress} from '@romejs/cli-reporter';

type LintWatchChanges = Array<{
  filename: undefined | string;
  ref: undefined | FileReference;
  diagnostics: Diagnostics;
}>;

export type LinterOptions = {
  fixLocation?: DiagnosticLocation;
  args?: Array<string>;
};

type ProgressFactory = (opts: ReporterProgressOptions) => ReporterProgress;

type WatchEvents = {
  onRunStart: () => void;
  createProgress: ProgressFactory;
  onChanges: (result: WatchResults, initial: boolean, runner: LintRunner) => void;
};

type WatchResults = {
  runner: LintRunner;
  evictedPaths: AbsoluteFilePathSet;
  changes: LintWatchChanges;
  fixedCount: number;
  totalCount: number;
};

type LintRunOptions = {
  evictedPaths: AbsoluteFilePathSet;
  processor: DiagnosticsProcessor;
};

function createDiagnosticsPrinter(
  request: MasterRequest,
  processor: DiagnosticsProcessor,
  totalCount: number,
  fixedCount: number,
): DiagnosticsPrinter {
  const printer = request.createDiagnosticsPrinter(processor);

  printer.onBeforeFooterPrint((reporter, isError) => {
    if (fixedCount > 0) {
      reporter.success(
        `<number emphasis>${fixedCount}</number> <grammarNumber plural="files" singular="file">${fixedCount}</grammarNumber> fixed`,
      );
    }

    if (isError) {
      let couldFix = false;
      let hasPendingFixes = false;

      for (const {description} of processor.getDiagnostics()) {
        if (description.category === 'lint/pendingFixes') {
          hasPendingFixes = true;
        }

        if (description.fixable) {
          couldFix = true;
        }
      }

      if (hasPendingFixes) {
        reporter.info(
          'Fixes available. Run <command>rome lint --fix</command> to apply.',
        );
      } else if (couldFix) {
        reporter.warn(
          'Autofixes are available for some of these errors when formatting is enabled. Run <command>rome config enable-category format</command> to enable.',
        );
      }
    } else {
      if (totalCount === 0) {
        reporter.warn('No files linted');
      } else {
        reporter.info(
          `<number emphasis>${totalCount}</number> <grammarNumber plural="files" singular="file">${totalCount}</grammarNumber> linted`,
        );
      }
    }
  });

  return printer;
}

class LintRunner {
  constructor(
    {
      request,
      graph,
      fix,
      events,
    }: {
      events: WatchEvents;
      request: MasterRequest;
      graph: DependencyGraph;
      fix: boolean;
    },
  ) {
    this.master = request.master;
    this.graph = graph;
    this.request = request;
    this.fix = fix;
    this.events = events;
    this.compilerDiagnosticsCache = new AbsoluteFilePathMap();
  }

  compilerDiagnosticsCache: AbsoluteFilePathMap<{
    diagnostics: Diagnostics;
    suppressions: DiagnosticSuppressions;
  }>;

  events: WatchEvents;
  master: Master;
  request: MasterRequest;
  graph: DependencyGraph;
  fix: boolean;

  async runLint(
    {
      evictedPaths: changedPaths,
      processor,
    }: LintRunOptions,
  ): Promise<{fixedCount: number}> {
    let fixedCount = 0;
    const {master} = this.request;
    const pathsByWorker = await master.fileAllocator.groupPathsByWorker(
      changedPaths,
    );

    const progress = this.events.createProgress({title: 'Linting'});
    progress.setTotal(changedPaths.size);

    await Promise.all(pathsByWorker.map(async (paths) => {
      for (const path of paths) {
        const text = `<filelink target="${path.join()}" />`;
        progress.pushText(text);

        const {
          diagnostics,
          suppressions,
          fixed,
        } = await this.request.requestWorkerLint(path, this.fix);
        processor.addSuppressions(suppressions);
        processor.addDiagnostics(diagnostics);
        this.compilerDiagnosticsCache.set(path, {suppressions, diagnostics});
        if (fixed) {
          fixedCount++;
        }

        progress.popText(text);
        progress.tick();
      }
    }));

    progress.end();

    return {fixedCount};
  }

  async runGraph(
    {
      evictedPaths,
      processor,
    }: LintRunOptions,
  ): Promise<AbsoluteFilePathSet> {
    const {graph} = this;

    const validateDependencyPaths: AbsoluteFilePathSet =
      new AbsoluteFilePathSet();

    for (const path of evictedPaths) {
      validateDependencyPaths.add(path);

      // Will be undefined if this is the first time initializing it in the graph
      const node = graph.maybeGetNode(path);
      if (node !== undefined) {
        for (const depNode of node.getDependents()) {
          validateDependencyPaths.add(depNode.path);
        }
      }
    }

    for (const path of validateDependencyPaths) {
      graph.deleteNode(path);
    }

    const progress = this.events.createProgress({title: 'Analyzing'});

    await graph.seed({
      paths: Array.from(validateDependencyPaths),
      diagnosticsProcessor: processor,
      validate: false,
      analyzeProgress: progress,
    });

    for (const path of validateDependencyPaths) {
      graph.validate(graph.getNode(path), processor);
    }

    return validateDependencyPaths;
  }

  computeChanges(
    {evictedPaths: changedPaths, processor}: LintRunOptions,
    validateDependencyPaths: AbsoluteFilePathSet,
  ): LintWatchChanges {
    const {master} = this;
    const changes: LintWatchChanges = [];

    const updatedPaths: AbsoluteFilePathSet = new AbsoluteFilePathSet([
      ...validateDependencyPaths,
    ]);

    // In case we pushed on any diagnostics that aren't from the input paths, try to resolve them
    const includedFilenamesInDiagnostics =
      master.projectManager.normalizeFilenamesToFilePaths(
        processor.getDiagnosticFilenames(),
      );
    for (const path of includedFilenamesInDiagnostics.absolutes) {
      updatedPaths.add(path);
    }

    // If we validated the diagnostics of the dependents, then we need to also push their previous compiler diagnostics
    for (const path of validateDependencyPaths) {
      if (!changedPaths.has(path)) {
        const compilerDiagnostics = this.compilerDiagnosticsCache.get(path);
        if (compilerDiagnostics !== undefined) {
          processor.addDiagnostics(compilerDiagnostics.diagnostics);
          processor.addSuppressions(compilerDiagnostics.suppressions);
        }
      }
    }

    // We can't just use getDiagnosticFilenames as we need to produce empty arrays for removed diagnostics
    for (const path of updatedPaths) {
      const ref = this.request.master.projectManager.getFileReference(path);
      const diagnostics = processor.getDiagnosticsForFile(ref.uid);

      changes.push({
        filename: ref.uid,
        ref,
        diagnostics,
      });
    }

    // We can produce diagnostics that don't actually point at a file. For LSP we will just throw these away,

    // otherwise inside of Rome we can display them.

    // These filenames may be relative or undefined
    for (const filename of includedFilenamesInDiagnostics.others) {
      changes.push({
        filename,
        ref: undefined,
        diagnostics: processor.getDiagnosticsForFile(filename),
      });
    }

    return changes;
  }

  async run(opts: LintRunOptions): Promise<WatchResults> {
    this.events.onRunStart();
    const {fixedCount} = await this.runLint(opts);
    const validateDependencyPaths = await this.runGraph(opts);
    const changes = await this.computeChanges(opts, validateDependencyPaths);
    return {
      evictedPaths: opts.evictedPaths,
      changes,
      fixedCount,
      totalCount: validateDependencyPaths.size,
      runner: this,
    };
  }
}

export default class Linter {
  constructor(req: MasterRequest, opts: LinterOptions) {
    this.request = req;
    this.options = opts;
  }

  request: MasterRequest;
  options: LinterOptions;

  getFileArgOptions(): MasterRequestGetFilesOptions {
    return {
      args: this.options.args,
      noun: 'lint',
      verb: 'linting',
      configCategory: 'lint',
      extensions: LINTABLE_EXTENSIONS,
      disabledDiagnosticCategory: 'lint/disabled',
    };
  }

  createDiagnosticsProcessor(
    evictedPaths: AbsoluteFilePathSet,
    runner: LintRunner,
  ): DiagnosticsProcessor {
    const processor = new DiagnosticsProcessor({
      origins: [
        {
          category: 'lint',
          message: 'Dispatched',
        },
      ],
    });

    processor.addAllowedUnusedSuppressionPrefix('bundler');

    // Only display files that aren't absolute, are in the changed paths, or have had previous compiler diagnostics

    // This hides errors that have been lint ignored but may have been produced by dependency analysis
    processor.addFilter({
      test: (diag) => {
        const absolute =
          this.request.master.projectManager.getFilePathFromUidOrAbsolute(
            diag.location.filename,
          );
        return absolute === undefined || evictedPaths.has(absolute) ||
        runner.compilerDiagnosticsCache.has(absolute);
      },
    });

    return processor;
  }

  async watch(events: WatchEvents): Promise<EventSubscription> {
    const graph = new DependencyGraph(
      this.request,
      this.request.getResolverOptionsFromFlags(),
    );

    const {fixLocation} = this.options;
    const shouldFix = fixLocation !== undefined;

    const runner = new LintRunner({
      events,
      request: this.request,
      fix: shouldFix,
      graph,
    });

    return this.request.watchFilesFromArgs(this.getFileArgOptions(), async (
      {paths: evictedPaths, projects},
      initial,
    ) => {
      const processor = this.createDiagnosticsProcessor(evictedPaths, runner);

      if (fixLocation !== undefined) {
        for (const project of projects) {
          if (!project.config.format.enabled) {
            processor.addDiagnostic({
              location: fixLocation,
              description: descriptions.FORMAT.DISABLED,
            });
          }
        }
      }

      const result = await runner.run({evictedPaths, processor});
      events.onChanges(result, initial, runner);
    });
  }

  async run(watch: boolean) {
    const {request} = this;
    const {reporter} = request;

    let printer: undefined | DiagnosticsPrinter;

    const diagnosticsByFilename: Map<undefined | string, Diagnostics> = new Map();

    const watchEvent = await this.watch({
      onRunStart: () => {
        if (watch) {
          reporter.clear();
        }
      },

      createProgress: () => {
        return reporter.progress();
      },

      onChanges: ({evictedPaths, changes, totalCount, fixedCount, runner}) => {
        printer = createDiagnosticsPrinter(
          request,
          this.createDiagnosticsProcessor(evictedPaths, runner),
          totalCount,
          fixedCount,
        );

        // Update our diagnostics with the changes
        for (const {filename, diagnostics} of changes) {
          if (diagnostics.length === 0) {
            diagnosticsByFilename.delete(filename);
          } else {
            diagnosticsByFilename.set(filename, diagnostics);
          }
        }

        // Print all diagnostics
        for (const diagnostics of diagnosticsByFilename.values()) {
          printer.addDiagnostics(diagnostics);
        }

        if (watch) {
          reporter.clear();
          printer.print();
          printer.footer();
        }
      },
    });

    if (watch) {
      await request.endEvent.wait();
    } else {
      watchEvent.unsubscribe();

      if (printer === undefined) {
        throw new Error('Expected a printer');
      }

      throw printer;
    }
  }
}
