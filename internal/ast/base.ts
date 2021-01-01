/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyComment} from "@internal/ast";
import {DiagnosticIntegrity, Diagnostics} from "@internal/diagnostics";
import {NodeBase} from "@internal/parser-core";

export interface NodeBaseWithComments extends NodeBase {
	leadingComments?: string[];
	trailingComments?: string[];
	innerComments?: string[];
}

export interface RootBase {
	comments: AnyComment[];
	filename: string;
	diagnostics: Diagnostics;
	integrity: undefined | DiagnosticIntegrity;
	corrupt: boolean;
}
