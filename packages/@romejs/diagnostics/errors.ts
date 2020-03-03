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
import {PartialDiagnostic, DiagnosticFilterJSON} from './types';

export class DiagnosticsError extends Error {
  constructor(
    message: string,
    diagnostics: PartialDiagnostics,
    filters: Array<DiagnosticFilterJSON> = [],
  ) {
    if (diagnostics.length === 0) {
      throw new Error('No diagnostics');
    }

    message += '\n';
    message += stripAnsi(
      printDiagnosticsToString(diagnostics, {
        origins: [],
      }),
    );
    message += stripAnsi(
      diagnostics.map(diag => `- ${diag.message}`).join('\n'),
    );

    super(escapeMarkup(message));
    this.diagnostics = diagnostics;
    this.filters = filters;
    this.name = 'DiagnosticsError';
  }

  diagnostics: PartialDiagnostics;
  filters: Array<DiagnosticFilterJSON>;
}

export function createSingleDiagnosticError(
  diag: PartialDiagnostic,
  filters?: Array<DiagnosticFilterJSON>,
): DiagnosticsError {
  return new DiagnosticsError(diag.message, [diag], filters);
}

export function getDiagnosticsFromError(
  err: Error,
): undefined | PartialDiagnostics {
  if (err instanceof DiagnosticsError) {
    const processor = new DiagnosticsProcessor({
      filters: err.filters,
    });
    processor.addDiagnostics(err.diagnostics);
    return processor.getPartialDiagnostics();
  }

  return undefined;
}
