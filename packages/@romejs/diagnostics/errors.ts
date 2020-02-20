/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialDiagnostics} from '@romejs/diagnostics';
import {escapeMarkup} from '@romejs/string-markup';
import {stripAnsi} from '@romejs/string-ansi';

export class DiagnosticsError extends Error {
  constructor(message: string, diagnostics: PartialDiagnostics) {
    if (diagnostics.length === 0) {
      throw new Error('No diagnostics');
    }

    message += '\n';
    /*message += stripAnsi(
      printDiagnosticsToString(diagnostics, {
        origins: [],
      }),
    );*/
    message += stripAnsi(
      diagnostics.map(diag => `- ${diag.message}`).join('\n'),
    );

    super(escapeMarkup(message));
    this.diagnostics = diagnostics;
    this.name = 'DiagnosticsError';

    // If we don't have a filename then this error wont target anywhere, so add a derived error diagnostic
    // to make it easier to track
    //if (diagnostic.filename === undefined) {
    //  diagnostic = mergeDiagnostics(
    //    deriveDiagnosticFromError({category: 'DiagnosticError', error: this}),
    //    diagnostic,
    //  );
    //}
  }

  diagnostics: PartialDiagnostics;
}

export function getDiagnosticsFromError(
  err: Error,
): undefined | PartialDiagnostics {
  if (err instanceof DiagnosticsError) {
    return err.diagnostics;
  }

  return undefined;
}
