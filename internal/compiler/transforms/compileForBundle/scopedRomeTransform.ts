/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {createVisitor, signals} from "@internal/compiler";
import {renameBindings} from "@internal/js-ast-utils";

export default createVisitor({
	name: "scopedRome",
	enter(path) {
		const {node, scope} = path;

		if (scope.node === node && scope.hasBinding("Rome")) {
			return signals.replace(
				renameBindings(path, new Map([["Rome", scope.generateUid("Rome")]])),
			);
		}

		return signals.retain;
	},
});
