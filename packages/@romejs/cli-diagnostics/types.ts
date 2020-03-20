/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Reporter} from '@romejs/cli-reporter';
import {AbsoluteFilePath} from '@romejs/path';
import {DiagnosticOrigin, DiagnosticFilters} from '@romejs/diagnostics';

export type DiagnosticsPrinterFlags = {
  grep: string;
  fieri: boolean;
  inverseGrep: boolean;
  focus: string;
  verboseDiagnostics: boolean;
  maxDiagnostics: number;
  showAllDiagnostics: boolean;
};

export type DiagnosticsFileReader = (path: AbsoluteFilePath) =>
  | undefined
  | DiagnosticsFileReaderStats;

export type DiagnosticsFileReaderStats = {
  content: string;
  mtime: number;
};

export type DiagnosticsPrinterOptions = {
  origins?: Array<DiagnosticOrigin>;
  reporter: Reporter;
  cwd?: AbsoluteFilePath;
  flags?: DiagnosticsPrinterFlags;
  readFile?: DiagnosticsFileReader;
  filters?: DiagnosticFilters;
};
