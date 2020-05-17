/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {NodeBase} from "@romejs/parser-core";

export type JSNodeBase = NodeBase & {
	leadingComments?: Array<string>;
	trailingComments?: Array<string>;
	innerComments?: Array<string>;
};
