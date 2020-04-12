/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Diagnostics, DiagnosticSuppressions} from '@romejs/diagnostics';
import {DiagnosticsPrinterOptions} from './types';
import {Reporter, ReporterStream} from '@romejs/cli-reporter';
import DiagnosticsPrinter from './DiagnosticsPrinter';

export {toLines} from './utils';
export {
  DEFAULT_PRINTER_FLAGS,
  readDiagnosticsFileLocal,
} from './DiagnosticsPrinter';
export {DiagnosticsPrinter};

export * from './constants';

export * from './types';

// Simple wrappers around DiagnosticsPrinter
export async function printDiagnostics(
  diagnostics: Diagnostics,
  opts: DiagnosticsPrinterOptions,
): Promise<DiagnosticsPrinter> {
  const printer = new DiagnosticsPrinter(opts);
  printer.addDiagnostics(diagnostics);
  await printer.print();
  return printer;
}

export function printDiagnosticsSync(
  diagnostics: Diagnostics,
  opts: DiagnosticsPrinterOptions,
): DiagnosticsPrinter {
  const printer = new DiagnosticsPrinter(opts);
  printer.addDiagnostics(diagnostics);
  printer.print();
  return printer;
}

export function printDiagnosticsToString({
  diagnostics,
  suppressions = [],
  format = 'none',
  printerOptions = {},
}: {
  diagnostics: Diagnostics;
  suppressions?: DiagnosticSuppressions;
  printerOptions?: Omit<DiagnosticsPrinterOptions, 'reporter'>;
  format?: ReporterStream['format'];
}): string {
  let buff = '';

  const reporter = new Reporter({
    streams: [
      {
        type: 'all',
        format,
        unicode: true,
        columns: 400,
        write(chunk) {
          buff += chunk;
        },
      },
    ],
  });

  const printer = new DiagnosticsPrinter({
    ...printerOptions,
    reporter,
  });

  printer.processor.addSuppressions(suppressions);
  printer.addDiagnostics(diagnostics);
  printer.print();
  printer.footer();
  return buff;
}
