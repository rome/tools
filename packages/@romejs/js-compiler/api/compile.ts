/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Mappings} from '@romejs/codec-source-map';
import {PartialDiagnostics, PartialDiagnostic} from '@romejs/diagnostics';
import {CompileRequest} from '../types';
import {Cache} from '@romejs/js-compiler';
import generate from '@romejs/js-generator';
import transform from '../methods/transform';
import lint from './lint';
import {requireGlobal} from '@romejs/node';
import {Position} from '@romejs/parser-core';
import {coerce1, coerce0} from '@romejs/ob1';
import {isPlainObject} from '@romejs/typescript-helpers';

export type CompileResult = {
  code: string;
  src: string;
  mappings: Mappings;
  diagnostics: PartialDiagnostics;
  cacheDependencies: Array<string>;
};

const compileCache: Cache<CompileResult> = new Cache();

// Validate passed error is BabelSyntaxError
function extractPositionFromBabelSyntaxError(
  err: unknown,
): undefined | Position {
  if (!isPlainObject(err)) {
    return;
  }

  const {pos} = err;
  if (typeof pos !== 'number') {
    return;
  }

  const {loc} = err;
  if (!isPlainObject(loc)) {
    return;
  }

  const {line, column} = loc;
  if (typeof line !== 'number') {
    return;
  }
  if (typeof column !== 'number') {
    return;
  }

  return {
    line: coerce1(line),
    column: coerce0(column),
    index: coerce0(pos),
  };
}

function babelInterop(req: CompileRequest, res: CompileResult): CompileResult {
  const {filename} = req.ast;

  try {
    const babelResult = requireGlobal(
      'babel-core',
      req.project.folder,
    ).transform(res.code, {
      filename,
    });

    const generatorResult = requireGlobal(
      'babel-generator',
      req.project.folder,
    ).default(
      babelResult.ast,
      {
        sourceMaps: true,
      },
      res.code,
    );

    return {
      ...res,
      code: generatorResult.code,
      mappings: generatorResult.rawMappings,
    };
  } catch (err) {
    const pos = extractPositionFromBabelSyntaxError(err);
    if (pos === undefined) {
      throw err;
    }

    // Clean message
    const message = err.message
      // Remove filename
      .replace(/^(.*?):/, '')
      // Remove location
      .replace(/ \(\d+:\d+\)/, '')
      // Use only the first line, the rest are redundant codeframes
      .split('\n')[0]
      .trim();

    const babelDiagnostic: PartialDiagnostic = {
      filename,
      category: 'babel',
      message,
      // Embed compiled Rome code as the Babel error will point to it and not the source on disk
      sourceText: res.code,
      start: pos,
      end: pos,
      advice: [
        {
          type: 'log',
          category: 'warn',
          message:
            'You are using Babel interop mode. This error is from Babel running over Rome emitted code',
        },
      ],
    };

    return {
      ...res,
      // TODO add any .babelrc dependencies
      cacheDependencies: [...res.cacheDependencies],
      diagnostics: [...res.diagnostics, babelDiagnostic],
    };
  }
}

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
  const {ast: transformedAst, diagnostics, cacheDependencies} = await transform(
    req,
  );
  const generator = generate(
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
  };
  compileCache.set(query, res);

  // BABEL HACK loool
  if (req.project.config.compiler.legacyBabelInterop) {
    res = babelInterop(req, res);
  }

  return res;
}
