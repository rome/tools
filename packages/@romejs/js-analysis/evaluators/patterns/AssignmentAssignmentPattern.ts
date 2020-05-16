/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	AnyNode,
	AssignmentAssignmentPattern,
	assignmentAssignmentPattern,
} from '@romejs/js-ast';

export default function AssignmentAssignmentPattern(node: AnyNode) {
	node = assignmentAssignmentPattern.assert(node);
	throw new Error('unimplemented');
}
