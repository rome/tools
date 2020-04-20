/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyNode,
  Program,
  ConstSourceType,
  AnyComment,
  program,
} from '@romejs/js-ast';
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
  DiagnosticOrigin,
  DiagnosticDescription,
  DiagnosticSuppressions,
  DiagnosticCategory,
  DiagnosticsProcessor,
  DiagnosticLocation,
} from '@romejs/diagnostics';
import Record from './Record';
import {RootScope} from '../scope/Scope';
import reduce from '../methods/reduce';
import {UnknownFilePath, createUnknownFilePath} from '@romejs/path';
import {
  TransformProjectDefinition,
  LintCompilerOptionsDecision,
  TransformVisitor,
} from '../types';
import {
  extractSuppressionsFromProgram,
  matchesSuppression,
} from '../suppressions';
import CommentsConsumer from '@romejs/js-parser/CommentsConsumer';
import {get1} from '@romejs/ob1';
import {hookVisitors} from '../transforms';

export type ContextArg = {
  ast: Program;
  project: TransformProjectDefinition;
  frozen?: boolean;
  options?: CompilerOptions;
  origin?: DiagnosticOrigin;
};

type AddDiagnosticResult = {
  diagnostic: undefined | Diagnostic;
  suppressed: boolean;
};

// We only want a Context to create diagnostics that belong to itself
type ContextDiagnostic = Omit<Diagnostic, 'location' | 'description'> & {
  marker?: string;
};

export default class CompilerContext {
  constructor(arg: ContextArg) {
    const {ast, project, frozen = false, options = {}, origin} = arg;

    this.records = [];

    this.path = createUnknownFilePath(ast.filename);
    this.filename = ast.filename;

    this.frozen = frozen;
    this.mtime = ast.mtime;
    this.project = project;
    this.options = options;
    this.origin = origin;
    this.cacheDependencies = new Set();
    this.sourceType = ast.sourceType;
    this.rootScope = new RootScope(this, ast);

    this.comments = new CommentsConsumer(ast.comments);

    const {suppressions, diagnostics} = extractSuppressionsFromProgram(ast);
    this.suppressions = suppressions;
    this.diagnostics = new DiagnosticsProcessor();
    this.diagnostics.addDiagnostics(diagnostics);
    this.fixableDiagnostics = new Set();
  }

  comments: CommentsConsumer;
  sourceType: ConstSourceType;
  cacheDependencies: Set<string>;
  records: Array<Record>;
  diagnostics: DiagnosticsProcessor;
  suppressions: DiagnosticSuppressions;
  fixableDiagnostics: Set<Diagnostic>;
  frozen: boolean;
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

  getComments(ids: undefined | Array<string>): Array<AnyComment> {
    return this.comments.getCommentsFromIds(ids);
  }

  hasLocSuppression(
    loc: undefined | DiagnosticLocation,
    category: DiagnosticCategory,
  ): boolean {
    if (loc === undefined) {
      return false;
    }

    for (const suppression of this.suppressions) {
      if (suppression.category === category && matchesSuppression(
          loc,
          suppression,
        )) {
        return true;
      }
    }

    return false;
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

  reduceRoot(
    ast: Program,
    visitors: TransformVisitor | TransformVisitors,
    pathOpts?: PathOptions,
  ): Program {
    return program.assert(reduce(ast, [
      ...hookVisitors,
      ...(Array.isArray(visitors) ? visitors : [visitors]),
    ], this, pathOpts));
  }

  reduce(
    ast: AnyNode,
    visitors: TransformVisitor | TransformVisitors,
    pathOpts?: PathOptions,
  ): TransformExitResult {
    return reduce(
      ast,
      Array.isArray(visitors) ? visitors : [visitors],
      this,
      pathOpts,
    );
  }

  record(record: Record) {
    this.records.push(record);
  }

  findLintDecisions(
    loc: undefined | DiagnosticLocation,
  ): undefined | Array<LintCompilerOptionsDecision> {
    if (loc === undefined) {
      return undefined;
    }

    const {lint} = this.options;
    if (lint === undefined) {
      return undefined;
    }

    const {start} = loc;
    if (start === undefined) {
      return undefined;
    }

    const {decisionsByLine: decisions} = lint;
    if (decisions === undefined) {
      return undefined;
    }

    return decisions[get1(start.line)];
  }

  addFixableDiagnostic<Old extends AnyNode, New extends TransformExitResult>(
    nodes: {
      target?: AnyNode | Array<AnyNode>;
      old: Old;
      fixed: New | (() => New);
    },

    description: DiagnosticDescription,
    diag: ContextDiagnostic = {},
  ): TransformExitResult {
    const {old, fixed} = nodes;
    const target = nodes.target === undefined ? nodes.old : nodes.target;

    diag = {
      ...diag,
      fixable: true,
    };

    let diagnostic;
    let suppressed = false;
    if (Array.isArray(target)) {
      ({suppressed, diagnostic} = this.addNodesRangeDiagnostic(
        target,
        description,
        diag,
      ));
    } else {
      ({suppressed, diagnostic} = this.addNodeDiagnostic(
        target,
        description,
        diag,
      ));
    }

    if (suppressed) {
      return old;
    }

    if (diagnostic !== undefined) {
      this.fixableDiagnostics.add(diagnostic);
    }

    let result: TransformExitResult;
    if (typeof fixed === 'function') {
      result = fixed();
    } else {
      result = fixed;
    }

    if (typeof result !== 'symbol' && !Array.isArray(result)) {
      result = {
        ...result,
        loc: old.loc,
      };
    }

    return result;
  }

  addLocDiagnostic(
    loc: undefined | DiagnosticLocation,
    description: DiagnosticDescription,
    contextDiag: ContextDiagnostic = {},
  ): AddDiagnosticResult {
    let origins: Array<DiagnosticOrigin> = [];
    if (this.origin !== undefined) {
      origins.push(this.origin);
    }
    if (contextDiag.origins !== undefined) {
      origins = origins.concat(contextDiag.origins);
    }

    if (loc !== undefined && loc.filename !== this.filename) {
      throw new Error(
          `Trying to add a location from ${loc.filename} on a Context from ${this.path}`,
        );
    }

    const {marker, ...diag} = contextDiag;

    const diagnostic = this.diagnostics.addDiagnostic({
      ...diag,
      description,
      location: {
        marker,
        mtime: this.mtime,
        filename: this.filename,
        start: loc === undefined ? undefined : loc.start,
        end: loc === undefined ? undefined : loc.end,
        language: 'js',
        sourceType: this.sourceType,
      },
      origins,
    });

    let suppressed = this.hasLocSuppression(loc, description.category);

    // If we've been passed lint decisions then consider it suppressed unless we have been specifically
    // told to fix it
    const diagCategory = description.category;
    if (this.options.lint !== undefined && this.options.lint.decisionsByLine !==
        undefined) {
      suppressed = true;

      const decisions = this.findLintDecisions(loc);
      if (decisions !== undefined) {
        for (const {category, action} of decisions) {
          if (category === diagCategory && action === 'fix') {
            suppressed = false;
          }
        }
      }
    }

    return {
      diagnostic,
      suppressed,
    };
  }

  addNodeDiagnostic(
    node: undefined | {loc?: SourceLocation},
    description: DiagnosticDescription,
    diag: ContextDiagnostic = {},
  ): AddDiagnosticResult {
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
  ): AddDiagnosticResult {
    return this.addLocDiagnostic(
      extractSourceLocationRangeFromNodes(nodes),
      description,
      diag,
    );
  }
}
