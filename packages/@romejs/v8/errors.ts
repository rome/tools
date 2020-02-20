/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Position, SourceLocation} from '@romejs/parser-core';
import {PartialDiagnosticAdvice} from '@romejs/diagnostics';
import {ErrorFrames, ErrorFrame} from './types';
import {isPlainObject} from '@romejs/typescript-helpers';
import {number1, number0, number0Neg1} from '@romejs/ob1';

export * from './types';

export const ERROR_FRAMES_PROP = Symbol();
export const ERROR_ADVICE_PROP = Symbol();
export const ERROR_POP_FRAMES_PROP = Symbol();

export type StructuredError = {
  name: string;
  message: string;
  stack: undefined | string;
  frames: ErrorFrames;
  advice: PartialDiagnosticAdvice;
  framesToPop: number;
};

class NativeStructuredError extends Error {
  constructor(struct: Partial<StructuredError>) {
    super(struct.message);
    this.name = struct.name === undefined ? 'Error' : struct.name;
    this.stack = struct.stack;

    this[ERROR_FRAMES_PROP] = struct.frames;
    this[ERROR_ADVICE_PROP] = struct.advice;
    this[ERROR_POP_FRAMES_PROP] = struct.framesToPop;
  }

  [ERROR_FRAMES_PROP]: undefined | ErrorFrames;
  [ERROR_ADVICE_PROP]: undefined | PartialDiagnosticAdvice;
  [ERROR_POP_FRAMES_PROP]: undefined | number;
}

export function createErrorFromStructure(
  struct: Partial<StructuredError>,
): Error {
  return new NativeStructuredError(struct);
}

export function getErrorStructure(err: unknown): StructuredError {
  let name = 'Error';
  let message = 'Unknown message';
  let stack = undefined;
  let frames = [];
  let advice = [];
  let framesToPop = 0;

  let looksLikeValidError = false;

  if (isPlainObject(err)) {
    if (typeof err.name === 'string') {
      looksLikeValidError = true;
      name = err.name;
    }

    if (typeof err.message === 'string') {
      looksLikeValidError = true;
      message = err.message;
    }

    if (typeof err.stack === 'string') {
      looksLikeValidError = true;
      stack = err.stack;
    }

    // @ts-ignore TODO validate
    if (Array.isArray(err[ERROR_FRAMES_PROP])) {
      // @ts-ignore TODO validate
      frames = err[ERROR_FRAMES_PROP];
    }

    // @ts-ignore TODO validate
    if (Array.isArray(err[ERROR_ADVICE_PROP])) {
      // @ts-ignore TODO validate
      advice = err[ERROR_ADVICE_PROP];
    }

    // @ts-ignore TODO validate
    if (typeof err[ERROR_POP_FRAMES_PROP] === 'number') {
      // @ts-ignore TODO validate
      framesToPop = err[ERROR_POP_FRAMES_PROP];
    }
  }

  if (!looksLikeValidError) {
    message = `Not an error instance: ${String(err)}`;
  }

  return {
    name,
    message,
    stack,
    frames,
    advice,
    framesToPop,
  };
}

export function getSourceLocationFromErrorFrame(
  frame: ErrorFrame,
): SourceLocation {
  const pos: Position = {
    index: number0Neg1,
    line: frame.lineNumber === undefined ? number1 : frame.lineNumber,
    column: frame.columnNumber === undefined ? number0 : frame.columnNumber,
  };

  return {
    filename: frame.filename === undefined ? 'unknown' : frame.filename,
    start: pos,
    end: pos,
  };
}
