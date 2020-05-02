/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment, AnyNode, Program} from '@romejs/js-ast';
import {
  DiagnosticLocation,
  DiagnosticSuppression,
  DiagnosticSuppressionType,
  DiagnosticSuppressions,
  Diagnostics,
  descriptions,
} from '@romejs/diagnostics';
import {Dict} from '@romejs/typescript-helpers';
import {Number1, ob1Inc} from '@romejs/ob1';
import CompilerContext from './lib/CompilerContext';
import Path from './lib/Path';

export const SUPPRESSION_NEXT_LINE_START = 'rome-disable-next-line';
const SUPPRESSION_CURRENT_LINE_START = 'rome-disable-line';
const SUPPRESSION_NEXT_STATEMENT_START = 'rome-disable-next-statement';

const prefixMistakes: Dict<string> = {
  disable: SUPPRESSION_NEXT_LINE_START,
  suppress: SUPPRESSION_NEXT_LINE_START,
  ignore: SUPPRESSION_NEXT_LINE_START,
  'next-statement': SUPPRESSION_NEXT_STATEMENT_START,
  'next-line': SUPPRESSION_NEXT_LINE_START,
  line: SUPPRESSION_CURRENT_LINE_START,
  'current-line': SUPPRESSION_CURRENT_LINE_START,
};
for (const key in prefixMistakes) {
  const suggestion = prefixMistakes[key];
  prefixMistakes[`ignore-${key}`] = suggestion;
  prefixMistakes[`suppress-${key}`] = suggestion;
  prefixMistakes[`disable-${key}`] = suggestion;
}

type ExtractedSuppressions = {
  suppressions: DiagnosticSuppressions;
  diagnostics: Diagnostics;
};

type NodeToComment = Map<AnyComment, AnyNode>;

// Perform a test on the comment line to see if it's a potential suppression typo
function detectPossibleMistake(
  line: string,
):
  | undefined
  | {
      prefix: string;
      suggestion: string;
    } {
  // Match the first word
  const prefixMatch = line.match(/^(?:[\s]+|)(.*?)[$\s]/);
  if (prefixMatch == null) {
    return undefined;
  }

  // Normalize the prefix, removing leading @
  const prefix = prefixMatch[1];
  let normalizedPrefix = prefix;
  normalizedPrefix = normalizedPrefix.replace(/^@/, '');

  // "Fast" check to ignore
  if (!normalizedPrefix.startsWith('rome')) {
    return undefined;
  }

  // Replace all underscores with dashes, remove leading rome and any dashes
  normalizedPrefix = normalizedPrefix.replace(/_/g, '-');
  normalizedPrefix = normalizedPrefix.replace(/^rome/, '');
  normalizedPrefix = normalizedPrefix.replace(/^-+/, '');

  for (const key in prefixMistakes) {
    const suggestion = prefixMistakes[key];
    if (normalizedPrefix.startsWith(key)) {
      return {
        suggestion,
        prefix,
      };
    }
  }

  return undefined;
}

