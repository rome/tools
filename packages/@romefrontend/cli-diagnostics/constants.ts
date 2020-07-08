/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export const GUTTER = " \u2502 ";
export const CODE_FRAME_INDENT = "  ";
export const CODE_FRAME_SELECTED_INDENT = `<error>\></error> `;

export const MAX_CODE_FRAME_LINES = 8;
export const HALF_MAX_CODE_FRAME_LINES = MAX_CODE_FRAME_LINES / 2;
export const CODE_FRAME_CONTEXT_LINES = 2;

// Constants that influence truncation
export const MAX_CODE_LENGTH = 500;
export const MAX_PATCH_LINES = 50;
export const MAX_LOG_LENGTH = 5_000;
