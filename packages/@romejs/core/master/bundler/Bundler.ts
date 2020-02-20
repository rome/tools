/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Master} from '@romejs/core';
import {Reporter} from '@romejs/cli-reporter';
import {
  BundlerConfig,
  BundleResult,
  BundlerFiles,
  BundleRequestResult,
  BundlerMode,
  BundlerInMemorySourceMap,
  BundlerUnsymbolicatedLocation,
  BundlerSymbolicatedStackFrame,
  BundleResultBundle,
} from '../../common/types/bundler';
import {MasterRequest} from '@romejs/core';
import {DiagnosticsProcessor} from '@romejs/diagnostics';
import DependencyGraph from '../dependencies/DependencyGraph';
import BundleRequest from './BundleRequest';
import {AbsoluteFilePath, createUnknownFilePath} from '@romejs/path';
import {
  ManifestDefinition,
  convertManifestToJSON,
  JSONManifest,
} from '@romejs/codec-js-manifest';
import {Number1, Number0, sub, add, get1, get0} from '@romejs/ob1';
import {WorkerCompileResult} from '../../common/bridges/WorkerBridge';
import {Dict} from '@romejs/typescript-helpers';

export type BundlerEntryResoluton = {
  manifestDef: undefined | ManifestDefinition;
  resolvedEntry: AbsoluteFilePath;
};

export default class Bundler {
  constructor(req: MasterRequest, reporter: Reporter, config: BundlerConfig) {
    this.config = config;
    this.master = req.master;
    this.reporter = reporter;
    this.request = req;

    this.entries = [];

    this.graph = new DependencyGraph(req, config.resolver);
  }

  graph: DependencyGraph;
  master: Master;
  request: MasterRequest;
  reporter: Reporter;
  entries: Array<AbsoluteFilePath>;
  config: BundlerConfig;

  static createFromMasterRequest(req: MasterRequest): Bundler {
    return new Bundler(req, req.reporter, req.getBundlerConfigFromFlags());
  }

  async getResolvedEntry(
    unresolvedEntry: string,
  ): Promise<BundlerEntryResoluton> {
    const {cwd} = this.config;

    const res = await this.master.resolver.resolveEntryAssert({
      ...this.config.resolver,
      origin: cwd,
      source: createUnknownFilePath(unresolvedEntry),
    });

    const {master} = this;
    const resolvedEntry = res.path;

    // Now do the same resolver request but with a package
    const manifestRootResolved = master.resolver.resolveLocal({
      ...this.config.resolver,
      origin: cwd,
      requestedType: 'package',
      source: createUnknownFilePath(unresolvedEntry),
    });
    const manifestRoot: undefined | AbsoluteFilePath =
      manifestRootResolved.type === 'FOUND'
        ? manifestRootResolved.path
        : undefined;
    let manifestDef;
    if (manifestRoot !== undefined) {
      const def = master.memoryFs.getManifestDefinition(manifestRoot);
      if (def !== undefined) {
        manifestDef = def;
      }
    }

    return {manifestDef, resolvedEntry};
  }

  createBundleRequest(resolvedEntry: AbsoluteFilePath): BundleRequest {
    const project = this.master.projectManager.assertProjectExisting(
      resolvedEntry,
    );
    const mode: BundlerMode = project.config.bundler.mode;

    this.entries.push(resolvedEntry);
    return new BundleRequest(this, mode, resolvedEntry);
  }

  async compile(
    path: AbsoluteFilePath,
    hmr: boolean = false,
  ): Promise<WorkerCompileResult> {
    const bundleRequest = this.createBundleRequest(path);
    await bundleRequest.stepAnalyze();
    bundleRequest.diagnostics.maybeThrowDiagnosticsError();
    return await bundleRequest.compileJS(path, hmr);
  }

