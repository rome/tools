/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position} from '@romejs/parser-core';
import {MarkupFormatOptions} from '@romejs/string-markup';
import {
  Diagnostic,
  DiagnosticAdviceItem,
  DiagnosticAdvice,
  Diagnostics,
  DiagnosticLanguage,
  DiagnosticAdviceStackFrame,
  DiagnosticDependencies,
  PartialDiagnostic,
  PartialDiagnosticAdvice,
  PartialDiagnosticAdviceItem,
  PartialDiagnosticAdviceStackFrame,
  PartialDiagnostics,
} from './types';
import {coerce1, coerce0, number1, number0, number0Neg1} from '@romejs/ob1';

function normalizeLanguage(
  language: undefined | DiagnosticLanguage,
): DiagnosticLanguage {
  if (language === undefined) {
    return 'unknown';
  } else {
    return language;
  }
}

function normalizePositionAssert(position: undefined | Position): Position {
  const normalized = normalizePosition(position);
  if (normalized === undefined) {
    return {
      index: number0Neg1,
      line: number1,
      column: number0,
    };
  } else {
    return normalized;
  }
}

export function normalizePosition(
  position: undefined | Position,
): undefined | Position {
  if (position === undefined) {
    return undefined;
  }

  const {line, column, index} = position;

  if (
    typeof line !== 'number' ||
    typeof column !== 'number' ||
    typeof index !== 'number'
  ) {
    return undefined;
  }

  return {
    index: coerce0(Math.max(index, -1)),
    line: coerce1(Math.max(line, 1)),
    column: coerce0(Math.max(column, 0)),
  };
}

function normalizeValue<T>(value: undefined | T, def: T): T {
  if (value === undefined) {
    return def;
  } else {
    return value;
  }
}

export function normalizeDiagnostics(
  diagnostics: PartialDiagnostics,
  opts: MarkupFormatOptions,
): Diagnostics {
  return diagnostics.map(diag => normalizeDiagnostic(diag, opts));
}

function normalizeFilename(
  filename: undefined | string,
  opts: MarkupFormatOptions,
): string {
  if (filename === undefined) {
    return 'unknown';
  } else if (opts.normalizeFilename === undefined) {
    return filename;
  } else {
    return opts.normalizeFilename(filename);
  }
}

export function normalizeDiagnostic(
  diagnostic: PartialDiagnostic,
  opts: MarkupFormatOptions,
): Diagnostic {
  const filename = normalizeFilename(diagnostic.filename, opts);
  const mtime = diagnostic.mtime;
  const origins = normalizeValue(diagnostic.origins, []);

  const start = normalizePosition(diagnostic.start);
  const end = normalizePosition(diagnostic.end);

  const fixable = normalizeValue(diagnostic.fixable, false);
  const sourceText = diagnostic.sourceText;
  const language = normalizeLanguage(diagnostic.language);
  const sourceType = normalizeValue(diagnostic.sourceType, 'unknown');
  const category = normalizeValue(diagnostic.category, 'unknown');
  const message = normalizeValue(diagnostic.message, 'No message provided');
  const marker = diagnostic.marker;
  const dependencies = normalizeDependencies(diagnostic.dependencies, opts);

  const diag: Diagnostic = {
    category,
    message,
    origins,
    marker,
    language,
    sourceType,
    sourceText,
    fixable,
    dependencies,
    advice: [],
    filename,
    mtime,
    start,
    end,
  };

  const advice = normalizeDiagnosticAdvice(diag, diagnostic.advice, opts);

  return {
    ...diag,
    advice,
  };
}

function normalizeDependencies(
  value: PartialDiagnostic['dependencies'],
  opts: MarkupFormatOptions,
): DiagnosticDependencies {
  if (value === undefined) {
    return [];
  } else {
    const deps: DiagnosticDependencies = [];

    for (const elem of value) {
      if (
        typeof elem === 'object' &&
        elem != null &&
        typeof elem.filename === 'string' &&
        typeof elem.mtime === 'number'
      ) {
        deps.push({
          filename: normalizeFilename(elem.filename, opts),
          mtime: elem.mtime,
        });
      }
    }

    return deps;
  }
}

function normalizeAdviceStackFrame(
  value: PartialDiagnosticAdviceStackFrame,
  opts: MarkupFormatOptions,
): DiagnosticAdviceStackFrame {
  return {
    prefix: value.prefix,
    suffix: value.suffix,
    object: value.object,
    property: value.property,
    filename: normalizeFilename(value.filename, opts),
    line: value.line,
    column: value.column,
    sourceText: value.sourceText,
    language: normalizeLanguage(value.language),
  };
}

function normalizeAdviceStackFrames(
  value: Array<PartialDiagnosticAdviceStackFrame>,
  opts: MarkupFormatOptions,
): Array<DiagnosticAdviceStackFrame> {
  if (Array.isArray(value)) {
    return value.map(item => normalizeAdviceStackFrame(item, opts));
  } else {
    return [];
  }
}

function normalizeDiagnosticAdvice(
  diag: Diagnostic,
  value: undefined | PartialDiagnosticAdvice,
  opts: MarkupFormatOptions,
): DiagnosticAdvice {
  if (Array.isArray(value)) {
    return value.map(item => normalizeDiagnosticAdviceItem(diag, item, opts));
  } else {
    return [];
  }
}

export function normalizeDiagnosticAdviceItem(
  diag: Diagnostic,
  part: PartialDiagnosticAdviceItem,
  opts: MarkupFormatOptions,
): DiagnosticAdviceItem {
  switch (part.type) {
    case 'log':
      return {
        type: 'log',
        category: part.category,
        message: normalizeValue(part.message, 'No message provided'),
        compact: normalizeValue(part.compact, false),
      };

    case 'list':
      return {
        type: 'list',
        list: part.list,
        truncate: normalizeValue(part.truncate, false),
        reverse: normalizeValue(part.reverse, false),
        ordered: normalizeValue(part.ordered, false),
      };

    case 'code':
      return {
        type: 'code',
        code: normalizeValue(part.code, ''),
        language: normalizeLanguage(part.language || diag.language),
        sourceType: normalizeValue(
          part.sourceType || diag.sourceType,
          'unknown',
        ),
      };

    case 'frame':
      return {
        type: 'frame',
        sourceType: normalizeValue(
          part.sourceType || diag.sourceType,
          'unknown',
        ),
        language: normalizeLanguage(part.language || diag.language),
        sourceText: part.sourceText,
        marker: part.marker,
        filename: normalizeFilename(part.filename, opts),
        mtime: part.mtime,
        start: normalizePositionAssert(part.start),
        end: normalizePositionAssert(part.end),
      };

    case 'diff':
      return {
        type: 'diff',
        diff: part.diff,
      };

    case 'stacktrace':
      return {
        type: 'stacktrace',
        title: part.title,
        frames: normalizeAdviceStackFrames(part.frames, opts),
        truncate: normalizeValue(part.truncate, false),
      };

    case 'inspect':
      return {
        type: 'inspect',
        // @ts-ignore TODO
        data: part.data as any,
      };
  }
}
