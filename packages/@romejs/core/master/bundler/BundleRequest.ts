/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Bundler from './Bundler';
import DependencyNode from '../dependencies/DependencyNode';
import {Mappings, SourceMapGenerator} from '@romejs/codec-source-map';
import {BundleRequestResult, BundlerMode} from '../../common/types/bundler';
import {
  WorkerBundleCompileOptions,
  WorkerCompileResult,
} from '../../common/bridges/WorkerBridge';
import {DependencyOrder} from '../dependencies/DependencyOrderer';
import {
  BundleCompileResolvedImports,
  CompileResult,
  getPrefixedBundleNamespace,
} from '@romejs/js-compiler';

import {DiagnosticsProcessor, descriptions} from '@romejs/diagnostics';
import {AbsoluteFilePath} from '@romejs/path';
import {ob1Add} from '@romejs/ob1';
import {readFile} from '@romejs/fs';
import crypto = require('crypto');

import {Dict} from '@romejs/typescript-helpers';
import {Reporter} from '@romejs/cli-reporter';
import WorkerQueue from '../WorkerQueue';

export type BundleOptions = {
  prefix?: string;
  interpreter?: string;
  deferredSourceMaps?: boolean;
};

export default class BundleRequest {
  constructor({
    bundler,
    reporter,
    mode,
    resolvedEntry,
    options,
  }: {
    bundler: Bundler;
    reporter: Reporter;
    mode: BundlerMode;
    resolvedEntry: AbsoluteFilePath;
    options: BundleOptions;
  }) {
    this.options = options;
    this.reporter = reporter;
    this.bundler = bundler;
    this.cached = true;
    this.mode = mode;

    this.resolvedEntry = resolvedEntry;
    this.resolvedEntryUid = bundler.master.projectManager.getUid(resolvedEntry);

      this.diagnostics =
      bundler.request.createDiagnosticsProcessor(
        {
          origins: [
            {
              category: 'bundler',
              message: `Requested bundle for <filelink target="${this.resolvedEntryUid}" />`,
            },
          ],
        },
      );
    this.diagnostics.addAllowedUnusedSuppressionPrefix('lint');

    this.compiles = new Map();
    this.assets = new Map();

    this.sourceMap = new SourceMapGenerator({
      file: resolvedEntry.getBasename(),
    });
  }

  options: BundleOptions;
  cached: boolean;
  reporter: Reporter;
  bundler: Bundler;
  resolvedEntry: AbsoluteFilePath;
  resolvedEntryUid: string;
  diagnostics: DiagnosticsProcessor;
  assets: Map<string, Buffer>;
  compiles: Map<string, CompileResult>;
  sourceMap: SourceMapGenerator;
  mode: BundlerMode;

  async stepAnalyze(): Promise<DependencyOrder> {
    const {graph} = this.bundler;
    const {reporter} = this;

    const analyzeProgress = reporter.progress({
      name: `bundler:analyze:${this.resolvedEntryUid}`,
      title: 'Analyzing',
    });
    this.diagnostics.setThrowAfter(100);
    try {
      await graph.seed({
        paths: [this.resolvedEntry],
        diagnosticsProcessor: this.diagnostics,
        analyzeProgress,
        validate: true,
      });
    } finally {
      analyzeProgress.end();
    }

    return this.bundler.graph.getNode(this.resolvedEntry).getDependencyOrder();
  }

  async stepCompile(paths: Array<AbsoluteFilePath>) {
    const {master} = this.bundler;
    const {reporter} = this;
    this.diagnostics.setThrowAfter(undefined);

    const compilingSpinner = reporter.progress({
      name: `bundler:compile:${this.resolvedEntryUid}`,
      title: 'Compiling',
    });
    compilingSpinner.setTotal(paths.length);

    const queue: WorkerQueue<void> = new WorkerQueue(master);

    queue.addCallback(async (path) => {
      const progressText = `<filelink target="${path.join()}" />`;
      compilingSpinner.pushText(progressText);
      await this.compileJS(path);
      compilingSpinner.tick();
      compilingSpinner.popText(progressText);
    });

    for (const path of paths) {
      await queue.pushQueue(path);
    }

    await queue.spin();
    compilingSpinner.end();
  }

  async compileJS(path: AbsoluteFilePath): Promise<WorkerCompileResult> {
    const {graph} = this.bundler;

    const source = path.join();
    const mod = graph.getNode(path);

    // Build a map of relative module sources to module id
    const relativeSourcesToModuleId: Dict<string> = {};
    for (const [relative, absolute] of mod.relativeToAbsolutePath) {
      const moduleId = graph.getNode(absolute).uid;
      relativeSourcesToModuleId[relative] = moduleId;
    }

    // Diagnostics would have already been added during the initial DependencyGraph.seed

    // We're doing the work of resolving everything again, maybe we should cache it?
    const resolvedImports: BundleCompileResolvedImports = mod.resolveImports().resolved;

    let assetPath: undefined | string;
    if (mod.handler !== undefined && mod.handler.isAsset) {
      const buffer = await readFile(mod.path);

      // Asset path in the form of: BASENAME-SHA1HASH.EXTENSIONS
      const hash = crypto.createHash('sha1').update(buffer).digest('hex');
      const basename = mod.path.getExtensionlessBasename();
      const exts = mod.path.getExtensions();

      assetPath = `${basename}-${hash}${exts}`;
      this.assets.set(assetPath, buffer);
    }

    const opts: WorkerBundleCompileOptions = {
      mode: this.mode,
      moduleAll: mod.all,
      moduleId: mod.uid,
      relativeSourcesToModuleId,
      resolvedImports,
      assetPath,
    };

    const lock = await this.bundler.compileLocker.getLock(source);

    const res: WorkerCompileResult = await this.bundler.request.requestWorkerCompile(
      path,
      'compileForBundle',
      {
        bundle: opts,
      },
      {},
    );

    lock.release();

    if (!res.cached) {
      this.cached = false;
    }

    this.diagnostics.addSuppressions(res.suppressions);
    this.diagnostics.addDiagnostics(res.diagnostics);

    this.compiles.set(source, res);
    return res;
  }

