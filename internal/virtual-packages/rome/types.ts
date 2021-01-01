/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

// These are copied from internal/codec-config/types.ts
export type VoidCallback = () => void | undefined;

export type AsyncVoidCallback = () =>
	| void
	| undefined
	| Promise<void | undefined>;
