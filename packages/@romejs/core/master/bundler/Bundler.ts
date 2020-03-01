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
  BundlerMode,
  BundleResultBundle,
} from '../../common/types/bundler';
import {MasterRequest} from '@romejs/core';
import {DiagnosticsProcessor} from '@romejs/diagnostics';
import DependencyGraph from '../dependencies/DependencyGraph';
import BundleRequest, {BundleOptions} from './BundleRequest';
import {AbsoluteFilePath, createUnknownFilePath} from '@romejs/path';
import {
  ManifestDefinition,
  convertManifestToJSON,
  JSONManifest,
} from '@romejs/codec-js-manifest';
import {WorkerCompileResult} from '../../common/bridges/WorkerBridge';
import {Dict} from '@romejs/typescript-helpers';
import {readFile} from '@romejs/fs';

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

  createBundleRequest(
    resolvedEntry: AbsoluteFilePath,
    options: BundleOptions,
  ): BundleRequest {
    const project = this.master.projectManager.assertProjectExisting(
      resolvedEntry,
    );
    const mode: BundlerMode = project.config.bundler.mode;

    this.entries.push(resolvedEntry);
    return new BundleRequest(this, mode, resolvedEntry, options);
  }

  async compile(path: AbsoluteFilePath): Promise<WorkerCompileResult> {
    const bundleRequest = this.createBundleRequest(path, {});
    await bundleRequest.stepAnalyze();
    bundleRequest.diagnostics.maybeThrowDiagnosticsError();
    return await bundleRequest.compileJS(path);
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
    await this.graph.seed({
      paths: entries,
      diagnosticsProcessor: processor,
      analyzeProgress,
      validate: false,
    });
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
      options: BundleOptions,
    ): Promise<BundleResultBundle> => {
      const bundle = await this.bundle(resolvedSegment, options);
      for (const [path, content] of bundle.files) {
        files.set(path, content);
      }
      bundles = bundles.concat(bundle.bundles);
      return bundle.entry;
    };

    const entryBundle = await createBundle(resolvedEntry, {});

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
        (relative, buffer) => {
          if (!files.has(relative)) {
            files.set(relative, {
              kind: 'file',
              content: buffer,
            });
          }
        },
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
      options: BundleOptions,
    ) => Promise<BundleResultBundle>,
    addFile: (relative: string, buffer: Buffer | string) => void,
  ): Promise<JSONManifest> {
    // TODO figure out some way to use bundleMultiple here
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

    // Copy manifest.files
    if (manifest.files !== undefined) {
      const paths = await this.master.memoryFs.glob(manifestDef.folder, {
        only: manifest.files,
      });

      for (const path of paths) {
        const relative = manifestDef.folder.relative(path).join();
        const buffer = await readFile(path);
        addFile(relative, buffer);
      }
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
            ...this.config.resolver,
            origin: manifestDef.folder,
            source: createUnknownFilePath(relative).toExplicitRelative(),
          },
          {
            pointer,
          },
        );

        const res = await createBundle(absolute.path, {
          prefix: `bin/${binName}`,
          interpreter: '/usr/bin/env node',
        });
        newBin[binName] = res.js.path;
      }
    }

    // TODO `{type: "module"}` will always fail since we've produced CJS bundles
    delete newManifest.type;

    return newManifest;
  }

  async bundle(
    resolvedEntry: AbsoluteFilePath,
    options: BundleOptions = {},
  ): Promise<BundleResult> {
    const {reporter} = this;

    reporter.info(
      `Bundling <filelink emphasis target="${resolvedEntry.join()}" />`,
    );

    const req = this.createBundleRequest(resolvedEntry, options);
    const res = await req.bundle();

    const processor = new DiagnosticsProcessor({origins: []});
    processor.addDiagnostics(res.diagnostics);
    processor.maybeThrowDiagnosticsError();

    if (res.cached) {
      reporter.warn('Bundle was built completely from cache');
    }

    const serialMap = JSON.stringify(res.map);

    const prefix = options.prefix === undefined ? '' : `${options.prefix}/`;
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
}