function extractSuppressionsFromComment(
  context: CompilerContext,
  comment: AnyComment,
  getNodeToComment: () => NodeToComment,
): undefined | ExtractedSuppressions {
  const commentLocation = comment.loc;
  if (commentLocation === undefined) {
    return undefined;
  }

  const suppressedCategories: Set<string> = new Set();
  const diagnostics: Diagnostics = [];
  const suppressions: DiagnosticSuppressions = [];

  const lines = comment.value.split('\n');
  const cleanLines = lines.map((line) => {
    // Trim line and remove leading star
    return line.trim().replace(/\*[\s]/, '');
  });

  for (const line of cleanLines) {
    // Find suppression start
    let matchedPrefix: undefined | string;
    let startLine: undefined | Number1;
    let endLine: undefined | Number1;
    let suppressionType: undefined | DiagnosticSuppressionType;
    if (line.startsWith(SUPPRESSION_CURRENT_LINE_START)) {
      matchedPrefix = SUPPRESSION_CURRENT_LINE_START;
      startLine = commentLocation.start.line;
      endLine = startLine;
      suppressionType = 'current';
    }
    if (line.startsWith(SUPPRESSION_NEXT_LINE_START)) {
      matchedPrefix = SUPPRESSION_NEXT_LINE_START;
      startLine = ob1Inc(commentLocation.start.line);
      endLine = startLine;
      suppressionType = 'next';
    }
    if (line.startsWith(SUPPRESSION_NEXT_STATEMENT_START)) {
      matchedPrefix = SUPPRESSION_NEXT_STATEMENT_START;

      const nodeToComment = getNodeToComment();
      const nextNode = nodeToComment.get(comment);
      if (nextNode === undefined || nextNode.loc === undefined) {
        diagnostics.push({
          description: descriptions.SUPPRESSIONS.NEXT_STATEMENT_NOT_FOUND,
          location: commentLocation,
        });
        continue;
      } else {
        suppressionType = 'range';
        startLine = nextNode.loc.start.line;
        endLine = nextNode.loc.end.line;
      }
    }

    if (
      matchedPrefix === undefined ||
      endLine === undefined ||
      startLine === undefined ||
      suppressionType === undefined
    ) {
      const mistake = detectPossibleMistake(line);
      if (mistake !== undefined) {
        diagnostics.push({
          description: descriptions.SUPPRESSIONS.PREFIX_TYPO(
            mistake.prefix,
            mistake.suggestion,
          ),
          location: commentLocation,
        });
      }
      continue;
    }

    const lineWithoutPrefix = line.slice(matchedPrefix.length);
    if (lineWithoutPrefix[0] !== ' ') {
      diagnostics.push({
        description: descriptions.SUPPRESSIONS.MISSING_SPACE,
        location: commentLocation,
      });
      continue;
    }

    const categories = lineWithoutPrefix.trim().split(' ');
    const cleanCategories = categories.map((category) => category.trim());

    for (let category of cleanCategories) {
      if (category === '') {
        continue;
      }

      // If a category ends with a colon then all the things that follow it are an explanation
      let shouldBreak = false;
      if (category[category.length - 1] === ':') {
        shouldBreak = true;
        category = category.slice(-1);
      }

      if (suppressedCategories.has(category)) {
        diagnostics.push({
          description: descriptions.SUPPRESSIONS.DUPLICATE(category),
          location: commentLocation,
        });
      } else {
        suppressedCategories.add(category);

        suppressions.push({
          suppressionType,
          filename: context.filename,
          category,
          commentLocation,
          startLine,
          endLine,
        });
      }

      if (shouldBreak) {
        break;
      }
    }
  }

  if (suppressions.length === 0 && diagnostics.length === 0) {
    return undefined;
  } else {
    return {diagnostics, suppressions};
  }
}

export function extractSuppressionsFromProgram(
  context: CompilerContext,
  ast: Program,
): ExtractedSuppressions {
  const {comments} = ast;

  let diagnostics: Diagnostics = [];
  let suppressions: DiagnosticSuppressions = [];

  let cachedNodeToComment: undefined | NodeToComment;

  // Lazily instantiate NodeToComment only when it's needed
  function getNodeToComment(): NodeToComment {
    if (cachedNodeToComment !== undefined) {
      return cachedNodeToComment;
    }

    const nodeToComment: NodeToComment = new Map();
    cachedNodeToComment = nodeToComment;

    context.reduce(
      ast,
      {
        name: 'extractSuppressions',
        enter(path: Path): AnyNode {
          const {node} = path;

          for (const comment of context.comments.getCommentsFromIds(
            node.leadingComments,
          )) {
            nodeToComment.set(comment, node);
          }

          return node;
        },
      },
      {
        noScopeCreation: true,
      },
    );

    return nodeToComment;
  }

  for (const comment of comments) {
    const result = extractSuppressionsFromComment(
      context,
      comment,
      getNodeToComment,
    );
    if (result !== undefined) {
      diagnostics = diagnostics.concat(result.diagnostics);
      suppressions = suppressions.concat(result.suppressions);
    }
  }

  return {suppressions, diagnostics};
}

export function matchesSuppression(
  {filename, start, end}: DiagnosticLocation,
  suppression: DiagnosticSuppression,
): boolean {
  return (
    filename === suppression.filename &&
    start !== undefined &&
    end !== undefined &&
    start.line >= suppression.startLine &&
    end.line <= suppression.endLine
  );
}
