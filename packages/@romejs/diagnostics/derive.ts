/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
  Diagnostic,
  PartialDiagnosticAdvice,
  PartialDiagnostic,
  PartialDiagnostics,
  DiagnosticOrigin,
} from './types';
import {Position} from '@romejs/parser-core';
import {escapeMarkup} from '@romejs/string-markup';
import {
  getErrorStructure,
  ErrorFrames,
  getSourceLocationFromErrorFrame,
} from '@romejs/v8';

function normalizeArray<T>(val: undefined | Array<T>): Array<T> {
  if (Array.isArray(val)) {
    return val;
  } else {
    return [];
  }
}

export function mergeDiagnostics(
  rootDiag: PartialDiagnostic,
  ...diags: Array<PartialDiagnostic>
): PartialDiagnostic {
  let mergedAdvice: PartialDiagnosticAdvice = [
    ...normalizeArray(rootDiag.advice),
  ];

  for (const diag of diags) {
    mergedAdvice = [
      ...mergedAdvice,
      ...deriveRootAdviceFromDiagnostic(diag).advice,
      ...normalizeArray(diag.advice),
    ];
  }

  return {
    ...rootDiag,
    advice: mergedAdvice,
  };
}

export function getDiagnosticHeader(opts: {
  filename: undefined | string;
  start: undefined | Position;
}): string {
  const {start, filename} = opts;

  if (filename === undefined) {
    return 'unknown';
  }

  if (start === undefined) {
    return `<filelink target="${filename}" />`;
  }

  return `<filelink target="${filename}" line="${start.line}" column="${start.column}" />`;
}

export function deriveRootAdviceFromDiagnostic(
  diag: PartialDiagnostic | Diagnostic,
  opts: {
    skipFrame: boolean;
    includeHeaderInAdvice: boolean;
    outdated: boolean;
  } = {
    skipFrame: false,
    includeHeaderInAdvice: true,
    outdated: false,
  },
): {
  advice: PartialDiagnosticAdvice;
  header: string;
} {
  const advice: PartialDiagnosticAdvice = [];

  let header = getDiagnosticHeader({
    start: diag.start,
    filename: diag.filename,
  });

  if (diag.category !== undefined) {
    header += ` <emphasis>${diag.category}</emphasis>`;
  }

  if (diag.fixable === true) {
    header += ` <inverse>FIXABLE</inverse>`;
  }

  if (opts.outdated === true) {
    header += ` <inverse>OUTDATED</inverse>`;
  }

  if (opts.includeHeaderInAdvice === true) {
    advice.push({
      type: 'log',
      category: 'none',
      message: header,
    });
  }

  advice.push({
    type: 'log',
    category: 'error',
    message: diag.message,
  });

  if (opts.skipFrame === false) {
    if (diag.start !== undefined && diag.end !== undefined) {
      advice.push({
        type: 'frame',
        sourceText: diag.sourceText,
        filename: diag.filename,
        mtime: diag.mtime,
        marker: diag.marker,
        start: diag.start,
        end: diag.end,
      });
    } else if (diag.marker !== undefined) {
      // If we have no start/end, but we do have a marker then output is a log error
      advice.push({
        type: 'log',
        category: 'error',
        message: diag.marker,
      });
    }
  }

  return {header, advice};
}

type DeriveErrorDiagnosticOpts = {
  error: unknown;
  category: string;
  filename?: string;
  cleanFrames?: (frames: ErrorFrames) => ErrorFrames;
};

export function deriveDiagnosticFromError(
  opts: DeriveErrorDiagnosticOpts,
): PartialDiagnostic {
  const {error, filename} = opts;

  let targetFilename: undefined | string = filename;
  let targetCode = undefined;
  let targetLoc = undefined;

  const structErr = getErrorStructure(error);
  let {frames, message, advice} = structErr;

  const {cleanFrames} = opts;
  if (cleanFrames !== undefined) {
    frames = cleanFrames(frames);
  }

  // Point the target to the closest frame with a filename
  for (const frame of frames) {
    if (frame.filename === undefined) {
      continue;
    }

    targetFilename = frame.filename;
    targetLoc = getSourceLocationFromErrorFrame(frame);
    break;
  }

  advice = [...getErrorStackAdvice(error, undefined, frames), ...advice];

  return {
    filename: targetFilename,
    start: targetLoc === undefined ? undefined : targetLoc.start,
    end: targetLoc === undefined ? undefined : targetLoc.end,
    sourceText: targetCode,
    category: opts.category,
    message,
    advice,
  };
}

export function getErrorStackAdvice(
  errorLike: unknown,
  title?: string,
  _frames?: ErrorFrames,
): PartialDiagnosticAdvice {
  const error = getErrorStructure(errorLike);
  const {stack} = error;

  const advice: PartialDiagnosticAdvice = [];
  const frames = _frames === undefined ? error.frames : _frames;

  if (frames.length === 0 && stack !== undefined) {
    // Just in case we didn't get the frames for some reason

    if (title !== undefined) {
      advice.push({
        type: 'log',
        category: 'info',
        message: title,
      });
    }

    // Remove the `message` from the `stack`
    let cleanStack = stack;
    let removeMessage = `${error.name}: ${error.message}`;
    if (cleanStack.startsWith(removeMessage)) {
      cleanStack = cleanStack.slice(removeMessage.length);
    }

    advice.push({
      type: 'log',
      category: 'error',
      message: escapeMarkup(cleanStack),
    });
  } else {
    const adviceFrames = frames.map(frame => {
      const {
        typeName,
        functionName,
        methodName,
        filename,
        lineNumber,
        columnNumber,
        isEval,
        isNative,
        isConstructor,
        isAsync,
      } = frame;

      const prefixes = [];
      if (isAsync) {
        prefixes.push('await');
      }
      if (isEval) {
        prefixes.push('eval');
      }
      if (isConstructor) {
        prefixes.push('new');
      }
      const prefix = prefixes.length === 0 ? undefined : prefixes.join(' ');

      let object = typeName;
      let property = '<anonymous>';
      if (functionName !== undefined) {
        property = functionName;
      }
      if (methodName !== undefined) {
        property = methodName;
      }

      let suffix;
      if (isNative) {
        suffix = 'native';
      }

      return {
        suffix,
        prefix,
        object,
        property,
        filename,
        line: lineNumber,
        column: columnNumber,
      };
    });

    advice.push({
      type: 'stacktrace',
      title,
      frames: adviceFrames,
    });
  }

  return advice;
}

export function addOriginsToDiagnostics(
  origins: Array<DiagnosticOrigin>,
  diagnostics: PartialDiagnostics,
): PartialDiagnostics {
  return diagnostics.map(diag => {
    return addOriginsToDiagnostic(origins, diag);
  });
}

export function addOriginsToDiagnostic(
  origins: Array<DiagnosticOrigin>,
  diag: PartialDiagnostic,
): PartialDiagnostic {
  const newOrigins =
    diag.origins === undefined ? origins : [...origins, ...diag.origins];
  return {
    ...diag,
    origins: newOrigins,
  };
}
