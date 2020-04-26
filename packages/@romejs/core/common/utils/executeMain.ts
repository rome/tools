/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject} from '@romejs/typescript-helpers';
import {SourceMapConsumer} from '@romejs/codec-source-map';
import {sourceMapManager} from '@romejs/v8';
import internalModule = require('module');

import vm = require('vm');

import {
  Diagnostic,
  INTERNAL_ERROR_LOG_ADVICE,
  descriptions,
  truncateSourceText,
} from '@romejs/diagnostics';
import {AbsoluteFilePath} from '@romejs/path';
import {Position} from '@romejs/parser-core';
import {ob1Coerce1, ob1Number0, ob1Number0Neg1} from '@romejs/ob1';

type ExecuteMainOptions = {
  path: AbsoluteFilePath;
  code: string;
  sourceMap?: SourceMapConsumer;
  globals?: UnknownObject;
};

export default async function executeMain(
  opts: ExecuteMainOptions,
): Promise<{
  syntaxError: undefined | Diagnostic;
}> {
  const {path, code, sourceMap, globals} = opts;

  const filename = path.join();

  // Create global context
  const sandbox: UnknownObject = {
    ...globals,
    process: {
      argv: [process.argv[0], filename],
      __proto__: process,
    },
    Buffer,
    clearImmediate,
    clearInterval,
    clearTimeout,
    setImmediate,
    setInterval,
    setTimeout,
    require: internalModule.createRequire
      ? internalModule.createRequire(filename)
      : internalModule.createRequireFromPath(filename),
    console,
    __dirname: path.getParent().join(),
    __filename: filename,
  };
  sandbox.global = sandbox;
  const context = vm.createContext(sandbox);

  // Here we do some gymnastics to catch a syntax error to correctly identify it as being our fault
  let script;
  try {
    script = new vm.Script(
      code,
      {
        filename,
        displayErrors: true,
      },
    );
  } catch (err) {
    if (err instanceof SyntaxError && err.stack !== undefined) {
      const lineMatch = err.stack.match(/^(.*?):(\d+)/);
      if (lineMatch == null) {
        throw err;
      }

      const line = Number(lineMatch[2]);

      const pos: Position = {
        index: ob1Number0Neg1,
        column: ob1Number0,
        line: ob1Coerce1(line),
      };

      const syntaxError: Diagnostic = {
        description: {
          ...descriptions.V8.SYNTAX_ERROR(err.message),
          advice: [INTERNAL_ERROR_LOG_ADVICE],
        },
        location: {
          start: pos,
          end: pos,
          filename,
          sourceText: truncateSourceText(code, pos, pos),
        },
      };
      return {syntaxError};
    }

    throw err;
  }

  // Execute the script if there was no syntax error
  if (sourceMap !== undefined) {
    sourceMapManager.addSourceMap(filename, () => sourceMap);
  }
  await script.runInContext(context);
  return {syntaxError: undefined};
}
