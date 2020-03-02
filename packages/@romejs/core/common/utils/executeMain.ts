/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {UnknownObject} from '@romejs/typescript-helpers';
import {SourceMap} from '@romejs/codec-source-map';
import {sourceMapManager} from '@romejs/v8';
import internalModule = require('module');
import vm = require('vm');
import {
  PartialDiagnostic,
  truncateSourceText,
  INTERNAL_ERROR_LOG_ADVICE,
} from '@romejs/diagnostics';
import {AbsoluteFilePath} from '@romejs/path';
import {Position} from '@romejs/parser-core';
import {number0Neg1, coerce1, number0} from '@romejs/ob1';

type ExecuteMainOptions = {
  path: AbsoluteFilePath;
  code: string;
  sourceMap: SourceMap;
  globals?: UnknownObject;
};

export default async function executeMain(
  opts: ExecuteMainOptions,
): Promise<{
  syntaxError: undefined | PartialDiagnostic;
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
    require:
      internalModule.createRequire(filename) ||
      internalModule.createRequireFromPath(filename),
    console,
    __dirname: path.getParent().join(),
    __filename: filename,
  };
  sandbox.global = sandbox;
  const context = vm.createContext(sandbox);

  // Here we do some gymnastics to catch a syntax error to correctly identify it as being our fault
  let script;
  try {
    script = new vm.Script(code, {
      filename,
      displayErrors: true,
    });
  } catch (err) {
    if (err instanceof SyntaxError && err.stack !== undefined) {
      const lineMatch = err.stack.match(/^(.*?):(\d+)/);
      if (lineMatch == null) {
        throw err;
      }

      const line = Number(lineMatch[2]);

      const pos: Position = {
        index: number0Neg1,
        column: number0,
        line: coerce1(line),
      };

      const syntaxError: PartialDiagnostic = {
        message: err.message,
        category: 'syntaxError',
        start: pos,
        end: pos,
        filename,
        sourceText: truncateSourceText(code, pos, pos),
        advice: [INTERNAL_ERROR_LOG_ADVICE],
      };
      return {syntaxError};
    }

    throw err;
  }

  // Execute the script if there was no syntax error
  sourceMapManager.addSourceMap(filename, sourceMap);
  await script.runInContext(context);
  return {syntaxError: undefined};
}
