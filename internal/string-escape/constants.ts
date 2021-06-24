/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

export const TRIPLE_DOUBLE_QUOTE = '"""' as const;
export const DOUBLE_QUOTE = '"' as const;
export const TRIPLE_SINGLE_QUOTE = "'''" as const;
export const SINGLE_QUOTE = "'" as const;
export const TICK_QUOTE = "`" as const;

export type EscapeStringQuoteChar =
	| typeof TRIPLE_DOUBLE_QUOTE
	| typeof DOUBLE_QUOTE
	| typeof TRIPLE_SINGLE_QUOTE
	| typeof SINGLE_QUOTE
	| typeof TICK_QUOTE;
