/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import mod = require('module');
import {AbsoluteFilePath, AbsoluteFilePathMap, CWD_PATH} from '@romejs/path';

// rome-suppress lint/noExplicitAny
type RequireFunction = (name: string) => any;

const requires: AbsoluteFilePathMap<RequireFunction> = new AbsoluteFilePathMap();

function getRequire(folder: AbsoluteFilePath = CWD_PATH): RequireFunction {
  const cached = requires.get(folder);
  if (cached !== undefined) {
    return cached;
  }

  const filename = folder.join();
  const req = mod.createRequire
    ? mod.createRequire(filename)
    : mod.createRequireFromPath(filename);
  requires.set(folder, req);
  return req;
}

// rome-suppress lint/noExplicitAny
export function requireGlobal(name: string, folder?: AbsoluteFilePath): any {
  return getRequire(folder)(name);
}
