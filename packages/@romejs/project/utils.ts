/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Consumer} from '@romejs/consume';
import {PathPatterns, parsePathPattern} from '@romejs/path-match';
import {AbsoluteFilePath, AbsoluteFilePathSet} from '@romejs/path';
import {ProjectConfigMeta, ProjectConfigMetaHard} from './types';
import {ROME_CONFIG_FILENAMES, ROME_CONFIG_FOLDER} from './constants';

export function assertHardMeta(meta: ProjectConfigMeta): ProjectConfigMetaHard {
  const {configPath, projectFolder: folder, consumer} = meta;
  if (configPath === undefined || folder === undefined || consumer === undefined) {
    throw new Error('This is not a disk project');
  }

  return {
    ...meta,
    configPath,
    consumer,
    projectFolder: folder,
  };
}

export function arrayOfStrings(consumer: Consumer): Array<string> {
  if (consumer.exists()) {
    return consumer.asArray().map((item) => item.asString());
  } else {
    return [];
  }
}

export function arrayOfPatterns(consumer: Consumer): PathPatterns {
  // TODO consumer.handleThrownDiagnostics
  return consumer.asArray().map((item) => {
    return parsePathPattern({
      path: consumer.filename,
      input: item.asString(),
      offsetPosition: item.getLocation('inner-value').start,
    });
  });
}

export function mergeArrays<T>(
  a: undefined | Array<T>,
  b: undefined | Array<T>,
): undefined | Array<T> {
  if (a === undefined) {
    return a;
  }

  if (b === undefined) {
    return a;
  }

  return [...a, ...b];
}

export function mergeAbsoluteFilePathSets(
  a: undefined | AbsoluteFilePathSet,
  b: undefined | AbsoluteFilePathSet,
): undefined | AbsoluteFilePathSet {
  if (a === undefined) {
    return a;
  }

  if (b === undefined) {
    return a;
  }

  return new AbsoluteFilePathSet([...a, ...b]);
}

// Get an array of possible files in parent folders that will cause a project cache invalidation
export function getParentConfigDependencies(
  path: AbsoluteFilePath,
): AbsoluteFilePathSet {
  const deps: AbsoluteFilePathSet = new AbsoluteFilePathSet();

  for (const folder of path.getChain()) {
    deps.add(folder.append('package.json'));

    for (const configFilename of ROME_CONFIG_FILENAMES) {
      deps.add(folder.append(configFilename));
      deps.add(folder.append(ROME_CONFIG_FOLDER).append(configFilename));
    }
  }

  return deps;
}
