/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export const ROME_CONFIG_PACKAGE_JSON_FIELD = 'rome';

export const ROME_CONFIG_FILENAME_VANILLA = 'rome.json';
export const ROME_CONFIG_FILENAME_EXTENSIONS = 'rome.rjson';

export const ROME_CONFIG_FILENAMES: Array<string> = [
  ROME_CONFIG_FILENAME_VANILLA,
  ROME_CONFIG_FILENAME_EXTENSIONS,
];

export const ROME_CONFIG_WARN_FILENAMES: Array<string> = [
  'romeconfig',
  'romerc',
  'rome.son',
  'rome.config.ts',
  'rome.config.js',
  'rome.config.json',
  'rome.config.rjson',
  'rome.config.son',
];

// Add dot versions
for (const basename of ROME_CONFIG_WARN_FILENAMES) {
  if (basename[0] !== '.') {
    ROME_CONFIG_WARN_FILENAMES.push('.' + basename);
  }
}
for (const filename of ROME_CONFIG_FILENAMES.slice()) {
  ROME_CONFIG_FILENAMES.push(`.${filename}`);
}
