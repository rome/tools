/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Mappings} from '@romejs/codec-source-map';
import {Diagnostics, DiagnosticSuppressions} from '@romejs/diagnostics';
import {CompileRequest} from '../types';
import {Cache} from '@romejs/js-compiler';
import {generateJS} from '@romejs/js-generator';
import transform from '../methods/transform';

export type CompileResult = {
  mappings: Mappings;
  diagnostics: Diagnostics;
  suppressions: DiagnosticSuppressions;
  cacheDependencies: Array<string>;
  compiledCode: string;
  sourceText: string;
};

const compileCache: Cache<CompileResult> = new Cache();

export default async function compile(
  req: CompileRequest,
): Promise<CompileResult> {
  const {sourceText, ast} = req;

  const query = Cache.buildQuery(req);
  const cached: undefined | CompileResult = compileCache.get(query);
  if (cached) {
    return cached;
  }

  const {filename} = ast;
  const {
    ast: transformedAst,
    diagnostics,
    suppressions,
    cacheDependencies,
  } = await transform(req);
  const generator = generateJS(transformedAst, {
    typeAnnotations: false,
    indent: req.stage === 'compileForBundle' ? 1 : 0,
    sourceMapTarget: filename,
    sourceFileName: filename,
    inputSourceMap: req.inputSourceMap,
  }, sourceText);

  const res: CompileResult = {
    compiledCode: generator.buf.getCode(),
    mappings: generator.buf.getMappings(),
    diagnostics: [...ast.diagnostics, ...diagnostics],
    cacheDependencies,
    suppressions,
    sourceText,
  };
  compileCache.set(query, res);
  return res;
}
