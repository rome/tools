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
import DependencyNode from '../dependencies/DependencyNode';
import {areAnalyzeDependencyResultsEqual} from '@romejs/js-compiler';
import {markup} from '@romejs/string-markup';
import WorkerQueue from '../WorkerQueue';

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
  onChanges: (
    result: WatchResults,
    initial: boolean,
    runner: LintRunner,
  ) => void;
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

  printer.onBeforeFooterPrint(
    (reporter, isError) => {
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
    },
  );

  return printer;
}

class LintRunner {
  constructor({
    request,
    graph,
    fix,
    events,
  }: {
    events: WatchEvents;
    request: MasterRequest;
    graph: DependencyGraph;
    fix: boolean;
  }) {
    this.master = request.master;
    this.graph = graph;
    this.request = request;
    this.fix = fix;
    this.events = events;
    this.compilerDiagnosticsCache = new AbsoluteFilePathMap();
    this.hadDependencyValidationErrors = new AbsoluteFilePathMap();
  }

  hadDependencyValidationErrors: AbsoluteFilePathMap<boolean>;

  compilerDiagnosticsCache: AbsoluteFilePathMap<{
    diagnostics: Diagnostics;
    suppressions: DiagnosticSuppressions;
  }>;

  events: WatchEvents;
  master: Master;
  request: MasterRequest;
  graph: DependencyGraph;
  fix: boolean;

  async runLint({
    evictedPaths: changedPaths,
    processor,
  }: LintRunOptions): Promise<{fixedCount: number}> {
    let fixedCount = 0;
    const {master} = this.request;

    const queue: WorkerQueue<void> = new WorkerQueue(master);

    const progress = this.events.createProgress({title: 'Linting'});
    progress.setTotal(changedPaths.size);

    queue.addCallback(async (path) => {
      const text = markup`<filelink target="${path.join()}" />`;
      progress.pushText(text);

      const {
        diagnostics,
        suppressions,
        fixed,
      } = await this.request.requestWorkerLint(path, {
        fix: this.fix,
        // TODO add this option?
        allowParserDiagnostics: false,
      });
      processor.addSuppressions(suppressions);
      processor.addDiagnostics(diagnostics);
      this.compilerDiagnosticsCache.set(path, {suppressions, diagnostics});
      if (fixed) {
        fixedCount++;
      }

      progress.popText(text);
      progress.tick();
    });

    for (const path of changedPaths) {
      await queue.pushQueue(path);
    }

    await queue.spin();
    progress.end();

    return {fixedCount};
  }

  async runGraph({
    evictedPaths,
    processor,
  }: LintRunOptions): Promise<AbsoluteFilePathSet> {
    const {graph} = this;

    // Get all the current dependency nodes for the evicted files, and invalidate their nodes
    const oldEvictedNodes: AbsoluteFilePathMap<DependencyNode> = new AbsoluteFilePathMap();
    for (const path of evictedPaths) {
      const node = graph.maybeGetNode(path);
      if (node !== undefined) {
        oldEvictedNodes.set(path, node);
        graph.deleteNode(path);
      }
    }

    // Refresh only the evicted paths
    const progress = this.events.createProgress({
      title: 'Analyzing changed files',
    });
    await graph.seed({
      paths: Array.from(evictedPaths),
      diagnosticsProcessor: processor,
      validate: false,
      analyzeProgress: progress,
    });

    // Maintain a list of all the dependencies we revalidated
    const validatedDependencyPaths: AbsoluteFilePathSet = new AbsoluteFilePathSet();

    // Maintain a list of all the dependents that need to be revalidated
    const validatedDependencyPathDependents: AbsoluteFilePathSet = new AbsoluteFilePathSet();

    // Build a list of dependents to recheck
    for (const path of evictedPaths) {
      validatedDependencyPaths.add(path);

      const newNode = graph.getNode(path);

      // Get the previous node and see if the exports have actually changed
      const oldNode = oldEvictedNodes.get(path);
      const sameShape = oldNode !== undefined &&
        areAnalyzeDependencyResultsEqual(oldNode.analyze, newNode.analyze);

      for (const depNode of newNode.getDependents()) {
        // If the old node has the same shape as the new one, only revalidate the dependent if it had dependency errors
        if (sameShape &&
              this.hadDependencyValidationErrors.get(depNode.path) ===
              false) {
          continue;
        }

        validatedDependencyPaths.add(depNode.path);
        validatedDependencyPathDependents.add(depNode.path);
      }
    }

    // Revalidate dependents
    if (validatedDependencyPathDependents.size > 0) {
      const progress = this.events.createProgress({
        title: 'Analyzing dependents',
      });

      await graph.seed({
        paths: Array.from(validatedDependencyPaths),
        diagnosticsProcessor: processor,
        validate: false,
        analyzeProgress: progress,
      });
    }

    // Validate connections
    for (const path of validatedDependencyPaths) {
      const hasValidationErrors = graph.validate(graph.getNode(path), processor);
      this.hadDependencyValidationErrors.set(path, hasValidationErrors);
    }

    return validatedDependencyPaths;
  }

  computeChanges(
    {evictedPaths, processor}: LintRunOptions,
    validatedDependencyPaths: AbsoluteFilePathSet,
  ): LintWatchChanges {
    const {master} = this;
    const changes: LintWatchChanges = [];

    const updatedPaths: AbsoluteFilePathSet = new AbsoluteFilePathSet([
      ...validatedDependencyPaths,
    ]);

    const diagnosticsByFilename = processor.getDiagnosticsByFilename();

    // In case we pushed on any diagnostics that aren't from the input paths, try to resolve them
    const includedFilenamesInDiagnostics = master.projectManager.normalizeFilenamesToFilePaths(
      diagnosticsByFilename.keys(),
    );
    for (const path of includedFilenamesInDiagnostics.absolutes) {
      updatedPaths.add(path);
    }

    // If we validated the diagnostics of the dependents, then we need to also push their previous compiler diagnostics
    for (const path of validatedDependencyPaths) {
      if (!evictedPaths.has(path)) {
        const compilerDiagnostics = this.compilerDiagnosticsCache.get(path);
        if (compilerDiagnostics !== undefined) {
          processor.addSuppressions(compilerDiagnostics.suppressions);
          processor.addDiagnostics(compilerDiagnostics.diagnostics);
        }
      }
    }

    // We can't just use getDiagnosticFilenames as we need to produce empty arrays for removed diagnostics
    for (const path of updatedPaths) {
      const ref = this.request.master.projectManager.getFileReference(path);
      const diagnostics: Diagnostics = [
        ...(diagnosticsByFilename.get(ref.uid) || []),
        ...(diagnosticsByFilename.get(ref.real.join()) || []),
      ];

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
        diagnostics: diagnosticsByFilename.get(filename) || [],
      });
    }

    return changes;
  }

  async run(opts: LintRunOptions): Promise<WatchResults> {
    this.events.onRunStart();
    const {fixedCount} = await this.runLint(opts);
    const validatedDependencyPaths = await this.runGraph(opts);
    const changes = await this.computeChanges(opts, validatedDependencyPaths);
    return {
      evictedPaths: opts.evictedPaths,
      changes,
      fixedCount,
      totalCount: validatedDependencyPaths.size,
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
    processor.addFilter(
      {
        test: (diag) => {
          const absolute = this.request.master.projectManager.getFilePathFromUidOrAbsolute(
            diag.location.filename,
          );
          return absolute === undefined || evictedPaths.has(absolute) ||
            runner.compilerDiagnosticsCache.has(absolute);
        },
      },
    );

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

      createProgress: (opts) => {
        return reporter.progress(opts);
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