  // This will take multiple entry points and do some magic to make them more efficient to build in parallel
  async bundleMultiple(
    entries: Array<AbsoluteFilePath>,
  ): Promise<Map<AbsoluteFilePath, BundleResult>> {
    // Seed the dependency graph with all the entries at the same time
    const processor = new DiagnosticsProcessor({
      origins: [
        {
          category: 'Bundler',
          message: 'Analyzing dependencies for bundleMultiple',
        },
      ],
    });
    const entryUids = entries.map(entry =>
      this.master.projectManager.getUid(entry),
    );
    const analyzeProgress = this.reporter.progress({
      name: `bundler:analyze:${entryUids.join(',')}`,
    });
    analyzeProgress.setTitle('Analyzing');
    processor.setThrowAfter(100);
    await this.graph.seed(entries, processor, analyzeProgress);
    processor.maybeThrowDiagnosticsError();

    // Now actually bundle them
    const map: Map<AbsoluteFilePath, BundleResult> = new Map();

    // Could maybe do some of this in parallel?
    for (const resolvedEntry of entries) {
      map.set(resolvedEntry, await this.bundle(resolvedEntry));
    }

    return map;
  }

  async bundleManifest({resolvedEntry, manifestDef}: BundlerEntryResoluton) {
    let bundles: Array<BundleResultBundle> = [];
    const files: BundlerFiles = new Map();

    const createBundle = async (
      resolvedSegment: AbsoluteFilePath,
      prefix?: string,
    ): Promise<BundleResultBundle> => {
      const bundle = await this.bundle(resolvedSegment, prefix);
      for (const [path, content] of bundle.files) {
        files.set(path, content);
      }
      bundles = bundles.concat(bundle.bundles);
      return bundle.entry;
    };

    const entryBundle = await createBundle(resolvedEntry);

    //
    const bundleBuddyStats = this.graph.getBundleBuddyStats(this.entries);
    files.set('bundlebuddy.json', {
      kind: 'stats',
      content: JSON.stringify(bundleBuddyStats, null, '  '),
    });

    // TODO ensure that __dirname is relative to the project root

    if (manifestDef !== undefined) {
      const newManifest = await this.deriveManifest(
        manifestDef,
        entryBundle,
        createBundle,
      );

      // Add a package.json with updated values
      files.set('package.json', {
        kind: 'manifest',
        content: JSON.stringify(newManifest, undefined, '  '),
      });
    }

    return {
      files,
      bundles,
      entry: entryBundle,
    };
  }

  async deriveManifest(
    manifestDef: ManifestDefinition,
    entryBundle: BundleResultBundle,
    createBundle: (
      resolvedSegment: AbsoluteFilePath,
      prefix?: string,
    ) => Promise<BundleResultBundle>,
  ): Promise<JSONManifest> {
    const manifest = manifestDef.manifest;

    const newManifest: JSONManifest = {
      ...convertManifestToJSON(manifest),
      main: entryBundle.js.path,
    };

    // TODO inherit some manifest properties from project configs
    const project = this.master.projectManager.findProjectExisting(
      manifestDef.folder,
    );
    if (project !== undefined) {
      if (newManifest.name === undefined) {
        newManifest.name = project.config.name;
      }
    }

    // TODO remove dependencies fields, probably?

    // TODO Compile a index.d.ts

    // TODO copy manifest.files
    if (manifest.files !== undefined) {
      // TODO `manifest.files` should be turned into matching patterns
      // TODO add all files that match the globs glob
    }

    // Compile manifest.bin files
    const bin = manifest.bin;
    if (bin !== undefined) {
      const newBin: Dict<string> = {};
      newManifest.bin = newBin;

      const binConsumer = manifestDef.consumer.get('bin');
      const isBinShorthand = typeof binConsumer.asUnknown() === 'string';

      for (const [binName, relative] of manifest.bin) {
        const pointer = (isBinShorthand
          ? binConsumer
          : binConsumer.get(binName)
        ).getDiagnosticPointer('inner-value');

        const absolute = await this.master.resolver.resolveAssert(
          {
            origin: manifestDef.folder,
            source: createUnknownFilePath(relative).toExplicitRelative(),
            ...this.config.resolver,
          },
          {
            pointer,
          },
        );

        const res = await createBundle(absolute.path, `bin/${binName}`);
        newBin[binName] = res.js.path;
      }
    }

    return newManifest;
  }

