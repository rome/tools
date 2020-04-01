/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SourceMap, SourceMapConsumer} from '@romejs/codec-source-map';
import {ErrorFrame} from '@romejs/v8';
import {coerce1, coerce1to0, Number1, Number0} from '@romejs/ob1';
import {
  getErrorStructure,
  ERROR_FRAMES_PROP,
  ERROR_POP_FRAMES_PROP,
} from './errors';

type ResolvedLocation = {
  found: boolean;
  filename: string;
  line: Number1;
  column: Number0;
  name: undefined | string;
};

let inited: boolean = false;
const maps: Map<string, SourceMapConsumer> = new Map();

// In case we want to defer the reading of a source map completely (parsing is always deferred)
const factories: Map<string, () => SourceMap> = new Map();

function prepareStackTrace(err: Error, frames: Array<NodeJS.CallSite>) {
  try {
    addErrorFrames(err, frames);
    return buildStackString(err);
  } catch (err2) {
    return (
      `${err.name}: ${err.message}\n  Failed to generate stacktrace: ${err2.message}`
    );
  }
}

export function init() {
  if (!inited) {
    inited = true;
    Error.prepareStackTrace = prepareStackTrace;
  }
}

export function teardown() {
  Error.prepareStackTrace = undefined;
}

function buildStackString(err: Error): string {
  const {frames} = getErrorStructure(err);
  const lines: Array<string> = [];

  lines.push(`${err.name}: ${err.message}`);

  for (const frame of frames) {
    const {
      resolvedLocation,
      methodName,
      functionName,
      typeName,
      isNative,
      isAsync,
      isEval,
      isConstructor,
      filename,
      lineNumber,
      columnNumber,
    } = frame;
    const parts: Array<string> = [];

    if (isAsync) {
      parts.push('await');
    }

    if (isEval) {
      parts.push('eval');
    }

    if (isConstructor) {
      parts.push('new');
    }

    let name = '<anonymous';
    if (functionName !== undefined) {
      name = functionName;
    }
    if (methodName !== undefined) {
      name = methodName;
    }
    if (typeName !== undefined) {
      parts.push(`${typeName}.${name}`);
    } else {
      parts.push(name);
    }

    if (isNative) {
      parts.push('native');
    } else if (filename !== undefined && lineNumber !== undefined &&
      columnNumber !== undefined) {
      parts.push(`(${filename}:${lineNumber}:${columnNumber})`);
    }

    if (resolvedLocation === false) {
      parts.push('generated source location');
    }

    lines.push(`  at ${parts.join(' ')}`);
  }

  return lines.join('\n');
}

function noNull<T>(val: null | T): undefined | T {
  if (val === null) {
    return undefined;
  } else {
    return val;
  }
}

function addErrorFrames(
  err:
    & Error
    & {
      [ERROR_FRAMES_PROP]?: unknown;
      [ERROR_POP_FRAMES_PROP]?: unknown;
    },

  frames: Array<NodeJS.CallSite>,
): void {
  if (err[ERROR_FRAMES_PROP]) {
    return;
  }

  let builtFrames = frames.map((frameApi): ErrorFrame => {
    const filename = frameApi.getFileName();
    const lineNumber = frameApi.getLineNumber();
    const columnNumber = frameApi.getColumnNumber();

    const frame: ErrorFrame = {
      typeName: noNull(frameApi.getTypeName()),
      functionName: noNull(frameApi.getFunctionName()),
      methodName: noNull(frameApi.getMethodName()),

      isTopLevel: frameApi.isToplevel(),
      isEval: frameApi.isEval(),
      isNative: frameApi.isNative(),
      isConstructor: frameApi.isConstructor(),

      // TODO frameApi.isAsync
      isAsync: false,

      resolvedLocation: true,

      filename: noNull(filename),
      lineNumber: lineNumber == null ? undefined : coerce1(lineNumber),

      // Rome expects 0-indexed columns, V8 provides 1-indexed
      columnNumber: columnNumber == null ? undefined : coerce1to0(columnNumber),
    };

    if (frame.filename !== undefined && frame.lineNumber !== undefined &&
      frame.columnNumber !== undefined) {
      const {found, line, column, filename, name} = resolveLocation(
        frame.filename,
        frame.lineNumber,
        frame.columnNumber,
      );

      return {
        ...frame,
        functionName: frame.functionName === undefined
          ? name : frame.functionName,
        methodName: frame.methodName === undefined ? name : frame.methodName,
        resolvedLocation: found,
        lineNumber: line,
        columnNumber: column,
        filename,
      };
    } else {
      return frame;
    }
  });

  // This is a property that an error object can define that will remove that amount of frames

  // This is useful for removing levels of indirection, for example, an invariant error
  const framesToProp = err[ERROR_POP_FRAMES_PROP];
  if (typeof framesToProp === 'number') {
    builtFrames = builtFrames.slice(framesToProp);
  }

  err[ERROR_FRAMES_PROP] = builtFrames;
}

export function resolveLocation(
  filename: string,
  line: Number1,
  column: Number0,
): ResolvedLocation {
  const map = getSourceMap(filename);
  if (map === undefined) {
    return {
      found: true,
      filename,
      line,
      column,
      name: undefined,
    };
  }

  const resolved = map.approxOriginalPositionFor(line, column);
  if (resolved === undefined) {
    return {
      found: false,
      filename,
      line,
      column,
      name: undefined,
    };
  }

  return {
    found: true,
    filename: resolved.source,
    line: resolved.line,
    column: resolved.column,
    name: resolved.name,
  };
}

export function addSourceMap(filename: string, map: SourceMap) {
  return addSourceMapFactory(filename, () => map);
}

// Add a source map factory. We jump through some hoops to return a function to remove the source map.
// We make sure not to remove the source map if it's been subsequently added by another call.
export function addSourceMapFactory(
  filename: string,
  factory: () => SourceMap,
): () => void {
  init();

  let map: undefined | SourceMap;
  function factoryCapture() {
    map = factory();
    return map;
  }

  factories.set(filename, factoryCapture);

  return () => {
    if (factories.get(filename) === factoryCapture) {
      factories.delete(filename);
    }

    if (maps.get(filename) === map) {
      maps.delete(filename);
    }
  };
}

export function getSourceMap(filename: string): undefined | SourceMapConsumer {
  if (maps.has(filename)) {
    return maps.get(filename);
  }

  const factory = factories.get(filename);
  if (factory !== undefined) {
    factories.delete(filename);
    const map = factory();
    const consumer = new SourceMapConsumer(map);
    maps.set(filename, consumer);
    return consumer;
  }

  return undefined;
}
