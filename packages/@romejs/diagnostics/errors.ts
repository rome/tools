/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics, DiagnosticsProcessor} from '@romejs/diagnostics';
import {escapeMarkup, stripAnsi} from '@romejs/string-markup';
import {printDiagnosticsToString} from '@romejs/cli-diagnostics';
import {Diagnostic, DiagnosticSuppressions} from './types';

export class DiagnosticsError extends Error {
  constructor(
    message: string,
    diagnostics: Diagnostics,
    suppressions: DiagnosticSuppressions = [],
  ) {
    if (diagnostics.length === 0) {
      throw new Error('No diagnostics');
    }

    message += '\n';
    message += stripAnsi(printDiagnosticsToString(diagnostics));
    message += stripAnsi(diagnostics.map(
      (diag) => `- ${diag.description.message.value}`,
    ).join('\n'));

    super(escapeMarkup(message));
    this.diagnostics = diagnostics;
    this.suppressions = suppressions;
    this.name = 'DiagnosticsError';
  }

  diagnostics: Diagnostics;
  suppressions: DiagnosticSuppressions;
}

export function createSingleDiagnosticError(
  diag: Diagnostic,
  suppressions?: DiagnosticSuppressions,
): DiagnosticsError {
  return new DiagnosticsError(
    diag.description.message.value,
    [diag],
    suppressions,
  );
}

export function getDiagnosticsFromError(err: Error): undefined | Diagnostics {
  if (err instanceof DiagnosticsError) {
    const processor = new DiagnosticsProcessor({});
    processor.addSuppressions(err.suppressions);
    processor.addDiagnostics(err.diagnostics);
    return processor.getDiagnostics();
  }

  return undefined;
}
