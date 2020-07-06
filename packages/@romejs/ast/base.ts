/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment} from "@romejs/ast";
import {Diagnostics} from "@romejs/diagnostics";
import {NodeBase} from "@romejs/parser-core";

export type NodeBaseWithComments = NodeBase & {
	leadingComments?: Array<string>;
	trailingComments?: Array<string>;
	innerComments?: Array<string>;
};

export type RootBase = {
	comments: Array<AnyComment>;
	filename: string;
	diagnostics: Diagnostics;
	mtime: undefined | number;
	corrupt: boolean;
};
