/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import packageJson from '../package.json';
import os = require('os');
import {TEMP_PATH, createAbsoluteFilePath} from '@romejs/path';

export const CHILD_ARGS = ['--max-old-space-size=8192'];

// @ts-ignore: this will be wrong if we weren't the entry node script
export const BIN = createAbsoluteFilePath(process.mainModule.filename);
export const MAP = BIN.addExtension('.map');

const MEGABYTE = 10000;

export const MAX_MASTER_BYTES_BEFORE_WORKERS = 0.5 * MEGABYTE;

export const MAX_WORKER_BYTES_BEFORE_ADD = 1 * MEGABYTE;

const CPU_COUNT: number = os.cpus().length;
export const MAX_WORKER_COUNT = Math.min(CPU_COUNT, 4);

export const VERSION = String(packageJson.version);

export const SOCKET_PATH = TEMP_PATH.append(`rome-${VERSION}.sock`);

export const CLI_SOCKET_PATH = TEMP_PATH.append(`rome-wait-${VERSION}.sock`);

export const TEST_FOLDER_NAME = '__rtests__';
export const MOCKS_FOLDER_NAME = '__rmocks__';
