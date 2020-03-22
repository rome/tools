/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialDiagnostics, DiagnosticsProcessor} from '@romejs/diagnostics';
import {escapeMarkup} from '@romejs/string-markup';
import {stripAnsi} from '@romejs/string-ansi';
import {printDiagnosticsToString} from '@romejs/cli-diagnostics';
import {PartialDiagnostic, DiagnosticSuppressions} from './types';

export class DiagnosticsError extends Error {
  constructor(
    message: string,
    diagnostics: PartialDiagnostics,
    suppressions: DiagnosticSuppressions = [],
  ) {
    if (diagnostics.length === 0) {
      throw new Error('No diagnostics');
    }

    message += '\n';
    message += stripAnsi(printDiagnosticsToString(diagnostics, {
      origins: [],
    }));
    message += stripAnsi(diagnostics.map((diag) => `- ${diag.message}`).join(
      '\n',
    ));

    super(escapeMarkup(message));
    this.diagnostics = diagnostics;
    this.suppressions = suppressions;
    this.name = 'DiagnosticsError';
  }

  diagnostics: PartialDiagnostics;
  suppressions: DiagnosticSuppressions;
}

export function createSingleDiagnosticError(
  diag: PartialDiagnostic,
  suppressions?: DiagnosticSuppressions,
): DiagnosticsError {
  return new DiagnosticsError(diag.message, [diag], suppressions);
}

export function getDiagnosticsFromError(
  err: Error,
): undefined | PartialDiagnostics {
  if (err instanceof DiagnosticsError) {
    const processor = new DiagnosticsProcessor({});
    processor.addSuppressions(err.suppressions);
    processor.addDiagnostics(err.diagnostics);
    return processor.getPartialDiagnostics();
  }

  return undefined;
}
