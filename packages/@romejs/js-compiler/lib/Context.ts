/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, Program, ConstSourceType} from '@romejs/js-ast';
import {
  SourceLocation,
  extractSourceLocationRangeFromNodes,
} from '@romejs/parser-core';
import {
  PathOptions,
  TransformExitResult,
  TransformVisitors,
  Transforms,
  CompilerOptions,
} from '@romejs/js-compiler';
import {
  Diagnostic,
  Diagnostics,
  DiagnosticOrigin,
  DiagnosticDescription,
} from '@romejs/diagnostics';
import Record from './Record';
import {RootScope} from '../scope/Scope';
import reduce from '../methods/reduce';
import {UnknownFilePath, createUnknownFilePath} from '@romejs/path';
import {TransformProjectDefinition} from '../types';

export type ContextArg = {
  ast: Program;
  project: TransformProjectDefinition;
  options?: CompilerOptions;
  origin?: DiagnosticOrigin;
};

// We only want a Context to create diagnostics that belong to itself
type ContextDiagnostic = Omit<Diagnostic, 'location' | 'description'>;

export default class Context {
  constructor(arg: ContextArg) {
    const {ast, project, options = {}, origin} = arg;

    this.diagnostics = [];
    this.records = [];

    this.path = createUnknownFilePath(ast.filename);
    this.filename = ast.filename;

    this.mtime = ast.mtime;
    this.project = project;
    this.options = options;
    this.origin = origin;
    this.cacheDependencies = new Set();
    this.sourceType = ast.sourceType;
    this.rootScope = new RootScope(this, ast);
  }

  sourceType: ConstSourceType;
  cacheDependencies: Set<string>;
  records: Array<Record>;
  diagnostics: Diagnostics;

  rootScope: undefined | RootScope;

  filename: undefined | string;
  path: undefined | UnknownFilePath;
  mtime: undefined | number;

  project: TransformProjectDefinition;
  origin: undefined | DiagnosticOrigin;
  options: CompilerOptions;

  async normalizeTransforms(transforms: Transforms): Promise<TransformVisitors> {
    return Promise.all(transforms.map(async (visitor) => {
      if (typeof visitor === 'function') {
        return await visitor(this);
      } else {
        return visitor;
      }
    }));
  }

  getRootScope(): RootScope {
    const {rootScope} = this;
    if (rootScope === undefined) {
      throw new Error('Expected root scope');
    }
    return rootScope;
  }

  getCacheDependencies(): Array<string> {
    return Array.from(this.cacheDependencies);
  }

  addCacheDependency(filename: string) {
    this.cacheDependencies.add(filename);
  }

  reduce(
    ast: AnyNode,
    visitors: TransformVisitors,
    pathOpts?: PathOptions,
  ): TransformExitResult {
    return reduce(ast, visitors, this, pathOpts);
  }

  record(record: Record) {
    this.records.push(record);
  }

  addDiagnostics(diagnostics: Diagnostics) {
    this.diagnostics = [...this.diagnostics, ...diagnostics];
  }

  addLocDiagnostic(
    loc: undefined | SourceLocation,
    description: DiagnosticDescription,
    diag: ContextDiagnostic = {},
  ) {
    let origins: Array<DiagnosticOrigin> = [];
    if (this.origin !== undefined) {
      origins.push(this.origin);
    }
    if (diag.origins !== undefined) {
      origins = origins.concat(diag.origins);
    }

    if (loc !== undefined && loc.filename !== this.filename) {
      throw new Error(
        `Trying to add a location from ${loc.filename} on a Context from ${this.path}`,
      );
    }

    this.diagnostics.push({
      ...diag,
      description,
      location: {
        mtime: this.mtime,
        filename: this.filename,
        start: loc === undefined ? undefined : loc.start,
        end: loc === undefined ? undefined : loc.end,
        language: 'js',
        sourceType: this.sourceType,
      },
      origins,
    });
  }

  addNodeDiagnostic(
    node: undefined | {loc?: SourceLocation},
    description: DiagnosticDescription,
    diag: ContextDiagnostic = {},
  ) {
    return this.addLocDiagnostic(
      node === undefined ? undefined : node.loc,
      description,
      diag,
    );
  }

  addNodesRangeDiagnostic(
    nodes: Array<{loc?: SourceLocation}>,
    description: DiagnosticDescription,
    diag: ContextDiagnostic = {},
  ) {
    return this.addLocDiagnostic(
      extractSourceLocationRangeFromNodes(nodes),
      description,
      diag,
    );
  }
}