  stepCombine(
    order: DependencyOrder,
    forceSourceMaps: boolean,
  ): BundleRequestResult {
    const {files} = order;
    const {inlineSourceMap} = this.bundler.config;
    const {graph} = this.bundler;
    const {resolvedEntry, mode, sourceMap} = this;

    // We allow deferring the generation of source maps. We don't do this by default as it's slower than generating them upfront
    // which is what most callers need. But for things like tests, we want to lazily compute the source map only when diagnostics
    // are present.
    let deferredSourceMaps = !forceSourceMaps &&
        this.options.deferredSourceMaps ===
        true;
    if (deferredSourceMaps) {
      sourceMap.addMaterializer(() => {
        this.stepCombine(order, true);
      });
    }

    let content: string = '';
    let lineOffset: number = 0;

    function push(str: string) {
      str += '\n';
      content += str;
      if (!deferredSourceMaps) {
        for (let cha of str) {
          if (cha === '\n') {
            lineOffset++;
          }
        }
      }
    }

    function addMappings(
      filename: string,
      sourceContent: string,
      mappings: Mappings,
    ) {
      if (deferredSourceMaps) {
        return;
      }

      sourceMap.setSourceContent(filename, sourceContent);
      for (const mapping of mappings) {
        sourceMap.addMapping({
          ...mapping,
          generated: {
            ...mapping.generated,
            line: ob1Add(lineOffset, mapping.generated.line),
          },
        });
      }
    }

    const {interpreter} = this.options;
    if (interpreter !== undefined) {
      push(`#!${interpreter}\n`);
    }

    // add on bootstrap
    if (order.firstTopAwaitLocations.length > 0) {
      if (mode === 'legacy') {
        for (const {loc, mtime} of order.firstTopAwaitLocations) {
          this.diagnostics.addDiagnostic({
            description: descriptions.BUNDLER.TOP_LEVEL_AWAIT_IN_LEGACY,
            location: {
              ...loc,
              mtime,
            },
          });
        }
      }

      push(`(async function(global) {`);
    } else {
      push(`(function(global) {`);
    }

    if (mode === 'modern') {
      push(`  'use strict';`);
    }

    // TODO prelude

    /*
    const path = createAbsoluteFilePath(loc);
    const res = await this.bundler.request.requestWorkerCompile(
      path,
      'compile',
    );
    push('(function() {');
    addMappings(
      this.bundler.master.projectManager.getUid(path),
      res.src,
      res.mappings,
    );
    push(res.code);
    push('})();');
    */
    const declaredCJS: Set<DependencyNode> = new Set();
    function declareCJS(module: DependencyNode) {
      if (mode !== 'modern' || module.type !== 'cjs' || declaredCJS.has(module)) {
        return;
      }

      declaredCJS.add(module);

      push(`  var ${getPrefixedBundleNamespace(module.uid)} = {};`);
    }

    // Add on files
    for (const source of files) {
      const module = graph.getNode(source);

      for (const path of module.getAbsoluteDependencies()) {
        declareCJS(graph.getNode(path));
      }

      const compileResult = this.compiles.get(source.join());
      if (compileResult === undefined) {
        continue;
        throw new Error('Expected compile result');
      }

      // Only do this in modern mode, the module id will already be in the wrapper otherwise
      if (mode === 'modern') {
        push(`  // ${module.uid}`);
      }

      declareCJS(module);

      addMappings(module.uid, compileResult.sourceText, compileResult.mappings);
      push(compileResult.compiledCode);
      push('');
    }

    // push on initial entry require
    const entryModule = graph.getNode(resolvedEntry);
    if (mode === 'modern') {
      push(`  return ${getPrefixedBundleNamespace(entryModule.uid)};`);
    } else {
      push(`  return Rome.requireNamespace("${entryModule.uid}");`);
    }

    // push footer
    push(
      "})(typeof global !== 'undefined' ? global : typeof window !== 'undefined' ? window : this);",
    );

    //
    if (inlineSourceMap === true) {
      const sourceMapComment = sourceMap.toComment();
      content += sourceMapComment;
    } else {
      content += `//# sourceMappingURL=${this.sourceMap.file}.map`;
    }

    return {
      diagnostics: this.diagnostics.getDiagnostics(),
      content,
      sourceMap: this.sourceMap,
      cached: this.cached,
      assets: this.assets,
    };
  }

  shouldAbort(): boolean {
    return this.diagnostics.hasDiagnostics();
  }

  abort(): BundleRequestResult {
    return {
      sourceMap: this.sourceMap,
      content: '',
      diagnostics: this.diagnostics.getDiagnostics(),
      cached: false,
      assets: this.assets,
    };
  }

  async bundle(): Promise<BundleRequestResult> {
    const order = await this.stepAnalyze();
    if (this.shouldAbort()) {
      return this.abort();
    }

    // Compile
    await this.stepCompile(order.files);
    if (this.shouldAbort()) {
      return this.abort();
    }

    // Combine
    return await this.stepCombine(order, false);
  }
}
