/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  AnyComment,
  AnyNode,
  ConstSourceType,
  Program,
  program,
} from '@romejs/js-ast';
import {
  SourceLocation,
  extractSourceLocationRangeFromNodes,
} from '@romejs/parser-core';
import {
  CompilerOptions,
  PathOptions,
  TransformExitResult,
  TransformVisitors,
  Transforms,
} from '@romejs/js-compiler';
import {
  Diagnostic,
  DiagnosticCategory,
  DiagnosticDescription,
  DiagnosticLocation,
  DiagnosticOrigin,
  DiagnosticSuppressions,
  DiagnosticsProcessor,
} from '@romejs/diagnostics';
import Record from './Record';
import {RootScope} from '../scope/Scope';
import reduce from '../methods/reduce';
import {UnknownFilePath, createUnknownFilePath} from '@romejs/path';
import {
  LintCompilerOptions,
  LintCompilerOptionsDecision,
  TransformProjectDefinition,
  TransformVisitor,
} from '../types';
import {
  extractSuppressionsFromProgram,
  matchesSuppression,
} from '../suppressions';
import CommentsConsumer from '@romejs/js-parser/CommentsConsumer';
import {ob1Get0, ob1Get1} from '@romejs/ob1';
import {hookVisitors} from '../transforms';
import stringDiff from '@romejs/string-diff';
import {formatJS} from '@romejs/js-formatter';
import {REDUCE_REMOVE} from '../constants';
import {FileReference} from '@romejs/core';
import {DEFAULT_PROJECT_CONFIG} from '@romejs/project';
import {buildLintDecisionAdviceAction} from '../lint/decisions';

export type ContextArg = {
  ast: Program;
  ref?: FileReference;
  sourceText?: string;
  project?: TransformProjectDefinition;
  frozen?: boolean;
  options?: CompilerOptions;
  origin?: DiagnosticOrigin;
};

type AddDiagnosticResult = {
  loc: undefined | DiagnosticLocation;
  diagnostic: undefined | Diagnostic;
  suppressed: boolean;
};

// We only want a Context to create diagnostics that belong to itself
type ContextDiagnostic = Omit<Diagnostic, 'location' | 'description'> & {
  marker?: string;
};

type DiagnosticTarget =
  | undefined
  | {
      loc?: SourceLocation;
    }
  | Array<{
      loc?: SourceLocation;
    }>;

function getFormattedCodeFromExitResult(result: TransformExitResult): string {
  if (Array.isArray(result)) {
    // TODO?
    return '';
  } else if (result === REDUCE_REMOVE) {
    return '';
  } else {
    return formatJS(result).code;
  }
}

export default class CompilerContext {
  constructor(arg: ContextArg) {
    const {
      ast,
      origin,
      ref,
      frozen = false,
      options = {},
      sourceText = '',
      project = {
        folder: undefined,
        config: DEFAULT_PROJECT_CONFIG,
      },
    } = arg;

    this.records = [];

    this.path = createUnknownFilePath(ast.filename);
    this.filename = ast.filename;
    this.sourceText = sourceText;
    this.displayFilename =
      ref === undefined ? ast.filename : ref.relative.join();
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
  }

  displayFilename: string;
  filename: string;
  path: UnknownFilePath;
  project: TransformProjectDefinition;
  sourceText: string;

  comments: CommentsConsumer;
  sourceType: ConstSourceType;
  cacheDependencies: Set<string>;
  records: Array<Record>;
  diagnostics: DiagnosticsProcessor;
  suppressions: DiagnosticSuppressions;
  frozen: boolean;
  rootScope: undefined | RootScope;
  mtime: undefined | number;
  origin: undefined | DiagnosticOrigin;
  options: CompilerOptions;

