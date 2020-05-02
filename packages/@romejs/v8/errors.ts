/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, SourceLocation} from '@romejs/parser-core';
import {DiagnosticAdvice} from '@romejs/diagnostics';
import {ErrorFrame, ErrorFrames} from './types';
import {isPlainObject} from '@romejs/typescript-helpers';
import {ob1Number0, ob1Number0Neg1, ob1Number1} from '@romejs/ob1';

export * from './types';

export const ERROR_FRAMES_PROP = Symbol();
export const ERROR_MARKUP_MESSAGE_PROP = Symbol();
export const ERROR_ADVICE_PROP = Symbol();

export type StructuredError = {
  name: string;
  markupMessage: undefined | string;
  message: string;
  stack: undefined | string;
  frames: ErrorFrames;
  advice: DiagnosticAdvice;
};

export class NativeStructuredError extends Error {
  constructor(struct: Partial<StructuredError>) {
    super(struct.message);
    this.name = struct.name === undefined ? 'Error' : struct.name;
    this.stack = struct.stack;

    this[ERROR_MARKUP_MESSAGE_PROP] = struct.markupMessage;
    this[ERROR_FRAMES_PROP] = struct.frames;
    this[ERROR_ADVICE_PROP] = struct.advice;
  }

  [ERROR_MARKUP_MESSAGE_PROP]: undefined | string;
  [ERROR_FRAMES_PROP]: undefined | ErrorFrames;
  [ERROR_ADVICE_PROP]: undefined | DiagnosticAdvice;
}

export function createErrorFromStructure(
  struct: Partial<StructuredError>,
): Error {
  return new NativeStructuredError(struct);
}

export function getErrorStructure(
  err: unknown,
  framesToShift: number = 0,
): StructuredError {
  let name = 'Error';
  let message = 'Unknown message';
  let stack = undefined;
  let markupMessage: string | undefined = undefined;
  let frames: ErrorFrames = [];
  let advice: DiagnosticAdvice = [];
  let looksLikeValidError = false;

  if (
    isPlainObject<{
      [ERROR_ADVICE_PROP]: unknown;
      [ERROR_FRAMES_PROP]: unknown;
      [ERROR_MARKUP_MESSAGE_PROP]: unknown;
    }>(err)
  ) {
    if (typeof err.name === 'string') {
      looksLikeValidError = true;
      name = err.name;
    }

    if (typeof err[ERROR_MARKUP_MESSAGE_PROP] === 'string') {
      // @ts-ignore
      markupMessage = err[ERROR_MARKUP_MESSAGE_PROP];
    }

    if (typeof err.message === 'string') {
      looksLikeValidError = true;
      message = err.message;
    }

    if (typeof err.stack === 'string') {
      looksLikeValidError = true;
      stack = err.stack;
    }

    if (Array.isArray(err[ERROR_FRAMES_PROP])) {
      // @ts-ignore
      frames = err[ERROR_FRAMES_PROP];
    }

    if (Array.isArray(err[ERROR_ADVICE_PROP])) {
      // @ts-ignore
      advice = err[ERROR_ADVICE_PROP];
    }
  }

  frames = frames.slice(framesToShift);

  if (!looksLikeValidError) {
    message = `Not an error instance: ${String(err)}`;
  }

  return {
    name,
    message,
    markupMessage,
    stack,
    frames,
    advice,
  };
}

export function getSourceLocationFromErrorFrame(
  frame: ErrorFrame,
): SourceLocation {
  const pos: Position = {
    index: ob1Number0Neg1,
    line: frame.lineNumber === undefined ? ob1Number1 : frame.lineNumber,
    column: frame.columnNumber === undefined ? ob1Number0 : frame.columnNumber,
  };

  return {
    filename: frame.filename === undefined ? 'unknown' : frame.filename,
    start: pos,
    end: pos,
  };
}
