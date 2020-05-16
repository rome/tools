/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {AnyNode, Program, program} from '@romejs/js-ast';
import BlockStatement from '../statements/BlockStatement';

export default function Program(node: AnyNode, scope: Scope) {
	node = program.assert(node);
	BlockStatement(node, scope);
}
