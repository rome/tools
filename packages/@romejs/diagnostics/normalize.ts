/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject} from '@romejs/typescript-helpers';
import {Position} from '@romejs/parser-core';
import {Diffs, Diff} from '@romejs/string-diff';
import {MarkupFormatOptions} from '@romejs/string-markup';
import {
  Diagnostic,
  DiagnosticAdviceItem,
  DiagnosticAdvice,
  Diagnostics,
  DiagnosticLanguage,
  DiagnosticLogCategory,
  DiagnosticAdviceStackFrame,
  DiagnosticOrigin,
  DiagnosticSourceType,
  DiagnosticDependencies,
} from './types';
import {coerce1, coerce0, number1, number0, number0Neg1} from '@romejs/ob1';

const DEFAULT_LIST_TRUNCATE = 100;
const DEFAULT_STACKTRACE_TRUNCATE = 10;

function normalizeLanguage(language: unknown): DiagnosticLanguage {
  if (
    language === 'js' ||
    language === 'json' ||
    language === 'shell' ||
    language === 'url'
  ) {
    return language;
  }

  return 'unknown';
}

function normalizePositionAssert(position: unknown): Position {
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

export function normalizePosition(position: unknown): undefined | Position {
  const {line, column, index} = normalizeDiagnosticsObject(position);

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

function normalizeLogCategory(value: unknown): DiagnosticLogCategory {
  if (
    value === 'none' ||
    value === 'question' ||
    value === 'info' ||
    value === 'warn' ||
    value === 'error'
  ) {
    return value;
  }

  return 'error';
}

function normalizeStringArray(value: unknown): Array<string> {
  if (Array.isArray(value)) {
    return value.map(item => normalizeString(item, 'Nothing provided'));
  } else {
    return [];
  }
}

function normalizeOptionalNumber(value: unknown): undefined | number {
  if (typeof value === 'number') {
    return value;
  } else {
    return undefined;
  }
}

function normalizeNumber(value: unknown, def: number): number {
  if (typeof value === 'number') {
    return value;
  } else {
    return def;
  }
}

function normalizeOptionalString(value: unknown): undefined | string {
  if (typeof value === 'string') {
    return value;
  } else {
    return undefined;
  }
}

function normalizeString(value: unknown, def: string): string {
  if (typeof value === 'string') {
    return value;
  } else {
    return def;
  }
}

function normalizeBoolean(value: unknown, def: boolean): boolean {
  if (typeof value === 'boolean') {
    return value;
  } else {
    return def;
  }
}

export function normalizeDiagnostics(
  diagnostics: unknown,
  opts: MarkupFormatOptions,
): Diagnostics {
  if (Array.isArray(diagnostics)) {
    return diagnostics.map(diag => normalizeDiagnostic(diag, opts));
  } else {
    return [];
  }
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

function normalizeDiagnosticsObject(obj: unknown): UnknownObject {
  if (typeof obj !== 'object' || obj == null || Array.isArray(obj)) {
    return {};
  } else {
    // @ts-ignore
    return obj;
  }
}

function normalizeSourceType(val: unknown): DiagnosticSourceType {
  if (val === 'module' || val === 'script' || val === 'template') {
    return val;
  } else {
    return 'unknown';
  }
}

export function normalizeDiagnostic(
  rawDiagnostic: unknown,
  opts: MarkupFormatOptions,
): Diagnostic {
  // Ensure that it's an object
  const diagnostic = normalizeDiagnosticsObject(rawDiagnostic);

  const filename = normalizeFilename(
    normalizeOptionalString(diagnostic.filename),
    opts,
  );
  const mtime = normalizeOptionalNumber(diagnostic.mtime);
  const origins = normalizeOrigins(diagnostic.origins);

  const start = normalizePosition(diagnostic.start);
  const end = normalizePosition(diagnostic.end);

  const fixable = normalizeBoolean(diagnostic.fixable, false);
  const sourceText = normalizeOptionalString(diagnostic.sourceText);
  const language = normalizeLanguage(diagnostic.language);
  const sourceType = normalizeSourceType(diagnostic.sourceType);
  const category = normalizeString(diagnostic.category, 'unknown');
  const message = normalizeString(diagnostic.message, 'No message provided');
  const marker = normalizeOptionalString(diagnostic.marker);
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
  value: unknown,
  opts: MarkupFormatOptions,
): DiagnosticDependencies {
  if (Array.isArray(value)) {
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
  } else {
    return [];
  }
}

function normalizeOrigins(value: unknown): Array<DiagnosticOrigin> {
  if (Array.isArray(value)) {
    return value.map(elem => {
      if (typeof elem === 'object' && elem != null) {
        return {
          category:
            typeof elem.category === 'string' ? elem.category : 'unknown',
          message: typeof elem.message === 'string' ? elem.message : undefined,
        };
      } else {
        return {category: 'unknown'};
      }
    });
  } else {
    return [];
  }
}

function normalizeAdviceStackFrame(
  value: UnknownObject,
  opts: MarkupFormatOptions,
): DiagnosticAdviceStackFrame {
  return {
    prefix: normalizeOptionalString(value.prefix),
    suffix: normalizeOptionalString(value.suffix),
    object: normalizeOptionalString(value.object),
    property: normalizeOptionalString(value.property),
    filename: normalizeFilename(normalizeOptionalString(value.filename), opts),
    line: coerce1(normalizeOptionalNumber(value.line)),
    column: coerce0(normalizeOptionalNumber(value.column)),
    sourceText: normalizeOptionalString(value.sourceText),
    language: normalizeLanguage(value.language),
  };
}

function normalizeAdviceStackFrames(
  value: unknown,
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
  value: unknown,
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
  rawPart: unknown,
  opts: MarkupFormatOptions,
): DiagnosticAdviceItem {
  const part = normalizeDiagnosticsObject(rawPart);

  if (typeof part.type !== 'string') {
    return {
      type: 'log',
      category: 'error',
      message: `Unsupported diagnostics part with no type`,
      compact: false,
    };
  }

  switch (part.type) {
    case 'log':
      return {
        type: 'log',
        category: normalizeLogCategory(part.category),
        message: normalizeString(part.message, 'No message provided'),
        compact: normalizeBoolean(part.compact, false),
      };

    case 'list':
      return {
        type: 'list',
        list: normalizeStringArray(part.list),
        truncate: normalizeNumber(part.truncate, DEFAULT_LIST_TRUNCATE),
        reverse: normalizeBoolean(part.reverse, false),
        ordered: normalizeBoolean(part.ordered, false),
      };

    case 'code':
      return {
        type: 'code',
        code: normalizeString(part.code, ''),
        language: normalizeLanguage(part.language || diag.language),
        sourceType: normalizeSourceType(part.sourceType || diag.sourceType),
      };

    case 'frame':
      return {
        type: 'frame',
        sourceType: normalizeSourceType(part.sourceType || diag.sourceType),
        language: normalizeLanguage(part.language || diag.language),
        sourceText: normalizeOptionalString(part.sourceText),
        marker: normalizeOptionalString(part.marker),
        filename: normalizeFilename(
          normalizeOptionalString(part.filename),
          opts,
        ),
        mtime: normalizeOptionalNumber(part.mtime),
        start: normalizePositionAssert(part.start),
        end: normalizePositionAssert(part.end),
      };

    case 'diff':
      return {
        type: 'diff',
        diff: normalizeAdviceDiff(part.diff),
      };

    case 'action':
      return {
        type: 'action',
        message: normalizeString(part.message, 'No message provided'),
        cancelable: normalizeBoolean(part.cancelable, false),
        buttons: normalizeAdviceActionButtons(part.buttons),
      };

    case 'stacktrace':
      return {
        type: 'stacktrace',
        title: normalizeOptionalString(part.title),
        frames: normalizeAdviceStackFrames(part.frames, opts),
        truncate: normalizeNumber(part.truncate, DEFAULT_STACKTRACE_TRUNCATE),
      };

    case 'inspect':
      return {
        type: 'inspect',
        // @ts-ignore TODO
        data: part.data as any,
      };

    default:
      return {
        type: 'log',
        category: 'error',
        message: `Unsupported diagnostics part ${part.type}`,
        compact: false,
      };
  }
}

function normalizeAdviceDiffEntry([op, text]: Array<unknown>): Diff {
  if (typeof text === 'string' && (op === -1 || op === 0 || op === 1)) {
    return [op, text];
  } else {
    // Maybe we should error here?
    return [0, ''];
  }
}

function normalizeAdviceDiff(value: unknown): Diffs {
  if (Array.isArray(value)) {
    return value.map(item => {
      if (Array.isArray(item)) {
        return normalizeAdviceDiffEntry(item);
      } else {
        return normalizeAdviceDiffEntry([]);
      }
    });
  } else {
    return [];
  }
}

function normalizeAdviceActionButtons(
  value: unknown,
): Array<{
  text: string;
  command: string;
}> {
  if (Array.isArray(value)) {
    return value.map(item => {
      if (typeof item !== 'object' || item == null) {
        item = {};
      }

      return {
        text: normalizeString(item.text, 'No text provided'),
        command: normalizeString(item.command, ''),
      };
    });
  } else {
    return [];
  }
}
