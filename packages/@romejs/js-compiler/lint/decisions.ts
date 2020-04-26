/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position} from '@romejs/parser-core';
import {
  DiagnosticAdviceAction,
  DiagnosticCategory,
  DiagnosticDescriptionOptionalCategory,
  descriptions,
} from '@romejs/diagnostics';
import {LintCompilerOptionsDecision} from '../types';
import {ob1Get0, ob1Get1} from '@romejs/ob1';
import {AbsoluteFilePath} from '@romejs/path';
import {LinterOptions} from '@romejs/core/master/linter/Linter';
import {escapeSplit} from '@romejs/string-utils';

export function parseDecisionStrings(
  decisions: Array<string>,
  cwd: AbsoluteFilePath,
  unexpected: (description: DiagnosticDescriptionOptionalCategory) => void,
): LinterOptions['compilerOptionsPerFile'] {
  const compilerOptionsPerFile: LinterOptions['compilerOptionsPerFile'] = {};

  for (let i = 0; i < decisions.length; i++) {
    const segment = decisions[i];
    const parts = escapeSplit(segment, '-');

    if (parts.length < 4) {
      unexpected(descriptions.LINT_COMMAND.INVALID_DECISION_PART_COUNT(i));
    }

    const [rawAction, rawCategory, rawFilename, pos, id] = parts;

    if (
      rawAction !== 'fix' &&
      rawAction !== 'suppress' &&
      rawAction !== 'ignore'
    ) {
      unexpected(descriptions.LINT_COMMAND.INVALID_DECISION_ACTION(rawAction));
      break;
    }

    const action = rawAction;
    const category = (rawCategory as DiagnosticCategory);
    const resolvedFilename = cwd.resolve(rawFilename).join();

    let compilerOptions = compilerOptionsPerFile[resolvedFilename];
    if (compilerOptions === undefined) {
      compilerOptions = {
        decisionsByPosition: {},
      };
      compilerOptionsPerFile[resolvedFilename] = compilerOptions;
    }

    let decisionsForPosition = compilerOptions.decisionsByPosition[pos];
    if (decisionsForPosition === undefined) {
      decisionsForPosition = [];
      compilerOptions.decisionsByPosition[pos] = decisionsForPosition;
    }

    decisionsForPosition.push({
      action,
      category,
      id: id === undefined ? undefined : Number(id),
    });
  }

  return compilerOptionsPerFile;
}

export function buildLintDecisionAdviceAction(
  {
    noun,
    instruction,
    filename,
    action,
    category,
    start,
    shortcut,
    id,
  }: {
    shortcut?: string;
    noun: string;
    instruction: string;
    filename: string;
    action: LintCompilerOptionsDecision['action'];
    category: DiagnosticCategory;
    start: Position;
    id?: number;
  },
): DiagnosticAdviceAction {
  const escapedFilename = filename.replace(/-/, '\\-');

  const pos =
    action === 'suppress'
      ? `${ob1Get1(start.line)}`
      : `${ob1Get1(start.line)}:${ob1Get0(start.column)}`;

  const parts = [action, category, escapedFilename, pos];

  if (id !== undefined) {
    parts.push(String(id));
  }

  const decision = parts.join('-');

  return {
    type: 'action',
    hidden: true,
    command: 'lint',
    shortcut,
    args: [filename],
    noun,
    instruction,
    commandFlags: {
      decisions: [decision],
    },
  };
}
