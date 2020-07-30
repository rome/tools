/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {markup} from "@internal/markup";

export const GUTTER = markup`<emphasis> \u2502 </emphasis>`;

export const CODE_FRAME_INDENT = "  ";
export const CODE_FRAME_SELECTED_INDENT = markup`<error>></error> `;

export const MAX_CODE_FRAME_LINES = 8;
export const HALF_MAX_CODE_FRAME_LINES = MAX_CODE_FRAME_LINES / 2;
export const CODE_FRAME_CONTEXT_LINES = 2;

// Constants that influence truncation
export const MAX_CODE_LENGTH = 5_000;
export const MAX_CODE_LINES = 150;
export const MAX_PATCH_LINES = 150;
export const MAX_LOG_LENGTH = 5_000;
