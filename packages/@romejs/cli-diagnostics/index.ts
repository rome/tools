/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {DiagnosticSuppressions, Diagnostics} from '@romejs/diagnostics';
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

export function printDiagnostics({
  diagnostics,
  suppressions,
  printerOptions,
  excludeFooter,
}: {
  diagnostics: Diagnostics;
  suppressions: DiagnosticSuppressions;
  printerOptions: DiagnosticsPrinterOptions;
  excludeFooter?: boolean;
}): DiagnosticsPrinter {
  const printer = new DiagnosticsPrinter(printerOptions);
  printer.processor.addDiagnostics(diagnostics);
  printer.processor.addSuppressions(suppressions);
  printer.print();
  if (!excludeFooter) {
    printer.footer();
  }
  return printer;
}

export function printDiagnosticsToString(opts: {
  diagnostics: Diagnostics;
  suppressions: DiagnosticSuppressions;
  printerOptions?: DiagnosticsPrinterOptions;
  format?: ReporterStream['format'];
  excludeFooter?: boolean;
}): string {
  const reporter = new Reporter();
  const stream = reporter.attachCaptureStream(opts.format);
  printDiagnostics({
    ...opts,
    printerOptions: {
      reporter,
      ...opts.printerOptions,
    },
  });
  return stream.read();
}
