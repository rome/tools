/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {naturalCompare} from '@romejs/string-utils';
import {CoverageFolder} from './types';

export function sortMapKeys<T>(map: Map<string, T>): Map<string, T> {
  const sortedKeys = Array.from(map.keys()).sort(naturalCompare);
  const newMap: Map<string, T> = new Map();
  for (const key of sortedKeys) {
    const val = map.get(key);
    if (val === undefined) {
      throw new Error('Expected value');
    }
    newMap.set(key, val);
  }
  return newMap;
}

export function formatPercent(num: number): string {
  const str = String(Math.floor(num));
  if (num > 80) {
    return `<success>${str}</success>`;
  } else if (num > 40) {
    return `<warn>${str}</warn>`;
  } else {
    return `<error>${str}</error>`;
  }
}

export function percentInsideCoverageFolder(folder: CoverageFolder): {
  functions: number;
  branches: number;
  lines: number;
} {
  let totalFiles = 0;
  let functions = 0;
  let branches = 0;
  let lines = 0;

  const folders = [folder];
  while (folders.length > 0) {
    const folder = folders.shift();
    if (folder === undefined) {
      throw new Error('folders.length has already been validated');
    }

    for (const file of folder.files.values()) {
      totalFiles++;
      functions += file.functions.percent;
      branches += file.branches.percent;
      lines += file.lines.percent;
    }

    for (const subFolder of folder.folders.values()) {
      folders.push(subFolder);
    }
  }

  return {
    functions: totalFiles === 0 ? 100 : functions / totalFiles,
    branches: totalFiles === 0 ? 100 : branches / totalFiles,
    lines: totalFiles === 0 ? 100 : lines / totalFiles,
  };
}
