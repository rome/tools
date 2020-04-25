/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, SourceLocation} from '@romejs/parser-core';
import {Diffs} from '@romejs/string-diff';
import {ConstSourceType} from '@romejs/js-ast';
import {Number0, Number1} from '@romejs/ob1';
import {JSONPropertyValue} from '@romejs/codec-json';
import {DiagnosticCategory} from './categories';
import {Dict} from '@romejs/typescript-helpers';
import {ClientRequestFlags} from '@romejs/core';

export type DiagnosticFilter = {
  category?: DiagnosticCategory;
  message?: string;
  filename?: string;
  start?: Position;
  line?: Number1;
};

export type DiagnosticFilters = Array<DiagnosticFilter>;

export type DiagnosticSuppressionType = 'current' | 'next';

export type DiagnosticSuppression = {
  type: DiagnosticSuppressionType;
  category: string;
  loc: SourceLocation;
};

export type DiagnosticSuppressions = Array<DiagnosticSuppression>;

export type DiagnosticFilterWithTest = DiagnosticFilter & {
  test?: (diagnostic: Diagnostic) => boolean;
};

export type DiagnosticLocation = {
  sourceText?: string;
  mtime?: number;
  marker?: string;
  language?: DiagnosticLanguage;
  sourceType?: DiagnosticSourceType;
  filename?: string;
  start?: Position;
  end?: Position;
};

export type DiagnosticOrigin = {
  category: string;
  message?: string;
};

export type DiagnosticLogCategory = 'none' | 'info' | 'warn' | 'error';

export type DiagnosticLanguage = 'json' | 'js' | 'url' | 'shell' | 'unknown';

export type DiagnosticSourceType = 'unknown' | ConstSourceType;

export type Diagnostic = {
  description: DiagnosticDescription;
  location: DiagnosticLocation;
  fixable?: boolean;
  label?: string;
  origins?: Array<DiagnosticOrigin>;
  dependencies?: Array<{
    filename: string;
    mtime: number;
  }>;
};

export type Diagnostics = Array<Diagnostic>;

export type DiagnosticDescription = {
  category: DiagnosticCategory;
  message: DiagnosticBlessedMessage;
  advice?: DiagnosticAdvice;
};

export type DiagnosticDescriptionOptionalCategory = {
  category?: DiagnosticCategory;
  message: DiagnosticBlessedMessage;
  advice?: DiagnosticAdvice;
};

// TS doesn't have opaque types so we need to use an intermediate object
export type DiagnosticBlessedMessage = {
  type: 'PARTIAL_BLESSED_DIAGNOSTIC_MESSAGE';
  value: string;
};

export type DiagnosticAdviceItem =
  | DiagnosticAdviceLog
  | DiagnosticAdviceList
  | DiagnosticAdviceInspect
  | DiagnosticAdviceCode
  | DiagnosticAdviceFrame
  | DiagnosticAdviceDiff
  | DiagnosticAdviceStacktrace
  | DiagnosticAdviceCommand
  | DiagnosticAdviceAction;

export type DiagnosticAdviceCommand = {
  type: 'command';
  command: string;
};

export type DiagnosticAdviceLog = {
  type: 'log';
  category: DiagnosticLogCategory;
  message: string;
  compact?: boolean;
};

export type DiagnosticAdviceList = {
  type: 'list';
  list: Array<string>;
  truncate?: boolean;
  reverse?: boolean;
  ordered?: boolean;
};

export type DiagnosticAdviceInspect = {
  type: 'inspect';
  data: JSONPropertyValue;
};

export type DiagnosticAdviceAction = {
  type: 'action';
  hidden?: boolean;
  instruction: string;
  noun: string;
  command: string;
  commandFlags?: Dict<boolean | string | Array<string>>;
  requestFlags?: ClientRequestFlags;
  args?: Array<string>;
};

export type DiagnosticAdviceCode = {
  type: 'code';
  code: string;
  sourceType?: ConstSourceType;
  language?: DiagnosticLanguage;
};

export type DiagnosticAdviceFrame = {
  type: 'frame';
  location: DiagnosticLocation;
};

export type DiagnosticAdviceDiff = {
  type: 'diff';
  diff: Diffs;
  legend?: {
    add: string;
    delete: string;
  };
};

export type DiagnosticAdviceStacktrace = {
  type: 'stacktrace';
  title?: string;
  truncate?: boolean;
  frames: Array<DiagnosticAdviceStackFrame>;
};

export type DiagnosticAdvice = Array<DiagnosticAdviceItem>;

export type DiagnosticAdviceStackFrame = {
  prefix?: string;
  suffix?: string;
  object?: string;
  property?: string;
  filename?: string;
  line?: Number1;
  column?: Number0;
  language?: DiagnosticLanguage;
  sourceText?: string;
};
