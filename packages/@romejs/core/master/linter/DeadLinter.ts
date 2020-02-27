/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import DependencyNode from '../dependencies/DependencyNode';
import {SourceLocation} from '@romejs/parser-core';
import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import DependencyGraph from '../dependencies/DependencyGraph';
import {AbsoluteFilePath, createUnknownFilePath} from '@romejs/path';
import {JS_EXTENSIONS} from '../../common/fileHandlers';

export default class DeadLinter {
  constructor(req: MasterRequest, printer: DiagnosticsPrinter) {
    this.request = req;
    this.printer = printer;
  }

  request: MasterRequest;
  printer: DiagnosticsPrinter;

  async lint(): Promise<void> {
    const {request, printer} = this;
    const {args} = request.query;
    const {flags} = request.client;
    const {master, reporter} = request;

    // If there are no arguments then default to all packages in this project
    let packageRefs = args;
    if (packageRefs.length === 0) {
      const project = await request.assertClientCwdProject();
      packageRefs = Array.from(project.packages.keys());
    }

    let unusedFiles: Set<string> = new Set();

    type UnusedExportsMap = Map<string, undefined | SourceLocation>;
    const unusedExports: Map<DependencyNode, UnusedExportsMap> = new Map();

    const graph = new DependencyGraph(request, {});

    //
    const entryNodes: Set<DependencyNode> = new Set();

    //
    const manifestRoots: Array<string> = [];
    const entryFiles: Array<AbsoluteFilePath> = [];
    for (const arg of packageRefs) {
      const manifestRootQuery = await master.resolver.resolveEntry({
        origin: flags.cwd,
        source: createUnknownFilePath(arg),
        requestedType: 'package',
      });
      if (manifestRootQuery.type !== 'FOUND') {
        reporter.warn('Unable to find package ' + arg);
        continue;
      }

      const manifestRoot = manifestRootQuery.path;
      manifestRoots.push(manifestRoot.join());

      const manifest = master.memoryFs.getManifest(manifestRoot);
      if (manifest === undefined) {
        throw new Error('Unable to find manifest ' + arg);
      }

      const {main} = manifest;
      if (main === undefined) {
        continue;
      }

      // Get all files in this package
      const allFiles: Array<string> = master.memoryFs
        .glob(manifestRoot, {
          extensions: JS_EXTENSIONS,
        })
        .map(path => path.join());
      unusedFiles = new Set([...allFiles, ...unusedFiles]);

      // Build up all possible entry points
      const entries: Array<AbsoluteFilePath> = [];
      entries.push(
        await master.resolver.resolveEntryAssertPath({
          origin: manifestRoot,
          source: createUnknownFilePath(main),
        }),
      );
      // TODO other `main` fields
      // TODO tests
      for (const relative of manifest.bin.values()) {
        entries.push(
          await master.resolver.resolveEntryAssertPath({
            origin: manifestRoot,
            source: createUnknownFilePath(relative),
          }),
        );
      }

      for (const path of entries) {
        entryFiles.push(path);

        const seed = await graph.seed([path], this.printer.processor);
        if (seed !== undefined) {
          // TODO entryNodes.add(seed.node);
        }
      }
    }

    // All the nodes in the graph are reachable files
    for (const [path, node] of graph.nodes) {
      const filename = path.join();
      unusedFiles.delete(filename);

      let isInFocus = false;
      for (const root of manifestRoots) {
        if (filename.startsWith(root)) {
          isInFocus = true;
          break;
        }
      }
      if (isInFocus) {
        const map: UnusedExportsMap = new Map();
        for (const dep of node.analyze.exports) {
          if (dep.type === 'local') {
            map.set(dep.name, dep.loc);
          }
        }
        unusedExports.set(node, map);
      }
    }

    for (const node of entryNodes) {
      let transitiveDeps = node.getTransitiveDependencies();

      // Mark all imports of dependencies as used
      for (const node of transitiveDeps) {
        // Mark imports
        for (const {
          analyze,
          path,
        } of node.absoluteToAnalyzeDependency.values()) {
          // Mark it all as used!
          if (analyze.all) {
            unusedExports.delete(graph.getNode(path));

            // TODO go through all exports
          }

          // Mark all the names as used
          for (const {name, loc} of analyze.names) {
            const resolved = graph.getNode(path).resolveImport(name, loc);
            if (resolved.type === 'NOT_FOUND') {
              continue;
            }

            const names = unusedExports.get(resolved.node);
            if (names !== undefined) {
              names.delete(resolved.record.name);
            }
          }
        }

        // Mark the exports we export as used
        for (const exp of node.analyze.exports) {
          if (exp.type === 'external') {
            const names = unusedExports.get(
              node.getNodeFromRelativeDependency(exp.source),
            );
            if (names !== undefined) {
              names.delete(exp.imported);
            }
          }
        }
      }
    }

    if (manifestRoots.length > 1) {
      for (const entry of entryFiles) {
        unusedExports.delete(graph.getNode(entry));
      }
    }

    // TODO be more specific with this, should be entries
    for (const file of unusedFiles) {
      if (file.includes('__rtests__')) {
        unusedFiles.delete(file);
      }
    }

    for (const filename of unusedFiles) {
      printer.addDiagnostic({
        category: 'deadLint/file',
        filename,
        message: "This file isn't used anywhere",
      });
    }

    for (const [node, names] of unusedExports) {
      for (const [name, loc] of names) {
        printer.addDiagnostic({
          mtime: node.getMtime(),
          category: 'deadLint/export',
          filename: node.id,
          message: `The export <emphasis>${name}</emphasis> isn't imported from 'anywhere`,
          start: loc === undefined ? undefined : loc.start,
          end: loc === undefined ? undefined : loc.end,
        });
      }
    }
  }
}
