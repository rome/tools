/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment} from "@internal/ast";
import {Diagnostics} from "@internal/diagnostics";
import {NodeBase} from "@internal/parser-core";

export interface NodeBaseWithComments extends NodeBase {
	leadingComments?: Array<string>;
	trailingComments?: Array<string>;
	innerComments?: Array<string>;
}

export interface RootBase {
	comments: Array<AnyComment>;
	filename: string;
	diagnostics: Diagnostics;
	mtime: undefined | number;
	corrupt: boolean;
}
