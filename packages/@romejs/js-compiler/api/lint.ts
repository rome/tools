/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  Diagnostics,
  DiagnosticSuppressions,
  DiagnosticAdvice,
} from '@romejs/diagnostics';
import {TransformRequest, LintCompilerOptionsDecision} from '../types';
import {lintTransforms} from '../transforms/lint/index';
import {Program, AnyComment, AnyNode} from '@romejs/js-ast';
import {Cache, CompilerContext} from '@romejs/js-compiler';
import {formatJS} from '@romejs/js-formatter';
import {Mappings, Mapping} from '@romejs/codec-source-map';
import {ob1Get0, Number0, Number1, ob1Get1} from '@romejs/ob1';
import stringDiff from '@romejs/string-diff';
import Path from '../lib/Path';
import {SUPPRESSION_NEXT_LINE_START} from '../suppressions';
import {commentInjector} from '../transforms/defaultHooks/index';

function findMapping(
  mappings: Mappings,
  line: Number1,
  column: Number0,
): undefined | Mapping['generated'] {
  for (const {original, generated} of mappings) {
    if (original !== undefined && original.line === line && original.column ===
        column) {
      return generated;
    }
  }

  return undefined;
}

export type LintResult = {
  diagnostics: Diagnostics;
  suppressions: DiagnosticSuppressions;
  src: string;
};

const lintCache: Cache<LintResult> = new Cache();

export type FormatRequest = TransformRequest & {format: boolean};

function getStartLine(node: AnyNode): undefined | Number1 {
  const {loc} = node;
  if (loc === undefined) {
    return undefined;
  } else {
    return loc.start.line;
  }
}

function buildSuppressionCommentValue(categories: Set<string>): string {
  return `${SUPPRESSION_NEXT_LINE_START} ${Array.from(categories).join(' ')}`;
}

function addSuppressions(context: CompilerContext, ast: Program): Program {
  const lintOptions = context.options.lint;
  if (lintOptions === undefined) {
    return ast;
  }

  const {decisionsByLine} = lintOptions;
  if (decisionsByLine === undefined) {
    return ast;
  }

  const firstNodePerLine: Map<Number1, AnyNode> = new Map();

  // Find the best node to attach comments to. This is generally the node with the largest range per line.
  context.reduce(ast, {
    name: 'suppressionVisitor',
    enter(path: Path): AnyNode {
      const {node} = path;

      const line = getStartLine(node);
      if (line === undefined) {
        return node;
      }

      let existing = firstNodePerLine.get(line);
      if (existing === undefined) {
        firstNodePerLine.set(line, node);
      }

      return node;
    },
  });

  function addComment(
    path: Path,
    node: AnyNode,
    decisions: Array<LintCompilerOptionsDecision>,
  ): AnyNode {
    // Find all suppression decisions
    const suppressionCategories: Set<string> = new Set();
    for (const {category, action} of decisions) {
      if (action === 'suppress') {
        suppressionCategories.add(category);
      }
    }
    if (suppressionCategories.size === 0) {
      return node;
    }

    // Find existing suppression comment
    let updateComment: undefined | AnyComment;
    const lastComment = context.comments.getCommentsFromIds(node.leadingComments).pop();
    if (lastComment !== undefined && lastComment.value.includes(
        SUPPRESSION_NEXT_LINE_START,
      )) {
      updateComment = lastComment;
    }

    // Insert new comment if there's none to update
    if (updateComment === undefined) {
      const id = path.callHook(commentInjector, {
        type: 'CommentLine',
        value: ` ${buildSuppressionCommentValue(suppressionCategories)}`,
      });

      return {
        ...node,
        leadingComments: [...(node.leadingComments || []), id],
      };
    }

    // Remove all categories that are already included in the suppression
    for (const category of suppressionCategories) {
      if (updateComment.value.includes(category)) {
        suppressionCategories.delete(category);
      }
    }

    // We may have eliminated them all
    if (suppressionCategories.size > 0) {
      path.callHook(commentInjector, {
        ...updateComment,
        value: updateComment.value.replace(
          SUPPRESSION_NEXT_LINE_START,
          buildSuppressionCommentValue(suppressionCategories),
        ),
      });
    }

    return node;
  }

  // Add suppressions
  return context.reduceRoot(ast, {
    name: 'suppressionVisitor',
    enter(path: Path): AnyNode {
      const {node} = path;

      const line = getStartLine(node);
      if (line === undefined) {
        return node;
      }

      const first = firstNodePerLine.get(line);
      if (first === undefined || first !== node) {
        return node;
      }

      const decisions = decisionsByLine[ob1Get1(line)];
      if (decisions === undefined) {
        return node;
      }

      return addComment(path, node, decisions);
    },
  });
}

export default async function lint(req: FormatRequest): Promise<LintResult> {
  const {ast, sourceText, project, options, format} = req;

  const query = Cache.buildQuery(req);
  const cached = lintCache.get(query);
  if (cached) {
    return cached;
  }

  let formattedMappings: undefined | Mappings;
  let formattedCode = sourceText;
  if (format) {
    // Perform autofixes
    const context = new CompilerContext({
      options,
      ast,
      project,
      frozen: false,
      origin: {
        category: 'lint',
      },
    });

    let newAst = context.reduceRoot(ast, lintTransforms);
    newAst = addSuppressions(context, newAst);

    const generator = formatJS(newAst, {
      typeAnnotations: true,
      sourceMaps: true,
      format: 'pretty',
      sourceText,
    });
    formattedCode = generator.getCode();
    formattedMappings = generator.getMappings();
  }

  // Run lints (could be with the autofixed AST)
  const context = new CompilerContext({
    ast,
    project,
    options,
    origin: {
      category: 'lint',
    },
    frozen: true,
  });
  context.reduceRoot(ast, lintTransforms);

  const diagnostics = context.diagnostics.getDiagnostics();

  // If we have a formatted source map then attempt to add a diff to all fixable diagnostics
  if (formattedMappings !== undefined) {
    for (let i = 0; i < diagnostics.length; i++) {
      const diag = diagnostics[i];
      if (!diag.fixable) {
        continue;
      }

      // Only allow diagnostics that have specifically gone through addFixableDiagnostic
      if (!context.fixableDiagnostics.has(diag)) {
        continue;
      }

      // Get the source text location
      const {start, end} = diag.location;
      if (start === undefined || end === undefined) {
        continue;
      }

      // Try to find the location in the formatted code
      const newStart = findMapping(formattedMappings, start.line, start.column);
      const newEnd = findMapping(formattedMappings, end.line, end.column);
      if (newStart === undefined || newEnd === undefined) {
        continue;
      }

      // Get the source text to compare
      const oldCode = req.sourceText.slice(ob1Get0(start.index), ob1Get0(
        end.index,
      ));
      const newCode = formattedCode.slice(ob1Get0(newStart.index), ob1Get0(
        newEnd.index,
      ));

      const advice: DiagnosticAdvice = [...(diag.description.advice || [])];

      advice.push({
        type: 'log',
        category: 'info',
        message: 'Possible fix',
      });

      advice.push({
        type: 'diff',
        diff: stringDiff(oldCode, newCode),
      });

      diagnostics[i] = {
        ...diag,
        description: {
          ...diag.description,
          advice,
        },
      };
    }
  }

  const result: LintResult = {
    suppressions: context.suppressions,
    diagnostics: [...ast.diagnostics, ...diagnostics],
    src: formattedCode,
  };
  lintCache.set(query, result);
  return result;
}
