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
  AbsoluteFilePath,
  HOME_PATH,
  TEMP_PATH,
  createAbsoluteFilePath,
} from '@romejs/path';
import {existsSync, readFileTextSync} from '@romejs/fs';

export type UserConfig = {
  runtimeModulesPath: AbsoluteFilePath;
  cachePath: AbsoluteFilePath;
};

const VERSION_PATH = TEMP_PATH.append(`rome-${VERSION}`);

export const DEFAULT_USER_CONFIG: UserConfig = {
  runtimeModulesPath: VERSION_PATH.append('runtime'),
  cachePath: VERSION_PATH.append('cache'),
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

    if (consumer.has('runtimeModulesPath')) {
      userConfig.runtimeModulesPath = createAbsoluteFilePath(
        consumer.get('runtimeModulesPath').asString(),
      );
    }

    consumer.enforceUsedProperties('config property');

    return userConfig;
  }

  return DEFAULT_USER_CONFIG;
}