  async bundle(
    resolvedEntry: AbsoluteFilePath,
    rawPrefix?: string,
  ): Promise<BundleResult> {
    const {reporter} = this;

    reporter.info(
      `Bundling <filelink emphasis target="${resolvedEntry.join()}" />`,
    );

    const req = this.createBundleRequest(resolvedEntry);
    const res = await req.bundle();

    const processor = new DiagnosticsProcessor({origins: []});
    processor.addDiagnostics(res.diagnostics);
    processor.maybeThrowDiagnosticsError();

    if (res.cached) {
      reporter.warn('Bundle was built completely from cache');
    }

    const serialMap = JSON.stringify(res.map);

    const prefix = rawPrefix === undefined ? '' : `${rawPrefix}/`;
    const jsPath = `${prefix}index.js`;
    const mapPath = `${jsPath}.map`;

    const files: BundlerFiles = new Map();
    files.set(jsPath, {
      kind: 'entry',
      content: res.content,
    });
    files.set(mapPath, {
      kind: 'sourcemap',
      content: serialMap,
    });

    for (const [relative, buffer] of res.assets) {
      files.set(relative, {
        kind: 'asset',
        content: buffer,
      });
    }

    const bundle: BundleResultBundle = {
      js: {
        path: jsPath,
        content: res.content,
      },
      sourceMap: {
        path: mapPath,
        map: res.map,
        content: serialMap,
      },
    };
    return {
      entry: bundle,
      bundles: [bundle],
      files,
    };
  }

  async symbolicate(
    resolvedEntry: AbsoluteFilePath,
    frame: BundlerUnsymbolicatedLocation,
  ): Promise<void | BundlerSymbolicatedStackFrame> {
    if (frame.lineNumber === undefined) {
      return undefined;
    }

    const {reporter} = this;

    reporter.info(
      `Bundling <filelink emphasis target="${resolvedEntry.join()}" /> for symbolication`,
    );

    const req = this.createBundleRequest(resolvedEntry);
    const res: BundleRequestResult = await req.bundle();

    // TODO: Maybe it should be an invariant that symbolication only happens
    // when bundles are fully cached.
    // FURTHER TODO: The client should send a graph revision/hash so we can
    // detect requests from stale bundles (and either support it or not - but
    // definitely not respond with the wrong stack like we would now).
    if (res.cached) {
      reporter.warn('In-memory source map was built completely from cache');
    }

    const map = res.inMemorySourceMap;

    const moduleIndex = greatestLowerBound(
      map,
      get1(frame.lineNumber),
      (target: number, candidate: BundlerInMemorySourceMap[0]) =>
        target - get1(candidate.firstLine),
    );
    if (moduleIndex == null) {
      return undefined;
    }
    const module = map[moduleIndex];

    const pos = findOriginalPos(frame, module);
    if (!pos) {
      return undefined;
    }

    return {
      file: module.path,
      lineNumber: pos.line,
      column: pos.column,
      // TODO: Also populate methodName (from function map).
    };
  }
}

type Position = {line: Number1; column: Number0};

function findOriginalPos(
  frame: BundlerUnsymbolicatedLocation,
  module: BundlerInMemorySourceMap[0],
): Position | undefined {
  if (
    module.map === undefined ||
    frame.lineNumber === undefined ||
    frame.column === undefined
  ) {
    return undefined;
  }

  const generatedPosInModule: {line: Number1; column: Number0} = {
    line: add(sub(frame.lineNumber, module.firstLine), 1),
    column: frame.column,
  };
  const mappingIndex = greatestLowerBound(
    module.map,
    generatedPosInModule,
    (target, candidate) => {
      if (target.line === candidate.generated.line) {
        return get0(sub(target.column, candidate.generated.column));
      } else {
        return get1(sub(target.line, candidate.generated.line));
      }
    },
  );
  if (mappingIndex === undefined) {
    return undefined;
  }

  const mapping = module.map[mappingIndex];
  if (!mapping.original) {
    return undefined;
  }

  return {
    line: mapping.original.line,
    column: mapping.original.column,
  };
}

// TODO: find a home for this function
function greatestLowerBound<T, U>(
  elements: ReadonlyArray<T>,
  target: U,
  comparator: (target: U, candidate: T) => number,
): number | undefined {
  let first = 0;
  let it = 0;
  let count = elements.length;
  let step;
  while (count > 0) {
    it = first;
    step = Math.floor(count / 2);
    it = it + step;
    if (comparator(target, elements[it]) >= 0) {
      first = ++it;
      count = count - (step + 1);
    } else {
      count = step;
    }
  }
  if (first > 0) {
    return first - 1;
  }
}
