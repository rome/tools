/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {consumeJSON} from '@romejs/codec-json';
import {VERSION} from './constants';
import {ROME_CONFIG_FILENAMES} from '@romejs/project';
import {
  HOME_PATH,
  AbsoluteFilePath,
  createAbsoluteFilePath,
  TEMP_PATH,
} from '@romejs/path';
import {readFileTextSync, existsSync} from '@romejs/fs';

export type UserConfig = {cachePath: AbsoluteFilePath};

export const DEFAULT_USER_CONFIG: UserConfig = {
  cachePath: TEMP_PATH.append(`rome-${VERSION}`),
};

export function loadUserConfig(): UserConfig {
  for (const configFilename of ROME_CONFIG_FILENAMES) {
    const configPath = HOME_PATH.append(['.config', configFilename]);

    if (!existsSync(configPath)) {
      continue;
    }

    const configFile = readFileTextSync(configPath);
    const consumer = consumeJSON({
      path: configPath,
      input: configFile,
    });

    const userConfig: UserConfig = {
      ...DEFAULT_USER_CONFIG,
    };

    if (consumer.has('cachePath')) {
      userConfig.cachePath = createAbsoluteFilePath(
        consumer.get('cachePath').asString(),
      );
    }

    consumer.enforceUsedProperties('config property');

    return userConfig;
  }

  return DEFAULT_USER_CONFIG;
}
