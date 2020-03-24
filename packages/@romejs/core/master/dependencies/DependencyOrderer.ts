/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SourceLocation} from '@romejs/parser-core';
import DependencyGraph from './DependencyGraph';
import DependencyNode from './DependencyNode';
import {AnalyzeDependencyImportUsageItem} from '@romejs/core';
import {Diagnostics, descriptions} from '@romejs/diagnostics';
import {AbsoluteFilePath} from '@romejs/path';

type FirstTopAwaitLocations = Array<{
  mtime: number;
  loc: SourceLocation;
}>;

export type DependencyOrder = {
  diagnostics: Diagnostics;
  firstTopAwaitLocations: FirstTopAwaitLocations;
  files: Array<AbsoluteFilePath>;
};

export default class DependencyOrderer {
  constructor(graph: DependencyGraph) {
    this.graph = graph;
    this.orderedNodes = new Set();
    this.visitedNodes = new Set();
    this.possibleCyclePaths = new Map();
    this.diagnostics = [];
    this.firstTopAwaitLocations = [];
  }

  firstTopAwaitLocations: FirstTopAwaitLocations;
  orderedNodes: Set<DependencyNode>;
  visitedNodes: Set<DependencyNode>;
  possibleCyclePaths: Map<DependencyNode, Array<string>>;
  diagnostics: Diagnostics;
  graph: DependencyGraph;

  handleAlreadyVisitedFile(
    node: DependencyNode,
    path: AbsoluteFilePath,
    ancestry: Array<string>,
  ) {
    const filename = path.join();

    // We flag a possible cycle when a dependency has yet to have it's own transitive dependencies resolve but it ends up going back to itself
    const isPossibleCycle = this.orderedNodes.has(node) === false &&
      ancestry.includes(filename);
    if (isPossibleCycle) {
      const ourCyclePath = ancestry.concat([filename]);
      const existingCycle = this.possibleCyclePaths.get(node);

      // We want to get the shortest cycle path since it's likely the most easily resolved
      const isShortestCycle = existingCycle === undefined ||
      existingCycle.length > ourCyclePath.length;
      if (isShortestCycle) {
        this.possibleCyclePaths.set(node, ourCyclePath);
      }
    }
  }

  addFile(path: AbsoluteFilePath, ancestry: Array<string>) {
    const node = this.graph.getNode(path);

    if (this.visitedNodes.has(node)) {
      this.handleAlreadyVisitedFile(node, path, ancestry);
      return undefined;
    }

    this.visitedNodes.add(node);

    const {firstTopAwaitLocation} = node.analyze;
    if (firstTopAwaitLocation !== undefined) {
      this.firstTopAwaitLocations.push({
        mtime: node.getMtime(),
        loc: firstTopAwaitLocation,
      });
    }

    const subAncestry = ancestry.concat([path.join()]);
    for (const depPath of node.getAbsoluteDependencies()) {
      const dep = node.getDependencyInfoFromAbsolute(depPath).analyze;
      if (dep.kind === 'value') {
        this.addFile(depPath, subAncestry);
      }
    }

    this.orderedNodes.add(node);
  }

  // We detect cycles by determining if there were any references to imports at the top level that

  // are for a module that will be initialized before
  detectCycles() {
    const flatOrder = Array.from(this.orderedNodes);

    for (let i = 0; i < flatOrder.length; i++) {
      const node = flatOrder[i];

      for (const imp of node.analyze.importFirstUsage) {
        const resolved =
          node.getNodeFromRelativeDependency(imp.source).resolveImport(
            imp.imported,
            imp.loc,
          );
        if (resolved.type !== 'FOUND') {
          continue;
        }

        // Hoisted exports will always be accessible
        if (resolved.record.valueType === 'function') {
          continue;
        }

        const dep = resolved.node;

        const isBefore = flatOrder.indexOf(dep) > i;
        if (isBefore) {
          this.flagCycle(node, dep, imp);
        }
      }
    }
  }

  flagCycle(
    node: DependencyNode,
    dep: DependencyNode,
    imp: AnalyzeDependencyImportUsageItem,
  ) {
    const path = this.possibleCyclePaths.get(dep);
    if (!path) {
      // idk??
      return undefined;
    }

    const target = path[path.length - 1];
    const culprit = String(path.find(
      (value, index) => path[index - 1] === target,
    ));

    this.diagnostics.push({
      description: descriptions.BUNDLER.DETECTED_CYCLE(
        imp.local,
        target,
        culprit,
        path,
      ),
      location: {
        filename: node.path.join(),
        mtime: node.getMtime(),
        start: imp.loc === undefined ? undefined : imp.loc.start,
        end: imp.loc === undefined ? undefined : imp.loc.end,
      },
    });
  }

  order(path: AbsoluteFilePath): DependencyOrder {
    this.addFile(path, []);
    // TODO only enable when bundlerMode === 'modern'

    // this.detectCycles();
    return {
      firstTopAwaitLocations: this.firstTopAwaitLocations,
      diagnostics: this.diagnostics,
      files: Array.from(this.orderedNodes, (node) => node.path),
    };
  }
}
