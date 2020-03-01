/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialDiagnostics} from '@romejs/diagnostics';
import {escapeMarkup} from '@romejs/string-markup';
import {stripAnsi} from '@romejs/string-ansi';
import {printDiagnosticsToString} from '@romejs/cli-diagnostics';
import {PartialDiagnostic} from './types';

export class DiagnosticsError extends Error {
  constructor(message: string, diagnostics: PartialDiagnostics) {
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
    this.name = 'DiagnosticsError';
  }

  diagnostics: PartialDiagnostics;
}

export function createSingleDiagnosticError(
  diag: PartialDiagnostic,
): DiagnosticsError {
  return new DiagnosticsError(diag.message, [diag]);
}

export function getDiagnosticsFromError(
  err: Error,
): undefined | PartialDiagnostics {
  if (err instanceof DiagnosticsError) {
    return err.diagnostics;
  }

  return undefined;
}
