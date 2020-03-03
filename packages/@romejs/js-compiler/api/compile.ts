/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Mappings} from '@romejs/codec-source-map';
import {PartialDiagnostics, DiagnosticFilterJSON} from '@romejs/diagnostics';
import {CompileRequest} from '../types';
import {Cache} from '@romejs/js-compiler';
import {generateJS} from '@romejs/js-generator';
import transform from '../methods/transform';
import lint from './lint';

export type CompileResult = {
  code: string;
  src: string;
  mappings: Mappings;
  diagnostics: PartialDiagnostics;
  filters: Array<DiagnosticFilterJSON>;
  cacheDependencies: Array<string>;
};

const compileCache: Cache<CompileResult> = new Cache();

export default async function compile(
  req: CompileRequest,
): Promise<CompileResult> {
  let {ast} = req;
  const {sourceText: src} = req;

  // Option to allow linting before compiling
  let lintDiagnostics: PartialDiagnostics = [];
  if (req.lint === true) {
    ({ast, diagnostics: lintDiagnostics} = await lint(req));
    req = {...req, ast};
  } else {
    lintDiagnostics = ast.diagnostics;
  }

  const query = Cache.buildQuery(req);
  const cached: undefined | CompileResult = compileCache.get(query);
  if (cached) {
    return cached;
  }

  const {filename} = ast;
  const {
    ast: transformedAst,
    diagnostics,
    filters,
    cacheDependencies,
  } = await transform(req);
  const generator = generateJS(
    transformedAst,
    {
      typeAnnotations: false,
      indent: req.stage === 'compileForBundle' ? 1 : 0,
      sourceMapTarget: filename,
      sourceFileName: filename,
      inputSourceMap: req.inputSourceMap,
    },
    src,
  );
  let res: CompileResult = {
    code: generator.getCode(),
    mappings: generator.getMappings(),
    src,
    diagnostics: [...lintDiagnostics, ...diagnostics],
    cacheDependencies,
    filters,
  };
  compileCache.set(query, res);

  return res;
}
