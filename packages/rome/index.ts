/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {parseJS} from '@romejs/js-parser';
import {
  AbsoluteFilePath,
  createAbsoluteFilePath,
  AbsoluteFilePathMap,
} from '@romejs/path';
import {SourceMap, SourceMapGenerator} from '@romejs/codec-source-map';
import * as compiler from '@romejs/js-compiler';
import {exists, lstat, readdir, watch} from '@romejs/fs';
import {ConstSourceType} from '@romejs/js-ast';
import {findProject} from './project';
import {throwDiagnostics, wrapForErrors} from './error';
import {getFileHandlerAssert} from '@romejs/core/common/fileHandlers';

export {RomeDiagnosticsError} from './error';

//

const cacheKeyPartsCache: AbsoluteFilePathMap<string> = new AbsoluteFilePathMap();

async function getCacheKeyPart(
  path: AbsoluteFilePath,
  relative: AbsoluteFilePath,
): Promise<string> {
  const cached = cacheKeyPartsCache.get(path);
  if (cached !== undefined) {
    return `${path.relative(relative).join()}:${cached}`;
  }

  let part: string = '??';

  if (await exists(path)) {
    const stats = await lstat(path);
    if (stats.isFile()) {
      part = `mtime:${String(stats.mtimeMs)}`;
    } else if (stats.isDirectory()) {
      const filenames = await readdir(path);
      part = `readdir:${filenames}`;
    }
  } else {
    part = `noexist`;
  }

  cacheKeyPartsCache.set(path, part);

  // Watch for changes to invalidate our cached... cache key
  const watcher = watch(path, {}, () => {
    cacheKeyPartsCache.delete(path);
    watcher.close();
  });

  return `${path.relative(relative).join()}:${part}`;
}

export async function getCacheKey(filename: string): Promise<string> {
  const path = createAbsoluteFilePath(filename);
  const project = await findProject(path);
  const parts: Array<string> = [];

  parts.push(await getCacheKeyPart(path, path));

  for (const configDepPath of project.meta.configDependencies) {
    parts.push(await getCacheKeyPart(configDepPath, path));
  }

  return parts.join(';');
}

//

export const compile = wrapForErrors(async function(opts: {
  filename: string;
  input: string;
  sourceType: ConstSourceType;
}): Promise<{
  cacheKey: string;
  code: string;
  sourceMap: SourceMap;
}> {
  const path = createAbsoluteFilePath(opts.filename);

  const project = await findProject(path);
  const cacheKey = await getCacheKey(opts.filename);

  const {handler} = getFileHandlerAssert(path, project.config);

  const sourceType =
    opts.sourceType === undefined ? handler.sourceType : opts.sourceType;

  const ast = parseJS({
    input: opts.input,
    path,
    syntax: handler.syntax,
    sourceType,
  });
  throwDiagnostics(ast.diagnostics);

  const res = await compiler.compile({
    ast,
    sourceText: opts.input,
    project: project.definition,
    options: {},
  });
  throwDiagnostics(res.diagnostics);

  // Build source map
  const sourceMapGenerator = new SourceMapGenerator({});
  for (const mapping of res.mappings) {
    sourceMapGenerator.addMapping(mapping);
  }
  const sourceMap = sourceMapGenerator.toJSON();

  return {
    cacheKey,
    code: res.compiledCode,
    sourceMap,
  };
});

export function lint(filename: string, input: string) {
  filename;
  input;
}
