/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {formatAnsi} from '@romejs/string-ansi';

export const GUTTER = ' \u2502 ';
export const CODE_FRAME_INDENT = '  ';
export const CODE_FRAME_SELECTED_INDENT = `${formatAnsi.red('>')} `;
export const FILENAME_INDENT = '  ';

export const MAX_CODE_FRAME_LINES = 8;
export const HALF_MAX_CODE_FRAME_LINES = MAX_CODE_FRAME_LINES / 2;
export const CODE_FRAME_CONTEXT_LINES = 2;
