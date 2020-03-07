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

export type DiagnosticFilter = {
  category?: string;
  message?: string;
  filename?: string;
  start?: Position;
  line?: Number1;
};

export type DiagnosticFilters = Array<DiagnosticFilter>;

export type DiagnosticSuppression = {
  category: string;
  loc: SourceLocation;
};

export type DiagnosticSuppressions = Array<DiagnosticSuppression>;

export type DiagnosticFilterWithTest = DiagnosticFilter & {
  test?: (diagnostic: PartialDiagnostic) => boolean;
};

export type DiagnosticPointer = SourceLocation & {
  sourceText?: string;
  mtime?: number;
  language?: DiagnosticLanguage;
};

export type DiagnosticOrigin = {
  category: string;
  message?: string;
};

//# FULL

export type DiagnosticLogCategory = 'none' | 'info' | 'warn' | 'error';

export type DiagnosticLanguage = 'json' | 'js' | 'url' | 'shell' | 'unknown';

export type DiagnosticSourceType = 'unknown' | ConstSourceType;

export type DiagnosticAdviceItemLog = {
  type: 'log';
  category: DiagnosticLogCategory;
  message: string;
  compact: boolean;
};

export type DiagnosticAdviceItemList = {
  type: 'list';
  list: Array<string>;
  truncate: number;
  reverse: boolean;
  ordered: boolean;
};

export type DiagnosticAdviceItemCode = {
  type: 'code';
  code: string;
  language: DiagnosticLanguage;
  sourceType: DiagnosticSourceType;
};

export type DiagnosticAdviceItemFrame = {
  type: 'frame';
  language: DiagnosticLanguage;
  sourceType: DiagnosticSourceType;
  sourceText: undefined | string;
  marker: undefined | string;

  filename: string;
  mtime: undefined | number;
  start: Position;
  end: Position;
};

export type DiagnosticAdviceItemInspect = {
  type: 'inspect';
  data: JSONPropertyValue;
};

export type DiagnosticAdviceItemDiff = {
  type: 'diff';
  diff: Diffs;
};

export type DiagnosticAdviceItemAction = {
  type: 'action';
  message: string;
  cancelable: boolean;
  buttons: Array<{
    text: string;
    command: string;
  }>;
};

export type DiagnosticAdviceItemStacktrace = {
  type: 'stacktrace';
  title: undefined | string;
  frames: Array<DiagnosticAdviceStackFrame>;
  truncate: number;
};

export type DiagnosticAdviceItem =
  | DiagnosticAdviceItemLog
  | DiagnosticAdviceItemList
  | DiagnosticAdviceItemCode
  | DiagnosticAdviceItemFrame
  | DiagnosticAdviceItemInspect
  | DiagnosticAdviceItemDiff
  | DiagnosticAdviceItemAction
  | DiagnosticAdviceItemStacktrace;

export type DiagnosticAdvice = Array<DiagnosticAdviceItem>;

export type DiagnosticDependency = {
  filename: string;
  mtime: number;
};

export type DiagnosticDependencies = Array<DiagnosticDependency>;

export type Diagnostic = {
  category: string;
  message: string;
  filename: undefined | string;

  origins: Array<DiagnosticOrigin>;
  mtime: undefined | number;
  dependencies: DiagnosticDependencies;

  sourceType: DiagnosticSourceType;
  language: DiagnosticLanguage;
  sourceText: undefined | string;
  start: undefined | Position;
  end: undefined | Position;

  marker: undefined | string;
  fixable: boolean;

  advice: DiagnosticAdvice;
};

export type Diagnostics = Array<Diagnostic>;

export type DiagnosticAdviceStackFrame = {
  suffix: undefined | string;
  prefix: undefined | string;
  object: undefined | string;
  property: undefined | string;

  filename: undefined | string;
  line: undefined | Number1;
  column: undefined | Number0;

  language: DiagnosticLanguage;
  sourceText: undefined | string;
};

//# PARTIAL

export type PartialDiagnostic = {
  category: string;
  message: string;

  origins?: Array<DiagnosticOrigin>;
  dependencies?: Array<{
    filename: string;
    mtime: string;
  }>;

  filename?: string;
  mtime?: number;
  sourceType?: DiagnosticSourceType;
  language?: DiagnosticLanguage;
  fixable?: boolean;
  sourceText?: string;

  marker?: string;
  start?: Position;
  end?: Position;

  advice?: PartialDiagnosticAdvice;
};

export type PartialDiagnostics = Array<PartialDiagnostic>;

export type PartialDiagnosticAdviceItem =
  | {
      type: 'log';
      category: DiagnosticLogCategory;
      message: string;
      compact?: boolean;
    }
  | {
      type: 'list';
      list: Array<string>;
      truncate?: number | undefined;
      reverse?: boolean;
      ordered?: boolean;
    }
  | {
      type: 'inspect';
      data: JSONPropertyValue;
    }
  | {
      type: 'code';
      code: string;
      sourceType?: ConstSourceType;
      language?: DiagnosticLanguage;
    }
  | {
      type: 'frame';
      mtime?: number;
      language?: DiagnosticLanguage;
      sourceType?: ConstSourceType;
      sourceText?: string;
      marker?: string;

      filename?: string;
      start: Position;
      end: Position;

      // From SourceLocation, will never appear in the final diagnostic, makes it easy to spread
      identifierName?: string;
    }
  | {
      type: 'diff';
      diff: Diffs;
    }
  | {
      type: 'action';
      message: string;
      cancelable: boolean;
      buttons: Array<{
        text: string;
        command: string;
      }>;
    }
  | {
      type: 'stacktrace';
      title?: string;
      truncate?: number;
      frames: Array<PartialDiagnosticAdviceStackFrame>;
    };

export type PartialDiagnosticAdvice = Array<PartialDiagnosticAdviceItem>;

export type PartialDiagnosticAdviceStackFrame = {
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