  async normalizeTransforms(transforms: Transforms): Promise<TransformVisitors> {
    return Promise.all(
      transforms.map(async (visitor) => {
        if (typeof visitor === 'function') {
          return await visitor(this);
        } else {
          return visitor;
        }
      }),
    );
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
      if (
        suppression.category === category &&
        matchesSuppression(loc, suppression)
      ) {
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
    return program.assert(
      reduce(
        ast,
        [...hookVisitors, ...(Array.isArray(visitors) ? visitors : [visitors])],
        this,
        pathOpts,
      ),
    );
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

  hasLintDecisions(): boolean {
    return this.getLintDecisions() !== undefined;
  }

  getLintDecisions(): LintCompilerOptions['decisionsByPosition'] {
    const {lint} = this.options;
    if (lint === undefined) {
      return undefined;
    }

    return lint.decisionsByPosition;
  }

  findLintDecisions(
    loc: undefined | DiagnosticLocation,
  ): Array<LintCompilerOptionsDecision> {
    if (loc === undefined) {
      return [];
    }

    const {start} = loc;
    if (start === undefined) {
      return [];
    }

    const decisionsByPosition = this.getLintDecisions();
    if (decisionsByPosition === undefined) {
      return [];
    }

    // Keep in update with packages/@romejs/core/client/commands/lint.ts
    const pos = `${ob1Get1(start.line)}:${ob1Get0(start.column)}`;
    return decisionsByPosition[pos] || [];
  }

  addFixableDiagnostic<Old extends AnyNode, New extends TransformExitResult>(
    nodes: {
      target?: AnyNode | Array<AnyNode>;
      old: Old;
      fixed?: New;
      suggestions?: Array<{
        description: string;
        title: string;
        fixed: New;
      }>;
    },
    description: DiagnosticDescription,
    diag: ContextDiagnostic = {},
  ): TransformExitResult {
    const {old, fixed: defaultFixed, suggestions} = nodes;
    const target = nodes.target === undefined ? nodes.old : nodes.target;

    const {category} = description;
    const advice = [...(description.advice || [])];
    const loc = this.getLoc(target);
    const oldCode =
      loc === undefined
        ? ''
        : this.sourceText.slice(
            ob1Get0(loc.start.index),
            ob1Get0(loc.end.index),
          );

    let fixed: undefined | New = defaultFixed;

    // Add recommended fix
    if (defaultFixed !== undefined) {
      advice.push({
        type: 'log',
        category: 'info',
        text: 'Recommended fix',
      });

      advice.push({
        type: 'diff',
        diff: stringDiff(oldCode, getFormattedCodeFromExitResult(defaultFixed)),
      });
      if (loc === undefined) {
        advice.push({
          type: 'log',
          category: 'error',
          text: 'Unable to find target location',
        });
      } else {
        advice.push(
          buildLintDecisionAdviceAction({
            noun: 'Apply fix',
            instruction: 'To apply this fix run',
            filename: this.displayFilename,
            action: 'fix',
            category,
            start: loc.start,
          }),
        );
      }
    }

    if (suggestions !== undefined) {
      // If we have lint decisions then find the fix that corresponds with this suggestion
      if (this.hasLintDecisions()) {
        const decisions = this.findLintDecisions(loc);
        for (const decision of decisions) {
          if (
            decision.category === category &&
            decision.action === 'fix' &&
            decision.id !== undefined
          ) {
            const suggestion = suggestions[decision.id];
            if (suggestion !== undefined) {
              fixed = suggestion.fixed;
            }
          }
        }
      }

      // Add advice suggestions
      let index = 0;
      for (const suggestion of suggestions) {
        const num = index + 1;

        const titlePrefix =
          suggestions.length === 1 ? 'Suggested fix' : `Suggested fix #${num}`;
        advice.push({
          type: 'log',
          category: 'none',
          text: `<emphasis>${titlePrefix}:</emphasis> ${suggestion.title}`,
        });

        advice.push({
          type: 'diff',
          diff: stringDiff(
            oldCode,
            getFormattedCodeFromExitResult(suggestion.fixed),
          ),
        });

        advice.push({
          type: 'log',
          category: 'info',
          text: suggestion.description,
        });

        if (loc === undefined) {
          advice.push({
            type: 'log',
            category: 'error',
            text: 'Unable to find target location',
          });
        } else {
          advice.push(
            buildLintDecisionAdviceAction({
              noun: suggestions.length === 1
                ? 'Apply suggested fix'
                : `Apply suggested fix "${suggestion.title}"`,
              shortcut: String(num),
              instruction: 'To apply this fix run',
              filename: this.displayFilename,
              action: 'fix',
              category,
              start: loc.start,
              id: index,
            }),
          );
        }

        index++;
      }
    }

    const {suppressed} = this.addLocDiagnostic(
      loc,
      {
        ...description,
        advice,
      },
      {
        ...diag,
        fixable: true,
      },
    );

    if (suppressed || fixed === undefined) {
      return old;
    }

    return fixed;
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

    const {category, advice = []} = description;
    if (loc !== undefined && loc.start !== undefined) {
      advice.push(
        buildLintDecisionAdviceAction({
          noun: 'Add suppression comment',
          shortcut: 's',
          instruction: 'To suppress this error run',
          filename: this.displayFilename,
          action: 'suppress',
          category,
          start: loc.start,
        }),
      );
    }

    const {marker, ...diag} = contextDiag;
    const diagnostic = this.diagnostics.addDiagnostic({
      ...diag,
      description: {
        ...description,
        advice,
      },
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

    // If we've been passed lint decisions then consider it suppressed unless we have been specifically told to fix it
    const diagCategory = description.category;
    if (this.hasLintDecisions()) {
      suppressed = true;

      const decisions = this.findLintDecisions(loc);
      for (const {category, action} of decisions) {
        if (category === diagCategory && action === 'fix') {
          suppressed = false;
        }
      }
    }

    return {
      loc,
      diagnostic,
      suppressed,
    };
  }

  getLoc(node: DiagnosticTarget): undefined | SourceLocation {
    if (node === undefined) {
      return undefined;
    }

    if (Array.isArray(node)) {
      return extractSourceLocationRangeFromNodes(node);
    } else {
      return node.loc;
    }
  }

  addNodeDiagnostic(
    node: DiagnosticTarget,
    description: DiagnosticDescription,
    diag: ContextDiagnostic = {},
  ): AddDiagnosticResult {
    return this.addLocDiagnostic(this.getLoc(node), description, diag);
  }
}
